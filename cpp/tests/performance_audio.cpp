// Controlled inputs exercise actual excitation -> actual P integration.
// They are not authenticated M2 production inputs. The Rust consumer fixture
// separately joins native M1/K/M2/B outputs before creating these packets.
#include "../test_support/allocation_hooks.hpp"
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <new>
#include <ql/performance_checkpoint_wire.hpp>
#include <thread>
#include <vector>
static std::atomic<bool> count_allocations{false};
static std::atomic<std::uint64_t> callback_allocations{0};
void *operator new(std::size_t size) {
  if (count_allocations.load(std::memory_order_relaxed))
    callback_allocations.fetch_add(1);
  if (void *p = ql_test_allocate(size))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t size) { return ::operator new(size); }
void operator delete(void *p) noexcept { ql_test_release(p); }
void operator delete[](void *p) noexcept { ql_test_release(p); }
void operator delete(void *p, std::size_t) noexcept { ql_test_release(p); }
void operator delete[](void *p, std::size_t) noexcept { ql_test_release(p); }
using namespace ql::performance;
static void near(double a, double b, double tolerance = 1e-10) {
  assert(std::isfinite(a) && std::abs(a - b) <= tolerance);
}
static ql::PhysicalBodyInput physical_input() {
  ql::PhysicalBodyInput in{};
  in.event_ref = "controlled:performance/event";
  in.subject_ref = "controlled:performance/subject";
  in.source_coordinate = "#3-2-1-1-1";
  in.source_revision = "controlled:physical/source-revision";
  in.geometry_ref = "controlled:metric-axial-bar";
  in.geometry_revision = "1";
  in.geometry_source_ref = "controlled:analytic-bar-geometry";
  in.geometry_standing = "reference";
  in.preparation_ref = "controlled:physical/preparation";
  in.state_ref = "controlled:physical/state";
  in.source_generation = 7;
  in.body_revision = 1;
  in.sample_rate = 48000;
  in.pratibimba = true;
  in.family = ql::BodyFamily::AxialTruss;
  in.material = {"controlled:elastic-material",
                 "1",
                 "controlled:analytic-material",
                 "reference",
                 1e6,
                 2,
                 4,
                 0};
  in.nodes = {{1, "#3-2-1-1-1", {0, 0, 0}, 0, {true, true, true}},
              {2, "#3-0", {1, 0, 0}, 0, {false, true, true}}};
  in.edges = {{0, 1, 1e-4, 0}};
  in.exciter = {{1, 0, 0}, {0, 1}};
  in.pickup = in.exciter;
  in.pickup_linear_per_metre = 1000;
  in.max_force_newtons = 10;
  in.max_impulse_newton_seconds = 0.01;
  in.max_displacement_metres = 0.1;
  return in;
}
static Determination determination() {
  Determination d{};
  d.identity.instance = reference("controlled:retained-expression/instance");
  d.identity.event = reference("controlled:performance/event");
  d.identity.subject = reference("controlled:performance/subject");
  d.identity.m1_revision = 11;
  d.identity.m2_generation = 7;
  d.m1_coordinate = reference("#1-5-1");
  d.m1_face = 1;
  d.m2_writer = reference("#2-1");
  d.registry_revision = reference(ql_m_live_base_registry_revision());
  d.source_revision = reference("controlled:native-source-revision");
  d.relation_plan_ref = reference("controlled:relation-plan");
  d.tuning_ref = reference("controlled:explicit-rational-targets");
  d.native_receipt_ref =
      reference("controlled:consumer-test-no-producer-authentication");
  d.body_preparation_ref = reference("controlled:physical/preparation");
  d.body_state_ref = reference("controlled:physical/state");
  d.tick12 = 0;
  d.basis = 0;
  d.lens12 = 0;
  d.context_frame = 1;
  d.degree720 = 0;
  d.audio_octet_hz = {220, 247.5, 264, 293.3333333333333, 330, 352, 396, 412.5};
  d.nodal_quartet = {NodalBoundary{0, 0, 1, 1}, NodalBoundary{0, 1, 1, 2},
                     NodalBoundary{5, 0, 2, 1}, NodalBoundary{5, 1, 2, 2}};
  d.excitation.policy_ref = reference("controlled:D30/root-only-tone-policy");
  d.excitation.standing =
      reference("reference-controlled-not-native-source-authority");
  d.excitation.reference_hertz = 220;
  d.body_revision = 1;
  d.tuning_available = true;
  return d;
}
static NoteTarget target(const Determination &d, std::uint64_t member = 1,
                         std::uint64_t touch = 1, std::uint64_t numerator = 1,
                         std::uint64_t denominator = 1) {
  NoteTarget n{};
  n.identity = d.identity;
  n.source_coordinate = d.m1_coordinate;
  n.tuning_ref = d.tuning_ref;
  n.member = member;
  n.touch = touch;
  n.touch_ref =
      reference(("controlled:physical-touch/" + std::to_string(touch)).c_str());
  n.source_face = d.m1_face;
  n.key = std::uint8_t((member - 1) % 12);
  n.position = n.key / 2;
  n.coordinate_face = n.key % 2;
  n.pitch_class = n.key;
  n.fundamental_hz = 220;
  n.hertz = 220 * double(numerator) / double(denominator);
  n.ratio_numerator = numerator;
  n.ratio_denominator = denominator;
  n.exact_ratio = true;
  return n;
}
static Operation operation(const Determination &d, Kind kind,
                           std::uint64_t sequence, std::uint64_t sample) {
  Operation op{};
  op.identity = d.identity;
  op.kind = kind;
  op.sequence = sequence;
  op.sample = sample;
  return op;
}
static Operation note_on(const Determination &d, std::uint64_t sequence,
                         std::uint64_t sample, NoteTarget n) {
  auto op = operation(d, Kind::NoteOn, sequence, sample);
  op.note = n;
  op.value = 0.8;
  return op;
}
struct Fixture {
  std::shared_ptr<ql::PhysicalBody> body;
  std::shared_ptr<Engine> engine;
  Determination d = determination();
  explicit Fixture(Parameters p = {}) {
    body = std::make_shared<ql::PhysicalBody>(
        ql::PreparedPhysicalBody(physical_input()));
    engine = std::make_shared<Engine>(d, 48000, physical_port(body), p);
    engine->enable_capture(true);
  }
};
static Readback render(Fixture &f, std::size_t frames = 128,
                       Capture *capture = nullptr) {
  std::array<float, 512> output{};
  count_allocations.store(true);
  const bool ok =
      f.engine->render(output.data(), frames, f.engine->samples_elapsed());
  count_allocations.store(false);
  assert(ok);
  Readback receipt{}, latest{};
  bool found = false;
  while (f.engine->pop_readback(receipt)) {
    latest = receipt;
    found = true;
  }
  assert(found);
  Capture c{};
  found = false;
  while (f.engine->pop_capture(c)) {
    if (capture)
      *capture = c;
    found = true;
  }
  assert(found);
  if (capture)
    for (std::size_t i = 0; i < frames; ++i)
      assert(output[i] == capture->output_linear[i]);
  assert(latest.samples_elapsed == f.body->samples_elapsed());
  return latest;
}
static void render_until(Fixture &f, std::uint64_t end,
                         std::size_t block = 128) {
  while (f.engine->samples_elapsed() < end)
    render(f,
           std::min<std::uint64_t>(block, end - f.engine->samples_elapsed()));
}
static double energy(Fixture &f) { return f.body->mechanical_energy_joules(); }
static void scheduled_sample_and_shared_physical_state() {
  Fixture f;
  assert(f.engine->enqueue(note_on(f.d, 1, 137, target(f.d))) ==
         Result::Accepted);
  Capture c{};
  render(f, 256, &c);
  for (unsigned i = 0; i < 137; ++i)
    assert(c.force_newtons[i] == 0 && c.pickup_linear[i] == 0 &&
           c.output_linear[i] == 0);
  assert(c.force_newtons[137] != 0 && c.pickup_linear[137] != 0);
  std::array<ql::Vec3, 2> shape{};
  assert(f.body->write_displacements(shape.data(), shape.size(), 1, 256));
  near(c.pickup_linear[255], shape[1][0] * 1000, 1e-9);
  // The body is mass/stiffness-derived (~159.15Hz), not retuned to 220Hz.
  near(f.body->preparation().frequency_hz(0), 1000 / (2 * ql::pi), 1e-10);
  assert(!f.body->write_displacements(shape.data(), shape.size(), 1, 255));
  const auto before = energy(f);
  std::array<float, 128> output;
  output.fill(1);
  assert(!f.engine->render(output.data(), 128, 255));
  assert(f.engine->samples_elapsed() == 256 && energy(f) == before);
  for (float x : output)
    assert(x == 0);
}
static std::vector<double> force_run(std::size_t block, bool observe) {
  Fixture f;
  auto on = note_on(f.d, 1, 137, target(f.d));
  assert(f.engine->enqueue(on) == Result::Accepted);
  auto parameter = operation(f.d, Kind::Parameter, 2, 777);
  parameter.parameter = Parameter::ForceNewtons;
  parameter.value = 0.025;
  assert(f.engine->enqueue(parameter) == Result::Accepted);
  auto off = operation(f.d, Kind::NoteOff, 3, 3501);
  off.touch = 1;
  assert(f.engine->enqueue(off) == Result::Accepted);
  std::vector<double> result;
  result.reserve(8192 * 3);
  while (f.engine->samples_elapsed() < 8192) {
    Capture c{};
    render(f,
           std::min<std::uint64_t>(block, 8192 - f.engine->samples_elapsed()),
           &c);
    for (unsigned i = 0; i < c.frames; ++i) {
      result.push_back(c.force_newtons[i]);
      result.push_back(c.pickup_linear[i]);
      result.push_back(c.output_linear[i]);
    }
    if (observe) {
      std::array<ql::Vec3, 2> shape{};
      assert(f.body->write_visible_positions(shape.data(), 2, 1,
                                             f.body->samples_elapsed()));
    }
  }
  return result;
}
static void partition_and_view_independence() {
  const auto a = force_run(128, false), b = force_run(256, true),
             c = force_run(1, true);
  assert(a == b &&
         a == c); // ≤1 sample requirement: bit-identical sample timeline.
}
static void repeated_touches_sustain_release_and_body_tail() {
  Parameters p{};
  p.release_seconds = 0.001;
  Fixture f(p);
  assert(f.engine->enqueue(note_on(f.d, 1, 0, target(f.d, 1, 10))) ==
         Result::Accepted);
  assert(f.engine->enqueue(note_on(f.d, 2, 0, target(f.d, 1, 11))) ==
         Result::Accepted);
  auto r = render(f);
  assert(r.active_voices == 1 && r.active_touches == 2 &&
         r.voices[0].held_touches == 2);
  auto off = operation(f.d, Kind::NoteOff, 3, 128);
  off.touch = 10;
  assert(f.engine->enqueue(off) == Result::Accepted);
  r = render(f);
  assert(r.active_voices == 1 && r.active_touches == 1 &&
         !r.voices[0].releasing);
  auto pedal = operation(f.d, Kind::Sustain, 4, 256);
  pedal.value = 1;
  assert(f.engine->enqueue(pedal) == Result::Accepted);
  off = operation(f.d, Kind::NoteOff, 5, 256);
  off.touch = 11;
  assert(f.engine->enqueue(off) == Result::Accepted);
  r = render(f);
  assert(r.active_voices == 1 && r.active_touches == 0 && r.sustain &&
         !r.voices[0].releasing);
  pedal = operation(f.d, Kind::Sustain, 6, 384);
  pedal.value = 0;
  assert(f.engine->enqueue(pedal) == Result::Accepted);
  r = render(f);
  assert(r.active_voices == 0 && !r.sustain);
  const auto e = energy(f);
  assert(e > 0);
  Capture c{};
  r = render(f, 128, &c);
  for (double x : c.force_newtons)
    assert(x == 0);
  assert(energy(f) > 0 && energy(f) < e);
  bool audible = false;
  for (unsigned i = 0; i < c.frames; ++i)
    audible |= c.pickup_linear[i] != 0;
  assert(audible); // physical tail survives all keys and excitation voices.
}
static void finite_polyphony_and_panic_cancel_pending_attacks() {
  Parameters p{};
  p.release_seconds = 0.001;
  Fixture f(p);
  for (std::uint64_t i = 1; i <= 25; ++i)
    assert(f.engine->enqueue(note_on(f.d, i, 0, target(f.d, i, i))) ==
           Result::Accepted);
  auto r = render(f);
  assert(r.active_voices == 24 && r.active_touches == 24 && r.stolen == 1 &&
         r.clipping_samples == 0);
  assert(f.engine->enqueue(note_on(f.d, 26, 1000, target(f.d, 26, 26))) ==
         Result::Accepted);
  f.engine->request_panic();
  render_until(f, 1152);
  r = render(f);
  assert(r.active_touches == 0 && r.active_voices == 0);
  assert(f.engine->enqueue(note_on(f.d, 27, f.engine->samples_elapsed(),
                                   target(f.d, 27, 27))) == Result::Accepted);
  r = render(f);
  assert(r.active_voices == 1 && r.active_touches == 1);
}
static void late_release_invalid_inputs_and_source_generations() {
  Fixture f;
  auto on = note_on(f.d, 1, 0, target(f.d));
  assert(f.engine->enqueue(on) == Result::Accepted);
  render(f);
  on.sequence = 2;
  on.sample = 0;
  on.note.touch = 2;
  assert(f.engine->enqueue(on) == Result::Late);
  auto off = operation(f.d, Kind::NoteOff, 2, 0);
  off.touch = 1;
  assert(f.engine->enqueue(off) == Result::Accepted);
  auto r = render(f);
  assert(r.late == 1 && r.active_touches == 0 && r.voices[0].releasing);
  auto bad = operation(f.d, Kind::Parameter, 3, 256);
  bad.parameter = Parameter::ForceNewtons;
  bad.value = std::numeric_limits<double>::quiet_NaN();
  assert(f.engine->enqueue(bad) == Result::Invalid);
  bad.kind = Kind(255);
  assert(f.engine->enqueue(bad) == Result::Invalid);
  auto mismatch = note_on(f.d, 3, 256, target(f.d, 2, 2, 3, 2));
  mismatch.note.hertz = 331;
  assert(f.engine->enqueue(mismatch) == Result::Invalid);
  mismatch.note = target(f.d, 2, 2);
  mismatch.note.source_coordinate = reference("#2-1");
  assert(f.engine->enqueue(mismatch) == Result::Invalid);
  auto next = f.d;
  ++next.identity.m2_generation;
  ++next.identity.m1_revision;
  auto update = operation(f.d, Kind::Determination, 3, 256);
  update.determination = next;
  update.determination.m2_writer = reference("#2-4.3-1-0");
  assert(f.engine->enqueue(update) == Result::Invalid);
  update.determination = next;
  assert(f.engine->enqueue(update) == Result::Accepted);
  assert(f.engine->enqueue(note_on(f.d, 4, 256, target(f.d, 2, 2))) ==
         Result::Stale);
  assert(f.engine->enqueue(note_on(next, 4, 256, target(next, 2, 2, 3, 2))) ==
         Result::Accepted);
  Capture c{};
  r = render(f, 128, &c);
  assert(r.identity == next.identity && c.identity == f.d.identity &&
         c.end_identity == next.identity);
  bool retained = false, new_note = false;
  for (const auto &v : r.voices) {
    retained |= v.member == 1 && v.m2_generation == 7;
    new_note |= v.member == 2 && v.m2_generation == 8;
  }
  assert(retained && new_note);
  auto expression = operation(next, Kind::Expression, 5, 384);
  expression.touch = 2;
  expression.value = 0.5;
  expression.pitch_hz = 440;
  assert(f.engine->enqueue(expression) == Result::Invalid);
  expression.note = target(next, 2, 2, 2, 1);
  assert(f.engine->enqueue(expression) == Result::Accepted);
  r = render(f);
  for (const auto &v : r.voices)
    if (v.member == 2)
      assert(v.target_hertz == 440 && v.effective_hertz > 330 &&
             v.effective_hertz < 440);
}
static void rational_pitch_accuracy_and_parameter_readback() {
  Fixture f;
  assert(f.engine->enqueue(note_on(f.d, 1, 0, target(f.d, 1, 1, 3, 2))) ==
         Result::Accepted);
  auto parameter = operation(f.d, Kind::Parameter, 2, 0);
  parameter.parameter = Parameter::ForceNewtons;
  parameter.value = 0.02;
  assert(f.engine->enqueue(parameter) == Result::Accepted);
  std::vector<double> crossings;
  double last = 0;
  std::uint64_t index = 0;
  Readback r{};
  while (index < 48000) {
    Capture c{};
    r = render(f, 256, &c);
    for (unsigned i = 0; i < c.frames; ++i, ++index) {
      const double x = c.force_newtons[i];
      if (index > 2000 && last <= 0 && x > 0)
        crossings.push_back(double(index - 1) - last / (x - last));
      last = x;
    }
  }
  assert(crossings.size() > 200);
  const double hz =
      48000 * (crossings.size() - 1) / (crossings.back() - crossings.front());
  const double cents = 1200 * std::log2(hz / 330);
  assert(std::abs(cents) < 1);
  near(r.source.force_newtons, 0.02);
  near(r.effective.force_newtons, 0.02, 1e-12);
  assert(r.clipping_samples == 0 && r.refused == 0 && r.active_voices == 1);
  near(f.body->preparation().frequency_hz(0), 1000 / (2 * ql::pi));
  std::cout << "rational excitation cents_error=" << cents
            << " physical_eigenfrequency_hz="
            << f.body->preparation().frequency_hz(0) << '\n';
}
static void late_noteoff_overtakes_future_automation_without_moving_it() {
  Parameters p{};
  p.release_seconds = 0.001;
  Fixture f(p);
  assert(f.engine->enqueue(note_on(f.d, 1, 0, target(f.d))) ==
         Result::Accepted);
  auto future = operation(f.d, Kind::Parameter, 2, 48000);
  future.parameter = Parameter::MasterLinear;
  future.value = 0.5;
  assert(f.engine->enqueue(future) == Result::Accepted);
  render(f, 128);
  auto late = operation(f.d, Kind::NoteOff, 3, 0);
  late.touch = 1;
  assert(f.engine->enqueue(late) == Result::Accepted);
  auto r = render(f, 128);
  assert(r.active_touches == 0 && r.active_voices == 0 &&
         r.last_sequence == 3 && r.late == 1);
  near(r.source.master_linear, 0.25);
  // Ordinary live attacks must also overtake the retained future automation.
  assert(f.engine->enqueue(note_on(f.d, 4, 256, target(f.d, 2, 2))) ==
         Result::Accepted);
  r = render(f, 128);
  assert(r.active_voices == 1 && r.active_touches == 1);
  near(r.source.master_linear, 0.25);
  auto release = operation(f.d, Kind::NoteOff, 5, 384);
  release.touch = 2;
  assert(f.engine->enqueue(release) == Result::Accepted);
  r = render(f, 128);
  assert(r.active_touches == 0 && r.active_voices == 0);
  render_until(f, 48000);
  r = render(f, 1);
  near(r.source.master_linear, 0.5);
  assert(r.last_sequence == 5 && r.identity == f.d.identity);
  // Same-sample NoteOn followed by NoteOff respects source sequence order.
  auto on = note_on(f.d, 6, 48001, target(f.d, 3, 3));
  assert(f.engine->enqueue(on) == Result::Accepted);
  release = operation(f.d, Kind::NoteOff, 7, 48001);
  release.touch = 3;
  assert(f.engine->enqueue(release) == Result::Accepted);
  r = render(f, 128);
  assert(r.active_touches == 0 && r.active_voices == 0);
}
static void future_source_determination_preserves_current_live_gestures() {
  Parameters p{};
  p.release_seconds = 0.001;
  Fixture f(p);
  assert(f.engine->enqueue(note_on(f.d, 1, 0, target(f.d, 1, 1))) ==
         Result::Accepted);
  render(f);
  auto next = f.d;
  ++next.identity.m2_generation;
  ++next.identity.m1_revision;
  next.m1_coordinate = reference("#1-5-2");
  next.m1_face = 0;
  auto future = operation(f.d, Kind::Determination, 2, 1000);
  future.determination = next;
  assert(f.engine->enqueue(future) == Result::Accepted);
  assert(f.engine->enqueue(note_on(f.d, 3, 128, target(f.d, 2, 2))) ==
         Result::Accepted);
  auto r = render(f);
  assert(r.identity == f.d.identity && r.active_touches == 2);
  auto off = operation(f.d, Kind::NoteOff, 4, 256);
  off.touch = 1;
  assert(f.engine->enqueue(off) == Result::Accepted);
  r = render(f);
  assert(r.identity == f.d.identity && r.active_touches == 1 &&
         r.voices[0].member == 2);
  render_until(f, 1000);
  r = render(f);
  assert(r.identity == next.identity);
  assert(f.engine->enqueue(note_on(next, 5, 1128, target(next, 3, 3))) ==
         Result::Accepted);
  // The origin of an already held touch remains releasable after source turn.
  off = operation(f.d, Kind::NoteOff, 6, 1128);
  off.touch = 2;
  assert(f.engine->enqueue(off) == Result::Accepted);
  r = render(f);
  assert(r.active_touches == 1 && r.voices[0].member == 3 && r.refused == 0);
  assert(f.engine->enqueue(note_on(f.d, 7, 1256, target(f.d, 4, 4))) ==
         Result::Stale);
  assert(f.engine->enqueue(note_on(next, 7, 1256, target(next, 4, 4))) ==
         Result::Accepted);
  r = render(f);
  assert(r.active_touches == 2 && r.identity == next.identity &&
         r.refused == 0);
}
static void disconnected_body_refuses_and_overflow_recovers_explicitly() {
  Fixture stale;
  auto replacement = physical_input();
  replacement.body_revision = 2;
  replacement.preparation_ref = "controlled:other-physical-preparation";
  assert(
      stale.body->replace_material(ql::PreparedPhysicalBody(replacement), 1));
  std::array<float, 128> out;
  out.fill(1);
  assert(!stale.engine->render(out.data(), 128, 0));
  assert(stale.body->samples_elapsed() == 0);
  for (float x : out)
    assert(x == 0);
  Fixture f;
  assert(f.engine->enqueue(note_on(f.d, 1, 0, target(f.d))) ==
         Result::Accepted);
  render(f);
  for (std::uint64_t i = 2; i <= 65; ++i)
    assert(f.engine->enqueue(note_on(f.d, i, 128, target(f.d, 1, i))) ==
           Result::Accepted);
  auto active = render(f);
  assert(active.active_touches == 65 && active.active_voices == 1);
  for (std::uint64_t i = 2; i <= 65; ++i) {
    auto release = operation(f.d, Kind::NoteOff, i + 64, 256);
    release.touch = i;
    assert(f.engine->enqueue(release) == Result::Accepted);
  }
  auto off = operation(f.d, Kind::NoteOff, 130, 256);
  off.touch = 1;
  assert(f.engine->enqueue(off) == Result::Overflow);
  const auto e = energy(f);
  assert(!f.engine->render(out.data(), 128, 256));
  assert(!f.engine->available() && f.body->samples_elapsed() == 256 &&
         energy(f) == e);
  assert(f.engine->enqueue(off) == Result::Unavailable);
  for (float x : out)
    assert(x == 0);
  // Recovery is an explicit stopped-owner construction over the SAME P state.
  f.engine = std::make_shared<Engine>(f.d, 48000, physical_port(f.body));
  f.engine->enable_capture(true);
  assert(f.engine->samples_elapsed() == 256);
  auto r = render(f);
  assert(r.active_voices == 0 && r.active_touches == 0 && energy(f) > 0 &&
         energy(f) < e);
}
static void live_octet_consumer_changes_force_and_pcm_without_retuning_body() {
  Fixture a, b, severed;
  auto source = a.d;
  source.excitation.policy_ref =
      reference("controlled:D30/root-octet-causal-policy");
  source.excitation.root_linear = 0.5;
  source.excitation.octet_linear = 0.5;
  auto changed = source;
  changed.identity.m2_generation++;
  for (auto &hz : changed.audio_octet_hz)
    hz *= 1.25;
  for (auto *f : {&a, &b, &severed}) {
    f->d = source;
    f->engine = std::make_shared<Engine>(source, 48000, physical_port(f->body));
    f->engine->enable_capture(true);
    assert(f->engine->enqueue(note_on(source, 1, 0, target(source))) ==
           Result::Accepted);
  }
  // Controlled native consumer vectors: lawful production derivation is
  // separately tested by the real Rust->C++ fixture, not inferred here.
  auto turn = operation(source, Kind::Determination, 2, 512);
  turn.determination = changed;
  assert(b.engine->enqueue(turn) == Result::Accepted);
  // Severed-negative: a populated producer copy changes without reaching
  // the actual callback's admitted source. The audio must match baseline.
  severed.d = changed;
  bool force_diff = false, pcm_diff = false;
  for (unsigned start = 0; start < 2048; start += 128) {
    Capture ca{}, cb{}, cc{};
    const auto ra = render(a, 128, &ca), rb = render(b, 128, &cb);
    render(severed, 128, &cc);
    assert(ca.force_newtons == cc.force_newtons &&
           ca.pickup_linear == cc.pickup_linear);
    if (start < 512)
      assert(ca.force_newtons == cb.force_newtons);
    for (unsigned i = 0; i < 128; ++i) {
      force_diff |= ca.force_newtons[i] != cb.force_newtons[i];
      pcm_diff |= ca.pickup_linear[i] != cb.pickup_linear[i];
    }
    assert(ra.physical.samples_elapsed == a.body->samples_elapsed() &&
           rb.physical.samples_elapsed == b.body->samples_elapsed());
    near(ra.physical.visible_positions_metres[1][0] - 1,
         ca.pickup_linear[127] / 1000, 1e-11);
  }
  assert(force_diff && pcm_diff);
  near(a.body->preparation().frequency_hz(0),
       b.body->preparation().frequency_hz(0), 0);
}
static std::vector<double> continuation(Fixture &f, std::uint64_t end) {
  std::vector<double> values;
  values.reserve((end - f.engine->samples_elapsed()) * 3);
  while (f.engine->samples_elapsed() < end) {
    Capture c{};
    render(f, std::min<std::uint64_t>(128, end - f.engine->samples_elapsed()),
           &c);
    for (unsigned i = 0; i < c.frames; ++i) {
      values.push_back(c.force_newtons[i]);
      values.push_back(c.pickup_linear[i]);
      values.push_back(c.output_linear[i]);
    }
  }
  return values;
}
static void exact_checkpoint_reopen_pending_events_tails_and_atomic_refusal() {
  Fixture f;
  f.d.excitation.root_linear = 0.5;
  f.d.excitation.octet_linear = 0.5;
  f.engine = std::make_shared<Engine>(f.d, 48000, physical_port(f.body));
  f.engine->enable_capture(true);
  for (std::uint64_t i = 1; i <= 25; ++i)
    assert(f.engine->enqueue(note_on(f.d, i, 0, target(f.d, i, i))) ==
           Result::Accepted);
  render(f, 17); // Live oscillator/envelope/filter states plus a steal tail.
  auto repeated = note_on(f.d, 26, 17, target(f.d, 2, 100));
  assert(f.engine->enqueue(repeated) == Result::Accepted);
  auto pedal = operation(f.d, Kind::Sustain, 27, 17);
  pedal.value = 1;
  assert(f.engine->enqueue(pedal) == Result::Accepted);
  auto off = operation(f.d, Kind::NoteOff, 28, 31);
  off.touch = 2;
  assert(f.engine->enqueue(off) == Result::Accepted);
  auto parameter = operation(f.d, Kind::Parameter, 29, 333);
  parameter.parameter = Parameter::ForceNewtons;
  parameter.value = .025;
  assert(f.engine->enqueue(parameter) == Result::Accepted);
  auto next = f.d;
  next.identity.m2_generation++;
  next.identity.m1_revision++;
  for (auto &hz : next.audio_octet_hz)
    hz *= 1.125;
  auto turn = operation(f.d, Kind::Determination, 30, 500);
  turn.determination = next;
  assert(f.engine->enqueue(turn) == Result::Accepted);
  assert(f.engine->enqueue(note_on(next, 31, 520, target(next, 30, 200))) ==
         Result::Accepted);
  off = operation(f.d, Kind::NoteOff, 32, 555);
  off.touch = 100;
  assert(f.engine->enqueue(off) == Result::Accepted);
  render(f, 18); // Both fixed pending queues now contain future operations.
  // Also retain newly admitted SPSC entries not consumed by a callback yet.
  auto panic = operation(next, Kind::Panic, 33, 777);
  assert(f.engine->enqueue(panic) == Result::Accepted);
  assert(f.engine->begin_device_callbacks());
  assert(!f.engine->acquire_stopped_custody());
  f.engine->end_device_callbacks_after_stop();
  std::unique_ptr<PairedCheckpoint> saved;
  {
    auto guard = f.engine->acquire_stopped_custody();
    assert(guard);
    saved = checkpoint_heap(*f.engine, *f.body, guard);
    assert(saved->audio.tails[0].left ||
           std::any_of(saved->audio.tails.begin(), saved->audio.tails.end(),
                       [](auto &t) { return t.left; }));
    assert(!f.engine->begin_device_callbacks());
    std::array<float, 128> output;
    output.fill(1);
    assert(!f.engine->render(output.data(), 128, 35));
    for (float x : output)
      assert(x == 0);
  }
  const auto expected = continuation(f, 1400);
  auto wire = checkpoint_transport::checkpoint_wire(*saved);
  const std::string serialized =
      json_object_to_json_string_ext(wire.get(), JSON_C_TO_STRING_PLAIN);
  auto parsed = ql::physical_wire::own(json_tokener_parse(serialized.c_str()));
  assert(parsed);
  auto decoded = checkpoint_transport::read_checkpoint_wire(parsed.get());
  Fixture reopened;
  {
    auto guard = reopened.engine->acquire_stopped_custody();
    assert(guard);
    assert(restore_checkpoint(*reopened.engine, *reopened.body, *decoded, guard,
                              0));
  }
  const auto actual = continuation(reopened, 1400);
  assert(actual == expected);
  // Rewind the same resident work by exact state, without replay from start.
  {
    auto guard = reopened.engine->acquire_stopped_custody();
    const auto before = reopened.body->checkpoint();
    decoded->audio.voices[0].sine = std::numeric_limits<double>::quiet_NaN();
    assert(!restore_checkpoint(*reopened.engine, *reopened.body, *decoded,
                               guard, 1400));
    assert(reopened.body->checkpoint().displacement_modal_metres ==
           before.displacement_modal_metres);
    decoded = checkpoint_transport::read_checkpoint_wire(parsed.get());
    decoded->physical.samples_elapsed++;
    assert(!restore_checkpoint(*reopened.engine, *reopened.body, *decoded,
                               guard, 1400));
    decoded = checkpoint_transport::read_checkpoint_wire(parsed.get());
    decoded->physical.eigenbasis_identity = "detached";
    assert(!restore_checkpoint(*reopened.engine, *reopened.body, *decoded,
                               guard, 1400));
    assert(reopened.engine->samples_elapsed() == 1400 &&
           reopened.body->samples_elapsed() == 1400);
    decoded = checkpoint_transport::read_checkpoint_wire(parsed.get());
    assert(!restore_checkpoint(*reopened.engine, *reopened.body, *decoded,
                               guard, 1399));
    assert(restore_checkpoint(*reopened.engine, *reopened.body, *decoded, guard,
                              1400));
  }
  assert(continuation(reopened, 1400) == expected);
  // Structural wire refusal precedes resident-state mutation.
  json_object_object_add(parsed.get(), "unknown", json_object_new_int(1));
  bool refused = false;
  try {
    checkpoint_transport::read_checkpoint_wire(parsed.get());
  } catch (const std::invalid_argument &) {
    refused = true;
  }
  assert(refused);
}
// A real producer refills the native SPSC while the consumer is inside its
// first accepted entry. Its new entry must remain for the next bounded drain.
static void bounded_spsc_snapshot_survives_concurrent_refill() {
  Spsc<std::uint64_t, 4> queue;
  std::atomic<unsigned> phase{0};
  std::thread producer([&] {
    assert(queue.push(1));
    assert(queue.push(2));
    phase.store(1, std::memory_order_release);
    while (phase.load(std::memory_order_acquire) != 2)
      std::this_thread::yield();
    assert(queue.push(3));
    assert(queue.push(4));
    phase.store(3, std::memory_order_release);
  });
  while (phase.load(std::memory_order_acquire) != 1)
    std::this_thread::yield();
  std::array<std::uint64_t, 4> consumed{};
  std::size_t count = 0;
  assert(queue.drain_snapshot([&](std::uint64_t value) noexcept {
    consumed[count++] = value;
    if (value == 1) {
      phase.store(2, std::memory_order_release);
      while (phase.load(std::memory_order_acquire) != 3)
        std::this_thread::yield();
    }
    return true;
  }));
  producer.join();
  assert(count == 2 && consumed[0] == 1 && consumed[1] == 2);
  assert(queue.drain_snapshot([&](std::uint64_t value) noexcept {
    consumed[count++] = value;
    return true;
  }));
  assert(count == 4 && consumed[2] == 3 && consumed[3] == 4);
}
static void relative_octet_band_limits_preserve_exact_checkpoint() {
  Fixture f;
  f.d.excitation.root_linear = 0;
  f.d.excitation.octet_linear = 1;
  f.d.excitation.reference_hertz = 21600;
  f.d.audio_octet_hz.fill(0.001);
  f.engine = std::make_shared<Engine>(f.d, 48000, physical_port(f.body));
  f.engine->enable_capture(true);
  auto low = target(f.d);
  low.fundamental_hz = low.hertz = 0.001;
  assert(f.engine->enqueue(note_on(f.d, 1, 0, low)) == Result::Accepted);
  Capture capture{};
  auto r = render(f, 128, &capture);
  assert(r.voices[0].suppressed_octet_components == 8);
  for (auto frequency : r.voices[0].octet_effective_hertz)
    assert(frequency == 0.001);
  for (unsigned i = 0; i < capture.frames; ++i)
    assert(capture.force_newtons[i] == 0 && capture.pickup_linear[i] == 0);
  {
    auto guard = f.engine->acquire_stopped_custody();
    auto cp = checkpoint_heap(*f.engine, *f.body, guard);
    assert(f.engine->validate_checkpoint(cp->audio, guard, 128));
  }
  Fixture high;
  high.d.excitation.root_linear = 0;
  high.d.excitation.octet_linear = 1;
  high.d.excitation.reference_hertz = 0.001;
  high.d.audio_octet_hz.fill(21600);
  high.engine =
      std::make_shared<Engine>(high.d, 48000, physical_port(high.body));
  high.engine->enable_capture(true);
  auto upper = target(high.d);
  upper.fundamental_hz = upper.hertz = 21600;
  assert(high.engine->enqueue(note_on(high.d, 1, 0, upper)) ==
         Result::Accepted);
  r = render(high, 128, &capture);
  assert(r.voices[0].suppressed_octet_components == 8);
  for (auto frequency : r.voices[0].octet_effective_hertz)
    assert(frequency == 21600);
  {
    auto guard = high.engine->acquire_stopped_custody();
    auto cp = checkpoint_heap(*high.engine, *high.body, guard);
    assert(high.engine->validate_checkpoint(cp->audio, guard, 128));
  }
}
int main() {
  count_allocations.store(true);
  void *probe = ::operator new(32);
  count_allocations.store(false);
  ::operator delete(probe);
  assert(callback_allocations.load() == 1);
  callback_allocations.store(0);
  std::cout << "native_fixed_sizes_bytes Engine=" << sizeof(Engine)
            << " Operation=" << sizeof(Operation)
            << " Readback=" << sizeof(Readback)
            << " EngineCheckpoint=" << sizeof(Engine::Checkpoint)
            << " PairedCheckpoint=" << sizeof(PairedCheckpoint) << '\n';
  scheduled_sample_and_shared_physical_state();
  partition_and_view_independence();
  repeated_touches_sustain_release_and_body_tail();
  finite_polyphony_and_panic_cancel_pending_attacks();
  late_release_invalid_inputs_and_source_generations();
  rational_pitch_accuracy_and_parameter_readback();
  late_noteoff_overtakes_future_automation_without_moving_it();
  future_source_determination_preserves_current_live_gestures();
  disconnected_body_refuses_and_overflow_recovers_explicitly();
  live_octet_consumer_changes_force_and_pcm_without_retuning_body();
  exact_checkpoint_reopen_pending_events_tails_and_atomic_refusal();
  bounded_spsc_snapshot_survives_concurrent_refill();
  relative_octet_band_limits_preserve_exact_checkpoint();
  assert(callback_allocations.load() == 0);
  std::cout << "real native performance/physical tests passed; "
               "callback_Cpp_allocations=0\n";
}
