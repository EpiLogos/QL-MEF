#ifndef QL_CONTINUOUS_FIELD_HPP
#define QL_CONTINUOUS_FIELD_HPP
// Native C++ continuation of supplied M2 modes, not a second symbolic engine.
// Control methods and rendering have one host-owned serial execution thread.
// Prepare/change topology off the audio callback. No provider/JSON/graph calls
// or allocation occur in render_audio / write_targets. Callers own output buffers.
#include <ql/coupled_clock.h>
#include <algorithm>
#include <array>
#include <cmath>
#include <complex>
#include <cstddef>
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
// One played excitation act: an impulse added to a named resident mode. The
// mode reference is the supplied M2 identity; the amplitude is modal metres.
struct Strike {
    std::string mode_ref;
    Complex amplitude_metres;
};
// The most played acts one continuation holds admitted but not yet landed. A player schedules ahead by
// its presentation lookahead, not a whole performance; the bound keeps the pending set finite.
inline constexpr std::size_t max_pending_strikes = 256;
// The most steps of the pulse one sample may carry: the M1′ session's fastest rate (40,000 steps a
// second) at the lowest supported sample rate (8 kHz) is five.
inline constexpr double max_carrier_steps_per_sample = 8;
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
// One bounded printable reference law for field, mode, sample and shape refs.
inline void reference(const std::string &s) {
    require(!s.empty() && s.size() <= 2048, "missing/excessive field reference");
    for (unsigned char c : s) require(c >= 32 && c != 127, "control character in reference");
}
class ContinuousField {
    struct Prepared { Mode input; Complex step, forcing, state; };
    // A strike admitted for a later sample: its impulses by prepared mode index, landing as the cursor
    // reaches `at`. Kept in sample order (admission order within a sample); [0, landed_) have landed.
    struct Pending { std::uint64_t at; std::vector<std::pair<std::size_t, Complex>> impulses; };
    // The M1′ pulse's drive of the body between strikes (the Torus's Carrier, O-I #850; DLW manual
    // §21.2). The pulse stands at `anchor_steps` at sample `anchor_samples` and runs at
    // `steps_per_second` since, the session's own law (m1_prime::Session::steps_at); `reached` is
    // the whole steps already landed. One impulse per prepared mode, zero where the pulse drives none.
    struct Carrier {
        double steps_per_second = 0, anchor_steps = 0;
        std::uint64_t anchor_samples = 0;
        double reached = 0;
        std::vector<Complex> impulses;
        double steps(std::uint64_t at, unsigned rate) const noexcept {
            const double elapsed = at > anchor_samples ? double(at - anchor_samples) : 0;
            return anchor_steps + elapsed / double(rate) * steps_per_second;
        }
    };
    ContinuationInput source_;
    std::vector<Prepared> modes_;
    std::vector<Pending> pending_;
    Carrier carrier_;
    // The output gains the receiver last moved from (O-I 856): the next rendered block reaches the gains in force from these,
    // linearly, and then they are left. Empty while no move waits to be reached.
    std::vector<double> heard_from_;
    std::size_t landed_ = 0;
    std::uint64_t elapsed_ = 0, driver_remainder_ = 0;
    double last_ = 0;
    void land(const Pending &act) noexcept {
        for (const auto &[mode, impulse] : act.impulses) modes_[mode].state += impulse;
    }
    Prepared prepare(const Mode &m) const {
        reference(m.reference); reference(m.source_coordinate);
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
                              &source_.material_ref, &source_.model_ref}) reference(*s);
        require(source_.sample_rate >= 8000 && source_.sample_rate <= 192000, "unsupported sample rate");
        require(ql_clock_validate(&source_.clock) == QL_CLOCK_OK, "invalid native coupled clock");
        require(source_.driver_denominator > 0 && source_.driver_denominator <= 1000000 &&
                source_.driver_numerator <= 1000000, "invalid clock/audio rate relation");
        require(!source_.modes.empty() && source_.modes.size() <= 4096, "invalid mode count");
        require(!source_.samples.empty() && source_.samples.size() <= 1048576 &&
                source_.samples.size() <= 16777216 / source_.modes.size(), "shape-function budget exceeded");
        modes_.reserve(source_.modes.size());
        pending_.reserve(max_pending_strikes);
        for (std::size_t i = 0; i < source_.modes.size(); ++i) {
            for (std::size_t j = 0; j < i; ++j)
                require(source_.modes[i].reference != source_.modes[j].reference, "duplicate mode identity");
            modes_.push_back(prepare(source_.modes[i]));
        }
        for (std::size_t i = 0; i < source_.samples.size(); ++i) {
            const auto &sample = source_.samples[i];
            reference(sample.constituent);
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
    // Explicit nodal re-reading: the same samples and modal voices receive a newly
    // supplied shape basis (shapes[i] replaces sample i's mode_shapes). Sample
    // identity/order/constituent/attachment/rest, mode identity, resident z and
    // clock are unchanged, so audio is unaffected; only targets move. Validation
    // and allocation precede the first mutation. Never a silent reseed.
    void replace_shapes(std::uint64_t expected, const std::vector<std::vector<Vec3>> &shapes) {
        require(expected == source_.generation && expected != UINT64_MAX, "stale/overflow field generation");
        require(shapes.size() == source_.samples.size(), "shape replacement must keep the sample basis");
        for (const auto &sample : shapes) {
            require(sample.size() == modes_.size(), "shape/mode basis mismatch");
            for (const auto &shape : sample)
                for (double x : shape)
                    require(std::isfinite(x) && std::abs(x) <= 1e6, "invalid dimensionless shape coefficient");
        }
        auto next = shapes; // all allocations complete before commit
        for (std::size_t i = 0; i < next.size(); ++i) source_.samples[i].mode_shapes.swap(next[i]);
        ++source_.generation;
    }
    void set_axis(std::uint64_t expected, unsigned axis, QL_PhaseLift phase) {
        require(expected == source_.generation && expected != UINT64_MAX, "stale/overflow field generation");
        QL_CoupledClock next{};
        require(ql_clock_set_axis(&source_.clock, source_.clock.generation, axis, phase, &next) == QL_CLOCK_OK,
                "native clock rejected independent phase operation");
        source_.clock = next; ++source_.generation;
    }
    // Played excitation: each named mode's resident state receives the supplied
    // impulse (a pluck/mallet displacement in metres). One strike act is one
    // generation. Validation and the render-budget preflight complete before
    // any state moves, so a refused act leaves the body exactly as it stood.
    // The clock, sample basis, mode identities and z are untouched.
    void strike(const std::vector<Strike> &strikes) { strike(strikes, elapsed_); }
    // The same act at a sample of the continuation (O-I #809): admitted now, one
    // generation, and landed by render_audio exactly before sample
    // `at_samples_elapsed` is rendered (at the cursor, it lands now). A sample
    // behind the cursor is sound already rendered and is refused, never moved.
    void strike(const std::vector<Strike> &strikes, std::uint64_t at_samples_elapsed) {
        require(at_samples_elapsed >= elapsed_,
                "a strike's sample is behind the continuation cursor: that sound is already rendered");
        require(at_samples_elapsed == elapsed_ || pending_.size() - landed_ < max_pending_strikes,
                "too many strikes wait for their samples");
        require(!strikes.empty() && strikes.size() <= modes_.size(), "invalid strike count");
        std::vector<Prepared *> struck;
        struck.reserve(strikes.size());
        for (const auto &act : strikes) {
            reference(act.mode_ref);
            require(finite(act.amplitude_metres) && std::abs(act.amplitude_metres) <= 1e6,
                    "invalid strike amplitude");
            require(!std::any_of(struck.begin(), struck.end(), [&](const Prepared *p) {
                        return p->input.reference == act.mode_ref;
                    }),
                    "one strike act names a mode twice");
            auto found = std::find_if(modes_.begin(), modes_.end(), [&](const Prepared &p) {
                return p.input.reference == act.mode_ref;
            });
            require(found != modes_.end(), "strike names no prepared mode");
            struck.push_back(&*found);
        }
        if (at_samples_elapsed > elapsed_) {
            // Allocation completes before the commit; render_audio only reads it.
            Pending act{at_samples_elapsed, {}};
            act.impulses.reserve(strikes.size());
            for (std::size_t i = 0; i < strikes.size(); ++i)
                act.impulses.emplace_back(std::size_t(struck[i] - modes_.data()), strikes[i].amplitude_metres);
            pending_.erase(pending_.begin(), pending_.begin() + std::ptrdiff_t(landed_));
            landed_ = 0;
            auto place = std::upper_bound(pending_.begin(), pending_.end(), at_samples_elapsed,
                                          [](std::uint64_t at, const Pending &p) { return at < p.at; });
            pending_.insert(place, std::move(act));
            ++source_.generation;
            return;
        }
        for (std::size_t i = 0; i < strikes.size(); ++i) {
            const double bound = (std::abs(struck[i]->state) + std::abs(strikes[i].amplitude_metres)) * 1.000000001;
            require(std::isfinite(bound) && bound <= 1e12, "strike exceeds the render state budget");
        }
        for (std::size_t i = 0; i < strikes.size(); ++i) struck[i]->state += strikes[i].amplitude_metres;
        ++source_.generation;
    }
    // Strikes admitted for a later sample that have not landed yet.
    std::size_t pending_strikes() const noexcept { return pending_.size() - landed_; }
    // The body as it rang at a checkpoint, set again (O-I 855): every prepared mode's resident state
    // (modal metres) in mode order, at the cursor. The voices, their shapes, the clock, the cursor, the
    // drive and the acts admitted for later samples stay; the next render continues from these states.
    // One body act: one generation. All validation precedes the commit.
    void set_state(std::uint64_t expected, const std::vector<Complex> &states) {
        require(expected == source_.generation && expected != UINT64_MAX, "stale/overflow field generation");
        require(states.size() == modes_.size(), "state/mode basis mismatch");
        for (const auto &state : states)
            require(finite(state) && std::abs(state) <= 1e6, "invalid resident modal state");
        for (std::size_t i = 0; i < modes_.size(); ++i) modes_[i].state = states[i];
        ++source_.generation;
    }
    // The pulse's drive of the body (the Torus's Carrier, O-I #850): from the cursor on, each step
    // of the M1′ pulse adds `impulses[i]` (modal metres) to prepared mode i, landing exactly before
    // the first sample whose steps reach it, as a strike at that sample would. The pulse is the
    // session's: `anchor_steps` at `anchor_samples` (at or behind the cursor), then `steps_per_second`.
    // A step the pulse reached before the cursor's sample does not land again. Zero rate or zero impulses is
    // no drive. Performance setting, not a body act: generation, cursor, clock and state stay. All
    // validation and allocation precede the commit; render_audio only reads the drive.
    void set_carrier(double steps_per_second, std::uint64_t anchor_samples, double anchor_steps,
                     const std::vector<Complex> &impulses) {
        require(std::isfinite(steps_per_second) && steps_per_second >= 0 &&
                    steps_per_second <= max_carrier_steps_per_sample * source_.sample_rate,
                "invalid carrier rate");
        require(anchor_samples <= elapsed_, "the carrier's pulse is anchored after the cursor");
        require(std::isfinite(anchor_steps) && anchor_steps >= 0 && anchor_steps <= 9007199254740991.0,
                "invalid carrier pulse phase");
        require(impulses.size() == modes_.size(), "carrier/mode basis mismatch");
        for (const auto &impulse : impulses)
            require(finite(impulse) && std::abs(impulse) <= 1e6, "invalid carrier impulse");
        Carrier next{steps_per_second, anchor_steps, anchor_samples, 0, impulses};
        // The whole steps already landed: those the pulse reached before the cursor's sample. At its
        // anchor the pulse stands on `anchor_steps`; a whole step it stands on there lands with that
        // sample, so setting the drive again on the same pulse neither loses nor repeats a step.
        next.reached = elapsed_ > anchor_samples ? std::floor(next.steps(elapsed_ - 1, source_.sample_rate))
                                                 : std::ceil(anchor_steps) - 1;
        require(next.reached <= 9007199254740991.0, "carrier pulse beyond exact steps");
        std::swap(carrier_, next);
    }
    // Where the body is heard from (the Torus's receiver, O-I 856; QL-MEF #289): one output gain per prepared mode, in mode order.
    // A receiving setting, not a body act: the states, clock, cursor and generation stand, and so does every mode's dynamics. The
    // next rendered block reaches the new gains linearly from those it was heard by, so a move never clicks; gains equal to those in
    // force change no sample. Validation precedes the first mutation.
    void set_gains(const std::vector<double> &gains) {
        require(gains.size() == modes_.size(), "gain/mode basis mismatch");
        for (double gain : gains) require(std::isfinite(gain) && std::abs(gain) <= 1e6, "nonfinite/excessive output gain");
        std::vector<double> from(modes_.size());
        for (std::size_t i = 0; i < modes_.size(); ++i)
            from[i] = heard_from_.empty() ? modes_[i].input.audio_gain_per_metre : heard_from_[i];
        bool moved = false;
        for (std::size_t i = 0; i < modes_.size(); ++i) moved = moved || from[i] != gains[i];
        for (std::size_t i = 0; i < modes_.size(); ++i) {
            modes_[i].input.audio_gain_per_metre = gains[i];
            source_.modes[i].audio_gain_per_metre = gains[i];
        }
        if (moved) heard_from_.swap(from);
        else heard_from_.clear();
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
        // Strikes landing in this block may all reach any one mode.
        const std::uint64_t end = elapsed_ + frames;
        double landing = 0;
        for (std::size_t i = landed_; i < pending_.size() && pending_[i].at <= end; ++i)
            for (const auto &impulse : pending_[i].impulses) landing += std::abs(impulse.second);
        // The pulse's steps that land in this block (the carrier), each on its driven modes.
        const bool driving = carrier_.steps_per_second > 0 && !carrier_.impulses.empty();
        const double kicks = driving ? std::floor(carrier_.steps(end, source_.sample_rate)) - carrier_.reached : 0;
        for (std::size_t i = 0; i < modes_.size(); ++i) {
            const auto &m = modes_[i];
            const double drive = driving ? kicks * std::abs(carrier_.impulses[i]) : 0;
            const double bound = (std::abs(m.state) + frames * std::abs(m.forcing) + landing + drive) * 1.000000001;
            if (!std::isfinite(bound) || bound > 1e12) return false;
        }
        // A pending strike lands exactly before its sample is rendered; one at
        // the block's end lands as the cursor reaches it. A step of the pulse
        // lands exactly before the first sample whose steps reach it.
        std::size_t due = landed_;
        for (std::size_t sample = 0; sample < frames; ++sample) {
            while (due < pending_.size() && pending_[due].at == elapsed_ + sample) land(pending_[due++]);
            if (driving) {
                const double reached = std::floor(carrier_.steps(elapsed_ + sample, source_.sample_rate));
                if (reached > carrier_.reached) {
                    const double steps = reached - carrier_.reached;
                    for (std::size_t i = 0; i < modes_.size(); ++i) modes_[i].state += steps * carrier_.impulses[i];
                    carrier_.reached = reached;
                }
            }
            double value = 0;
            if (heard_from_.empty()) {
                for (auto &m : modes_) {
                    m.state = m.step * m.state + m.forcing;
                    value += m.state.real() * m.input.audio_gain_per_metre;
                }
            } else {
                // The block a receiver's move is reached in: each gain from the one it was heard by to the one in force.
                const double reached = double(sample + 1) / double(frames);
                for (std::size_t i = 0; i < modes_.size(); ++i) {
                    auto &m = modes_[i];
                    m.state = m.step * m.state + m.forcing;
                    value += m.state.real() * (heard_from_[i] + (m.input.audio_gain_per_metre - heard_from_[i]) * reached);
                }
            }
            last_ = value;
            output[sample] = muted ? 0.0f : static_cast<float>(value);
        }
        while (due < pending_.size() && pending_[due].at == end) land(pending_[due++]);
        landed_ = due;
        heard_from_.clear();
        elapsed_ = end; source_.clock = next; driver_remainder_ = total % denominator;
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
