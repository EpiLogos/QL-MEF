// Each note its own voices (QL-MEF #287 packet A; DLW manual §21.2 Rules "Each note strikes its own eight voices. Polyphony counts
// notes."): a chord voices each note's eight in the field's bank, every voice at its own frequency, rung in the material of the
// resident voice it is like, heard as that voice is heard and drawn along its shape; a note the owner counts as making way fades without a click and frees
// its slot, and a full bank takes the note voiced longest ago; the bank renders alike however the host blocks its samples, is handed
// over and continued sample for sample, set again from a checkpoint and stilled by silence; a note voiced for a later sample sounds
// from that sample; the Torus's keys voice the cell their moving point struck and count notes as the owner does; and voicing and
// rendering a full bank allocate nothing and stay far inside a 128-frame block's deadline at 48 kHz.
#include <ql/continuous_field.hpp>
#include <ql/torus_keys.hpp>
#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
#include <cstdlib>
#include <ctime>
#include <iostream>
#include <new>
#include <vector>

// Every allocation this test makes, counted (voicing and rendering on the audio thread make none).
static std::size_t allocations = 0;
void *operator new(std::size_t n) {
    ++allocations;
    if (void *p = std::malloc(n ? n : 1)) return p;
    throw std::bad_alloc();
}
void operator delete(void *p) noexcept { std::free(p); }
void operator delete(void *p, std::size_t) noexcept { std::free(p); }

using ql::Complex;
static ql::ContinuationInput input() {
    // Two resident voices, quiet and undriven, so what sounds is what the bank voices.
    return {"acceptance:one-world", "acceptance:nara-a", "fixture:mesh", "fixture:material",
        "model:complex-modal-linear/v1", 7, 48000,
        {{0, 718}, {1, 17}, {3, 9, 21}, {9, 8}, 8, {0, 0}, 4}, 360, 1,
        {{"mode:earth", "#2-2-2-5-5", 220, 0.5, {0, 0}, {0, 0}, 2.0},
         {"mode:earth-two", "#2-2-2-5-5", 330, 1.5, {0, 0}, {0, 0}, 1.5}},
        {{1, "#3-5-5/0", ql::ClockAttachment::Fixed, {0, 0, 0}, {{0, 0, 1}, {0, 0, 1}}}}};
}
static std::vector<float> run(ql::ContinuousField &f, std::size_t frames, std::size_t block) {
    std::vector<float> out(frames);
    for (std::size_t at = 0; at < frames; at += block) assert(f.render_audio(out.data() + at, std::min(block, frames - at)));
    return out;
}
// A note whose eight voices stand 37 Hz apart from `base`, like resident voice k % 2, struck by `amplitude`.
static ql::NoteAct note(std::uint64_t id, double base, double amplitude = 0.01) {
    ql::NoteAct n{id, {}};
    for (std::size_t k = 0; k < ql::note_voices; ++k) n.voices[k] = {k % 2, base + 37.0 * double(k), Complex(amplitude, 0)};
    return n;
}
// The signal's power at `hz` (Goertzel over a Hann window, so a neighbour a few bins away leaks nothing that matters).
static double power(const std::vector<float> &x, double hz, double rate = 48000) {
    const double w = 2 * ql::pi * hz / rate, c = 2 * std::cos(w);
    double s1 = 0, s2 = 0;
    for (std::size_t i = 0; i < x.size(); ++i) {
        const double hann = 0.5 - 0.5 * std::cos(2 * ql::pi * double(i) / double(x.size() - 1));
        const double s = hann * x[i] + c * s1 - s2; s2 = s1; s1 = s;
    }
    return (s1 * s1 + s2 * s2 - c * s1 * s2) / double(x.size() * x.size());
}
// Sounding: its power stands far above the power halfway to the nearest other voice of the test's chords (6.65 Hz on).
static bool sounds(const std::vector<float> &x, double hz) { return power(x, hz) > 50 * power(x, hz + 6.65); }

