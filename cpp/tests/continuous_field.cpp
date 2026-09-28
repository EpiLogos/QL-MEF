#include <ql/continuous_field.hpp>
#include <algorithm>
#include <cassert>
#include <iostream>
#include <numeric>
#include <vector>

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
    // Explicit shape replacement: nodal lines redistribute over the same samples
    // and modal voices. Resident z, clock and PCM continue; only targets move.
    auto voiced = in; voiced.modes[1].amplitude_metres = {0.004, 0.002};
    ql::ContinuousField shaped(voiced), control(voiced);
    advance(shaped, 4096, 512); advance(control, 4096, 512);
    const auto generation = shaped.receipt().generation;
    const Complex z0 = shaped.amplitude(0), z1 = shaped.amplitude(1);
    const auto held = shaped.receipt().clock;
    const auto *stable = shaped.samples().data();
    std::array<float, 9> prior{}, after{}, again{};
    assert(shaped.write_targets(prior.data(), prior.size()));
    const std::vector<std::vector<ql::Vec3>> nodal{{{0, 0, 1}, {0, 0, 0}}, {{0, 0, 0}, {0, 0, 1}}, {{1, 0, 0}, {0, 0, 0}}};
    shaped.replace_shapes(generation, nodal);
    assert(shaped.receipt().generation == generation + 1);
    assert(shaped.amplitude(0) == z0 && shaped.amplitude(1) == z1); // bit-identical resident state
    assert(shaped.receipt().samples_elapsed == control.receipt().samples_elapsed);
    assert(shaped.receipt().clock.inscription.turns == held.inscription.turns &&
           shaped.receipt().clock.inscription.half_degrees == held.inscription.half_degrees &&
           shaped.receipt().clock.lensing.half_degrees == held.lensing.half_degrees &&
           shaped.receipt().clock.generation == held.generation);
    assert(shaped.samples().data() == stable && shaped.samples().size() == voiced.samples.size());
    for (std::size_t i = 0; i < voiced.samples.size(); ++i) {
        const auto &s = shaped.samples()[i], &o = voiced.samples[i];
        assert(s.identity == o.identity && s.constituent == o.constituent && s.attachment == o.attachment &&
               s.rest_metres == o.rest_metres && s.mode_shapes == nodal[i]);
    }
    assert(shaped.write_targets(after.data(), after.size()));
    // Fixed sample at the origin: the former sum of both voices now reads z0 alone.
    assert(prior[2] == static_cast<float>(z0.real() + z1.real()));
    assert(after[0] == 0 && after[1] == 0 && after[2] == static_cast<float>(z0.real()));
    assert(prior != after);
    // Stale, structural, nonfinite and excessive bases are refused with nothing changed.
    auto refused = [&](std::uint64_t expected, const std::vector<std::vector<ql::Vec3>> &shapes) {
        bool no = false;
        try { shaped.replace_shapes(expected, shapes); } catch (const std::invalid_argument &) { no = true; }
        assert(no && shaped.receipt().generation == generation + 1);
        assert(shaped.amplitude(0) == z0 && shaped.amplitude(1) == z1);
        assert(shaped.write_targets(again.data(), again.size()) && again == after);
        for (std::size_t i = 0; i < nodal.size(); ++i) assert(shaped.samples()[i].mode_shapes == nodal[i]);
    };
    refused(generation, nodal);
    auto invalid_shapes = nodal; invalid_shapes.pop_back(); refused(generation + 1, invalid_shapes);
    invalid_shapes = nodal; invalid_shapes.push_back(nodal[0]); refused(generation + 1, invalid_shapes);
    invalid_shapes = nodal; invalid_shapes[1].pop_back(); refused(generation + 1, invalid_shapes);
    invalid_shapes = nodal; invalid_shapes[2].push_back({0, 0, 0}); refused(generation + 1, invalid_shapes);
    invalid_shapes = nodal; invalid_shapes[2][0][1] = std::nan(""); refused(generation + 1, invalid_shapes);
    invalid_shapes = nodal; invalid_shapes[2][1][2] = 2e6; refused(generation + 1, invalid_shapes);
    // Reshaping never touches PCM: the next block equals the no-reshape control.
    std::array<float, 8192> reshaped_audio{}, control_audio{};
    assert(shaped.render_audio(reshaped_audio.data(), 2048) && control.render_audio(control_audio.data(), 2048));
    assert(reshaped_audio == control_audio);
    assert(std::any_of(control_audio.begin(), control_audio.begin() + 2048, [](float x) { return x != 0; }));
    assert(shaped.amplitude(0) == control.amplitude(0) && shaped.amplitude(1) == control.amplitude(1));
    assert(shaped.receipt().clock.inscription.half_degrees == control.receipt().clock.inscription.half_degrees);
    // A consumer left on the old basis sees different targets from the same state.
    std::array<float, 9> old_basis{}, new_basis{};
    assert(control.write_targets(old_basis.data(), old_basis.size()) && shaped.write_targets(new_basis.data(), new_basis.size()));
    assert(old_basis != new_basis && control.receipt().generation == generation);
    std::cout << "{\"schema\":\"ql.continuous-acceptance/v1\",\"analytic_error_48k\":" << error48
              << ",\"analytic_error_96k\":" << error96
              << ",\"equal_modes_cancel\":true,\"stable_samples\":true,\"mute_continues\":true,"
                 "\"partition_independent\":true,\"independent_subjects\":true,\"exact_replay\":true,"
                 "\"shape_replacement_keeps_state_and_pcm\":true}\n";
}
