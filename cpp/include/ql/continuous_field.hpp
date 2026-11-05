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
// Each note's own voices (QL-MEF #287 packet A; O-I #850; DLW manual §21.2 Rules: "Each note strikes its own eight voices.
// Polyphony counts notes."). A note is the cell its key struck, the Vimarsha reading at the struck arc (m2_vimarsha::read_seed):
// eight voices over four boundaries, each at its own frequency. A note's voice rings in the material of the resident voice it is
// like (that mode's damping) and is heard as that voice is heard (its output gain, a receiver's move included).
//
// The owner counts the notes (m1_prime::Session::admit); the bank sounds them. An act names its notes and the notes that made way
// for them (a key struck again, or the oldest past Polyphony): those fade out over `steal_seconds`, so nothing clicks, and their
// slots are free. A note takes the first free slot (never voiced, made way, or rung out); when all `max_bank_notes` still sound,
// the note voiced longest ago is taken, fading as one that made way. A key's release moves no native state: a released note keeps
// its own tail. Voicing, making way, rendering and silence are noexcept and allocation-free, on a host's audio thread.
inline constexpr std::size_t note_voices = 8;
// m1_prime::POLYPHONY_MAX
inline constexpr std::size_t max_bank_notes = 32;
inline constexpr double steal_seconds = 0.004;
// Below this squared modal amplitude a note's voice is silent and no longer rendered (judged sample by sample, so any block
// partition renders alike).
inline constexpr double bank_silent_norm = 1e-60;
// One voice of a note as it is struck: the resident mode it is like, its own frequency and its impulse (modal metres).
struct NoteVoice { std::size_t like = 0; double frequency_hz = 0; Complex impulse{0, 0}; };
// One note of an act: its owner's id (m1_prime::Note::id) and its eight voices.
struct NoteAct { std::uint64_t id = 0; std::array<NoteVoice, note_voices> voices{}; };
// One voice of a bank note as it rings: the mode it is like, its frequency and its resident state.
struct BankVoice { std::size_t like = 0; double frequency_hz = 0; Complex state{0, 0}; };
// One slot of the bank as it stands: whether it holds a note (`held`: not when its last note made way), that note's id, the
// order its last note was voiced in, its voices, and the voices of a note that made way there, still fading.
struct BankNote {
    std::size_t slot = 0;
    bool held = false;
    std::uint64_t id = 0, order = 0;
    std::array<BankVoice, note_voices> voices{}, fading{};
};
// An act admitted for a later sample and not yet landed, by prepared mode index, as a continuation hands it over.
struct PendingAct {
    std::uint64_t at_samples_elapsed;
    std::vector<std::pair<std::size_t, Complex>> impulses;
    // An act voicing notes (QL-MEF #287): its notes and the ids of the notes that made way for them; no impulses.
    std::vector<NoteAct> notes{};
    std::vector<std::uint64_t> made_way{};
};
// The most notes one act may name as making way: each of its notes a key struck again, and as many again past Polyphony.
inline constexpr std::size_t max_made_way = 2 * max_bank_notes;
// The M1′ pulse's drive of the body as `set_carrier` set it, and the whole steps it has landed (`reached`).
struct CarrierDrive {
    double steps_per_second = 0;
    std::uint64_t anchor_samples = 0;
    double anchor_steps = 0, reached = 0;
    std::vector<Complex> impulses;
};
// The continuation as it stands, for a native audio host to continue it (O-I #849: the Torus in Live's one engine). The input
// carries each mode's resident state as its amplitude and the output gain in force, the clock and the samples' shapes as they
// stand; beside it the cursor, the driver's remainder, the Carrier's drive, the acts waiting for later samples and a receiver
// move not yet reached. A field made from it renders, sample for sample, what the field it was taken from would have: it is the
// same continuation, never a reseed. `handover()` takes it; `ContinuousField(Continuation)` continues it.
struct Continuation {
    ContinuationInput input;
    std::uint64_t samples_elapsed = 0, driver_remainder = 0;
    double last_unmuted_sample = 0;
    CarrierDrive carrier;
    std::vector<PendingAct> pending;
    std::vector<double> heard_from;
    // The bank as it stands (QL-MEF #287): every slot ever voiced, and the order the last note was voiced in.
    std::vector<BankNote> bank{};
    std::uint64_t bank_order = 0;
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
// The most samples a receiver's move is reached over when a host names its length (`move_gains`): the owner's block ceiling, the
// longest part of a block the owner reaches one over.
inline constexpr std::uint64_t max_gain_ramp_frames = 8192;
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
    struct Pending {
        std::uint64_t at;
        std::vector<std::pair<std::size_t, Complex>> impulses;
        std::vector<NoteAct> notes{};
        std::vector<std::uint64_t> made_way{};
    };
    // A note's voice as it rings (QL-MEF #287): the mode it is like, its frequency, its step and its state.
    struct Voiced { std::size_t like = 0; double frequency_hz = 0; Complex step{0, 0}, state{0, 0}; };
    struct Slot {
        std::uint64_t id = 0, order = 0;
        std::array<Voiced, note_voices> voices{}, fading{};
        bool held = false, ringing = false;
    };
    std::array<Slot, max_bank_notes> bank_{};
    std::uint64_t bank_order_ = 0;
    // The per-sample factor a voice that makes way fades by: a thousandth over steal_seconds.
    double fade_ = 0;
    Complex step_of(std::size_t like, double hz) const noexcept {
        return std::exp(Complex(-modes_[like].input.damping_per_second, 2 * pi * hz) / double(source_.sample_rate));
    }
    bool admits(const NoteAct &note) const noexcept {
        for (const auto &v : note.voices)
            if (v.like >= modes_.size() || !std::isfinite(v.frequency_hz) || v.frequency_hz <= 0 ||
                v.frequency_hz >= source_.sample_rate * 0.45 || !finite(v.impulse) || std::abs(v.impulse) > 1e6)
                return false;
        return true;
    }
    // A slot's note makes way: its voices fade out, and the slot is free.
    void fade(Slot &slot) noexcept {
        for (std::size_t k = 0; k < note_voices; ++k) {
            slot.fading[k] = slot.voices[k];
            slot.fading[k].step *= fade_;
            slot.voices[k].state = Complex(0, 0);
        }
        slot.held = false;
    }
    void make_way(std::uint64_t id) noexcept {
        for (auto &slot : bank_)
            if (slot.held && slot.id == id) fade(slot);
    }
    // A note takes the first free slot; with every slot sounding, the one voiced longest ago, whose note fades.
    void voice(const NoteAct &note) noexcept {
        std::size_t at = max_bank_notes;
        for (std::size_t s = 0; s < max_bank_notes && at == max_bank_notes; ++s)
            if (!bank_[s].held || !bank_[s].ringing) at = s;
        if (at == max_bank_notes) {
            at = 0;
            for (std::size_t s = 1; s < max_bank_notes; ++s)
                if (bank_[s].order < bank_[at].order) at = s;
        }
        auto &slot = bank_[at];
        if (slot.held) fade(slot);
        for (std::size_t k = 0; k < note_voices; ++k) {
            const auto &v = note.voices[k];
            slot.voices[k] = {v.like, v.frequency_hz, step_of(v.like, v.frequency_hz), v.impulse};
        }
        slot.id = note.id;
        slot.order = ++bank_order_;
        slot.held = slot.ringing = true;
    }
    static bool among(std::uint64_t id, const std::uint64_t *ids, std::size_t m) noexcept {
        for (std::size_t j = 0; j < m; ++j)
            if (ids[j] == id) return true;
        return false;
    }
    // The whole act judged before anything moves: 1 to 32 notes, each a note of this body named once and not already held (unless
    // it is among those making way), and at most `max_made_way` made way. A note of the act that is itself among them (its key
    // struck twice, or past Polyphony at once) made way as it came and is not voiced.
    bool judges(const NoteAct *notes, std::size_t n, const std::uint64_t *made_way, std::size_t m) const noexcept {
        if (!notes || n == 0 || n > max_bank_notes || m > max_made_way || (m && !made_way)) return false;
        for (std::size_t i = 0; i < n; ++i) {
            if (!admits(notes[i])) return false;
            for (std::size_t j = 0; j < i; ++j)
                if (notes[j].id == notes[i].id) return false;
            const bool leaves = among(notes[i].id, made_way, m);
            for (const auto &slot : bank_)
                if (slot.held && slot.id == notes[i].id && !leaves) return false;
        }
        return true;
    }
    // The gain mode `i` is heard by at `sample` of the block being rendered, as the resident voices are (render_audio).
    double heard_at(std::size_t i, std::size_t sample, std::uint64_t ramp) const noexcept {
        const double gain = modes_[i].input.audio_gain_per_metre;
        if (heard_from_.empty()) return gain;
        if (ramp_awaits_) return heard_from_[i];
        if (ramp_done_ + sample < ramp) return heard_from_[i] + (gain - heard_from_[i]) * (double(ramp_done_ + sample + 1) / double(ramp));
        return gain;
    }
    // The bank's voices at one sample: each rung on by its step and heard as the mode it is like.
    double ring_bank(std::size_t sample, std::uint64_t ramp) noexcept {
        double value = 0;
        for (auto &slot : bank_) {
            if (!slot.ringing) continue;
            bool sounding = false;
            for (auto *lane : {&slot.voices, &slot.fading})
                for (auto &v : *lane) {
                    if (std::norm(v.state) < bank_silent_norm) continue;
                    v.state = v.step * v.state;
                    value += v.state.real() * heard_at(v.like, sample, ramp);
                    sounding = true;
                }
            slot.ringing = sounding;
        }
        return value;
    }
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
    // linearly, and then they are left. Empty while no move waits to be reached. Its room for one gain per mode is made with the
    // field, so a move named on the audio thread (`move_gains`) allocates nothing.
    std::vector<double> heard_from_;
    // The samples a move is reached over when a host named them (`move_gains`; 0: over the next rendered block, as the owner
    // reaches one), the samples of it already rendered, and whether a handed-over move waits for its length (`await_gain_ramp`).
    std::uint64_t ramp_frames_ = 0, ramp_done_ = 0;
    bool ramp_awaits_ = false;
    // The gain mode `i` is heard by at the cursor: the one in force, or where a move under way stands.
    double heard_now(std::size_t i) const noexcept {
        if (heard_from_.empty()) return modes_[i].input.audio_gain_per_metre;
        if (ramp_awaits_ || ramp_done_ == 0) return heard_from_[i];
        const double reached = double(ramp_done_) / double(ramp_frames_);
        return heard_from_[i] + (modes_[i].input.audio_gain_per_metre - heard_from_[i]) * reached;
    }
    std::size_t landed_ = 0;
    std::uint64_t elapsed_ = 0, driver_remainder_ = 0;
    double last_ = 0;
    void land(const Pending &act) noexcept {
        for (const auto &[mode, impulse] : act.impulses) modes_[mode].state += impulse;
        for (auto id : act.made_way) make_way(id);
        for (const auto &note : act.notes)
            if (!among(note.id, act.made_way.data(), act.made_way.size())) voice(note);
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
        fade_ = std::exp(std::log(1e-3) / (steal_seconds * double(source_.sample_rate)));
        heard_from_.reserve(source_.modes.size());
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
    // Continues a handed-over continuation at its cursor (O-I #849): every mode resumes from its resident state, the clock and the
    // driver's remainder from where they stood, the Carrier's drive with the steps it landed, the acts waiting for later samples in
    // their order and a receiver move not yet reached. Everything is judged before the field stands: one that is not a continuation
    // of its own input (a remainder past the driver, an act at or behind the cursor, a basis of another size) is refused.
    explicit ContinuousField(Continuation c) : ContinuousField(std::move(c.input)) {
        const std::uint64_t denominator = std::uint64_t(source_.sample_rate) * source_.driver_denominator;
        require(c.driver_remainder < denominator, "a continuation's driver remainder is past its driver");
        require(std::isfinite(c.last_unmuted_sample), "a continuation's last sample is not finite");
        const auto &drive = c.carrier;
        require(std::isfinite(drive.steps_per_second) && drive.steps_per_second >= 0 &&
                    drive.steps_per_second <= max_carrier_steps_per_sample * source_.sample_rate &&
                    drive.anchor_samples <= c.samples_elapsed && std::isfinite(drive.anchor_steps) && drive.anchor_steps >= 0 &&
                    drive.anchor_steps <= 9007199254740991.0 && std::isfinite(drive.reached) && drive.reached >= -1 &&
                    drive.reached <= 9007199254740991.0 && (drive.impulses.empty() || drive.impulses.size() == modes_.size()),
                "a continuation's Carrier is not a drive of this body");
        for (const auto &impulse : drive.impulses)
            require(finite(impulse) && std::abs(impulse) <= 1e6, "invalid carrier impulse");
        require(c.pending.size() <= max_pending_strikes, "too many strikes wait for their samples");
        for (std::size_t i = 0; i < c.pending.size(); ++i) {
            const auto &act = c.pending[i];
            require(act.at_samples_elapsed > c.samples_elapsed &&
                        (i == 0 || c.pending[i - 1].at_samples_elapsed <= act.at_samples_elapsed),
                    "a continuation's waiting acts stand after its cursor, in sample order");
            require(act.notes.empty() ? !act.impulses.empty() && act.impulses.size() <= modes_.size()
                                      : act.impulses.empty() && act.notes.size() <= max_bank_notes &&
                                            act.made_way.size() <= max_made_way,
                    "invalid strike count");
            for (const auto &[mode, impulse] : act.impulses)
                require(mode < modes_.size() && finite(impulse) && std::abs(impulse) <= 1e6,
                        "a continuation's waiting act names no prepared mode or an invalid impulse");
            for (const auto &note : act.notes) require(admits(note), "a continuation's waiting note is not a note of this body");
        }
        restore_bank(c.bank, c.bank_order);
        require(c.heard_from.empty() || c.heard_from.size() == modes_.size(), "gain/mode basis mismatch");
        for (double gain : c.heard_from) require(std::isfinite(gain) && std::abs(gain) <= 1e6, "nonfinite/excessive output gain");
        elapsed_ = c.samples_elapsed;
        driver_remainder_ = c.driver_remainder;
        last_ = c.last_unmuted_sample;
        carrier_ = {drive.steps_per_second, drive.anchor_steps, drive.anchor_samples, drive.reached, std::move(c.carrier.impulses)};
        pending_.clear();
        for (auto &act : c.pending)
            pending_.push_back({act.at_samples_elapsed, std::move(act.impulses), std::move(act.notes), std::move(act.made_way)});
        heard_from_ = std::move(c.heard_from);
        heard_from_.reserve(modes_.size());
    }
    const ContinuationInput &source() const noexcept { return source_; }
    const std::vector<Sample> &samples() const noexcept { return source_.samples; }
    Complex amplitude(std::size_t i) const { return modes_.at(i).state; }
    // The bank as it stands (QL-MEF #287): every slot ever voiced, in slot order (a free slot's id is 0). Off the audio thread.
    std::vector<BankNote> bank() const {
        std::vector<BankNote> out;
        for (std::size_t s = 0; s < max_bank_notes; ++s) {
            const auto &slot = bank_[s];
            if (slot.order == 0) continue;
            BankNote note{s, slot.held, slot.id, slot.order, {}, {}};
            for (std::size_t k = 0; k < note_voices; ++k) {
                note.voices[k] = {slot.voices[k].like, slot.voices[k].frequency_hz, slot.voices[k].state};
                note.fading[k] = {slot.fading[k].like, slot.fading[k].frequency_hz, slot.fading[k].state};
            }
            out.push_back(note);
        }
        return out;
    }
    std::uint64_t bank_order() const noexcept { return bank_order_; }
    // Voices an act's notes at the cursor (QL-MEF #287), for a host's audio thread: noexcept and allocation-free. The notes named
    // in `made_way` fade and free their slots, then each note takes a slot; the whole act is judged first, and false refuses it
    // with nothing moved. A made-way id the bank does not hold (taken at a full bank, or rung before the body was chosen) moves
    // nothing. One act, one generation.
    bool voice_now(const NoteAct *notes, std::size_t n, const std::uint64_t *made_way, std::size_t m) noexcept {
        if (!judges(notes, n, made_way, m)) return false;
        for (std::size_t j = 0; j < m; ++j) make_way(made_way[j]);
        for (std::size_t i = 0; i < n; ++i)
            if (!among(notes[i].id, made_way, m)) voice(notes[i]);
        ++source_.generation;
        return true;
    }
    // The same act at a sample of the continuation (as `strike` at a sample): admitted now, one generation, voiced by render_audio
    // exactly before sample `at_samples_elapsed` is rendered (at the cursor, now). Off the audio thread: it allocates.
    void voice(const std::vector<NoteAct> &notes, const std::vector<std::uint64_t> &made_way, std::uint64_t at_samples_elapsed) {
        require(at_samples_elapsed >= elapsed_, "a note's sample is behind the continuation cursor: that sound is already rendered");
        require(at_samples_elapsed == elapsed_ || pending_.size() - landed_ < max_pending_strikes, "too many strikes wait for their samples");
        require(!notes.empty() && notes.size() <= max_bank_notes, "an act voices 1 to 32 notes");
        require(made_way.size() <= max_made_way, "an act names at most 64 notes making way");
        for (std::size_t i = 0; i < notes.size(); ++i) {
            require(admits(notes[i]), "a note names no prepared mode, a frequency outside the audio band or an invalid impulse");
            for (std::size_t j = 0; j < i; ++j) require(notes[j].id != notes[i].id, "one act voices a note twice");
        }
        if (at_samples_elapsed > elapsed_) {
            Pending act{at_samples_elapsed, {}, notes, made_way};
            pending_.erase(pending_.begin(), pending_.begin() + std::ptrdiff_t(landed_));
            landed_ = 0;
            auto place = std::upper_bound(pending_.begin(), pending_.end(), at_samples_elapsed,
                                          [](std::uint64_t at, const Pending &p) { return at < p.at; });
            pending_.insert(place, std::move(act));
            ++source_.generation;
            return;
        }
        require(voice_now(notes.data(), notes.size(), made_way.data(), made_way.size()), "a note already sounds: it is voiced once");
    }
    // The bank set as a checkpoint kept it (O-I 855, with `set_state`): every slot it names, in slot order, and the order the last
    // note was voiced in. Validation precedes the commit; slots it does not name stand empty.
    void restore_bank(const std::vector<BankNote> &bank, std::uint64_t order) {
        std::array<Slot, max_bank_notes> next{};
        for (std::size_t i = 0; i < bank.size(); ++i) {
            const auto &note = bank[i];
            require(note.slot < max_bank_notes && (i == 0 || bank[i - 1].slot < note.slot) && note.order >= 1 && note.order <= order,
                    "a bank names its slots once, in order, voiced no later than its last note");
            for (std::size_t j = 0; j < i; ++j)
                require(!(note.held && bank[j].held && bank[j].id == note.id), "a bank holds a note once");
            auto &slot = next[note.slot];
            slot.id = note.id;
            slot.order = note.order;
            slot.held = note.held;
            const auto set = [&](const BankVoice &from, Voiced &to, bool faded) {
                require(from.like < modes_.size() && std::isfinite(from.frequency_hz) && from.frequency_hz >= 0 &&
                            from.frequency_hz < source_.sample_rate * 0.45 && finite(from.state) && std::abs(from.state) <= 1e6,
                        "a bank voice names no prepared mode, a frequency outside the audio band or an invalid state");
                to = {from.like, from.frequency_hz, step_of(from.like, from.frequency_hz), from.state};
                if (faded) to.step *= fade_;
            };
            for (std::size_t k = 0; k < note_voices; ++k) {
                set(note.voices[k], slot.voices[k], false);
                set(note.fading[k], slot.fading[k], true);
            }
            slot.ringing = true;
        }
        bank_ = next;
        bank_order_ = order;
    }
    ContinuationReceipt receipt() const noexcept { return {source_.generation, elapsed_, source_.clock, last_}; }
    // The continuation as it stands (`Continuation`): a read, nothing moves. Off the audio thread: it copies the whole input.
    Continuation handover() const {
        Continuation c{source_, elapsed_, driver_remainder_, last_,
                       {carrier_.steps_per_second, carrier_.anchor_samples, carrier_.anchor_steps, carrier_.reached, carrier_.impulses},
                       {}, heard_from_};
        // a move under way is handed over from where it stands, still to be reached
        for (std::size_t i = 0; i < c.heard_from.size(); ++i) c.heard_from[i] = heard_now(i);
        for (std::size_t i = 0; i < modes_.size(); ++i) c.input.modes[i].amplitude_metres = modes_[i].state;
        c.pending.reserve(pending_.size() - landed_);
        for (std::size_t i = landed_; i < pending_.size(); ++i)
            c.pending.push_back({pending_[i].at, pending_[i].impulses, pending_[i].notes, pending_[i].made_way});
        c.bank = bank();
        c.bank_order = bank_order_;
        return c;
    }
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
    // The same act as `strike(strikes)` at the cursor, by prepared mode index (`impulses[i]`, modal metres, to mode `modes[i]`), for a
    // native audio host's audio thread (O-I #849): noexcept and allocation-free. The same validation and render-budget preflight
    // complete before any state moves; false refuses the act and the body stands exactly as it was. One generation.
    bool strike_now(const std::size_t *modes, const Complex *impulses, std::size_t n) noexcept {
        if (!modes || !impulses || n == 0 || n > modes_.size()) return false;
        for (std::size_t i = 0; i < n; ++i) {
            if (modes[i] >= modes_.size() || !finite(impulses[i]) || std::abs(impulses[i]) > 1e6) return false;
            for (std::size_t j = 0; j < i; ++j)
                if (modes[j] == modes[i]) return false;  // one strike act names a mode twice
            const double bound = (std::abs(modes_[modes[i]].state) + std::abs(impulses[i])) * 1.000000001;
            if (!std::isfinite(bound) || bound > 1e12) return false;
        }
        for (std::size_t i = 0; i < n; ++i) modes_[modes[i]].state += impulses[i];
        ++source_.generation;
        return true;
    }
    // Silence (Panic, O-I #849; manual §21.1 Rules "Panic releases all notes at once"): the body stilled at the cursor. Every
    // voice's resident state is at rest and the acts waiting for later samples are dropped; the voices (with the drive their
    // material supplies), their gains and shapes, the clock, the cursor and the Carrier's drive stand, so a body the pulse or its
    // material drives is driven again from rest: silence stills what was struck, it does not change the body.
    // A key's release moves no state (the body rings on with its own damping); silence is the one act that stills it. One body
    // act: one generation. noexcept and allocation-free, so a native host calls it on its audio thread.
    void silence() noexcept {
        for (auto &m : modes_) m.state = Complex(0, 0);
        for (auto &slot : bank_) {
            for (auto &v : slot.voices) v.state = Complex(0, 0);
            for (auto &v : slot.fading) v.state = Complex(0, 0);
            slot.ringing = false;
        }
        landed_ = pending_.size();
        ++source_.generation;
    }
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
    // The body and its bank as they rang at a checkpoint, set again in one body act (QL-MEF #287 with O-I 855).
    void set_state(std::uint64_t expected, const std::vector<Complex> &states, const std::vector<BankNote> &bank, std::uint64_t order) {
        require(expected == source_.generation && expected != UINT64_MAX, "stale/overflow field generation");
        require(states.size() == modes_.size(), "state/mode basis mismatch");
        for (const auto &state : states)
            require(finite(state) && std::abs(state) <= 1e6, "invalid resident modal state");
        restore_bank(bank, order);
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
    // The body chosen for this continuation (O-I #852; DLW manual §21.2 "The body is what rings"): the voices, their shapes and the
    // geometry, material and model become `next`'s, ringing from its supplied states, as a body opened on them would. The event,
    // subject, sample rate, clock, driver relation, cursor and last sample stand; nothing drives the new voices and they are heard by
    // their supplied gains until the carrier and the gains are set again. One body act: one generation. An act admitted for a later
    // sample names the standing body's voices, so a body is chosen once every such act has landed. `next` is validated whole, as a
    // new body is, and every allocation completes before the commit.
    void replace_body(std::uint64_t expected, ContinuationInput next) {
        require(expected == source_.generation && expected != UINT64_MAX, "stale/overflow field generation");
        require(pending_strikes() == 0,
                "a strike waits for its sample on the body standing: a body is chosen once it has landed");
        require(next.event_ref == source_.event_ref && next.subject_ref == source_.subject_ref &&
                next.sample_rate == source_.sample_rate, "a body is chosen for the same event, subject and sample rate");
        const auto &a = next.clock, &b = source_.clock;
        require(next.driver_numerator == source_.driver_numerator && next.driver_denominator == source_.driver_denominator &&
                a.inscription.turns == b.inscription.turns && a.inscription.half_degrees == b.inscription.half_degrees &&
                a.lensing.turns == b.lensing.turns && a.lensing.half_degrees == b.lensing.half_degrees &&
                std::equal(std::begin(a.grid_origin_half_degrees), std::end(a.grid_origin_half_degrees),
                           std::begin(b.grid_origin_half_degrees)) &&
                std::equal(std::begin(a.rate_numerators), std::end(a.rate_numerators), std::begin(b.rate_numerators)) &&
                a.rate_denominator == b.rate_denominator &&
                std::equal(std::begin(a.rate_remainders), std::end(a.rate_remainders), std::begin(b.rate_remainders)) &&
                a.generation == b.generation, "a body is chosen under the clock in force: choosing it moves no clock");
        next.generation = source_.generation + 1;
        ContinuousField chosen(std::move(next));
        chosen.elapsed_ = elapsed_;
        chosen.driver_remainder_ = driver_remainder_;
        chosen.last_ = last_;
        *this = std::move(chosen);
    }
    // Where the body is heard from (the Torus's receiver, O-I 856; QL-MEF #289): one output gain per prepared mode, in mode order.
    // A receiving setting, not a body act: the states, clock, cursor and generation stand, and so does every mode's dynamics. The
    // next rendered block reaches the new gains linearly from those it was heard by, so a move never clicks; gains equal to those in
    // force change no sample. Validation precedes the first mutation.
    void set_gains(const std::vector<double> &gains) {
        require(gains.size() == modes_.size(), "gain/mode basis mismatch");
        for (double gain : gains) require(std::isfinite(gain) && std::abs(gain) <= 1e6, "nonfinite/excessive output gain");
        std::vector<double> from(modes_.size());
        for (std::size_t i = 0; i < modes_.size(); ++i) from[i] = heard_now(i);
        bool moved = false;
        for (std::size_t i = 0; i < modes_.size(); ++i) moved = moved || from[i] != gains[i];
        for (std::size_t i = 0; i < modes_.size(); ++i) {
            modes_[i].input.audio_gain_per_metre = gains[i];
            source_.modes[i].audio_gain_per_metre = gains[i];
        }
        if (moved) heard_from_.swap(from);
        else heard_from_.clear();
        ramp_frames_ = 0; ramp_done_ = 0; ramp_awaits_ = false;
    }
    // The same move for a host continuing the owner a frame at a time (O-I #849 round 14: the Torus in Live's one engine): reached
    // linearly from what is heard now over the next `frames` samples however they are rendered, sample for sample as the owner's
    // block of `frames` reaches it. A move waiting for its length (`await_gain_ramp`) is reached from the gains it waits at. The
    // audio thread's: it allocates nothing. False, changing nothing, for gains that are not one finite, bounded gain per mode, or a
    // length outside 1 to max_gain_ramp_frames.
    bool move_gains(const double *gains, std::size_t n, std::uint64_t frames) noexcept {
        if (!gains || n != modes_.size() || frames == 0 || frames > max_gain_ramp_frames || heard_from_.capacity() < n)
            return false;
        for (std::size_t i = 0; i < n; ++i)
            if (!std::isfinite(gains[i]) || std::abs(gains[i]) > 1e6) return false;
        const bool under_way = !heard_from_.empty();
        if (!under_way) heard_from_.resize(n); // within the room made with the field
        bool moved = false;
        for (std::size_t i = 0; i < n; ++i) {
            const double from = under_way ? heard_now(i) : modes_[i].input.audio_gain_per_metre;
            heard_from_[i] = from;
            moved = moved || from != gains[i];
            modes_[i].input.audio_gain_per_metre = gains[i];
            source_.modes[i].audio_gain_per_metre = gains[i];
        }
        if (!moved) heard_from_.clear();
        ramp_frames_ = moved ? frames : 0; ramp_done_ = 0; ramp_awaits_ = false;
        return true;
    }
    // A receiver move this field was handed over with and has not reached waits for `move_gains` to say over how many samples it
    // is reached: the owner reaches it over its next block, whose length a host continuing it learns from the owner's advance
    // (`torus_heard`). Until then the body is heard by the gains the move leaves. No move waiting: nothing.
    void await_gain_ramp() noexcept {
        if (!heard_from_.empty() && ramp_done_ == 0) ramp_awaits_ = true;
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
        // The samples a receiver's move is reached over: those its host named, or this block's.
        const std::uint64_t ramp = ramp_frames_ ? ramp_frames_ : frames;
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
            } else if (ramp_awaits_) {
                // A handed-over move waiting for its length: heard by the gains it leaves.
                for (std::size_t i = 0; i < modes_.size(); ++i) {
                    auto &m = modes_[i];
                    m.state = m.step * m.state + m.forcing;
                    value += m.state.real() * heard_from_[i];
                }
            } else if (ramp_done_ + sample < ramp) {
                // The samples a receiver's move is reached over: each gain from the one it was heard by to the one in force.
                const double reached = double(ramp_done_ + sample + 1) / double(ramp);
                for (std::size_t i = 0; i < modes_.size(); ++i) {
                    auto &m = modes_[i];
                    m.state = m.step * m.state + m.forcing;
                    value += m.state.real() * (heard_from_[i] + (m.input.audio_gain_per_metre - heard_from_[i]) * reached);
                }
            } else {
                // reached within this block: the gains in force
                for (auto &m : modes_) {
                    m.state = m.step * m.state + m.forcing;
                    value += m.state.real() * m.input.audio_gain_per_metre;
                }
            }
            value += ring_bank(sample, ramp);
            last_ = value;
            output[sample] = muted ? 0.0f : static_cast<float>(value);
        }
        while (due < pending_.size() && pending_[due].at == end) land(pending_[due++]);
        landed_ = due;
        if (!heard_from_.empty() && !ramp_awaits_) {
            ramp_done_ += frames;
            if (ramp_done_ >= ramp) { heard_from_.clear(); ramp_frames_ = 0; ramp_done_ = 0; }
        }
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
            // The notes are drawn as they are heard (QL-MEF #287; O-I 851): each ringing voice of the bank moves the sample along the
            // shape of the mode it is like, by its own state, as the resident voices move it by theirs.
            for (const auto &slot : bank_) {
                if (!slot.ringing) continue;
                for (const auto *lane : {&slot.voices, &slot.fading})
                    for (const auto &v : *lane)
                        for (unsigned axis = 0; axis < 3; ++axis) p[axis] += sample.mode_shapes[v.like][axis] * v.state.real();
            }
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
