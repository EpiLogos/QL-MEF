#include <ql/continuous_field.hpp>
#include <algorithm>
#include <cassert>
#include <iostream>
#include <numeric>

using ql::Complex;
ql::ContinuationInput input(unsigned rate = 48000) {
    return {"acceptance:one-world", "acceptance:nara-a", "fixture:mesh", "fixture:material",
        "model:complex-modal-linear/v1", 7, rate,
        {{0, 718}, {1, 17}, {3, 9, 21}, {9, 8}, 8, {0, 0}, 4}, 360, 1,
        {{"mode:earth", "#2-2-2-5-5", 220, 0.125, {0.01, 0.005}, {0.003, 0.001}, 2.0},
         {"mode:earth-two", "#2-2-2-5-5", 220, 0.125, {-0.01, -0.005}, {-0.003, -0.001}, 2.0}},
        {{1, "#3-5-5/0", ql::ClockAttachment::Fixed, {0, 0, 0}, {{0, 0, 1}, {0, 0, 1}}},
         {2, "#3-0", ql::ClockAttachment::Inscription, {1, 0, 0}, {{0, 0, 1}, {0, 0, 0}}},
         {3, "#2-0-0", ql::ClockAttachment::Lensing, {0, 1, 0}, {{0, 0, 0}, {0, 0, 1}}}}};
}
void advance(ql::ContinuousField &f, std::size_t frames, std::size_t partition, bool muted = false) {
    std::array<float, 8192> output{};
    while (frames) { auto n = std::min(frames, partition); assert(f.render_audio(output.data(), n, muted)); frames -= n; }
}
int main() {
    // The singular limit and a genuinely repeated frequency are both valid.
    assert(ql::phi1({0, 0}) == Complex(1, 0));
    assert(std::abs(ql::phi1({-1e-12, 1e-12}) - Complex(1 - 5e-13, 5e-13)) < 1e-15);
    auto in = input();
    ql::ContinuousField f(in);
    std::array<float, 8192> audio{};
    assert(f.render_audio(audio.data(), audio.size()));
    assert(std::all_of(audio.begin(), audio.end(), [](float x) { return x == 0; }));
    assert(f.amplitude(0) == -f.amplitude(1));
    assert(f.samples().data() == f.source().samples.data());
    std::array<float, 9> points{};
    assert(f.write_targets(points.data(), points.size(), 100));
    assert(points[0] == 0 && points[1] == 0 && points[2] == 0); // true cancellation node
    assert(points[5] != points[8]); // independent shape functions, not a winning mode
    auto before = f.receipt(); auto state = f.amplitude(0);
    assert(!f.render_audio(audio.data(), 8193)); assert(f.amplitude(0) == state);
    assert(f.receipt().samples_elapsed == before.samples_elapsed);
    assert(!f.write_targets(points.data(), 8));
    auto bad = in.modes; bad[1].frequency_hz = 24000;
    bool rejected = false;
    try { f.replace_modes(7, bad); } catch (const std::invalid_argument &) { rejected = true; }
    assert(rejected && f.receipt().generation == 7 && f.amplitude(0) == state);
    // Parameter updates change the next dynamics while retaining resident state.
    auto updated = in.modes; updated[0].frequency_hz = 330; updated[0].amplitude_metres = {0.9, 0.9};
    const auto *stable_samples = f.samples().data();
    f.replace_modes(7, updated);
    assert(f.amplitude(0) == state && f.receipt().generation == 8);
    assert(f.samples().data() == stable_samples);
    rejected = false;
    try { f.replace_modes(7, updated); } catch (const std::invalid_argument &) { rejected = true; }
    assert(rejected);
    f.set_axis(8, 0, {2, 11});
    assert(f.receipt().clock.inscription.turns == 2 && f.receipt().clock.inscription.half_degrees == 11);
    assert(f.receipt().clock.lensing.half_degrees == before.clock.lensing.half_degrees);
    f.set_axis(9, 1, {-1, 19});
    assert(f.receipt().clock.inscription.half_degrees == 11);
    // Muting and hidden/detached reads do not halt, fork or double the simulation.
    ql::ContinuousField audible(in), muted(in);
    advance(audible, 48000, 127); advance(muted, 48000, 4096, true);
    assert(std::abs(audible.amplitude(0) - muted.amplitude(0)) == 0);
    auto a = audible.receipt(), b = muted.receipt();
    assert(a.samples_elapsed == b.samples_elapsed);
    assert(a.clock.inscription.turns == b.clock.inscription.turns &&
           a.clock.inscription.half_degrees == b.clock.inscription.half_degrees);
    assert(a.clock.lensing.turns == b.clock.lensing.turns && a.clock.lensing.half_degrees == b.clock.lensing.half_degrees);
    assert(a.clock.inscription.turns >= 1); // retained 359 -> 360 winding
    for (unsigned view = 0; view < 8; ++view) assert(audible.write_targets(points.data(), points.size()));
    assert(audible.receipt().samples_elapsed == a.samples_elapsed);
    // Independent sample rates approximate the same exact analytic solution.
    ql::ContinuousField higher(input(96000)); advance(higher, 96000, 512);
    const Complex lambda(-0.125, 2 * ql::pi * 220);
    const Complex exact = in.modes[0].amplitude_metres * std::exp(lambda) +
                          in.modes[0].drive_metres_per_second * ql::phi1(lambda);
    const double error48 = std::abs(audible.amplitude(0) - exact);
    const double error96 = std::abs(higher.amplitude(0) - exact);
    assert(error48 < 1e-11 && error96 < 1e-11);
    // Explicit DC boundary inside analytic owner (the M2 ingress still requires f>0).
    auto dc = in; dc.modes.resize(1); dc.modes[0].frequency_hz = 0; dc.modes[0].damping_per_second = 0;
    for (auto &sample : dc.samples) sample.mode_shapes.resize(1);
    ql::ContinuousField zero(dc); advance(zero, 48000, 256);
    assert(std::abs(zero.amplitude(0) - (dc.modes[0].amplitude_metres + dc.modes[0].drive_metres_per_second)) < 1e-11);
    // Two subjects can receive the same clock without shared mutable local state.
    auto other = in; other.subject_ref = "acceptance:nara-b"; other.modes[0].amplitude_metres = {0.02, 0};
    ql::ContinuousField nara_b(other); advance(nara_b, 48000, 512);
    assert(nara_b.source().subject_ref != audible.source().subject_ref);
    assert(nara_b.amplitude(0) != audible.amplitude(0));
    assert(nara_b.receipt().clock.inscription.half_degrees == audible.receipt().clock.inscription.half_degrees);
    // Fresh construction from original inputs reproduces original evolution.
    ql::ContinuousField replay(in); advance(replay, 48000, 8192);
    assert(replay.amplitude(0) == audible.amplitude(0));
    // Resource and source boundaries are hard refusal, never truncation.
    auto invalid = in; invalid.samples[1].identity = invalid.samples[0].identity;
    rejected = false; try { ql::ContinuousField no(invalid); } catch (const std::invalid_argument &) { rejected = true; }
    assert(rejected);
    invalid = in; invalid.modes[0].source_coordinate = "#3";
    rejected = false; try { ql::ContinuousField no(invalid); } catch (const std::invalid_argument &) { rejected = true; }
    assert(rejected);
    std::cout << "{\"schema\":\"ql.continuous-acceptance/v1\",\"analytic_error_48k\":" << error48
              << ",\"analytic_error_96k\":" << error96
              << ",\"equal_modes_cancel\":true,\"stable_samples\":true,\"mute_continues\":true,"
                 "\"partition_independent\":true,\"independent_subjects\":true,\"exact_replay\":true}\n";
}
