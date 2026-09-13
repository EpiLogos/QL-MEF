#ifndef QL_CONTINUOUS_FIELD_HPP
#define QL_CONTINUOUS_FIELD_HPP
// Native C++ continuation of supplied M2 modes, not a second symbolic engine.
// Control methods and rendering have one host-owned serial execution thread.
// Prepare/change topology off the audio callback. No provider/JSON/graph calls
// or allocation occur in render_audio / write_targets. Callers own output buffers.
#include <ql/coupled_clock.h>
#include <array>
#include <cmath>
#include <complex>
#include <cstdint>
#include <limits>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

namespace ql {
inline constexpr const char *continuous_contract = "ql.continuous-field/v1";
inline constexpr double pi = 3.141592653589793238462643383279502884;
using Complex = std::complex<double>;
using Vec3 = std::array<double, 3>;
struct Mode {
    std::string reference, source_coordinate;
    double frequency_hz, damping_per_second;
    Complex amplitude_metres, drive_metres_per_second;
    double audio_gain_per_metre;
};
enum class ClockAttachment : unsigned { Fixed = 0, Inscription = 1, Lensing = 2 };
struct Sample {
    std::uint64_t identity;
    std::string constituent;
    ClockAttachment attachment;
    Vec3 rest_metres;
    // One dimensionless vector shape-function per mode. Supplied by the actual
    // mesh/raster/material provider; neither guessed from an M address nor a
    // new point-cloud sampler. Signed functions preserve nodes/cancellation.
    std::vector<Vec3> mode_shapes;
};
struct ContinuationInput {
    std::string event_ref, subject_ref, geometry_ref, material_ref, model_ref;
    std::uint64_t generation;
    unsigned sample_rate;
    QL_CoupledClock clock;
    // Half-degree driver units per SECOND before the native rational axis rates.
    std::uint32_t driver_numerator, driver_denominator;
    std::vector<Mode> modes;
    std::vector<Sample> samples;
};
struct ContinuationReceipt {
    std::uint64_t generation, samples_elapsed;
    QL_CoupledClock clock;
    double last_unmuted_sample;
};
inline bool finite(Complex x) { return std::isfinite(x.real()) && std::isfinite(x.imag()); }
inline void require(bool condition, const char *message) {
    if (!condition) throw std::invalid_argument(message);
}
// Stable expm1(z)/z including the equal/zero-frequency, undamped limit. This
// analytic local solution does not divide differences between mode frequencies.
inline Complex phi1(Complex z) {
    if (std::abs(z) < 1e-4) {
        Complex sum(1, 0), term(1, 0);
        for (unsigned k = 1; k <= 12; ++k) { term *= z / double(k + 1); sum += term; }
        return sum;
    }
    return (std::exp(z) - Complex(1, 0)) / z;
}
class ContinuousField {
    struct Prepared { Mode input; Complex step, forcing, state; };
    ContinuationInput source_;
    std::vector<Prepared> modes_;
    std::uint64_t elapsed_ = 0, driver_remainder_ = 0;
    double last_ = 0;
    static void text(const std::string &s) {
        require(!s.empty() && s.size() <= 2048, "missing/excessive field reference");
        for (unsigned char c : s) require(c >= 32 && c != 127, "control character in reference");
    }
    Prepared prepare(const Mode &m) const {
        text(m.reference); text(m.source_coordinate);
        auto node = ql_m_live_resolve(m.source_coordinate.c_str());
        require(node && node->root_position == 2, "mode needs an existing exact M2 coordinate");
        require(std::isfinite(m.frequency_hz) && m.frequency_hz >= 0 &&
                m.frequency_hz < source_.sample_rate * 0.45, "mode outside qualified audio band");
        require(std::isfinite(m.damping_per_second) && m.damping_per_second >= 0 &&
                m.damping_per_second <= 1e6, "invalid damping per second");
        require(finite(m.amplitude_metres) && finite(m.drive_metres_per_second) &&
                std::abs(m.amplitude_metres) <= 1e6 && std::abs(m.drive_metres_per_second) <= 1e6 &&
                std::isfinite(m.audio_gain_per_metre) && std::abs(m.audio_gain_per_metre) <= 1e6,
                "nonfinite/excessive modal amplitude, forcing or output gain");
        const Complex rate(-m.damping_per_second, 2 * pi * m.frequency_hz);
        const Complex z = rate / double(source_.sample_rate);
        return {m, std::exp(z), (m.drive_metres_per_second / double(source_.sample_rate)) * phi1(z),
                m.amplitude_metres};
    }
public:
    explicit ContinuousField(ContinuationInput input) : source_(std::move(input)) {
        for (const auto *s : {&source_.event_ref, &source_.subject_ref, &source_.geometry_ref,
                              &source_.material_ref, &source_.model_ref}) text(*s);
        require(source_.sample_rate >= 8000 && source_.sample_rate <= 192000, "unsupported sample rate");
        require(ql_clock_validate(&source_.clock) == QL_CLOCK_OK, "invalid native coupled clock");
        require(source_.driver_denominator > 0 && source_.driver_denominator <= 1000000 &&
                source_.driver_numerator <= 1000000, "invalid clock/audio rate relation");
        require(!source_.modes.empty() && source_.modes.size() <= 4096, "invalid mode count");
        require(!source_.samples.empty() && source_.samples.size() <= 1048576 &&
                source_.samples.size() <= 16777216 / source_.modes.size(), "shape-function budget exceeded");
        modes_.reserve(source_.modes.size());
        for (std::size_t i = 0; i < source_.modes.size(); ++i) {
            for (std::size_t j = 0; j < i; ++j)
                require(source_.modes[i].reference != source_.modes[j].reference, "duplicate mode identity");
            modes_.push_back(prepare(source_.modes[i]));
        }
        for (std::size_t i = 0; i < source_.samples.size(); ++i) {
            const auto &sample = source_.samples[i];
            text(sample.constituent);
            require(ql_m_live_resolve(sample.constituent.c_str()), "unknown sample constituent");
            // Ordered monotone IDs provide linear validation without quadratic
            // sample lookup or changing the supplied correspondence.
            require((i == 0 || source_.samples[i - 1].identity < sample.identity) &&
                    sample.identity <= 9007199254740991ULL, "sample identities must be unique/ordered/exact");
            require(unsigned(sample.attachment) <= 2, "unknown clock attachment");
            require(sample.mode_shapes.size() == modes_.size(), "shape/mode basis mismatch");
            for (double x : sample.rest_metres)
                require(std::isfinite(x) && std::abs(x) <= 1e6, "invalid rest position in metres");
            for (const auto &shape : sample.mode_shapes)
                for (double x : shape)
                    require(std::isfinite(x) && std::abs(x) <= 1e6, "invalid dimensionless shape coefficient");
        }
    }
    const ContinuationInput &source() const noexcept { return source_; }
    const std::vector<Sample> &samples() const noexcept { return source_.samples; }
    Complex amplitude(std::size_t i) const { return modes_.at(i).state; }
    ContinuationReceipt receipt() const noexcept { return {source_.generation, elapsed_, source_.clock, last_}; }
    // Same material/sample topology only. Initial amplitude is not reapplied on
    // continuation: new frequency/drive/damping takes effect from resident state.
    // A deliberate state replacement is an explicit operation, never a reseed.
    void replace_modes(std::uint64_t expected, const std::vector<Mode> &modes, bool replace_state = false) {
        require(expected == source_.generation && expected != UINT64_MAX, "stale/overflow field generation");
        require(modes.size() == modes_.size(), "structural mode change requires explicit new topology");
        std::vector<Prepared> prepared; prepared.reserve(modes.size());
        for (std::size_t i = 0; i < modes.size(); ++i) {
            require(modes[i].reference == modes_[i].input.reference &&
                    modes[i].source_coordinate == modes_[i].input.source_coordinate,
                    "mode identity/order/source changed during continuation");
            auto p = prepare(modes[i]);
            if (!replace_state) p.state = modes_[i].state;
            prepared.push_back(std::move(p));
        }
        auto retained = modes; // all allocations complete before commit
        source_.modes.swap(retained); modes_.swap(prepared); ++source_.generation;
    }
    void set_axis(std::uint64_t expected, unsigned axis, QL_PhaseLift phase) {
        require(expected == source_.generation && expected != UINT64_MAX, "stale/overflow field generation");
        QL_CoupledClock next{};
        require(ql_clock_set_axis(&source_.clock, source_.clock.generation, axis, phase, &next) == QL_CLOCK_OK,
                "native clock rejected independent phase operation");
        source_.clock = next; ++source_.generation;
    }
    // One buffer is one bounded block. Validation/preflight precedes mutation.
    // Audio gain is explicit linear output gain, not a claim of sound pressure.
    // Muting affects output only. Host must not integrate again per visual view.
    bool render_audio(float *output, std::size_t frames, bool muted = false) noexcept {
        if (!output || frames > 8192 || elapsed_ > UINT64_MAX - frames) return false;
        if (frames == 0) return true;
        const std::uint64_t denominator = std::uint64_t(source_.sample_rate) * source_.driver_denominator;
        const std::uint64_t total = driver_remainder_ + std::uint64_t(frames) * source_.driver_numerator;
        const auto steps = static_cast<std::int64_t>(total / denominator);
        QL_CoupledClock next = source_.clock;
        if (steps != 0 && ql_clock_advance(&source_.clock, source_.clock.generation, steps, &next) != QL_CLOCK_OK)
            return false;
        // Conservative finite-state budget, checked before touching either
        // state or output. Covers cumulative forcing and roundoff growth without
        // pretending an unbounded integration is numerically safe forever.
        for (const auto &m : modes_) {
            const double bound = (std::abs(m.state) + frames * std::abs(m.forcing)) * 1.000000001;
            if (!std::isfinite(bound) || bound > 1e12) return false;
        }
        for (std::size_t sample = 0; sample < frames; ++sample) {
            double value = 0;
            for (auto &m : modes_) {
                m.state = m.step * m.state + m.forcing;
                value += m.state.real() * m.input.audio_gain_per_metre;
            }
            last_ = value;
            output[sample] = muted ? 0.0f : static_cast<float>(value);
        }
        elapsed_ += frames; source_.clock = next; driver_remainder_ = total % denominator;
        return true;
    }
    // Target displacement, NOT resident GPU position/velocity integration.
    // Native clock angles act once on named sample attachments. Winding remains
    // in the receipt even though the rendered rotation is non-injective.
    bool write_targets(float *xyz, std::size_t capacity, double presentation_units_per_metre = 1) const noexcept {
        if (!xyz || capacity < source_.samples.size() * 3 ||
            !std::isfinite(presentation_units_per_metre) || presentation_units_per_metre <= 0 ||
            presentation_units_per_metre > 1e6) return false;
        const std::array<double, 3> angles{0, source_.clock.inscription.half_degrees * pi / 360,
                                            source_.clock.lensing.half_degrees * pi / 360};
        for (std::size_t i = 0; i < source_.samples.size(); ++i) {
            const auto &sample = source_.samples[i];
            Vec3 p = sample.rest_metres;
            for (std::size_t j = 0; j < modes_.size(); ++j)
                for (unsigned axis = 0; axis < 3; ++axis)
                    p[axis] += sample.mode_shapes[j][axis] * modes_[j].state.real();
            const double c = std::cos(angles[unsigned(sample.attachment)]), s = std::sin(angles[unsigned(sample.attachment)]);
            xyz[i * 3] = static_cast<float>((c * p[0] - s * p[1]) * presentation_units_per_metre);
            xyz[i * 3 + 1] = static_cast<float>((s * p[0] + c * p[1]) * presentation_units_per_metre);
            xyz[i * 3 + 2] = static_cast<float>(p[2] * presentation_units_per_metre);
        }
        return true;
    }
};
} // namespace ql
#endif