int main() {
    // A chord: three notes, each its own eight voices at its own frequencies, all sounding.
    {
        ql::ContinuousField f(input());
        const std::array<ql::NoteAct, 3> chord{note(1, 200), note(2, 213.3), note(3, 226.6)};
        const auto g = f.receipt().generation;
        assert(f.voice_now(chord.data(), chord.size(), nullptr, 0));
        assert(f.receipt().generation == g + 1);  // one act, one generation
        const auto pcm = run(f, 24000, 128);
        for (const auto &n : chord)
            for (const auto &v : n.voices) assert(sounds(pcm, v.frequency_hz));
        const auto bank = f.bank();
        assert(bank.size() == 3 && bank[0].id == 1 && bank[1].id == 2 && bank[2].id == 3 && bank[0].held && bank[2].held);
        for (std::size_t s = 0; s < 3; ++s)
            for (std::size_t k = 0; k < ql::note_voices; ++k) assert(bank[s].voices[k].frequency_hz == chord[s].voices[k].frequency_hz);
        // A note's voice rings in the material of the mode it is like: the heavier-damped mode's voices decay faster.
        assert(std::abs(bank[0].voices[1].state) < std::abs(bank[0].voices[0].state));
    }
    // The notes are drawn as they are heard (O-I 851, one cause): a sample stands displaced by every ringing voice's state along the
    // shape of the mode it is like, as by the resident voices' own; once the bank is still the figure is the residents' alone.
    {
        ql::ContinuousField f(input());
        const std::array<ql::NoteAct, 2> chord{note(1, 200), note(2, 213.3)};
        assert(f.voice_now(chord.data(), chord.size(), nullptr, 0));
        run(f, 333, 128);
        const auto drawn = [&f] {
            std::array<float, 3> xyz{};
            assert(f.write_targets(xyz.data(), xyz.size()));
            return xyz;
        };
        double z = f.amplitude(0).real() + f.amplitude(1).real();  // both resident shapes stand along z at the sample
        for (const auto &slot : f.bank())
            for (const auto &v : slot.voices) z += v.state.real();
        const auto xyz = drawn();
        assert(std::abs(z) > 1e-4 && std::abs(double(xyz[2]) - z) < 1e-6 * std::abs(z) + 1e-9 && xyz[0] == 0 && xyz[1] == 0);
        f.silence();
        assert(drawn()[2] == 0.0f);
    }
    // A refused act moves nothing: a note twice, a frequency past the band, a mode the field does not hold, a note already held,
    // more notes making way than one act can name.
    {
        ql::ContinuousField f(input());
        const auto held = note(9, 200);
        assert(f.voice_now(&held, 1, nullptr, 0));
        const auto g = f.receipt().generation;
        std::array<ql::NoteAct, 2> twice{note(4, 200), note(4, 300)};
        assert(!f.voice_now(twice.data(), 2, nullptr, 0));
        auto bad = note(5, 30000);
        assert(!f.voice_now(&bad, 1, nullptr, 0));
        bad = note(6, 200);
        bad.voices[3].like = 2;
        assert(!f.voice_now(&bad, 1, nullptr, 0));
        assert(!f.voice_now(&held, 1, nullptr, 0));
        std::array<std::uint64_t, ql::max_made_way + 1> many{};
        assert(!f.voice_now(&twice[0], 1, many.data(), many.size()));
        assert(f.receipt().generation == g && f.bank().size() == 1);
        // Held again once it makes way for itself (its key struck again).
        const std::uint64_t self = 9;
        assert(f.voice_now(&held, 1, &self, 1));
    }
    // A note the owner counts as making way fades out over steal_seconds rather than stopping, and frees its slot.
    {
        ql::ContinuousField f(input());
        std::array<ql::NoteAct, 3> notes{note(1, 200), note(2, 250), note(3, 300)};
        assert(f.voice_now(notes.data(), 2, nullptr, 0));
        run(f, 480, 128);
        const double ringing = std::abs(f.bank()[0].voices[0].state);
        const std::uint64_t oldest = 1;
        assert(f.voice_now(&notes[2], 1, &oldest, 1));
        auto bank = f.bank();
        assert(bank.size() == 2 && bank[0].id == 3 && bank[1].id == 2);  // note 1 made way for note 3, in its slot
        assert(std::abs(std::abs(bank[0].fading[0].state) - ringing) < 1e-12);
        // No click: the note that made way rings on from where it stood, down a thousandth over steal_seconds (192 samples).
        run(f, 192, 64);
        bank = f.bank();
        assert(std::abs(bank[0].fading[0].state) < 1e-3 * ringing);
        // A note of the act that is itself among those making way (past Polyphony at once) is not voiced.
        std::array<ql::NoteAct, 2> pair{note(4, 400), note(5, 450)};
        const std::uint64_t four = 4;
        assert(f.voice_now(pair.data(), 2, &four, 1));
        bank = f.bank();
        assert(bank.size() == 3 && bank[2].id == 5 && std::none_of(bank.begin(), bank.end(), [](const ql::BankNote &b) { return b.held && b.id == 4; }));
        // A full bank takes the note voiced longest ago, which fades as one that made way.
        ql::ContinuousField full(input());
        for (std::uint64_t i = 0; i < ql::max_bank_notes; ++i) {
            const auto n = note(i, 100 + 7 * double(i), 0.001);
            assert(full.voice_now(&n, 1, nullptr, 0));
        }
        const auto late = note(99, 900, 0.001);
        assert(full.voice_now(&late, 1, nullptr, 0));
        assert(full.bank()[0].id == 99 && full.bank()[0].held && std::abs(full.bank()[0].fading[0].state) > 0);
    }
    // Polyphony lowered below the notes sounding (DLW build ruling 2026-11-05, O-I #850): an act that voices no note makes the oldest
    // make way, each fading over steal_seconds as one that made way for a newer note, never cut; one generation, allocation-free. An
    // act that names neither a note nor one making way is refused, and one that only makes way is made at the cursor, never later.
    {
        ql::ContinuousField f(input());
        const std::array<ql::NoteAct, 3> three{note(1, 200), note(2, 250), note(3, 300)};
        assert(f.voice_now(three.data(), 3, nullptr, 0));
        run(f, 480, 128);
        const auto rang = f.bank();
        const auto g = f.receipt().generation;
        assert(!f.voice_now(nullptr, 0, nullptr, 0) && f.receipt().generation == g);
        const std::array<std::uint64_t, 2> oldest{1, 2};
        const std::size_t before = allocations;
        assert(f.voice_now(nullptr, 0, oldest.data(), oldest.size()));
        assert(allocations == before && f.receipt().generation == g + 1);
        auto bank = f.bank();
        assert(bank.size() == 3 && !bank[0].held && !bank[1].held && bank[2].held && bank[2].id == 3);
        for (std::size_t s = 0; s < 2; ++s)
            for (std::size_t k = 0; k < ql::note_voices; ++k)
                assert(bank[s].fading[k].state == rang[s].voices[k].state && bank[s].voices[k].state == Complex(0, 0));
        run(f, 96, 32);
        bank = f.bank();
        assert(std::abs(bank[0].fading[0].state) > 1e-3 * std::abs(rang[0].voices[0].state));
        run(f, 96, 32);
        bank = f.bank();
        assert(std::abs(bank[0].fading[0].state) < 1e-3 * std::abs(rang[0].voices[0].state));
        assert(std::abs(bank[2].voices[0].state) > 0);
        bool refused = false;
        try { f.voice({}, {3}, f.receipt().samples_elapsed + 10); } catch (const std::exception &) { refused = true; }
        assert(refused && f.bank()[2].held);
        f.voice({}, {3}, f.receipt().samples_elapsed);
        assert(!f.bank()[2].held);
    }
    // Any block partition renders the bank alike, sample for sample.
    {
        ql::ContinuousField a(input()), b(input());
        const std::array<ql::NoteAct, 2> chord{note(1, 220), note(2, 277.18)};
        assert(a.voice_now(chord.data(), 2, nullptr, 0) && b.voice_now(chord.data(), 2, nullptr, 0));
        assert(run(a, 4096, 4096) == run(b, 4096, 37));
    }
    // Handed over, the bank continues sample for sample; a note waiting for its sample goes with it.
    {
        ql::ContinuousField f(input());
        const std::array<ql::NoteAct, 2> chord{note(1, 220), note(2, 277.18)};
        assert(f.voice_now(chord.data(), 2, nullptr, 0));
        run(f, 1000, 128);
        f.voice({note(3, 329.63)}, {1}, f.receipt().samples_elapsed + 300);
        ql::ContinuousField g(f.handover());
        assert(run(f, 4096, 128) == run(g, 4096, 256));
        // Note 1 made way at that sample, and note 3 took its slot.
        assert(f.bank().size() == 2 && g.bank().size() == 2 && g.bank()[0].id == 3 && g.bank()[1].id == 2);
    }
    // A checkpoint's bank, set again with its body, rings on as it rang.
    {
        ql::ContinuousField f(input()), g(input());
        const std::array<ql::NoteAct, 2> chord{note(1, 220), note(2, 277.18)};
        assert(f.voice_now(chord.data(), 2, nullptr, 0));
        const std::uint64_t first = 1;
        const auto third = note(3, 330);
        run(f, 300, 128);
        assert(f.voice_now(&third, 1, &first, 1));  // the checkpoint keeps a note still fading
        run(f, 400, 128);
        run(g, 700, 128);
        g.set_state(g.receipt().generation, {f.amplitude(0), f.amplitude(1)}, f.bank(), f.bank_order());
        assert(run(f, 2048, 128) == run(g, 2048, 128));
    }
    // A note voiced for a later sample sounds from that sample, and not before.
    {
        ql::ContinuousField f(input()), quiet(input());
        f.voice({note(1, 440)}, {}, 300);
        const auto pcm = run(f, 1024, 128), none = run(quiet, 1024, 128);
        assert(std::equal(pcm.begin(), pcm.begin() + 300, none.begin()) && pcm[300] != none[300]);
    }
    // Silence stills the bank: every voice at rest, the slots and their order kept.
    {
        ql::ContinuousField f(input());
        const auto n = note(1, 220);
        assert(f.voice_now(&n, 1, nullptr, 0));
        run(f, 256, 128);
        f.silence();
        const auto pcm = run(f, 512, 128);
        assert(std::all_of(pcm.begin(), pcm.end(), [](float v) { return v == 0.0f; }));
        assert(f.bank().size() == 1 && f.bank_order() == 1);
    }
    // The Torus's keys voice the cell of the arc each key's moving point struck, each voice by its own shape term there.
    {
        ql::ContinuousField f(input());
        ql::TorusKeyLaw law;
        for (std::size_t i = 0; i < law.arcs.size(); ++i) law.arcs[i] = std::uint8_t(((i / 6) % 2) * 6 + i % 6);
        law.strike = 0.5;
        law.sample_rate = 48000;
        law.polyphony = 8;
        for (std::size_t arc = 0; arc < 12; ++arc) {
            ql::KeyCell cell{};
            for (std::size_t k = 0; k < ql::note_voices; ++k)
                cell[k] = {k % 2, 200.0 * (1.0 + 0.0625 * double(arc)) * (1.0 + 0.125 * double(k)), 0.5 * double(k), 1.0, 0.25, unsigned(k % 4 + 1), unsigned(k % 3 + 1)};
            law.cells.push_back(cell);
        }
        ql::TorusKeys keys(law, f);
        const std::uint8_t rows[3] = {0, 2, 4};
        const std::uint32_t columns[3] = {0, 1, 3};
        assert(keys.play(f, rows, columns, 3, Complex(0.02, 0)));
        const auto bank = f.bank();
        assert(bank.size() == 3);
        for (std::size_t i = 0; i < 3; ++i) {
            const auto arc = law.arcs[rows[i] * 6 + columns[i] % 6];
            for (std::size_t k = 0; k < ql::note_voices; ++k) assert(bank[i].voices[k].frequency_hz == law.cells[arc][k].frequency_hz);
        }
        // Each voice's state is its impulse: velocity × Strike × its own shape term at the key's point (janko.rs POINT_LAW).
        for (std::size_t i = 0; i < 3; ++i) {
            const double arcs = law.arcs[rows[i] * 6 + columns[i] % 6], sheet = std::fmod(arcs, 6.0);
            const double theta = arcs * (ql::pi / 6.0), phi = sheet * (ql::pi / 3.0);
            for (std::size_t k = 0; k < ql::note_voices; ++k) {
                const auto &v = law.cells[std::size_t(arcs)][k];
                const double a = v.weight * std::cos(v.phase_radians), b = v.weight * std::sin(v.phase_radians), p = phi - v.longitude_radians;
                const double shape = a * std::sin(v.m * p / 2.0) * std::sin(v.n * theta / 2.0) + b * std::cos(v.m * p / 2.0) * std::cos(v.n * theta / 2.0);
                assert(std::abs(bank[i].voices[k].state - Complex(0.02 * 0.5 * shape, 0)) < 1e-15);
            }
        }
        // Counted as the owner counts: Polyphony 2, so the third key's note makes the first make way; a key struck again makes
        // way for itself; a released key leaves the count and its note rings on.
        ql::ContinuousField g(input());
        auto two = law;
        two.polyphony = 2;
        two.next_note = 7;
        two.notes = {{5, 1, 1}};  // the owner's count as it stood: note 5, key (1, 1)
        ql::TorusKeys counted(two, g);
        const std::uint8_t r1[1] = {0}, r2[1] = {2}, r3[1] = {1};
        const std::uint32_t c1[1] = {0}, c2[1] = {1}, c3[1] = {1};
        assert(counted.play(g, r1, c1, 1, Complex(0.02, 0)) && counted.counted() == 2 && counted.counted(1).id == 7);
        assert(counted.play(g, r2, c2, 1, Complex(0.02, 0)) && counted.counted() == 2 && counted.counted(0).id == 7);
        assert(counted.play(g, r1, c1, 1, Complex(0.02, 0)) && counted.counted() == 2 && counted.counted(0).id == 8 && counted.counted(1).id == 9);
        auto held = g.bank();
        assert(std::count_if(held.begin(), held.end(), [](const ql::BankNote &b) { return b.held; }) == 2);
        counted.release(r3, c3, 1);
        assert(counted.counted() == 2);  // key (1, 1) no longer counted: nothing to release
        counted.release(r2, c2, 1);
        assert(counted.counted() == 1 && counted.counted(0).id == 9 && g.bank().size() == held.size());
        assert(counted.next_note() == 10);
        // A law that voices notes needs all twelve cells.
        auto short_law = law;
        short_law.cells.pop_back();
        bool refused = false;
        try { ql::TorusKeys bad(short_law, f); } catch (const std::exception &) { refused = true; }
        assert(refused);
    }
    // Realtime: a full bank (32 notes, eight voices each) voiced and rendered at 48 kHz in 128-frame blocks allocates nothing, and
    // a block's CPU time stays far inside its 2.67 ms deadline.
    double block_ms = 0;
    {
        ql::ContinuousField f(input());
        std::vector<ql::NoteAct> full;
        for (std::uint64_t i = 1; i <= ql::max_bank_notes; ++i) full.push_back(note(i, 110.0 * std::pow(2.0, double(i) / 12.0), 0.001));
        std::vector<float> out(128);
        ql::NoteAct more = note(100, 523.25);
        const std::size_t before = allocations;
        assert(f.voice_now(full.data(), full.size(), nullptr, 0));
        const std::uint64_t gone = 1;
        const std::clock_t start = std::clock();
        const std::size_t blocks = 375;  // one second
        for (std::size_t b = 0; b < blocks; ++b) {
            assert(f.render_audio(out.data(), out.size()));
            if (b == 100) assert(f.voice_now(&more, 1, &gone, 1));  // a note makes way mid-stream
        }
        block_ms = 1000.0 * double(std::clock() - start) / CLOCKS_PER_SEC / double(blocks);
        f.silence();
        assert(f.render_audio(out.data(), out.size()));
        assert(allocations == before);
        assert(block_ms < 2.6667 / 4);
    }
    std::cout << "{\"schema\":\"ql.note-bank-acceptance/v1\",\"chord_sounds_each_notes_own_eight\":true,\"notes_drawn_as_heard\":true,\"refused_acts_move_nothing\":true,"
                 "\"made_way_fades_full_bank_takes_oldest\":true,\"polyphony_lowered_fades_the_oldest\":true,\"keys_count_as_the_owner\":true,\"partition_independent\":true,\"handover_continues\":true,"
                 "\"checkpoint_rings_on\":true,\"timed_note_lands_at_its_sample\":true,\"silence_stills_the_bank\":true,"
                 "\"keys_voice_their_struck_cell\":true,\"full_bank_allocates_nothing\":true,\"full_bank_block_cpu_ms\":"
              << block_ms << ",\"block_deadline_ms\":2.6667}\n";
}
