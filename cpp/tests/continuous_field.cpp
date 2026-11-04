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
    // A played strike adds a bounded impulse to one named mode and nothing else.
    // The clock, cursor and the unstruck voice continue bit-identically.
    ql::ContinuousField played(voiced), untouched(voiced);
    advance(played, 1024, 512); advance(untouched, 1024, 512);
    const auto pre = played.receipt();
    const Complex held0 = played.amplitude(0), held1 = played.amplitude(1);
    const Complex impulse{0.2, -0.1};
    played.strike({{"mode:earth-two", impulse}});
    assert(played.receipt().generation == pre.generation + 1);
    assert(played.receipt().samples_elapsed == pre.samples_elapsed);
    assert(played.receipt().clock.inscription.turns == pre.clock.inscription.turns &&
           played.receipt().clock.inscription.half_degrees == pre.clock.inscription.half_degrees);
    assert(played.amplitude(0) == held0);
    assert(played.amplitude(1) == held1 + impulse);
    // PCM energy after the strike sits at the struck mode's frequency.
    std::array<float, 4096> struck_audio{}, quiet_audio{};
    assert(played.render_audio(struck_audio.data(), struck_audio.size()));
    assert(untouched.render_audio(quiet_audio.data(), quiet_audio.size()));
    auto power = [](const float *x, std::size_t n, double hz, unsigned rate) {
        double re = 0, im = 0;
        for (std::size_t i = 0; i < n; ++i) {
            const double w = 2 * ql::pi * hz * double(i) / double(rate);
            re += double(x[i]) * std::cos(w); im -= double(x[i]) * std::sin(w);
        }
        return re * re + im * im;
    };
    assert(power(struck_audio.data(), struck_audio.size(), 220, 48000) >
           100.0 * power(quiet_audio.data(), quiet_audio.size(), 220, 48000));
    // Ring-down follows the prepared damping between two post-strike blocks.
    std::array<float, 4096> later{};
    assert(played.render_audio(later.data(), later.size()));
    const double decay = std::exp(-0.125 * 4096.0 / 48000.0);
    const double struck_power = power(struck_audio.data(), struck_audio.size(), 220, 48000);
    const double later_power = power(later.data(), later.size(), 220, 48000);
    assert(std::abs(later_power / struck_power - decay * decay) < 0.05);
    // Refusals leave state, cursor and targets untouched.
    const auto refused_generation = played.receipt().generation;
    const Complex after0 = played.amplitude(0), after1 = played.amplitude(1);
    auto refused_strike = [&](std::vector<ql::Strike> acts) {
        bool no = false;
        try { played.strike(acts); } catch (const std::invalid_argument &) { no = true; }
        assert(no && played.receipt().generation == refused_generation);
        assert(played.amplitude(0) == after0 && played.amplitude(1) == after1);
    };
    refused_strike({});
    refused_strike({{"mode:absent", impulse}});
    refused_strike({{"mode:earth", impulse}, {"mode:earth", impulse}});
    refused_strike({{"mode:earth", Complex(std::nan(""), 0)}});
    refused_strike({{"mode:earth", Complex(2e6, 0)}});
    refused_strike({{"", impulse}});
    // Repeated plucks accumulate: two half-impulses carry the combined energy.
    played.strike({{"mode:earth", {0.005, 0}}});
    played.strike({{"mode:earth", {0.005, 0}}});
    assert(played.amplitude(0) == after0 + Complex(0.01, 0));
    // A strike at a sample (O-I #809): the act is admitted now, one generation, and lands inside
    // render_audio exactly before that sample is rendered. Its PCM is bit-identical to rendering up to
    // the sample, striking now and rendering on; every sample before it is the unstruck body's.
    ql::ContinuousField scheduled(voiced), stepped(voiced), quiet(voiced);
    advance(scheduled, 1024, 512); advance(stepped, 1024, 512); advance(quiet, 1024, 512);
    const auto start = scheduled.receipt();
    const std::uint64_t at = start.samples_elapsed + 1000;
    scheduled.strike({{"mode:earth-two", impulse}}, at);
    assert(scheduled.receipt().generation == start.generation + 1);
    assert(scheduled.receipt().samples_elapsed == start.samples_elapsed);
    assert(scheduled.amplitude(1) == quiet.amplitude(1) && scheduled.pending_strikes() == 1);
    std::array<float, 4096> landed{}, reference{}, unstruck{};
    assert(scheduled.render_audio(landed.data(), landed.size()));
    assert(stepped.render_audio(reference.data(), 1000));
    stepped.strike({{"mode:earth-two", impulse}});
    assert(stepped.render_audio(reference.data() + 1000, reference.size() - 1000));
    assert(quiet.render_audio(unstruck.data(), unstruck.size()));
    assert(landed == reference);
    assert(std::equal(landed.begin(), landed.begin() + 1000, unstruck.begin()) && landed[1000] != unstruck[1000]);
    assert(scheduled.pending_strikes() == 0 && scheduled.amplitude(1) == stepped.amplitude(1));
    // A sample at a block's end lands as the cursor reaches it; acts at one sample land in admission order;
    // a sample at the cursor is struck now.
    const auto edge = scheduled.receipt().samples_elapsed + 512;
    scheduled.strike({{"mode:earth", {0.01, 0}}}, edge);
    scheduled.strike({{"mode:earth", {0.02, 0}}}, edge);
    assert(scheduled.render_audio(landed.data(), 512) && scheduled.pending_strikes() == 0);
    assert(stepped.render_audio(reference.data(), 512));
    stepped.strike({{"mode:earth", {0.01, 0}}}); stepped.strike({{"mode:earth", {0.02, 0}}});
    assert(scheduled.amplitude(0) == stepped.amplitude(0));
    const Complex now0 = scheduled.amplitude(0);
    scheduled.strike({{"mode:earth", {0.01, 0}}}, scheduled.receipt().samples_elapsed);
    assert(scheduled.amplitude(0) == now0 + Complex(0.01, 0) && scheduled.pending_strikes() == 0);
    // A sample behind the cursor is sound already rendered: refused, nothing moves. The pending acts are bounded.
    const auto held_generation = scheduled.receipt().generation;
    const auto cursor = scheduled.receipt().samples_elapsed;
    bool behind = false;
    try { scheduled.strike({{"mode:earth", impulse}}, cursor - 1); } catch (const std::invalid_argument &) { behind = true; }
    assert(behind && scheduled.receipt().generation == held_generation && scheduled.pending_strikes() == 0);
    for (std::uint64_t i = 1; i <= ql::max_pending_strikes; ++i) scheduled.strike({{"mode:earth", {1e-6, 0}}}, cursor + i);
    assert(scheduled.pending_strikes() == ql::max_pending_strikes);
    bool full = false;
    try { scheduled.strike({{"mode:earth", {1e-6, 0}}}, cursor + 1); } catch (const std::invalid_argument &) { full = true; }
    assert(full && scheduled.receipt().generation == held_generation + ql::max_pending_strikes);
    assert(scheduled.render_audio(landed.data(), ql::max_pending_strikes) && scheduled.pending_strikes() == 0);
    // The pulse drives the body between strikes (the Torus's Carrier, O-I #850; DLW §21.2). At each
    // step of the M1′ pulse every driven mode receives its impulse, landing exactly before the first
    // sample whose steps reach the step: steps(n) = anchor_steps + (n − anchor_samples) / rate ·
    // steps_per_second, the session's own law. Its PCM is bit-identical to strikes at those samples.
    // 375 steps a second at 48 kHz is 1/128 step a sample; from 0.3 steps the first step falls at +90.
    ql::ContinuousField driven(voiced), struck(voiced), still(voiced);
    advance(driven, 1024, 512); advance(struck, 1024, 512); advance(still, 1024, 512);
    const auto before_drive = driven.receipt();
    const std::vector<Complex> kick{{0.003, 0}, {0, 0}};
    driven.set_carrier(375, before_drive.samples_elapsed, 0.3, kick);
    assert(driven.receipt().generation == before_drive.generation &&
           driven.receipt().samples_elapsed == before_drive.samples_elapsed && driven.amplitude(0) == still.amplitude(0));
    std::array<float, 1024> pulsed{}, plucked{}, resting{};
    assert(driven.render_audio(pulsed.data(), 600) && driven.render_audio(pulsed.data() + 600, 424));
    std::size_t from = 0;
    for (std::size_t at = 90; at < 1024; at += 128) {
        assert(struck.render_audio(plucked.data() + from, at - from));
        struck.strike({{"mode:earth", kick[0]}});
        from = at;
    }
    assert(struck.render_audio(plucked.data() + from, 1024 - from));
    assert(still.render_audio(resting.data(), resting.size()));
    assert(pulsed == plucked && pulsed[89] == resting[89] && pulsed[90] != resting[90]);
    assert(driven.amplitude(0) == struck.amplitude(0) && driven.amplitude(1) == struck.amplitude(1));
    // The mode the pulse does not drive is untouched; no drive is no carrier; a refused drive changes nothing.
    assert(driven.amplitude(1) == still.amplitude(1));
    driven.set_carrier(375, driven.receipt().samples_elapsed, 0, {{0, 0}, {0, 0}});
    still.set_carrier(0, still.receipt().samples_elapsed, 0, {{0.5, 0}, {0.5, 0}});
    std::array<float, 512> quiet_driven{}, quiet_struck{}, quiet_still{}, untouched_still{};
    ql::ContinuousField never(voiced); advance(never, 2048, 512);
    assert(driven.render_audio(quiet_driven.data(), 512) && struck.render_audio(quiet_struck.data(), 512));
    assert(still.render_audio(quiet_still.data(), 512) && never.render_audio(untouched_still.data(), 512));
    assert(quiet_driven == quiet_struck && quiet_still == untouched_still);
    auto refused_drive = [&](double rate, std::uint64_t anchor, double phase, std::vector<Complex> impulses) {
        bool no = false;
        try { driven.set_carrier(rate, anchor, phase, impulses); } catch (const std::invalid_argument &) { no = true; }
        assert(no);
    };
    const auto cursor_now = driven.receipt().samples_elapsed;
    refused_drive(-1, cursor_now, 0, {{0, 0}, {0, 0}});
    refused_drive(std::nan(""), cursor_now, 0, {{0, 0}, {0, 0}});
    refused_drive(48000.0 * 9, cursor_now, 0, {{0, 0}, {0, 0}});
    refused_drive(375, cursor_now + 1, 0, {{0, 0}, {0, 0}});
    refused_drive(375, cursor_now, std::nan(""), {{0, 0}, {0, 0}});
    refused_drive(375, cursor_now, 0, {{0, 0}});
    refused_drive(375, cursor_now, 0, {{2e6, 0}, {0, 0}});
    std::array<float, 512> after_refusals{};
    assert(driven.render_audio(after_refusals.data(), 512) && struck.render_audio(quiet_struck.data(), 512));
    assert(after_refusals == quiet_struck);
    // A step the pulse reaches exactly at the cursor has not landed yet (it lands before that sample
    // is rendered): a drive set there lands it once, and a drive set again on the same pulse neither
    // loses nor repeats it. 375 steps a second is 1/128 step a sample.
    ql::ContinuousField at_step(voiced), at_step_struck(voiced), kept(voiced), resent(voiced);
    for (auto *f : {&at_step, &at_step_struck, &kept, &resent}) advance(*f, 1024, 512);
    at_step.set_carrier(375, 1024, 2, kick); // the pulse stands on step 2 at the cursor
    std::array<float, 256> on_step{}, struck_at_step_pcm{};
    assert(at_step.render_audio(on_step.data(), 256));
    at_step_struck.strike({{"mode:earth", kick[0]}});
    assert(at_step_struck.render_audio(struck_at_step_pcm.data(), 128));
    at_step_struck.strike({{"mode:earth", kick[0]}});
    assert(at_step_struck.render_audio(struck_at_step_pcm.data() + 128, 128));
    assert(on_step == struck_at_step_pcm);
    kept.set_carrier(375, 1024, 0.5, kick); resent.set_carrier(375, 1024, 0.5, kick);
    std::array<float, 512> straight{}, resent_pcm{};
    assert(kept.render_audio(straight.data(), 512) && resent.render_audio(resent_pcm.data(), 192));
    resent.set_carrier(375, 1024, 0.5, kick); // step 2 falls at the cursor, 1216
    assert(resent.render_audio(resent_pcm.data() + 192, 320));
    assert(straight == resent_pcm && kept.amplitude(0) == resent.amplitude(0));
    // The body as it rang, set again (O-I 855: a Live take's checkpoint): a field given another's resident states at the
    // same cursor rings on as that one does, bit for bit, in one generation; the cursor stays. A state that does not
    // name every mode, or is not finite and bounded, or a stale generation, is refused and changes nothing.
    ql::ContinuousField rung_field(voiced), reset(voiced);
    advance(rung_field, 1024, 512); advance(reset, 1024, 512);
    rung_field.strike({{"mode:earth", {0.004, 0}}});
    reset.strike({{"mode:earth", {0.0001, 0}}});
    advance(rung_field, 512, 256); advance(reset, 512, 256);
    const std::vector<Complex> rung{rung_field.amplitude(0), rung_field.amplitude(1)};
    assert(reset.amplitude(0) != rung_field.amplitude(0));
    const auto set_before = reset.receipt();
    auto refused_state = [&](std::uint64_t generation, std::vector<Complex> states) {
        const auto held = reset.amplitude(0);
        bool no = false;
        try { reset.set_state(generation, states); } catch (const std::invalid_argument &) { no = true; }
        assert(no && reset.receipt().generation == set_before.generation && reset.amplitude(0) == held);
    };
    refused_state(set_before.generation, {rung[0]});
    refused_state(set_before.generation, {rung[0], {std::nan(""), 0}});
    refused_state(set_before.generation, {rung[0], {2e6, 0}});
    refused_state(set_before.generation + 1, rung);
    reset.set_state(set_before.generation, rung);
    assert(reset.receipt().generation == set_before.generation + 1 && reset.receipt().samples_elapsed == set_before.samples_elapsed);
    assert(reset.amplitude(0) == rung[0] && reset.amplitude(1) == rung[1]);
    std::array<float, 512> rung_pcm{}, set_pcm{};
    assert(rung_field.render_audio(rung_pcm.data(), 512) && reset.render_audio(set_pcm.data(), 512));
    assert(rung_pcm == set_pcm && reset.amplitude(0) == rung_field.amplitude(0));
    std::cout << "{\"schema\":\"ql.continuous-acceptance/v1\",\"analytic_error_48k\":" << error48
              << ",\"analytic_error_96k\":" << error96
              << ",\"equal_modes_cancel\":true,\"stable_samples\":true,\"mute_continues\":true,"
                 "\"partition_independent\":true,\"independent_subjects\":true,\"exact_replay\":true,"
                 "\"shape_replacement_keeps_state_and_pcm\":true,\"played_strike_sounds_and_continues\":true,"
                 "\"strike_lands_at_its_sample\":true,\"pulse_drives_the_body_at_its_steps\":true}\n";
}
