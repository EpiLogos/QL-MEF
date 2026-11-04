#ifndef QL_TORUS_KEYS_HPP
#define QL_TORUS_KEYS_HPP
// The Torus's played-address law for a native audio host (O-I #849; DLW manual §21.2 "What strikes the Torus"). A Jankó key stands
// at one place of its helix; the pulse carries that place round the torus, and the key strikes the point it stands at now on the
// voices of the tuning in force, each voice by its own shape term at the point, summed over the keys of one act, times the key's
// velocity and Strike (the body's share of every strike). QL's owner holds the law (the scene instrument's `torus-play`:
// janko.rs `JankoSurface::point` and POINT_LAW, scene_field.rs `strike_at_tuned` and `chi`); this header is the same law on a host's
// audio thread, made off it from what the owner hands over (`ql.torus-native/v1`) and then allocation-free per act. It adds no
// voice, pitch, place or weight of its own: the key places, the voices' terms, Strike and the pulse are the owner's.
//
// What a key's release does: nothing to the body (it rings on with its own damping; the owner counts the notes). How the body is
// silenced: `ContinuousField::silence()`. A law stands until the owner's Torus moves (a setting, the event or its voices); the host
// takes it again then.
#include <ql/continuous_field.hpp>
#include <array>
#include <cmath>
#include <cstdint>
#include <vector>

namespace ql {
inline constexpr const char *torus_keys_contract = "ql.torus-keys/v1";
// The most keys one act strikes (m1_prime::POLYPHONY_MAX) and the surface's rows and column period (janko::ROWS, COLUMN_PERIOD).
inline constexpr std::size_t max_torus_keys = 32;
inline constexpr unsigned torus_rows = 6, torus_column_period = 6;
// The arcs of the ring of twelve (spanda_field::RING_SIZE) and of one sheet (RING_HALF).
inline constexpr double torus_ring_arcs = 12, torus_sheet_arcs = 6;

// One voice of the tuning in force: the prepared mode it sounds through and its surface term (scene_field.rs `Voice`).
struct KeyVoice {
    std::size_t mode;
    double longitude_radians, weight, phase_radians;
    unsigned m, n;
};
struct TorusKeyLaw {
    // Key (row, column mod 6)'s arc of the ring, `row * 6 + column % 6` (janko.rs `helix_place`): 6·helix + position.
    std::array<std::uint8_t, torus_rows * torus_column_period> arcs{};
    std::vector<KeyVoice> voices;
    // Strike, 0..1: the share of a key's velocity that excites the body.
    double strike = 0;
    // The pulse: `anchor_steps` at `anchor_samples`, then `steps_per_second` (m1_prime::Session::steps_at).
    std::uint64_t anchor_samples = 0;
    double anchor_steps = 0, steps_per_second = 0;
    unsigned sample_rate = 0;
    // The declared material policy's cap on a struck amplitude, metres (STRIKE_CAP_METRES).
    double cap_metres = 1.0;
};

class TorusKeys {
    TorusKeyLaw law_;
    std::vector<std::size_t> modes_;
    std::vector<Complex> impulses_;

