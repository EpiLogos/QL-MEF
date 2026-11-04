// The continuation a native host takes over (O-I #849): handed over as it stands it continues sample for
// sample, a strike by mode index is the strike by name without allocating, silence stills the body, and the
// Torus's keys strike their moving points by QL's played-address law (<ql/torus_keys.hpp>).
#include <ql/continuous_field.hpp>
#include <ql/torus_keys.hpp>
#include <algorithm>
#include <array>
#include <cassert>
#include <cstring>
#include <cmath>
#include <iostream>
#include <vector>

using ql::Complex;
static ql::ContinuationInput input() {
    return {"acceptance:one-world", "acceptance:nara-a", "fixture:mesh", "fixture:material",
        "model:complex-modal-linear/v1", 7, 48000,
        {{0, 718}, {1, 17}, {3, 9, 21}, {9, 8}, 8, {0, 0}, 4}, 360, 1,
        {{"mode:earth", "#2-2-2-5-5", 220, 0.125, {0.01, 0.005}, {0.003, 0.001}, 2.0},
         {"mode:earth-two", "#2-2-2-5-5", 330, 0.25, {-0.01, -0.005}, {0, 0}, 1.5}},
        {{1, "#3-5-5/0", ql::ClockAttachment::Fixed, {0, 0, 0}, {{0, 0, 1}, {0, 0, 1}}},
         {2, "#3-0", ql::ClockAttachment::Inscription, {1, 0, 0}, {{0, 0, 1}, {0, 0, 0}}},
         {3, "#2-0-0", ql::ClockAttachment::Lensing, {0, 1, 0}, {{0, 0, 0}, {0, 0, 1}}}}};
}
static std::vector<float> run(ql::ContinuousField &f, std::size_t frames, std::size_t block) {
    std::vector<float> out(frames);
    for (std::size_t at = 0; at < frames; at += block) assert(f.render_audio(out.data() + at, std::min(block, frames - at)));
    return out;
}
static bool same(const ql::ContinuousField &a, const ql::ContinuousField &b) {
    auto x = a.receipt(), y = b.receipt();
    if (x.generation != y.generation || x.samples_elapsed != y.samples_elapsed || x.last_unmuted_sample != y.last_unmuted_sample ||
        std::memcmp(&x.clock, &y.clock, sizeof x.clock) != 0 || a.pending_strikes() != b.pending_strikes()) return false;
    for (std::size_t i = 0; i < a.source().modes.size(); ++i) if (a.amplitude(i) != b.amplitude(i)) return false;
    std::array<float, 9> p{}, q{};
    return a.write_targets(p.data(), 9) && b.write_targets(q.data(), 9) && p == q;
}
// The point and the shape term exactly as QL's Rust owner writes them (janko.rs JankoSurface::point, scene_field.rs chi).
static double chi(const ql::KeyVoice &v, double phi, double theta) {
    phi = phi - v.longitude_radians;
    double m = v.m, n = v.n, a = v.weight * std::cos(v.phase_radians), b = v.weight * std::sin(v.phase_radians);
    double first = a * std::sin(m * phi / 2.0); first = first * std::sin(n * theta / 2.0);
    double second = b * std::cos(m * phi / 2.0); second = second * std::cos(n * theta / 2.0);
    return first + second;
}
int main() {
    // 1. Handed over as it stands, the continuation continues exactly: the same states, cursor, clock and remainder,
    // the Carrier's drive and the steps it landed, the acts waiting for later samples and a receiver move not yet reached.
    ql::ContinuousField f(input());
    f.set_carrier(3.0, 0, 0.5, {{1e-4, 0}, {0, 0}});
    run(f, 1001, 333);
    f.strike({{"mode:earth", {0.02, 0}}});
    f.strike({{"mode:earth-two", {0.03, 0.01}}}, f.receipt().samples_elapsed + 700);
    f.set_gains({2.5, 1.0});
    ql::Continuation handed = f.handover();
    assert(handed.samples_elapsed == f.receipt().samples_elapsed && handed.input.generation == f.receipt().generation);
    assert(handed.input.modes[0].amplitude_metres == f.amplitude(0) && handed.input.modes[1].audio_gain_per_metre == 1.0);
    assert(handed.pending.size() == 1 && handed.pending[0].impulses.size() == 1 && handed.heard_from.size() == 2);
    ql::ContinuousField g(handed);
    assert(same(f, g));
    auto heard = run(f, 4096, 512), continued = run(g, 4096, 512);
    assert(heard == continued && same(f, g));
    // A field that is not a continuation is refused: a cursor behind a waiting act, a remainder past the driver, a basis mismatch.
    auto bad = f.handover(); bad.pending.push_back({bad.samples_elapsed, {{0, {1e-3, 0}}}});
    bool refused = false; try { ql::ContinuousField h(bad); } catch (const std::invalid_argument &) { refused = true; } assert(refused);
    bad = f.handover(); bad.driver_remainder = std::uint64_t(bad.input.sample_rate) * bad.input.driver_denominator;
    refused = false; try { ql::ContinuousField h(bad); } catch (const std::invalid_argument &) { refused = true; } assert(refused);
    bad = f.handover(); bad.carrier.impulses.pop_back();
    refused = false; try { ql::ContinuousField h(bad); } catch (const std::invalid_argument &) { refused = true; } assert(refused);

    // 2. A strike by prepared mode index is the strike by name, without allocating: the same states, one generation.
    ql::ContinuousField byname(f.handover()), byindex(f.handover());
    byname.strike({{"mode:earth", {0.1, -0.02}}, {"mode:earth-two", {0.05, 0}}});
    const std::size_t modes[2] = {0, 1}; const Complex impulses[2] = {{0.1, -0.02}, {0.05, 0}};
    assert(byindex.strike_now(modes, impulses, 2));
    assert(same(byname, byindex));
    const auto before = byindex.receipt().generation; const auto state = byindex.amplitude(0);
    const std::size_t twice[2] = {0, 0}, beyond[1] = {2};
    const Complex huge[1] = {{2e6, 0}}, nonfinite[1] = {{std::nan(""), 0}};
    assert(!byindex.strike_now(twice, impulses, 2) && !byindex.strike_now(beyond, impulses, 1));
    assert(!byindex.strike_now(modes, huge, 1) && !byindex.strike_now(modes, nonfinite, 1) && !byindex.strike_now(modes, impulses, 0));
    assert(byindex.receipt().generation == before && byindex.amplitude(0) == state);

    // 3. Silence stills the body at the cursor: every resident state at rest and the acts waiting for later samples dropped,
    // one body act; the voices, gains, clock, cursor and the Carrier's drive stand, so a driven body is driven again from rest.
    ql::ContinuousField quiet(f.handover());
    const auto cursor = quiet.receipt().samples_elapsed, generation = quiet.receipt().generation;
    quiet.silence();
    assert(quiet.receipt().generation == generation + 1 && quiet.receipt().samples_elapsed == cursor && quiet.pending_strikes() == 0);
    for (std::size_t i = 0; i < 2; ++i) assert(quiet.amplitude(i) == Complex(0, 0));
    // A voice the material itself drives (its supplied drive) keeps its drive: silence stills what was struck, not the material.
    auto struck_body = input(); for (auto &mode : struck_body.modes) mode.drive_metres_per_second = {0, 0};
    ql::ContinuousField still(struck_body); still.strike({{"mode:earth", {0.2, 0}}}); still.silence();
    auto rest = run(still, 2048, 256);
    for (float x : rest) assert(x == 0.0f);
    auto driven = run(quiet, 48000, 4096);
    assert(std::any_of(driven.begin(), driven.end(), [](float x) { return x != 0.0f; }));

    // 4. The Torus's keys (ql::TorusKeys): a key strikes its moving point on the voices of the tuning in force, each voice by
    // its own shape term at the point × velocity × Strike, summed over the keys of one act; refused whole past the cap.
    ql::TorusKeyLaw law;
    for (unsigned row = 0; row < 6; ++row)
        for (unsigned column = 0; column < 6; ++column) law.arcs[row * 6 + column] = std::uint8_t((row % 2) * 6 + (column + row / 2) % 6);
    law.voices = {{0, 0.4, 0.9, 0.3, 2, 3}, {1, 2.1, 0.6, -1.2, 1, 5}};
    law.strike = 0.7; law.anchor_samples = 0; law.anchor_steps = 0.25; law.steps_per_second = 2.0; law.sample_rate = 48000;
    law.cap_metres = 1.0;
    ql::ContinuousField played(f.handover()), expected(f.handover());
    ql::TorusKeys keys(law, played);
    const std::uint64_t now = played.receipt().samples_elapsed;
    const double steps = law.anchor_steps + double(now - law.anchor_samples) / double(law.sample_rate) * law.steps_per_second;
    assert(keys.steps_at(now) == steps && keys.steps_at(0) == law.anchor_steps);
    const std::uint8_t rows[2] = {0, 3}; const std::uint32_t columns[2] = {2, 7};
    const Complex velocity{0.35, 0};
    Complex sum[2] = {{0, 0}, {0, 0}};
    for (int k = 0; k < 2; ++k) {
        double arcs = std::fmod(double(law.arcs[rows[k] * 6 + columns[k] % 6]) + steps, 12.0); if (arcs < 0) arcs += 12.0; if (arcs >= 12.0) arcs = 0.0;
        double sheet = std::fmod(arcs, 6.0); if (sheet < 0) sheet += 6.0; if (sheet >= 6.0) sheet = 0.0;
        const double theta = arcs * (ql::pi / 6.0), phi = sheet * (ql::pi / (6.0 / 2.0));
        for (int v = 0; v < 2; ++v) {
            const double shape = chi(law.voices[v], phi, theta);
            sum[v] = {sum[v].real() + (velocity.real() * law.strike) * shape, sum[v].imag() + (velocity.imag() * law.strike) * shape};
        }
    }
    assert(keys.play(played, rows, columns, 2, velocity));
    expected.strike_now(modes, sum, 2);
    for (std::size_t i = 0; i < 2; ++i) assert(std::abs(played.amplitude(i) - expected.amplitude(i)) < 1e-15);
    assert(played.receipt().generation == expected.receipt().generation);
    const auto struck = played.receipt().generation; const auto at = played.amplitude(1);
    const std::uint8_t outside[1] = {6};
    assert(!keys.play(played, outside, columns, 1, velocity));
    assert(!keys.play(played, rows, columns, 0, velocity) && !keys.play(played, rows, columns, ql::max_torus_keys + 1, velocity));
    assert(!keys.play(played, rows, columns, 2, {1.5, 0}));  // a point's velocity is within the 1 metre material policy
    ql::TorusKeyLaw loud = law; loud.voices = {{0, 0.0, 2.0, ql::pi / 2, 0, 0}, {1, 2.1, 0.6, -1.2, 1, 5}};  // the first voice's term is its weight everywhere
    ql::TorusKeys past(loud, played);
    assert(!past.play(played, rows, columns, 1, {1.0, 0}));  // 1.0 × Strike 0.7 × 2.0 = 1.4 metres on a voice: refused whole
    assert(played.receipt().generation == struck && played.amplitude(1) == at);
    // A law that cannot stand on this field is refused when it is made: a voice the field does not hold, an arc off the ring,
    // no voices (the tuning in force holds none), a voice twice.
    for (auto broken : {[&] { auto l = law; l.voices[1].mode = 2; return l; }(), [&] { auto l = law; l.arcs[3] = 12; return l; }(),
                        [&] { auto l = law; l.voices.clear(); return l; }(), [&] { auto l = law; l.voices[1].mode = 0; return l; }(),
                        [&] { auto l = law; l.sample_rate = 44100; return l; }()}) {
        refused = false; try { ql::TorusKeys k(broken, played); } catch (const std::invalid_argument &) { refused = true; } assert(refused);
    }
    std::cout << "continuation handover, strike by index, silence and the Torus's keys passed\n";
}
