// Genuine sourceForm producer -> actual Engine/P/Manager mechanisms. The
// receipt distinguishes measurement from unimplemented device/UI/queue joins.
#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
#include <iostream>
#include <memory>
#include <ql/performance_management_wire.hpp>
#include <ql/performance_source_packet.hpp>
#include <ql/physical_contact.hpp>
#include <ql/physical_receiving.hpp>
#include <vector>
using namespace ql;
using namespace ql::performance;
using J = json_object;
using Json = ql::physical_wire::Json;
using namespace ql::performance::checkpoint_transport;
static Json json_clone(J *value) {
  auto copied = own(json_tokener_parse(
      json_object_to_json_string_ext(value, JSON_C_TO_STRING_PLAIN)));
  if (!copied)
    throw std::invalid_argument("native research fixture copy refused");
  return copied;
}
static NativePerformance prepare(J *fixture) {
  auto packet_value = packet::field(fixture, "native_preparation");
  Parameters parameters;
  parameters.force_newtons = .01;
  parameters.monitor_linear = 0;
  return prepare_source_performance_packet(
      json_object_to_json_string_ext(packet_value, JSON_C_TO_STRING_PLAIN),
      packet_value, packet::field(fixture, "native_basis"), true, true,
      parameters);
}
static Operation note_operation(const NativePerformance &p,
                                std::uint64_t sequence, std::uint64_t sample,
                                Kind kind = Kind::NoteOn) {
  Operation op{};
  op.kind = kind;
  op.identity = p.determination.identity;
  op.sequence = sequence;
  op.sample = sample;
  op.note = p.notes.at(0);
  op.touch = op.note.touch;
  op.value = .8;
  return op;
}
static Json vector_wire(const std::vector<float> &x) {
  auto out = array();
  for (float v : x)
    append(out.get(), json_object_new_double(v));
  return out;
}
static Json spectrum(const PreparedPhysicalBody &body) {
  auto out = array();
  for (std::size_t i = 0; i < body.mode_count(); ++i)
    append(out.get(), json_object_new_double(body.frequency_hz(i)));
  return out;
}
struct Trial {
  std::vector<float> pcm, pickup;
  std::vector<double> force;
  PhysicalSnapshot snapshot{};
  PhysicalBodyCheckpoint body_checkpoint{};
};
static Trial render(NativePerformance &p, std::size_t frames) {
  Trial t;
  t.pcm.reserve(frames);
  t.pickup.reserve(frames);
  t.force.reserve(frames);
  p.engine->enable_capture(true);
  while (t.pcm.size() < frames) {
    const auto count = std::min<std::size_t>(128, frames - t.pcm.size()),
               start = p.engine->samples_elapsed();
    std::array<float, 128> output{};
    assert(p.engine->render(output.data(), count, start));
    Capture capture{};
    assert(p.engine->pop_capture(capture));
    Readback reading{};
    assert(p.engine->pop_readback(reading));
    assert(reading.samples_elapsed == start + count &&
           reading.physical.samples_elapsed == reading.samples_elapsed);
    t.snapshot = reading.physical;
    t.pcm.insert(t.pcm.end(), output.begin(), output.begin() + count);
    t.pickup.insert(t.pickup.end(), capture.pickup_linear.begin(),
                    capture.pickup_linear.begin() + count);
    t.force.insert(t.force.end(), capture.force_newtons.begin(),
                   capture.force_newtons.begin() + count);
  }
  auto guard = p.engine->acquire_stopped_custody();
  assert(guard);
  t.body_checkpoint = p.body->checkpoint();
  assert(t.body_checkpoint.samples_elapsed == t.snapshot.samples_elapsed);
  return t;
}
template <class F> static void refuses(F f) {
  bool rejected = false;
  try {
    f();
  } catch (const std::invalid_argument &) {
    rejected = true;
  }
  assert(rejected);
}
static Json causal_source_and_bus(J *baseline, J *changed) {
  auto a = prepare(baseline), b = prepare(changed);
  const auto before_a = a.body->preparation(), before_b = b.body->preparation();
  assert(before_a.input().nodes.size() == 12 &&
         before_a.input().edges.size() == 34);
  assert(before_a.eigenbasis_identity() == before_b.eigenbasis_identity());
  assert(a.notes.size() == 7 && b.notes.size() == 7);
  for (std::size_t i = 0; i < a.notes.size(); ++i)
    assert(a.notes[i].hertz == b.notes[i].hertz &&
           a.notes[i].exact_ratio == b.notes[i].exact_ratio);
  assert(a.determination.audio_octet_hz != b.determination.audio_octet_hz);
  assert(a.engine->enqueue(note_operation(a, 1, 0)) == Result::Accepted);
  assert(b.engine->enqueue(note_operation(b, 1, 0)) == Result::Accepted);
  auto first = render(a, 2048), second = render(b, 2048);
  assert(first.force != second.force && first.pcm != second.pcm &&
         first.pickup != second.pickup);
  const auto &q1 = first.body_checkpoint, &q2 = second.body_checkpoint;
  assert(q1.samples_elapsed == 2048 && q2.samples_elapsed == 2048 &&
         first.snapshot.samples_elapsed == q1.samples_elapsed &&
         second.snapshot.samples_elapsed == q2.samples_elapsed &&
         q1.event_ref == q2.event_ref && q1.subject_ref == q2.subject_ref &&
         q1.preparation_ref == q2.preparation_ref &&
         q1.state_ref == q2.state_ref && q1.body_revision == q2.body_revision &&
         q1.source_coordinate == q2.source_coordinate &&
         q1.source_revision == q2.source_revision &&
         q1.source_generation == q2.source_generation &&
         q1.pratibimba == q2.pratibimba &&
         q1.eigenbasis_identity == q2.eigenbasis_identity &&
         q1.sample_rate == q2.sample_rate);
  assert(q1.displacement_modal_metres != q2.displacement_modal_metres &&
         q1.velocity_modal_metres_per_second !=
             q2.velocity_modal_metres_per_second);
  assert(first.snapshot.node_identity == second.snapshot.node_identity &&
         first.snapshot.node_count == second.snapshot.node_count &&
         first.snapshot.eigenbasis_identity ==
             second.snapshot.eigenbasis_identity &&
         first.snapshot.state_ref == second.snapshot.state_ref &&
         first.snapshot.body_revision == second.snapshot.body_revision &&
         first.snapshot.visible_positions_metres !=
             second.snapshot.visible_positions_metres);
  double q_difference = 0, v_difference = 0, position_difference = 0;
  for (std::size_t i = 0; i < q1.displacement_modal_metres.size(); ++i) {
    q_difference =
        std::max(q_difference, std::abs(q1.displacement_modal_metres[i] -
                                        q2.displacement_modal_metres[i]));
    v_difference = std::max(v_difference,
                            std::abs(q1.velocity_modal_metres_per_second[i] -
                                     q2.velocity_modal_metres_per_second[i]));
  }
  for (std::size_t n = 0; n < first.snapshot.node_count; ++n)
    for (unsigned axis = 0; axis < 3; ++axis)
      position_difference =
          std::max(position_difference,
                   std::abs(first.snapshot.visible_positions_metres[n][axis] -
                            second.snapshot.visible_positions_metres[n][axis]));
  assert(q_difference > 0 && v_difference > 0 && position_difference > 0);
  assert(a.body->preparation().eigenbasis_identity() ==
         before_a.eigenbasis_identity());
  auto packet_value = packet::field(baseline, "native_preparation");
  Json severed = json_clone(packet_value);
  auto d = packet::field(severed.get(), "determination");
  auto octet = packet::field(d, "audio_octet_hz");
  const auto old = packet::number(json_object_array_get_idx(octet, 0));
  assert(json_object_array_put_idx(octet, 0,
                                   json_object_new_double(old * 1.01)) == 0);
  refuses([&] {
    prepare_performance_packet(
        json_object_to_json_string_ext(severed.get(), JSON_C_TO_STRING_PLAIN),
        packet::field(baseline, "native_basis"), true, true);
  });
  auto detached = prepare(baseline);
  auto port = physical_port(detached.body);
  port.advance = [](void *, const double *, float *, std::size_t, std::uint64_t,
                    std::uint64_t) noexcept { return false; };
  Engine broken(detached.determination, 48000, port);
  assert(broken.enqueue(note_operation(detached, 1, 0)) == Result::Accepted);
  std::array<float, 128> silence{};
  assert(!broken.render(silence.data(), 128, 0));
  assert(detached.body->samples_elapsed() == 0 &&
         broken.samples_elapsed() == 0 &&
         std::all_of(silence.begin(), silence.end(),
                     [](float v) { return v == 0; }));
  auto out = object();
  text(out.get(), "native_body",
       "canonical-source-form/12nodes/34members/3frames");
  put(out.get(), "eigenfrequencies_hz", spectrum(before_a).release());
  text(out.get(), "audio_role",
       "eight antinodal M2 excitation components over actual M1 source-key "
       "voices");
  text(out.get(), "nodal_role",
       "four source-bearing boundary receipts, never extra oscillator voices");
  u64(out.get(), "available_m1_source_keys", 7);
  u64(out.get(), "physical_end_cursor", first.snapshot.samples_elapsed);
  put(out.get(), "physical",
      management_transport::physical(first.snapshot).release());
  put(out.get(), "changed_physical",
      management_transport::physical(second.snapshot).release());
  put(out.get(), "baseline_body_checkpoint",
      ql::physical_wire::checkpoint_wire(q1).release());
  put(out.get(), "changed_body_checkpoint",
      ql::physical_wire::checkpoint_wire(q2).release());
  real(out.get(), "max_modal_q_difference_metres", q_difference);
  real(out.get(), "max_modal_v_difference_metres_per_second", v_difference);
  real(out.get(), "max_visible_position_difference_metres",
       position_difference);
  put(out.get(), "baseline_pickup", vector_wire(first.pickup).release());
  put(out.get(), "changed_pickup", vector_wire(second.pickup).release());
  text(out.get(), "result",
       "lawful determinant changes force/q/v/visible/PCM/pickup at the same "
       "cursor with fixed native key "
       "pitches/body/policy; populated severed octet refuses; detached body "
       "refuses before cursor commit");
  text(out.get(), "open",
       "M2 nodal-to-live-boundary material transaction, installed stage and "
       "hardware sound remain separately required");
  return out;
}
static Json receivers(J *baseline) {
  auto p = prepare(baseline);
  const auto prepared = p.body->preparation();
  Vec3 point{};
  for (std::size_t n = 0; n < prepared.input().nodes.size(); ++n)
    for (unsigned axis = 0; axis < 3; ++axis)
      point[axis] += prepared.input().nodes[n].rest_metres[axis] *
                     prepared.input().pickup.node_weights[n];
  auto receiving = [&](double distance) {
    SpatialReceivingInput r;
    r.receiver_ref = "research:receiver";
    r.context_ref = "research:neutral-world";
    r.source_ref = "research:actual-source-body-pickup";
    r.policy_ref = "research:declared-static-point-receiving";
    r.policy_revision = "1";
    r.standing = "architecture-model";
    r.receiver_position_metres = point;
    r.receiver_position_metres[0] += distance;
    r.receiver_forward = {-1, 0, 0};
    r.transition_samples = 0;
    return PreparedSpatialReceiving(prepared, r);
  };
  const auto near = receiving(1), far = receiving(2);
  SpatialReceiving rx1(prepared, 0, near), rx2(prepared, 0, far);
  assert(std::abs(near.delay_samples() - 48000. / 340) < 1e-10 &&
         std::abs(far.delay_samples() - 2 * 48000. / 340) < 1e-10 &&
         near.gain() == 1 && far.gain() == .5);
  assert(p.engine->enqueue(note_operation(p, 1, 0)) == Result::Accepted);
  p.engine->enable_capture(true);
  std::vector<float> pickup, one, two;
  double maximum_error = 0;
  for (unsigned block = 0; block < 32; ++block) {
    std::array<float, 128> output{}, received1{}, received2{};
    const auto start = p.engine->samples_elapsed();
    assert(p.engine->render(output.data(), 128, start));
    Capture capture{};
    assert(p.engine->pop_capture(capture));
    Readback reading{};
    assert(p.engine->pop_readback(reading));
    assert(rx1.process_pickup_block(*p.body, capture.pickup_linear.data(),
                                    received1.data(), 128, start));
    assert(rx2.process_pickup_block(*p.body, capture.pickup_linear.data(),
                                    received2.data(), 128, start));
    pickup.insert(pickup.end(), capture.pickup_linear.begin(),
                  capture.pickup_linear.begin() + 128);
    one.insert(one.end(), received1.begin(), received1.end());
    two.insert(two.end(), received2.begin(), received2.end());
    assert(rx1.samples_elapsed() == reading.physical.samples_elapsed &&
           rx2.samples_elapsed() == reading.samples_elapsed);
  }
  auto expected = [&](std::size_t i, double delay, double gain) {
    const auto whole = std::uint64_t(std::floor(delay));
    const double fraction = delay - whole;
    const auto tap = [&](std::uint64_t lag) {
      return lag > i ? 0. : double(pickup[i - lag]);
    };
    return float(gain *
                 ((1 - fraction) * tap(whole) + fraction * tap(whole + 1)));
  };
  for (std::size_t i = 0; i < pickup.size(); ++i) {
    maximum_error = std::max(
        maximum_error,
        double(std::abs(one[i] - expected(i, near.delay_samples(), 1))));
    maximum_error = std::max(
        maximum_error,
        double(std::abs(two[i] - expected(i, far.delay_samples(), .5))));
  }
  assert(maximum_error == 0 && one != two);
  std::array<float, 128> input{}, refused_output{};
  assert(!rx1.process_pickup_block(*p.body, input.data(), refused_output.data(),
                                   128, rx1.samples_elapsed() - 1));
  assert(rx1.samples_elapsed() == 4096);
  auto out = object();
  real(out.get(), "one_metre_delay_samples", near.delay_samples());
  real(out.get(), "two_metre_delay_samples", far.delay_samples());
  real(out.get(), "extra_metre_delay_samples",
       far.delay_samples() - near.delay_samples());
  real(out.get(), "far_gain", far.gain());
  real(out.get(), "exact_expected_tap_error", maximum_error);
  u64(out.get(), "same_body_cursor", p.body->samples_elapsed());
  put(out.get(), "pickup", vector_wire(pickup).release());
  put(out.get(), "one_metre_received", vector_wire(one).release());
  put(out.get(), "two_metre_received", vector_wire(two).release());
  text(out.get(), "doppler",
       "unavailable: current declared static fractional-delay transport is not "
       "Sound Particles moving-source retarded-time/Doppler solver");
  return out;
}
static Json contacts(J *baseline) {
  auto p = prepare(baseline);
  const auto prepared = p.body->preparation();
  GravityContactInput input;
  input.contact_ref = "research:contact/one";
  input.particle_ref = "research:particle";
  input.collider_ref = "research:plane";
  input.route_ref = "research:native-midi-articulation";
  input.source_ref = "research:declared-gravity-model";
  input.policy_ref = "research:collision-to-native-key";
  input.policy_revision = "1";
  input.standing = "architecture-model";
  input.normal = {0, 0, -1};
  input.height_metres = .5;
  input.mass_kg = 1e-6;
  input.duration_samples = 32;
  const auto contact = prepare_gravity_contact(prepared, 0, input);
  assert(contact.start_sample == 15326 && contact.seed == 0 &&
         contact.seed_standing == "deterministic-no-randomness" &&
         contact_matches_preparation(contact, prepared, 0));
  auto again = prepare_gravity_contact(prepared, 0, input);
  assert(again.dedup_ref == contact.dedup_ref);
  auto stale = contact;
  stale.body_revision++;
  assert(!contact_matches_preparation(stale, prepared, 0));
  // Direct Newton trial uses the actual source-derived P loop, not a substitute
  // collision integrator. The separate MIDI trial uses actual M1 key
  // articulation.
  PhysicalBody body(prepared);
  std::vector<float> direct;
  direct.reserve(16384);
  std::array<double, 128> force{};
  std::array<float, 128> pickup{};
  for (std::uint64_t start = 0; start < 16384; start += 128) {
    force.fill(0);
    for (std::size_t i = 0; i < 128; ++i)
      if (start + i >= contact.start_sample &&
          start + i < contact.start_sample + contact.frames)
        force[i] = contact.force_newtons[start + i - contact.start_sample];
    assert(body.advance_force_block(force.data(), pickup.data(), 128,
                                    body.body_revision(), start));
    direct.insert(direct.end(), pickup.begin(), pickup.end());
  }
  assert(std::all_of(direct.begin(), direct.begin() + contact.start_sample,
                     [](float v) { return v == 0; }));
  assert(std::any_of(direct.begin() + contact.start_sample, direct.end(),
                     [](float v) { return v != 0; }));
  auto manager = std::make_unique<PerformanceManagement>(
      p, reference("research:contact/manager"));
  const auto original_input =
      reference("research:contact/exact-original-touch");
  auto note = note_operation(p, 1, contact.start_sample);
  assert(manager->enqueue_score_input(note, original_input) ==
         Result::Accepted);
  auto duplicate = note;
  duplicate.sequence = 2;
  assert(manager->enqueue_score_input(duplicate, original_input) ==
         Result::Exhausted);
  assert(manager->native().engine->accepted_sequence() == 1);
  auto off = note_operation(p, 2, contact.start_sample + 144, Kind::NoteOff);
  assert(manager->enqueue_score_input(off, original_input) == Result::Accepted);
  p.engine->enable_capture(true);
  bool timed_attack = false;
  for (std::uint64_t start = 0; start < 16384; start += 128) {
    std::array<float, 128> output{};
    assert(manager->offline_advance(output.data(), 128, start));
    auto pulse = manager->pulse();
    for (const auto &a : pulse->applications)
      if (a.kind == Kind::NoteOn) {
        assert(a.applied && a.admitted_sample == 15326 &&
               a.applied_sample == 15326 && a.applied_application_ordinal == 1);
        timed_attack = true;
      }
    Capture capture{};
    assert(manager->pop_audio_capture(capture));
  }
  assert(timed_attack);
  auto out = object();
  u64(out.get(), "analytic_contact_sample", contact.start_sample);
  real(out.get(), "analytic_seconds", contact.impact_seconds);
  real(out.get(), "impulse_newton_seconds", contact.impulse_newton_seconds);
  text(out.get(), "dedup_ref", contact.dedup_ref);
  text(out.get(), "seed_standing", contact.seed_standing);
  put(out.get(), "direct_physical_pickup", vector_wire(direct).release());
  text(out.get(), "midi_articulation",
       "actual managed native key attack at analytic contact sample; exact "
       "repeated input/touch refuses before consuming a native sequence");
  text(
      out.get(), "open",
      "direct prepared-contact injection plus persistent contact delivery "
      "dedup across newly resolved touch identities into A/N9 queue/checkpoint "
      "remain unimplemented; direct Newton and timed MIDI trials are distinct");
  return out;
}
static Json retained_controls(J *baseline) {
  auto p = prepare(baseline);
  auto owner = std::make_unique<PerformanceManagement>(
      p, reference("research:retained/manager"));
  assert(owner->enqueue_score_input(
             note_operation(p, 1, 0),
             reference("research:retained/original-touch")) ==
         Result::Accepted);
  Operation parameter{};
  parameter.kind = Kind::Parameter;
  parameter.identity = p.determination.identity;
  parameter.sequence = 2;
  parameter.sample = 64;
  parameter.parameter = Parameter::MasterLinear;
  parameter.value = .73;
  assert(owner->enqueue_score_input(parameter) == Result::Accepted);
  auto unautomated = prepare(baseline);
  assert(unautomated.engine->enqueue(note_operation(unautomated, 1, 0)) ==
         Result::Accepted);
  auto control = render(unautomated, 384);
  p.engine->enable_capture(true);
  auto journals = array(), applications = array(), effective_values = array();
  std::vector<NativeGestureApplication> original_applications;
  std::vector<InputBindingRecord> original_history;
  std::vector<float> automated_pcm, automated_pickup;
  std::vector<double> automated_force;
  double previous_effective = .25;
  std::array<float, 128> output{};
  for (unsigned block = 0; block < 3; ++block) {
    assert(owner->offline_advance(output.data(), 128, block * 128));
    auto pulse = owner->pulse();
    assert(pulse->has_readback && pulse->reading.source.master_linear == .73 &&
           pulse->reading.effective.master_linear > previous_effective &&
           pulse->reading.effective.master_linear < .73 &&
           pulse->reading.samples_elapsed == (block + 1) * 128 &&
           pulse->reading.physical.samples_elapsed ==
               pulse->reading.samples_elapsed);
    previous_effective = pulse->reading.effective.master_linear;
    append(effective_values.get(), json_object_new_double(previous_effective));
    Capture captured{};
    assert(owner->pop_audio_capture(captured));
    assert(std::equal(output.begin(), output.end(),
                      captured.output_linear.begin()));
    automated_pcm.insert(automated_pcm.end(), output.begin(), output.end());
    automated_pickup.insert(automated_pickup.end(),
                            captured.pickup_linear.begin(),
                            captured.pickup_linear.begin() + 128);
    automated_force.insert(automated_force.end(),
                           captured.force_newtons.begin(),
                           captured.force_newtons.begin() + 128);
    for (const auto &a : pulse->applications) {
      original_applications.push_back(a);
      append(applications.get(), application(a).release());
    }
    for (const auto &h : pulse->input_history) {
      original_history.push_back(h);
      append(journals.get(), management_transport::history(h).release());
    }
  }
  assert(original_applications.size() == 2 && original_history.size() == 2);
  const auto &attack = original_applications[0],
             &automation = original_applications[1];
  const auto original_note = note(p.notes.at(0));
  const auto applied_note = note(attack.note);
  assert(attack.applied && attack.kind == Kind::NoteOn &&
         attack.sequence == 1 && attack.applied_application_ordinal == 1 &&
         attack.identity == p.determination.identity && attack.has_note &&
         attack.touch == p.notes.at(0).touch &&
         json_object_equal(original_note.get(), applied_note.get()) &&
         attack.value == .8 && attack.has_requested_sample &&
         attack.requested_sample == 0 && attack.admitted_sample == 0 &&
         attack.applied_sample == 0 && attack.committed_cursor == 128 &&
         !attack.late_admitted);
  assert(
      automation.applied && automation.kind == Kind::Parameter &&
      automation.sequence == 2 && automation.applied_application_ordinal == 2 &&
      automation.identity == p.determination.identity && !automation.has_note &&
      automation.parameter == Parameter::MasterLinear &&
      automation.value == .73 && automation.has_requested_sample &&
      automation.requested_sample == 64 && automation.admitted_sample == 64 &&
      automation.applied_sample == 64 && automation.committed_cursor == 128 &&
      !automation.late_admitted);
  for (std::size_t i = 0; i < original_history.size(); ++i) {
    const auto &h = original_history[i];
    const auto target = note(h.target);
    assert(h.ordinal == i + 1 && h.native_sequence == 1 &&
           h.operation == Kind::NoteOn &&
           h.input_ref == reference("research:retained/original-touch") &&
           json_object_equal(target.get(), original_note.get()));
  }
  assert(original_history[0].change == InputBindingChange::PressAdmitted &&
         original_history[1].change == InputBindingChange::Applied);
  assert(automated_force == control.force &&
         automated_pickup == control.pickup &&
         std::equal(automated_pcm.begin(), automated_pcm.begin() + 64,
                    control.pcm.begin()) &&
         !std::equal(automated_pcm.begin() + 64, automated_pcm.end(),
                     control.pcm.begin() + 64));
  auto saved = owner->stopped_checkpoint();
  assert(saved->native_pair.physical.displacement_modal_metres ==
             control.body_checkpoint.displacement_modal_metres &&
         saved->native_pair.physical.velocity_modal_metres_per_second ==
             control.body_checkpoint.velocity_modal_metres_per_second);
  double expected_effective = .25;
  const double smoothing =
      owner->native().engine->parameter_smoothing_coefficient();
  for (unsigned sample = 64; sample < 384; ++sample)
    expected_effective += (.73 - expected_effective) * smoothing;
  assert(expected_effective ==
             saved->native_pair.audio.effective.master_linear &&
         saved->native_pair.audio.source.master_linear == .73);
  auto wire = management_checkpoint_transport::checkpoint_wire(*saved);
  auto restored =
      management_checkpoint_transport::read_checkpoint_wire(wire.get());
  auto other = prepare(baseline);
  auto reopened = std::make_unique<PerformanceManagement>(
      other, reference("research:retained/manager"));
  TransportAcknowledgement acknowledgement;
  assert(reopened->stopped_restore(*restored, 0, reference("research:restore"),
                                   reference("research:checkpoint"),
                                   acknowledgement));
  std::array<float, 128> next{};
  for (unsigned block = 0; block < 8; ++block) {
    const auto start = owner->native().engine->samples_elapsed();
    assert(owner->offline_advance(output.data(), 128, start));
    assert(reopened->offline_advance(next.data(), 128, start));
    assert(output == next);
    owner->pulse();
    reopened->pulse();
  }
  assert(owner->native().body->checkpoint().displacement_modal_metres ==
         reopened->native().body->checkpoint().displacement_modal_metres);
  auto out = object();
  real(out.get(), "master_baseline_linear", .25);
  real(out.get(), "master_target_linear", .73);
  real(out.get(), "master_actual_effective_at_384", expected_effective);
  put(out.get(), "master_effective_per_block", effective_values.release());
  put(out.get(), "baseline_pcm", vector_wire(control.pcm).release());
  put(out.get(), "automated_pcm", vector_wire(automated_pcm).release());
  u64(out.get(), "original_application_count", original_applications.size());
  u64(out.get(), "original_input_history_count", original_history.size());
  flag(out.get(), "same_force_and_pickup_different_master_output", true);
  flag(out.get(),
       "original_target_and_requested_admitted_applied_dates_verified", true);
  put(out.get(), "actual_checkpoint", wire.release());
  put(out.get(), "applied_applications", applications.release());
  put(out.get(), "original_input_history", journals.release());
  put(out.get(), "source_assets",
      json_clone(packet::field(baseline, "source_assets")).release());
  u64(out.get(), "continued_frames", 1024);
  u64(out.get(), "committed_cursor", owner->native().engine->samples_elapsed());
  text(out.get(), "open",
       "full C/Act persisted record/edit/routes/undo/clear/learn/export and "
       "ordinary U installed interactions remain separately required; this is "
       "actual manager parameter/paired-checkpoint continuation");
  return out;
}
int main() {
  try {
    std::string bytes;
    std::getline(std::cin, bytes);
    assert(!bytes.empty() && bytes.size() < 16 * 1024 * 1024);
    Json fixture = own(json_tokener_parse(bytes.c_str()));
    assert(fixture && packet::string(packet::field(fixture.get(), "schema")) ==
                          "ql.native-research-mechanisms-fixture/v1");
    auto baseline = packet::field(fixture.get(), "baseline"),
         changed = packet::field(fixture.get(), "changed");
    auto out = object();
    text(out.get(), "schema", "ql.native-research-mechanisms-receipt/v1");
    text(out.get(), "execution_scope",
         "actual native source producers and in-memory Engine/P/Manager; no "
         "vendor binary, device, installed UI or full R acceptance claim");
    put(out.get(), "R_D1_D4_same_cause_and_bus",
        causal_source_and_bus(baseline, changed).release());
    put(out.get(), "R_D2_spatial_receiving", receivers(baseline).release());
    put(out.get(), "R_D3_contact_and_midi", contacts(baseline).release());
    put(out.get(), "R_D5_retained_controls",
        retained_controls(baseline).release());
    std::cout << json_object_to_json_string_ext(out.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