    // A voice's shape term at (φ, θ), exactly as the owner writes it (scene_field.rs `chi`): no fused multiply-add, so the host
    // strikes the very impulses the owner would.
    static double chi(const KeyVoice &voice, double phi, double theta) noexcept {
#if defined(__clang__)
#pragma clang fp contract(off)
#endif
        phi = phi - voice.longitude_radians;
        const double m = voice.m, n = voice.n;
        const double a = voice.weight * std::cos(voice.phase_radians);
        const double b = voice.weight * std::sin(voice.phase_radians);
        return a * std::sin(m * phi / 2.0) * std::sin(n * theta / 2.0) + b * std::cos(m * phi / 2.0) * std::cos(n * theta / 2.0);
    }
    static double rem(double x, double ring) noexcept {
        const double r = std::fmod(x, ring);
        return r < 0.0 ? r + ring : r;
    }

public:
    // Made off the audio thread, against the field it plays: every voice a distinct prepared mode of that field, every arc on the
    // ring, the field's rate, a finite pulse, Strike within 0..1. A law that cannot stand on the field is refused in words.
    TorusKeys(TorusKeyLaw law, const ContinuousField &field) : law_(std::move(law)) {
        require(!law_.voices.empty(), "the tuning in force holds no voices to strike");
        require(law_.voices.size() <= field.source().modes.size(), "more voices than the field holds modes");
        for (std::size_t i = 0; i < law_.voices.size(); ++i) {
            const auto &voice = law_.voices[i];
            require(voice.mode < field.source().modes.size(), "a voice names a mode the field does not hold");
            for (std::size_t j = 0; j < i; ++j) require(law_.voices[j].mode != voice.mode, "a voice is named twice");
            require(std::isfinite(voice.longitude_radians) && std::isfinite(voice.weight) && std::isfinite(voice.phase_radians) &&
                        voice.m <= 255 && voice.n <= 255,
                    "a voice's surface term is not finite");
        }
        for (auto arc : law_.arcs) require(arc < torus_ring_arcs, "a key's arc lies off the ring of twelve");
        require(law_.sample_rate == field.source().sample_rate, "the law's pulse runs at another sample rate than the field");
        require(std::isfinite(law_.anchor_steps) && law_.anchor_steps >= 0 && std::isfinite(law_.steps_per_second) &&
                    law_.steps_per_second >= 0,
                "the pulse is not finite");
        require(std::isfinite(law_.strike) && law_.strike >= 0 && law_.strike <= 1, "Strike is a share of 0 to 1");
        require(std::isfinite(law_.cap_metres) && law_.cap_metres > 0, "the material policy's cap is not finite");
        modes_.reserve(law_.voices.size());
        for (const auto &voice : law_.voices) modes_.push_back(voice.mode);
        impulses_.assign(law_.voices.size(), Complex(0, 0));
    }
    const TorusKeyLaw &law() const noexcept { return law_; }
    // The pulse's continuous steps at a sample (a sample before the anchor reads the anchor: the pulse never runs back).
    double steps_at(std::uint64_t samples) const noexcept {
        const double elapsed = samples > law_.anchor_samples ? double(samples - law_.anchor_samples) : 0;
        return law_.anchor_steps + elapsed / double(law_.sample_rate) * law_.steps_per_second;
    }
    // One act at the field's cursor: keys (rows[i], columns[i]) at `velocity` (modal metres, each part within the cap) strike their
    // points as one strike of the voices. noexcept and allocation-free; false refuses the whole act (no key, too many, a row off
    // the surface, a velocity past the cap, or impulses that sum past it on a voice) and nothing moves.
    bool play(ContinuousField &field, const std::uint8_t *rows, const std::uint32_t *columns, std::size_t n, Complex velocity) noexcept {
#if defined(__clang__)
#pragma clang fp contract(off)
#endif
        if (!rows || !columns || n == 0 || n > max_torus_keys || !finite(velocity) || std::abs(velocity.real()) > law_.cap_metres ||
            std::abs(velocity.imag()) > law_.cap_metres)
            return false;
        const double steps = steps_at(field.receipt().samples_elapsed);
        const double re = velocity.real() * law_.strike, im = velocity.imag() * law_.strike;
        for (auto &impulse : impulses_) impulse = Complex(0, 0);
        for (std::size_t k = 0; k < n; ++k) {
            if (rows[k] >= torus_rows) return false;
            // The key's moving point (POINT_LAW): its arc carried by the pulse, round the ring and round the tube.
            double arcs = rem(double(law_.arcs[rows[k] * torus_column_period + columns[k] % torus_column_period]) + steps, torus_ring_arcs);
            if (arcs >= torus_ring_arcs) arcs = 0.0;
            double sheet = rem(arcs, torus_sheet_arcs);
            if (sheet >= torus_sheet_arcs) sheet = 0.0;
            const double theta = arcs * (pi / torus_sheet_arcs), phi = sheet * (pi / (torus_sheet_arcs / 2.0));
            if (!(theta >= 0 && theta < 2 * pi && phi >= 0 && phi < 2 * pi)) return false;
            for (std::size_t v = 0; v < impulses_.size(); ++v) {
                const double shape = chi(law_.voices[v], phi, theta);
                impulses_[v] = Complex(impulses_[v].real() + re * shape, impulses_[v].imag() + im * shape);
            }
        }
        for (const auto &impulse : impulses_)
            if (!finite(impulse) || std::abs(impulse.real()) > law_.cap_metres || std::abs(impulse.imag()) > law_.cap_metres)
                return false;
        return field.strike_now(modes_.data(), impulses_.data(), impulses_.size());
    }
};
} // namespace ql
#endif
