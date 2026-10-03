// Actual Rust N receiving/replay and M1/M2/K/P admission enter the retained A
// callback through the one resident P numerical owner. No substitute dynamics.
#include "../test_support/allocation_hooks.hpp"
#include "../test_support/native_fixture_carrier.hpp"

#include "../test_support/independent_genuine_carrier_scalar_cases.hpp"
#include <cassert>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <new>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_management.hpp>
#include <ql/performance_management_wire.hpp>
#include <ql/performance_packet.hpp>
#include <ql/performance_physical_revision.hpp>
#include <ql/performance_physical_routes.hpp>
#include <ql/performance_route_programs.hpp>
#include <ql/physical_transition.hpp>

static bool callback_probe = false;
static std::size_t allocations = 0, releases = 0;
void *operator new(std::size_t bytes) {
  if (callback_probe)
    ++allocations;
  if (void *p = ql_test_allocate(bytes))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t bytes) { return ::operator new(bytes); }
void operator delete(void *p) noexcept {
  if (p && callback_probe)
    ++releases;
  ql_test_release(p);
}
void operator delete[](void *p) noexcept { ::operator delete(p); }
void operator delete(void *p, std::size_t) noexcept { ::operator delete(p); }
void operator delete[](void *p, std::size_t) noexcept { ::operator delete(p); }
void *operator new(std::size_t bytes, std::align_val_t a) {
  if (callback_probe)
    ++allocations;
  if (void *p = ql_test_allocate_aligned(bytes, std::size_t(a)))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t bytes, std::align_val_t a) {
  return ::operator new(bytes, a);
}
void operator delete(void *p, std::align_val_t) noexcept {
  ::operator delete(p);
}
void operator delete[](void *p, std::align_val_t a) noexcept {
  ::operator delete(p, a);
}
void operator delete(void *p, std::size_t, std::align_val_t a) noexcept {
  ::operator delete(p, a);
}
void operator delete[](void *p, std::size_t, std::align_val_t a) noexcept {
  ::operator delete(p, a);
}

using namespace ql;
using namespace ql::performance;
using J = json_object;
using Json = ql::physical_wire::Json;
template <class F> static void refused(F action) {
  bool rejected = false;
  try {
    action();
  } catch (const std::invalid_argument &) {
    rejected = true;
  }
  assert(rejected);
}
static PhysicalForceSourceBasis source_basis(J *value) {
  using namespace ql::physical_wire;
  keys(value, {"event_ref", "subject_ref", "registry_revision",
               "source_revision", "definition_ref", "source_instance_ref",
               "determination_ref", "m2_writer_coordinate", "m2_pratibimba",
               "m1_revision", "m2_generation", "m3_generation"});
  PhysicalForceSourceBasis out;
  out.event_ref = text(field(value, "event_ref"));
  out.subject_ref = text(field(value, "subject_ref"));
  out.registry_revision = text(field(value, "registry_revision"));
  out.source_revision = text(field(value, "source_revision"));
  out.definition_ref = text(field(value, "definition_ref"));
  out.source_instance_ref = text(field(value, "source_instance_ref"));
  out.determination_ref = text(field(value, "determination_ref"));
  out.m2_writer_coordinate = text(field(value, "m2_writer_coordinate"));
  out.m2_pratibimba = boolean(field(value, "m2_pratibimba"));
  out.m1_revision = checkpoint_decimal(field(value, "m1_revision"));
  out.m2_generation = checkpoint_decimal(field(value, "m2_generation"));
  out.m3_generation = checkpoint_decimal(field(value, "m3_generation"));
  return out;
}
static std::vector<std::string> program_refs(J *value) {
  using namespace ql::physical_wire;
  std::vector<std::string> out;
  const auto n = count(value, physical_max_personal_force_routes);
  for (std::size_t i = 0; i < n; ++i)
    out.push_back(text(at(value, i)));
  return out;
}
struct Joined {
  NativePerformance native;
  std::shared_ptr<PhysicalRoutesPortBinding> binding;
  std::shared_ptr<const ql::physical_wire::PreparedNativePhysicalForceRoutes>
      qualified;
  NativeRouteProgramSet programmes{};
  std::shared_ptr<const AdmittedNativeReceivingSource> source_admission;
};
static Joined prepare(J *fixture, bool world = false, int enabled_route = -1,
                      double requested_note_force = .01) {
  auto *admission = packet::field(fixture, "performance_preparation");
  const std::string bytes =
      json_object_to_json_string_ext(admission, JSON_C_TO_STRING_PLAIN);
  auto native = prepare_performance_packet(
      bytes, packet::field(admission, "native_basis"), true, true);
  const auto immutable = native.body->preparation(); // fresh, stopped owner
  const auto refs = world
                        ? std::vector<std::string>{}
                        : program_refs(packet::field(fixture, "program_refs"));
  auto source_admission = std::make_shared<const AdmittedNativeReceivingSource>(
      read_native_receiving_admission(
          packet::field(fixture,
                        world ? "world_native_admission" : "native_admission"),
          packet::field(fixture, world ? "world_current_native_admission"
                                       : "current_native_admission"),
          native, packet::field(admission, "native_basis"), immutable, refs,
          0));
  auto qualified = source_admission->routes();
  auto binding = std::make_shared<PhysicalRoutesPortBinding>(
      native.body, source_admission, immutable, native.determination,
      packet::field(admission, "native_basis"), true, 0);
  const auto scalar = physical_port(native.body);
  const auto joined = physical_routes_port(scalar, binding);
  assert(joined.owner == native.body.get() &&
         joined.custody.get() == native.body.get());
  assert(joined.advance == scalar.advance && joined.observe == scalar.observe &&
         joined.cursor == scalar.cursor && joined.revision == scalar.revision);
  assert(joined.route_manifest == &binding->manifest() &&
         joined.routes_owner == binding.get());
  Parameters parameters;
  parameters.monitor_linear = 0; // rendered output must be physical pickup
  parameters.force_newtons = requested_note_force;
  parameters.release_seconds =
      .003; // admitted bounded linear release,144 samples
  native.engine = std::make_shared<Engine>(
      native.determination, immutable.input().sample_rate, joined, parameters);
  // This real callback trial requires the original PCM/force capture queue.
  native.engine->enable_capture(true);
  NativeRouteProgramSet programmes;
  programmes.manifest = binding->manifest();
  programmes.program_count = programmes.manifest.route_count;
  programmes.scalar_note_enabled = true;
  programmes.scalar_note_gain = 1;
  assert(!native.notes.empty());
  for (std::size_t i = 0; i < programmes.program_count; ++i) {
    auto &p = programmes.programs[i];
    p.handle = programmes.manifest.programs[i];
    p.phase_source_ref = native.determination.m1_coordinate;
    p.sine = native.notes.front().phase_sin;
    p.cosine = native.notes.front().phase_cos;
    p.enabled = enabled_route < 0 || i == std::size_t(enabled_route);
    p.target_gain = p.effective_gain = .5;
  }
  {
    auto custody = native.engine->acquire_stopped_custody();
    assert(native.engine->install_route_programs(programmes, custody, 0));
  }
  assert(native.engine->owns_physical_owner(native.body.get()));
  return {std::move(native), std::move(binding), std::move(qualified),
          programmes, std::move(source_admission)};
}
static Operation operation(const Determination &d, Kind kind,
                           std::uint64_t sequence, std::uint64_t sample) {
  Operation out;
  out.kind = kind;
  out.identity = d.identity;
  out.sequence = sequence;
  out.sample = sample;
  return out;
}
static void note(Joined &joined, std::uint64_t sequence = 1,
                 std::uint64_t sample = 0, std::size_t key = 0) {
  auto op =
      operation(joined.native.determination, Kind::NoteOn, sequence, sample);
  op.note = joined.native.notes.at(key);
  op.value = .8;
  assert(joined.native.engine->enqueue(op) == Result::Accepted);
}
struct Trace {
  std::vector<float> pcm, pickup;
  std::vector<double> note_force;
  PhysicalSnapshot snapshot{};
  PhysicalForceRouteReceipt receipt{};
  std::uint64_t force_limited_samples = 0;
};
static Trace render(Joined &joined, std::size_t count,
                    std::size_t partition = max_frames) {
  assert(count && count % partition == 0 && partition <= max_frames);
  Trace out;
  out.pcm.resize(count);
  out.pickup.resize(count);
  out.note_force.resize(count);
  for (std::size_t base = 0; base < count; base += partition) {
    const auto start = joined.native.engine->samples_elapsed();
    callback_probe = true;
    const bool ok =
        joined.native.engine->render(out.pcm.data() + base, partition, start);
    callback_probe = false;
    assert(ok && allocations == 0 && releases == 0);
    Readback readback;
    Capture capture;
    assert(joined.native.engine->pop_readback(readback) &&
           joined.native.engine->pop_capture(capture));
    assert(readback.available && readback.samples_elapsed == start + partition);
    assert(readback.physical.samples_elapsed == readback.samples_elapsed &&
           readback.physical.body_revision == readback.body_revision);
    assert(capture.start_sample == start && capture.frames == partition &&
           capture.body_revision == readback.physical.body_revision);
    assert(readback.physical.pickup_linear ==
           capture.pickup_linear[partition - 1]);
    std::copy_n(capture.pickup_linear.begin(), partition,
                out.pickup.begin() + base);
    std::copy_n(capture.force_newtons.begin(), partition,
                out.note_force.begin() + base);
    for (std::size_t sample = 0; sample < partition; ++sample) {
      double total = std::abs(capture.force_newtons[sample]);
      for (std::size_t route = 0; route < capture.route_count; ++route)
        total += std::abs(capture.route_force_newtons[route][sample]);
      assert(total <= joined.binding->manifest().max_force_newtons);
    }
    out.force_limited_samples = readback.force_limited_samples;
    out.snapshot = readback.physical;
    auto custody = joined.native.engine->acquire_stopped_custody();
    assert(joined.binding->observe_routes(out.receipt, readback.body_revision,
                                          start + partition));
    assert(out.receipt.source_basis_seal ==
               joined.binding->manifest().source_basis_seal &&
           out.receipt.route_count == joined.programmes.program_count &&
           out.receipt.start_sample == start &&
           out.receipt.end_sample == readback.samples_elapsed);
  }
  return out;
}
static void source_and_independent_routes(J *fixture) {
  auto joined = prepare(fixture);
  const auto &manifest = joined.binding->manifest();
  assert(manifest.route_count == 9 && manifest.earth_frame_node_id);
  assert(manifest.scalar_note_enabled && manifest.scalar_note_gain == 1 &&
         !manifest.legacy_native_scalar_enabled &&
         manifest.legacy_native_scalar_gain == 0);
  std::array<bool, 10> planets{};
  std::array<bool, 7> centres{};
  for (std::size_t i = 0; i < 9; ++i) {
    const auto &p = manifest.programs[i];
    assert(p.route_index == i && p.preparation_seal && p.program_seal);
    assert(p.native_planet_index < 10 && p.native_planet_index != 7 &&
           !planets[p.native_planet_index]);
    assert(p.planet_node_id != manifest.earth_frame_node_id &&
           p.centre_ordinal < 7);
    planets[p.native_planet_index] = true;
    centres[p.centre_ordinal] = true;
    assert(p.source_hertz == joined.qualified->programs[i].source_hertz &&
           p.peak_force_newtons ==
               joined.qualified->programs[i].peak_force_newtons &&
           std::abs(double(p.share_numerator) / double(p.share_denominator) -
                    p.original_denominator_share) < 1e-14);
  }
  for (bool present : centres)
    assert(present);
  auto full = render(joined, 512);
  assert(std::any_of(full.pickup.begin(), full.pickup.end(),
                     [](float v) { return v != 0; }));
  assert(std::all_of(full.note_force.begin(), full.note_force.end(),
                     [](double v) { return v == 0; }));
  assert(joined.native.body->mechanical_energy_joules() > 0);
  for (std::size_t i = 0; i < 9; ++i) {
    auto one = prepare(fixture, false, int(i));
    const auto trace = render(one, 512);
    assert(trace.pickup != full.pickup &&
           trace.snapshot.state_ref == full.snapshot.state_ref);
    assert(one.native.body->mechanical_energy_joules() > 0 &&
           trace.receipt.routes[i].enabled);
    for (std::size_t r = 0; r < 9; ++r) {
      assert(trace.receipt.routes[r].enabled == (r == i));
      assert(trace.receipt.routes[r].preparation_seal ==
             manifest.programs[r].preparation_seal);
      assert(trace.receipt.routes[r].peak_applied_force_newtons <=
             manifest.programs[r].peak_force_newtons * .5);
    }
  }
}
static void genuine_note_release_panic(J *fixture) {
  auto baseline = prepare(fixture), keyed = prepare(fixture);
  note(keyed);
  const auto native_only = render(baseline, 512),
             with_note = render(keyed, 512);
  assert(with_note.pickup != native_only.pickup &&
         with_note.pcm != native_only.pcm &&
         with_note.snapshot.visible_positions_metres !=
             native_only.snapshot.visible_positions_metres);
  assert(keyed.native.body->checkpoint().displacement_modal_metres !=
         baseline.native.body->checkpoint().displacement_modal_metres);
  assert(std::any_of(with_note.note_force.begin(), with_note.note_force.end(),
                     [](double v) { return v != 0; }));
  for (std::size_t i = 0; i < 9; ++i) {
    assert(keyed.programmes.programs[i].handle.driver_ref ==
           baseline.programmes.programs[i].handle.driver_ref);
    assert(keyed.programmes.programs[i].handle.source_hertz ==
           baseline.programmes.programs[i].handle.source_hertz);
  }
  auto held = prepare(fixture), released = prepare(fixture),
       panicked = prepare(fixture);
  note(held);
  note(released);
  note(panicked);
  note(held, 2, 0, 1);
  note(released, 2, 0, 1);
  note(panicked, 2, 0, 1);
  auto off = operation(released.native.determination, Kind::NoteOff, 3, 256);
  off.touch = released.native.notes.front().touch;
  assert(released.native.engine->enqueue(off) == Result::Accepted);
  auto panic = operation(panicked.native.determination, Kind::Panic, 3, 256);
  assert(panicked.native.engine->enqueue(panic) == Result::Accepted);
  const auto hold = render(held, 1024), release = render(released, 1024),
             kill = render(panicked, 1024);
  assert(hold.pickup != release.pickup && hold.pickup != kill.pickup &&
         release.pickup != kill.pickup);
  assert(hold.snapshot.visible_positions_metres !=
             release.snapshot.visible_positions_metres &&
         hold.snapshot.visible_positions_metres !=
             kill.snapshot.visible_positions_metres);
  assert(std::all_of(kill.note_force.begin() + 512, kill.note_force.end(),
                     [](double v) { return v == 0; }));
  // Panic ends genuine note forcing, while independent N sources keep this
  // same body sounding. A accepted control alone would not satisfy this.
  assert(std::any_of(kill.pickup.begin() + 512, kill.pickup.end(),
                     [](float v) { return v != 0; }));
}
static void checkpoint_and_partitions(J *fixture) {
  auto original = prepare(fixture);
  note(original, 1, 137);
  auto off = operation(original.native.determination, Kind::NoteOff, 2, 768);
  off.touch = original.native.notes.front().touch;
  assert(original.native.engine->enqueue(off) == Result::Accepted);
  render(original, 512);
  std::unique_ptr<PairedCheckpoint> saved;
  {
    auto custody = original.native.engine->acquire_stopped_custody();
    saved = checkpoint_heap(*original.native.engine, *original.native.body,
                            custody);
  }
  assert(saved->audio.cursor == 512 && saved->physical.samples_elapsed == 512);
  assert(saved->audio.has_route_programs &&
         saved->audio.route_programs.program_count == 9);
  bool phases_advanced = false;
  for (std::size_t i = 0; i < 9; ++i) {
    const auto &p = saved->audio.route_programs.programs[i];
    phases_advanced |= p.sine != original.programmes.programs[i].sine ||
                       p.cosine != original.programmes.programs[i].cosine;
    assert(
        p.handle.program_ref ==
            original.programmes.programs[i].handle.program_ref &&
        p.handle.preparation_seal ==
            original.programmes.programs[i].handle.preparation_seal &&
        p.handle.source_hertz ==
            original.programmes.programs[i].handle.source_hertz &&
        p.handle.original_denominator_share ==
            original.programmes.programs[i].handle.original_denominator_share);
  }
  assert(phases_advanced);
  auto wire = checkpoint_transport::checkpoint_wire(*saved);
  auto retained = checkpoint_transport::read_checkpoint_wire(wire.get());
  auto reopened = prepare(fixture);
  auto broken = std::make_unique<PairedCheckpoint>(*retained);
  broken->audio.route_programs.programs[0].handle.preparation_seal ^= 1;
  const auto uncommitted = reopened.native.body->checkpoint();
  {
    auto custody = reopened.native.engine->acquire_stopped_custody();
    assert(!restore_checkpoint(*reopened.native.engine, *reopened.native.body,
                               *broken, custody, 0));
  }
  assert(reopened.native.body->samples_elapsed() == 0 &&
         reopened.native.body->checkpoint().displacement_modal_metres ==
             uncommitted.displacement_modal_metres);
  {
    auto custody = reopened.native.engine->acquire_stopped_custody();
    assert(restore_checkpoint(*reopened.native.engine, *reopened.native.body,
                              *retained, custody, 0));
  }
  const auto uninterrupted = render(original, 1024, 128),
             resumed = render(reopened, 1024, 128);
  assert(uninterrupted.pcm == resumed.pcm &&
         uninterrupted.pickup == resumed.pickup &&
         uninterrupted.note_force == resumed.note_force &&
         uninterrupted.snapshot.visible_positions_metres ==
             resumed.snapshot.visible_positions_metres);
  assert(original.native.body->checkpoint().displacement_modal_metres ==
         reopened.native.body->checkpoint().displacement_modal_metres);
  assert(original.native.body->checkpoint().velocity_modal_metres_per_second ==
         reopened.native.body->checkpoint().velocity_modal_metres_per_second);
  auto lossy = checkpoint_transport::checkpoint_wire(*saved);
  json_object_object_del(packet::field(lossy.get(), "audio"), "route_programs");
  refused([&] { checkpoint_transport::read_checkpoint_wire(lossy.get()); });
  auto partitioned = prepare(fixture), blocked = prepare(fixture);
  const auto one = render(partitioned, 1024, 1),
             many = render(blocked, 1024, 512);
  assert(one.pcm == many.pcm && one.pickup == many.pickup);
}
static void qualification_and_atomic_refusal(J *fixture) {
  auto joined = prepare(fixture);
  const auto immutable = joined.native.body->preparation();
  const auto basis = source_basis(packet::field(fixture, "source_basis"));
  auto wrong = joined.native.determination;
  wrong.m2_face = 0;
  refused([&] {
    PhysicalRoutesPortBinding(
        joined.native.body, joined.source_admission, immutable, wrong,
        packet::field(packet::field(fixture, "performance_preparation"),
                      "native_basis"),
        true, 0);
  });
  wrong = joined.native.determination;
  wrong.m2_writer = reference("#2-5-5");
  refused([&] {
    PhysicalRoutesPortBinding(
        joined.native.body, joined.source_admission, immutable, wrong,
        packet::field(packet::field(fixture, "performance_preparation"),
                      "native_basis"),
        true, 0);
  });
  wrong = joined.native.determination;
  ++wrong.identity.m1_revision;
  refused([&] {
    PhysicalRoutesPortBinding(
        joined.native.body, joined.source_admission, immutable, wrong,
        packet::field(packet::field(fixture, "performance_preparation"),
                      "native_basis"),
        true, 0);
  });
  refused([&] {
    PhysicalRoutesPortBinding(
        joined.native.body, joined.source_admission, immutable,
        joined.native.determination,
        packet::field(packet::field(fixture, "performance_preparation"),
                      "native_basis"),
        false, 0);
  });
  // Genuine same-generation post-command M3 pose produced by the native
  // Rust owner, with the exact same audio Determination. D-only matching is
  // deliberately insufficient; old typed admission must fail before mutation.
  auto *pose = packet::field(fixture, "after_pose");
  auto *pose_preparation = packet::field(pose, "performance_preparation");
  assert(json_object_equal(
      packet::field(pose_preparation, "determination"),
      packet::field(packet::field(fixture, "performance_preparation"),
                    "determination")));
  refused([&] {
    PhysicalRoutesPortBinding(joined.native.body, joined.source_admission,
                              immutable, joined.native.determination,
                              packet::field(pose_preparation, "native_basis"),
                              true, 0);
  });
  auto detached = std::make_shared<PhysicalBody>(immutable);
  refused(
      [&] { physical_routes_port(physical_port(detached), joined.binding); });
  // Current World is an independently produced context operation, not an
  // edited copy. A held personal operation must fail against that current.
  refused([&] {
    ql::physical_wire::read_prepared_physical_force_routes(
        packet::field(fixture, "operation"),
        packet::field(fixture, "world_current_operation"), basis, immutable,
        program_refs(packet::field(fixture, "program_refs")), 0);
  });
  auto invalid = joined.programmes;
  invalid.programs[0].handle.preparation_seal ^= 1;
  {
    auto custody = joined.native.engine->acquire_stopped_custody();
    assert(!joined.native.engine->install_route_programs(invalid, custody, 0));
  }
  invalid = joined.programmes;
  invalid.programs[0].handle.source_hertz *= 2;
  {
    auto custody = joined.native.engine->acquire_stopped_custody();
    assert(!joined.native.engine->install_route_programs(invalid, custody, 0));
  }
  const auto before = joined.native.body->checkpoint();
  std::array<double, 8> force{};
  std::array<PhysicalRouteForceBlock, 9> blocks{};
  for (std::size_t i = 0; i < 9; ++i)
    blocks[i] = {i, joined.binding->manifest().programs[i].preparation_seal,
                 force.data(), .5, true};
  blocks[4].preparation_seal ^= 1;
  std::array<float, 8> pcm;
  pcm.fill(99);
  bool applied = false;
  {
    auto custody = joined.native.engine->acquire_stopped_custody();
    callback_probe = true;
    applied = joined.binding->advance_routes({}, blocks.data(), 9, pcm.data(),
                                             8, before.body_revision, 0);
    callback_probe = false;
  }
  assert(!applied && allocations == 0 && releases == 0);
  for (float v : pcm)
    assert(v == 99);
  assert(joined.native.body->checkpoint().displacement_modal_metres ==
             before.displacement_modal_metres &&
         joined.native.body->samples_elapsed() == 0);
  auto *after = packet::field(fixture, "after_material");
  const PreparedPhysicalBody after_preparation(
      ql::physical_wire::read_prepared_physical_body(
          packet::field(after, "preparation"),
          packet::field(after, "current_m3"), true));
  const auto after_basis = source_basis(packet::field(after, "source_basis"));
  assert(!joined.binding->preflight(after_preparation, after_basis, 512));
  const auto old_world =
      source_basis(packet::field(fixture, "world_source_basis"));
  refused([&] {
    ql::physical_wire::read_prepared_physical_force_routes(
        packet::field(fixture, "world_operation"),
        packet::field(fixture, "world_current_operation"), old_world,
        after_preparation, {}, 0);
  });
}
static std::vector<KeyboardCell> native_catalog(J *fixture) {
  auto *cells = packet::field(fixture, "native_catalog");
  std::vector<KeyboardCell> out;
  for (std::size_t i = 0; i < json_object_array_length(cells); ++i)
    out.push_back(
        management_transport::read_key(json_object_array_get_idx(cells, i)));
  return out;
}
struct RevisionSource {
  NativePerformance prepared;
  std::shared_ptr<const AdmittedNativeReceivingSource> admitted;
  std::shared_ptr<PhysicalRoutesPortBinding> binding;
  NativeRouteProgramSet seed{};
};
static RevisionSource revision_source(J *fixture,
                                      std::shared_ptr<PhysicalBody> resident) {
  auto *packet =
      ql::performance::packet::field(fixture, "performance_preparation");
  auto *basis = ql::performance::packet::field(packet, "native_basis");
  auto prepared = prepare_performance_packet(
      json_object_to_json_string_ext(packet, JSON_C_TO_STRING_PLAIN), basis,
      true, true);
  const auto &after = prepared.body->preparation();
  auto admitted = std::make_shared<const AdmittedNativeReceivingSource>(
      read_native_receiving_admission(
          ql::performance::packet::field(fixture, "native_admission"),
          ql::performance::packet::field(fixture, "current_native_admission"),
          prepared, basis, after,
          program_refs(ql::performance::packet::field(fixture, "program_refs")),
          512));
  if (!resident)
    resident = prepared.body;
  auto binding = std::make_shared<PhysicalRoutesPortBinding>(
      resident, admitted, after, prepared.determination, basis, true, 512);
  NativeRouteProgramSet seed;
  seed.manifest = binding->manifest();
  seed.program_count = seed.manifest.route_count;
  for (std::size_t i = 0; i < seed.program_count; ++i) {
    seed.programs[i].handle = seed.manifest.programs[i];
    seed.programs[i].phase_source_ref = prepared.determination.m1_coordinate;
    seed.programs[i].sine = prepared.notes.front().phase_sin;
    seed.programs[i].cosine = prepared.notes.front().phase_cos;
  }
  return {std::move(prepared), std::move(admitted), std::move(binding), seed};
}
static void retained_material_revision(J *fixture) {
  auto joined = prepare(fixture);
  auto owner = std::make_unique<PerformanceManagement>(
      joined.native,
      reference("expression:native-nine/retained-material-manager"));
  owner->admit_catalog(native_catalog(fixture));
  owner->native().engine->enable_capture(true);
  for (std::uint64_t i = 0; i < 2; ++i) {
    auto op = operation(owner->native().determination, Kind::NoteOn, i + 1, 0);
    op.note = owner->native().notes.at(i);
    op.value = .75;
    assert(owner->enqueue_score_input(
               op, reference(i ? "native-score:material/touch2"
                               : "native-score:material/touch1")) ==
           Result::Accepted);
  }
  auto future =
      operation(owner->native().determination, Kind::Parameter, 3, 600);
  future.parameter = Parameter::MasterLinear;
  future.value = .25;
  assert(owner->enqueue_score_input(future) == Result::Accepted);
  auto release =
      operation(owner->native().determination, Kind::NoteOff, 4, 900);
  release.touch = owner->native().notes.front().touch;
  assert(owner->enqueue_score_input(
             release, reference("native-score:material/touch1")) ==
         Result::Accepted);
  std::array<float, 128> pcm{};
  Capture capture;
  for (unsigned block = 0; block < 4; ++block) {
    assert(owner->offline_advance(pcm.data(), pcm.size(), block * 128));
    owner->pulse();
    assert(owner->pop_audio_capture(capture));
  }
  const auto before = owner->stopped_checkpoint();
  assert(before->native_pair.audio.cursor == 512);
  const auto before_wire =
      management_checkpoint_transport::checkpoint_wire(*before);
  const auto before_state = owner->native().body->checkpoint();
  const auto original_owner = owner->native().body.get();
  const auto before_preparation = owner->native().body->preparation();
  auto *after_fixture = packet::field(fixture, "after_material");
  auto source = revision_source(after_fixture, owner->native().body);
  auto physical = std::make_unique<PreparedPhysicalTransition>(
      before_preparation, source.prepared.body->preparation(), 1, 512,
      PhysicalLiveUpdateKind::Material,
      PhysicalFormTransition::ProjectCorrespondingNodes,
      "native-score:material/actual-fourfold-stiffness-edit");
  PhysicalLiveTransitionReceipt receipt;
  std::unique_ptr<PreparedRetainedBodyRevision> transaction;
  {
    auto guard = owner->native().engine->acquire_stopped_custody();
    transaction = std::make_unique<PreparedRetainedBodyRevision>(
        *owner, std::move(physical), source.admitted,
        source.prepared.determination,
        packet::field(packet::field(after_fixture, "performance_preparation"),
                      "native_basis"),
        source.seed, source.prepared.notes, native_catalog(after_fixture),
        guard);
    assert(transaction->current(guard));
    callback_probe = true;
    const bool committed = transaction->commit(guard, receipt);
    callback_probe = false;
    assert(committed && allocations == 0 && releases == 0);
    owner->refresh_stopped_reading(guard);
  }
  assert(transaction->committed() && receipt.transaction == 1 &&
         receipt.before_revision == 1 && receipt.after_revision == 2 &&
         receipt.samples_elapsed == 512 &&
         std::isfinite(receipt.external_work_joules));
  assert(owner->native().body.get() == original_owner &&
         owner->native().engine->owns_physical_owner(original_owner));
  const auto after = owner->stopped_checkpoint();
  assert(after->native_pair.physical.samples_elapsed == 512 &&
         after->native_pair.audio.cursor == 512 &&
         after->native_pair.audio.determination.body_revision == 2);
  const auto after_wire =
      management_checkpoint_transport::checkpoint_wire(*after);
  for (const char *field : {"inputs", "input_history", "transport_epoch"})
    assert(json_object_equal(packet::field(before_wire.get(), field),
                             packet::field(after_wire.get(), field)));
  auto *before_audio =
      packet::field(packet::field(before_wire.get(), "native_pair"), "audio");
  auto *after_audio =
      packet::field(packet::field(after_wire.get(), "native_pair"), "audio");
  for (const char *field : {"voices", "touches", "tails", "operations",
                            "pending_operations", "applications"})
    assert(json_object_equal(packet::field(before_audio, field),
                             packet::field(after_audio, field)));
  assert(after->native_pair.physical.displacement_modal_metres ==
             before_state.displacement_modal_metres &&
         after->native_pair.physical.velocity_modal_metres_per_second ==
             before_state.velocity_modal_metres_per_second);
  for (std::size_t i = 0; i < 9; ++i) {
    const auto &a = before->native_pair.audio.route_programs.programs[i];
    const auto &b = after->native_pair.audio.route_programs.programs[i];
    assert(a.sine == b.sine && a.cosine == b.cosine &&
           a.target_gain == b.target_gain &&
           a.effective_gain == b.effective_gain && a.enabled == b.enabled &&
           a.handle.driver_ref == b.handle.driver_ref &&
           a.handle.source_hertz == b.handle.source_hertz);
    assert(a.handle.preparation_seal != b.handle.preparation_seal);
  }
  // Reopen the exact revised preparation plus original full native Manager
  // checkpoint, never replay through an obsolete body or guessed oscillator.
  auto reopened_source = revision_source(after_fixture, {});
  auto reopened_native = std::move(reopened_source.prepared);
  auto port = physical_routes_port(physical_port(reopened_native.body),
                                   reopened_source.binding);
  reopened_native.engine =
      std::make_shared<Engine>(reopened_native.determination, 48000, port);
  auto reopened = std::make_unique<PerformanceManagement>(reopened_native,
                                                          owner->session_ref());
  const auto serialized =
      management_checkpoint_transport::checkpoint_wire(*after);
  const auto restored =
      management_checkpoint_transport::read_checkpoint_wire(serialized.get());
  TransportAcknowledgement acknowledgement;
  assert(reopened->stopped_restore(
      *restored, 0, reference("native-score:material/reopen-transaction"),
      reference("native-score:material/original-complete-checkpoint"),
      acknowledgement));
  assert(acknowledgement.target_sample == 512);
  for (unsigned block = 0; block < 8; ++block) {
    std::array<float, 128> resumed{};
    const auto start = 512 + block * 128;
    assert(owner->offline_advance(pcm.data(), pcm.size(), start));
    assert(reopened->offline_advance(resumed.data(), resumed.size(), start));
    const auto current = owner->pulse(), returned = reopened->pulse();
    assert(pcm == resumed &&
           current->reading.physical.visible_positions_metres ==
               returned->reading.physical.visible_positions_metres);
    Capture original, replay;
    assert(owner->pop_audio_capture(original) &&
           reopened->pop_audio_capture(replay));
    assert(original.force_newtons == replay.force_newtons &&
           original.route_force_newtons == replay.route_force_newtons);
  }
  assert(owner->native().body->checkpoint().displacement_modal_metres ==
         reopened->native().body->checkpoint().displacement_modal_metres);
  std::cout
      << "actual native N9+M1 held touches+future parameter/release -> same P "
         "material projection+phase/history -> exact1024 reopen passed\n";
}
// Optional native CI artifact output. Every value is taken from this actual
// same-owner stopped queue/application activity; no precomputed receipt.
static void prearm_artifact(const char *name, const Json &value) {
  const auto *directory = std::getenv("QL_NATIVE_PREARM_RECEIPT_DIR");
  if (!directory || !*directory)
    return;
  std::ofstream file(std::string(directory) + "/" + name,
                     std::ios::binary | std::ios::trunc);
  assert(file.good());
  file << json_object_to_json_string_ext(value.get(), JSON_C_TO_STRING_PLAIN)
       << "\n";
  file.close();
  assert(!file.fail());
}
static void stopped_force_savecut_artifacts(J *fixture) {
  auto prepared = prepare(fixture);
  auto owner = std::make_unique<PerformanceManagement>(
      prepared.native,
      reference("native-test:prearm/force-savecut/same-manager"));
  const auto born = owner->stopped_checkpoint();
  assert(born->native_pair.audio.source.force_newtons != 2 &&
         born->native_pair.audio.accepted_sequence == 0);
  const auto admission = owner->set_parameter(Parameter::ForceNewtons, 2);
  assert(admission.result == Result::Accepted && admission.has_stopped_queue &&
         admission.stopped_queue.queue().queued());
  const auto &event = admission.stopped_queue.queue().operation();
  assert(event.kind == Kind::Parameter &&
         event.parameter == Parameter::ForceNewtons && event.value == 2 &&
         event.sequence == 1 && event.sample == 0 &&
         event.has_requested_sample && event.requested_sample == 0 &&
         !valid_ref(admission.stopped_queue.input_ref()) &&
         event.native_clock.epoch == 0 &&
         event.native_clock.anchor_ordinal == 0 &&
         event.native_clock.trigger_host_ticks == 0 &&
         event.native_clock.admitted_host_ticks == 0 &&
         event.native_clock.mapping_uncertainty_samples == 0 &&
         event.native_clock.input_transit_unknown);
  const auto pending = owner->stopped_checkpoint();
  assert(pending->native_pair.audio.accepted_sequence == 1 &&
         pending->native_pair.audio.applied_application_ordinal == 0 &&
         pending->native_pair.audio.source.force_newtons ==
             born->native_pair.audio.source.force_newtons);
  prearm_artifact("stopped-force-born0-management.json",
                  management_checkpoint_transport::checkpoint_wire(*born));
  prearm_artifact(
      "stopped-force-admission.json",
      management_transport::score_admission(admission.stopped_queue));
  prearm_artifact("stopped-force-pending-management.json",
                  management_checkpoint_transport::checkpoint_wire(*pending));
  std::array<float, 128> pcm{};
  callback_probe = true;
  const bool advanced = owner->offline_advance(pcm.data(), pcm.size(), 0);
  callback_probe = false;
  assert(advanced && allocations == 0 && releases == 0);
  const auto pulse = owner->pulse();
  assert(pulse->applications.size() == 1 && pulse->applications[0].applied &&
         pulse->applications[0].kind == Kind::Parameter &&
         pulse->applications[0].parameter == Parameter::ForceNewtons &&
         pulse->applications[0].value == 2 &&
         pulse->applications[0].sequence == 1 &&
         pulse->applications[0].applied_sample == 0 &&
         pulse->reading.source.force_newtons == 2);
  prearm_artifact("stopped-force-applied.json",
                  checkpoint_transport::application(pulse->applications[0]));
  const auto after = owner->stopped_checkpoint();
  assert(after->native_pair.audio.source.force_newtons == 2 &&
         after->native_pair.audio.cursor == 128);
  prearm_artifact("stopped-force-after-management.json",
                  management_checkpoint_transport::checkpoint_wire(*after));
  // A separate genuine cold native continuation preserves the original born,
  // pending and application trial above. Decode the complete actual pending
  // wrapper and restore to a fresh SAME-source manager through its real API.
  auto encoded = management_checkpoint_transport::checkpoint_wire(*pending);
  auto decoded =
      management_checkpoint_transport::read_checkpoint_wire(encoded.get());
  auto fresh = prepare(fixture);
  auto reopened = std::make_unique<PerformanceManagement>(fresh.native,
                                                          owner->session_ref());
  const auto restore_before = reopened->stopped_checkpoint();
  prearm_artifact(
      "stopped-force-restore-before-management.json",
      management_checkpoint_transport::checkpoint_wire(*restore_before));
  TransportAcknowledgement ack{};
  assert(reopened->stopped_restore(
      *decoded, 0, reference("native-test:prearm/force-restore/transaction"),
      reference("native-test:prearm/force-restore/pending-cut"), ack));
  assert(ack.previous_epoch == 1 && ack.epoch == 2 &&
         ack.previous_cursor == 0 && ack.previous_sequence == 0 &&
         ack.target_sample == 0 && ack.accepted_sequence == 1);
  auto ack_wire = checkpoint_transport::object();
  checkpoint_transport::u64(ack_wire.get(), "previous_epoch",
                            ack.previous_epoch);
  checkpoint_transport::u64(ack_wire.get(), "epoch", ack.epoch);
  checkpoint_transport::u64(ack_wire.get(), "previous_cursor",
                            ack.previous_cursor);
  checkpoint_transport::u64(ack_wire.get(), "previous_sequence",
                            ack.previous_sequence);
  checkpoint_transport::u64(ack_wire.get(), "target_sample", ack.target_sample);
  checkpoint_transport::u64(ack_wire.get(), "accepted_sequence",
                            ack.accepted_sequence);
  checkpoint_transport::ref(ack_wire.get(), "transaction_ref", ack.transaction);
  checkpoint_transport::ref(ack_wire.get(), "checkpoint_ref", ack.checkpoint);
  prearm_artifact("stopped-force-restore-ack.json", ack_wire);
  const auto restored_pending = reopened->stopped_checkpoint();
  assert(restored_pending->transport_epoch == 2 &&
         restored_pending->native_pair.audio.applied_application_ordinal == 0 &&
         restored_pending->native_pair.audio.accepted_sequence == 1 &&
         restored_pending->native_pair.audio.source.force_newtons ==
             born->native_pair.audio.source.force_newtons);
  prearm_artifact(
      "stopped-force-restored-pending-management.json",
      management_checkpoint_transport::checkpoint_wire(*restored_pending));
  std::array<float, 128> continued{};
  callback_probe = true;
  const bool restored_advance =
      reopened->offline_advance(continued.data(), continued.size(), 0);
  callback_probe = false;
  assert(restored_advance && allocations == 0 && releases == 0 &&
         continued == pcm);
  const auto restored_pulse = reopened->pulse();
  assert(restored_pulse->applications.size() == 1 &&
         restored_pulse->applications[0].applied &&
         restored_pulse->applications[0].kind == Kind::Parameter &&
         restored_pulse->applications[0].parameter == Parameter::ForceNewtons &&
         restored_pulse->applications[0].value == 2 &&
         restored_pulse->applications[0].sequence == 1 &&
         restored_pulse->applications[0].applied_application_ordinal == 1 &&
         restored_pulse->applications[0].applied_sample == 0 &&
         restored_pulse->reading.source.force_newtons == 2);
  prearm_artifact(
      "stopped-force-restored-applied.json",
      checkpoint_transport::application(restored_pulse->applications[0]));
  const auto restored_after = reopened->stopped_checkpoint();
  assert(restored_after->transport_epoch == 2 &&
         restored_after->native_pair.audio.cursor == 128 &&
         restored_after->native_pair.audio.applied_application_ordinal == 1 &&
         restored_after->native_pair.audio.source.force_newtons == 2);
  prearm_artifact(
      "stopped-force-restored-after-management.json",
      management_checkpoint_transport::checkpoint_wire(*restored_after));
  assert(reopened->offline_advance(continued.data(), continued.size(), 128));
  const auto next = reopened->pulse();
  assert(next->applications.empty() && next->reading.source.force_newtons == 2);
}
static void stopped_parameter_admission_cases(J *fixture) {
  auto prepared = prepare(fixture), unchanged = prepare(fixture);
  auto owner = std::make_unique<PerformanceManagement>(
      prepared.native, reference("native-test:prearm/same-manager"));
  auto twin = std::make_unique<PerformanceManagement>(
      unchanged.native,
      reference("native-test:prearm/independent-unedited-manager"));
  owner->native().engine->enable_capture(true);
  twin->native().engine->enable_capture(true);
  const auto before = owner->stopped_checkpoint();
  const auto admitted = owner->set_parameter(Parameter::MasterLinear, .5);
  assert(admitted.result == Result::Accepted && admitted.has_stopped_queue &&
         admitted.clock.epoch == 0 && admitted.clock.sequence == 0 &&
         admitted.stopped_queue.queue().queued());
  const auto &q = admitted.stopped_queue.queue();
  assert(q.operation().kind == Kind::Parameter && q.operation().sequence == 1 &&
         q.operation().sample == 0 && q.operation().requested_sample == 0 &&
         q.operation().has_requested_sample &&
         q.operation().native_clock.epoch == 0 && q.queue_cursor() == 0 &&
         q.queue_horizon() == 0 &&
         admitted.stopped_queue.session_ref() == owner->session_ref());
  const auto queued = owner->stopped_checkpoint();
  assert(queued->native_pair.audio.accepted_sequence == 1 &&
         queued->native_pair.audio.source.master_linear ==
             before->native_pair.audio.source.master_linear &&
         queued->native_pair.audio.applied_application_ordinal == 0 &&
         queued->native_pair.audio.applications.write ==
             queued->native_pair.audio.applications.read);
  const auto original_wire =
      management_checkpoint_transport::checkpoint_wire(*queued);
  auto wrong_source = owner->native().determination.identity;
  wrong_source.instance = reference("native-test:prearm/foreign-instance");
  assert(owner
             ->enqueue_stopped_parameter_admission(Parameter::MasterLinear, .75,
                                                   wrong_source, 0)
             .result() == Result::Stale);
  assert(owner
             ->enqueue_stopped_parameter_admission(
                 Parameter::MasterLinear, .75,
                 owner->native().determination.identity, 1)
             .result() == Result::Stale);
  {
    auto foreign_guard = twin->native().engine->acquire_stopped_custody();
    assert(foreign_guard &&
           owner->native()
                   .engine
                   ->enqueue_stopped_parameter_with_receipt(
                       Parameter::MasterLinear, .75,
                       owner->native().determination.identity, foreign_guard, 0)
                   .result() == Result::Unavailable);
  }
  const auto refused_state = owner->stopped_checkpoint();
  const auto refused_wire =
      management_checkpoint_transport::checkpoint_wire(*refused_state);
  assert(json_object_equal(original_wire.get(), refused_wire.get()));
  std::array<float, 128> pcm{}, original{};
  callback_probe = true;
  const bool first = owner->offline_advance(pcm.data(), pcm.size(), 0);
  const bool second =
      twin->offline_advance(original.data(), original.size(), 0);
  callback_probe = false;
  assert(first && second && allocations == 0 && releases == 0);
  const auto pulse = owner->pulse();
  assert(pulse->applications.size() == 1 && pulse->applications[0].applied &&
         pulse->applications[0].kind == Kind::Parameter &&
         pulse->applications[0].sequence == 1 &&
         pulse->applications[0].applied_sample == 0 &&
         pulse->applications[0].value == .5 &&
         pulse->reading.source.master_linear == .5);
  Capture changed{}, unedited{};
  assert(owner->pop_audio_capture(changed) &&
         twin->pop_audio_capture(unedited));
  assert(changed.pickup_linear == unedited.pickup_linear &&
         changed.force_newtons == unedited.force_newtons &&
         changed.route_force_newtons == unedited.route_force_newtons &&
         pcm != original &&
         changed.body_gain_linear != unedited.body_gain_linear);
  auto full = prepare(fixture);
  auto full_owner = std::make_unique<PerformanceManagement>(
      full.native, reference("native-test:prearm/full-native-queue"));
  for (std::uint64_t i = 0; i < queue_capacity; ++i) {
    auto op = operation(full_owner->native().determination, Kind::Parameter,
                        i + 1, 0);
    op.parameter = Parameter::MasterLinear;
    op.value = .5;
    assert(full_owner->enqueue_score_input(op) == Result::Accepted);
  }
  const auto full_before = full_owner->stopped_checkpoint();
  const auto full_before_wire =
      management_checkpoint_transport::checkpoint_wire(*full_before);
  const auto overflow = full_owner->set_parameter(Parameter::MasterLinear, .75);
  assert(overflow.has_stopped_queue && overflow.result == Result::Overflow &&
         !overflow.stopped_queue.queue().queued());
  const auto full_after = full_owner->stopped_checkpoint();
  const auto full_after_wire =
      management_checkpoint_transport::checkpoint_wire(*full_after);
  assert(json_object_equal(full_before_wire.get(), full_after_wire.get()));
  assert(full_owner->offline_advance(pcm.data(), pcm.size(), 0));
  full_owner->pulse();
  const auto recovered =
      full_owner->set_parameter(Parameter::MasterLinear, .75);
  assert(recovered.result == Result::Accepted &&
         recovered.stopped_queue.queue().operation().sequence ==
             queue_capacity + 1 &&
         recovered.stopped_queue.queue().operation().sample == 128);
  assert(full_owner->offline_advance(pcm.data(), pcm.size(), 128));
  const auto final = full_owner->pulse();
  assert(final->applications.size() == 1 &&
         final->applications[0].sequence == queue_capacity + 1 &&
         final->applications[0].applied &&
         final->applications[0].applied_sample == 128 &&
         final->reading.source.master_linear == .75);
}
static void neutral_world(J *fixture) {
  auto quiet = prepare(fixture, true), keyed = prepare(fixture, true);
  note(keyed);
  const auto silent = render(quiet, 512), sound = render(keyed, 512);
  assert(quiet.binding->manifest().route_count == 0 &&
         std::all_of(silent.pickup.begin(), silent.pickup.end(),
                     [](float v) { return v == 0; }));
  assert(std::any_of(sound.pickup.begin(), sound.pickup.end(),
                     [](float v) { return v != 0; }));
  assert(sound.receipt.route_count == 0 && sound.receipt.scalar_m1_enabled &&
         sound.receipt.scalar_peak_applied_force_newtons > 0);
}
static void allroute_management_hold(J *fixture) {
  auto joined = prepare(fixture);
  joined.native.engine->enable_capture(true);
  auto owner = std::make_unique<PerformanceManagement>(
      joined.native, reference("expression:native-nine/current-manager"));
  auto attack = operation(joined.native.determination, Kind::NoteOn, 1, 0);
  attack.note = joined.native.notes.at(0);
  attack.value = .8;
  const auto original_input = reference("native-score:janko/original-touch/42");
  assert(owner->enqueue_score_input(attack, original_input) ==
         Result::Accepted);
  std::array<float, 128> pcm{};
  callback_probe = true;
  const bool began = owner->offline_advance(pcm.data(), pcm.size(), 0);
  callback_probe = false;
  assert(began && allocations == 0 && releases == 0);
  auto before = owner->pulse();
  assert(before->reading.has_route_programs &&
         !before->reading.routes_suspended);
  assert(before->reading.physical_routes.route_count == 9);
  assert(before->applications.size() == 1 && before->applications[0].applied);
  Capture capture;
  assert(owner->pop_audio_capture(capture));
  bool any_native = false;
  for (std::size_t route = 0; route < 9; ++route)
    for (std::size_t i = 0; i < 128; ++i)
      any_native |= capture.route_force_newtons[route][i] != 0;
  assert(any_native && before->reading.physical.mechanical_energy_joules > 0);
  auto saved_before = owner->stopped_checkpoint();
  const auto release_request = owner->hold();
  assert(release_request != 0);
  bool actual_ringdown = false, retired_original_input = false;
  std::unique_ptr<ManagementPulse> pulse;
  for (unsigned block = 0; block < 4; ++block) {
    const auto start = owner->native().engine->samples_elapsed();
    callback_probe = true;
    const bool ok = owner->offline_advance(pcm.data(), pcm.size(), start);
    callback_probe = false;
    assert(ok && allocations == 0 && releases == 0);
    pulse = owner->pulse();
    assert(owner->pop_audio_capture(capture));
    assert(pulse->reading.routes_suspended &&
           pulse->reading.has_route_programs);
    assert(pulse->reading.physical.samples_elapsed ==
           pulse->reading.samples_elapsed);
    for (const auto &h : pulse->input_history)
      retired_original_input |= h.change == InputBindingChange::Retired &&
                                h.input_ref == original_input;
    for (std::size_t i = 0; i < 128; ++i) {
      assert(capture.force_newtons[i] == 0);
      for (std::size_t route = 0; route < 9; ++route)
        assert(capture.route_force_newtons[route][i] == 0);
      actual_ringdown |= capture.pickup_linear[i] != 0;
    }
    assert(pulse->release_zero_proven == (block == 3));
  }
  assert(actual_ringdown && retired_original_input);
  assert(pulse->release_request == release_request && !pulse->release_pending);
  assert(pulse->reading.force_zero_samples >= 512 &&
         pulse->reading.active_touches == 0 &&
         pulse->reading.active_tails == 0 && !pulse->reading.sustain);
  auto saved_after = owner->stopped_checkpoint();
  assert(saved_after->native_pair.audio.route_programs.owner_suspended);
  for (std::size_t route = 0; route < 9; ++route) {
    const auto &before_program =
        saved_before->native_pair.audio.route_programs.programs[route];
    const auto &after_program =
        saved_after->native_pair.audio.route_programs.programs[route];
    assert(before_program.handle.program_seal ==
           after_program.handle.program_seal);
    assert(before_program.handle.source_hertz ==
           after_program.handle.source_hertz);
    assert(before_program.sine != after_program.sine ||
           before_program.cosine != after_program.cosine);
  }
  auto encoded = management_checkpoint_transport::checkpoint_wire(*saved_after);
  auto decoded =
      management_checkpoint_transport::read_checkpoint_wire(encoded.get());
  auto other = prepare(fixture);
  auto reopened = std::make_unique<PerformanceManagement>(
      other.native, reference("expression:native-nine/current-manager"));
  TransportAcknowledgement ack;
  assert(reopened->stopped_restore(*decoded, 0, reference("native:N9/restore"),
                                   reference("native:N9/checkpoint"), ack));
  std::array<float, 128> continued{};
  for (unsigned block = 0; block < 8; ++block) {
    const auto start = owner->native().engine->samples_elapsed();
    assert(owner->offline_advance(pcm.data(), pcm.size(), start));
    assert(
        reopened->offline_advance(continued.data(), continued.size(), start));
    assert(pcm == continued);
    owner->pulse();
    reopened->pulse();
  }
}
// This overload regression deliberately retains a valid historical 100 N
// requested scale, then plays 24 separately native-produced same-pitch touches
// alongside the original N9 programme. Before the scalar budget repair, P
// refuses the combined absolute force and render() fails before cursor commit.
static void high_force_reserves_original_routes(J *fixture) {
  auto high = prepare(fixture, false, -1, 100);
  assert(high.native.notes.size() == 48);
  for (std::size_t i = 0; i < max_voices; ++i)
    note(high, i + 1, 0, i);
  const auto before_manifest = high.binding->manifest();
  double reserved = 0;
  for (std::size_t route = 0; route < 9; ++route)
    reserved += before_manifest.programs[route].peak_force_newtons;
  assert(reserved > 0 && reserved < before_manifest.max_force_newtons);
  const auto first = render(high, 512);
  assert(high.native.body->samples_elapsed() == 512 &&
         high.native.engine->samples_elapsed() == 512);
  assert(std::any_of(first.note_force.begin(), first.note_force.end(),
                     [](double force) { return force != 0; }));
  assert(std::any_of(first.pickup.begin(), first.pickup.end(),
                     [](float value) { return value != 0; }));
  assert(first.force_limited_samples > 0);
  const double note_budget = before_manifest.max_force_newtons - reserved;
  for (double force : first.note_force)
    assert(std::abs(force) < note_budget);
  auto too_high = operation(high.native.determination, Kind::Parameter,
                            max_voices + 1, 512);
  too_high.parameter = Parameter::ForceNewtons;
  too_high.value = 100;
  assert(high.native.engine->enqueue(too_high) == Result::Invalid);
  assert(high.native.engine->accepted_sequence() == max_voices);
  too_high.value = note_budget * .5;
  assert(high.native.engine->enqueue(too_high) == Result::Accepted);
  for (std::size_t route = 0; route < 9; ++route) {
    const auto &before = before_manifest.programs[route];
    const auto &after = high.binding->manifest().programs[route];
    assert(before.source_hertz == after.source_hertz &&
           before.share_numerator == after.share_numerator &&
           before.share_denominator == after.share_denominator &&
           before.peak_force_newtons == after.peak_force_newtons &&
           before.program_seal == after.program_seal);
  }
  std::unique_ptr<PairedCheckpoint> checkpoint;
  {
    auto custody = high.native.engine->acquire_stopped_custody();
    checkpoint =
        checkpoint_heap(*high.native.engine, *high.native.body, custody);
  }
  auto reopened = prepare(fixture, false, -1, 100);
  {
    auto custody = reopened.native.engine->acquire_stopped_custody();
    assert(restore_checkpoint(*reopened.native.engine, *reopened.native.body,
                              *checkpoint, custody, 0));
  }
  const auto continued = render(high, 1024, 128),
             restored = render(reopened, 1024, 128);
  assert(continued.pcm == restored.pcm &&
         continued.note_force == restored.note_force &&
         continued.pickup == restored.pickup &&
         continued.snapshot.visible_positions_metres ==
             restored.snapshot.visible_positions_metres);
  assert(high.native.body->checkpoint().displacement_modal_metres ==
         reopened.native.body->checkpoint().displacement_modal_metres);
  std::cout << "{\"schema\":\"ql.n9-note-force-budget-experiment/v1\","
            << "\"original_nine_reserved_newtons\":" << reserved
            << ",\"body_limit_newtons\":" << before_manifest.max_force_newtons
            << ",\"native_touches\":24,\"committed_cursor\":\"1536\","
               "\"continuation_frames\":1024,\"scope\":\"actual source N9+A/P "
               "in-memory; no device verdict\"}\n";
}

static void committed_v2_scalar_import(J *fixture) {
  auto *packet_value = packet::field(fixture, "performance_preparation");
  const std::string bytes =
      json_object_to_json_string_ext(packet_value, JSON_C_TO_STRING_PLAIN);
  auto scalar = prepare_performance_packet(
      bytes, packet::field(packet_value, "native_basis"), true, true);
  auto attack = operation(scalar.determination, Kind::NoteOn, 1, 37);
  attack.note = scalar.notes.at(0);
  attack.value = .8;
  assert(scalar.engine->enqueue(attack) == Result::Accepted);
  auto future = operation(scalar.determination, Kind::Parameter, 2, 48000);
  future.parameter = Parameter::MasterLinear;
  future.value = .25;
  assert(scalar.engine->enqueue(future) == Result::Accepted);
  std::array<float, 128> pcm{};
  assert(scalar.engine->render(pcm.data(), pcm.size(),
                               scalar.engine->samples_elapsed()));
  std::unique_ptr<PairedCheckpoint> original;
  {
    auto custody = scalar.engine->acquire_stopped_custody();
    original = checkpoint_heap(*scalar.engine, *scalar.body, custody);
  }
  assert(!original->audio.has_route_programs &&
         original->audio.applied_application_ordinal == 1 &&
         original->audio.accepted_sequence == 2);
  auto wire = checkpoint_transport::checkpoint_wire(*original);
  auto *audio = packet::field(wire.get(), "audio");
  assert(checkpoint_transport::audio_checkpoint_encoding(audio) ==
         checkpoint_transport::AudioCheckpointEncoding::V2WithRoutePrograms);
  // Exact original scalar v2 lacked this additive pair. All ordinals, source,
  // voice phase, journals and future queues remain the actual committed state.
  json_object_object_del(audio, "has_route_programs");
  json_object_object_del(audio, "route_programs");
  const std::string retained_original_bytes =
      json_object_to_json_string_ext(wire.get(), JSON_C_TO_STRING_PLAIN);
  assert(checkpoint_transport::audio_checkpoint_encoding(audio) ==
         checkpoint_transport::AudioCheckpointEncoding::OriginalV2Scalar);
  auto imported = checkpoint_transport::read_checkpoint_wire(wire.get());
  assert(!imported->audio.has_route_programs &&
         imported->audio.applied_application_ordinal == 1 &&
         imported->audio.accepted_sequence == 2);
  assert(retained_original_bytes ==
         json_object_to_json_string_ext(wire.get(), JSON_C_TO_STRING_PLAIN));
  auto reopened = prepare_performance_packet(
      bytes, packet::field(packet_value, "native_basis"), true, true);
  {
    auto custody = reopened.engine->acquire_stopped_custody();
    assert(restore_checkpoint(*reopened.engine, *reopened.body, *imported,
                              custody, 0));
  }
  std::array<float, 128> continued{};
  for (unsigned block = 0; block < 8; ++block) {
    assert(scalar.engine->render(pcm.data(), pcm.size(),
                                 scalar.engine->samples_elapsed()));
    assert(reopened.engine->render(continued.data(), continued.size(),
                                   reopened.engine->samples_elapsed()));
    assert(pcm == continued);
  }
  // A stripped route pair cannot acquire scalar meaning inside the actual N9
  // port. Restoration checks programme/port presence before P mutation.
  auto n9 = prepare(fixture);
  auto n9_saved = std::unique_ptr<PairedCheckpoint>{};
  {
    auto custody = n9.native.engine->acquire_stopped_custody();
    n9_saved = checkpoint_heap(*n9.native.engine, *n9.native.body, custody);
  }
  auto n9_wire = checkpoint_transport::checkpoint_wire(*n9_saved);
  auto *n9_audio = packet::field(n9_wire.get(), "audio");
  json_object_object_del(n9_audio, "route_programs");
  refused([&] { checkpoint_transport::read_checkpoint_wire(n9_wire.get()); });
  json_object_object_del(n9_audio, "has_route_programs");
  auto stripped = checkpoint_transport::read_checkpoint_wire(n9_wire.get());
  {
    auto custody = n9.native.engine->acquire_stopped_custody();
    assert(!restore_checkpoint(*n9.native.engine, *n9.native.body, *stripped,
                               custody, 0));
  }
  assert(n9.native.body->samples_elapsed() == 0);
}
static void accepted_future_attack_after_hold(J *fixture) {
  auto joined = prepare(fixture);
  joined.native.engine->enable_capture(true);
  auto owner = std::make_unique<PerformanceManagement>(
      joined.native, reference("expression:native-nine/current-manager"));
  auto attack = operation(joined.native.determination, Kind::NoteOn, 1, 0);
  attack.note = joined.native.notes.at(0);
  attack.value = .8;
  const auto played_input = reference("native-score:held/original-input");
  assert(owner->enqueue_score_input(attack, played_input) == Result::Accepted);
  auto future = operation(joined.native.determination, Kind::NoteOn, 2, 48000);
  future.note = joined.native.notes.at(1);
  future.value = .8;
  const auto future_input = reference("native-score:future/original-input");
  assert(future.note.touch > attack.note.touch);
  assert(owner->enqueue_score_input(future, future_input) == Result::Accepted);
  std::array<float, 128> pcm{};
  assert(owner->offline_advance(pcm.data(), pcm.size(), 0));
  auto before = owner->pulse();
  assert(before->applications.size() == 1 && before->applications[0].applied);
  bool future_admission = false;
  for (const auto &h : before->input_history)
    future_admission |= h.change == InputBindingChange::PressAdmitted &&
                        h.native_sequence == 2 && h.input_ref == future_input;
  assert(future_admission);
  Capture capture;
  assert(owner->pop_audio_capture(capture));
  const auto token = owner->hold();
  assert(token);
  std::unique_ptr<ManagementPulse> pulse;
  for (unsigned block = 0; block < 4; ++block) {
    assert(owner->offline_advance(pcm.data(), pcm.size(), 128 + block * 128));
    pulse = owner->pulse();
    assert(owner->pop_audio_capture(capture));
    assert(pulse->release_zero_proven == (block == 3));
    for (std::size_t i = 0; i < pcm.size(); ++i) {
      assert(capture.force_newtons[i] == 0);
      for (std::size_t route = 0; route < 9; ++route)
        assert(capture.route_force_newtons[route][i] == 0);
    }
  }
  auto checkpoint = owner->stopped_checkpoint();
  assert(checkpoint->native_pair.audio.panic_fence >= 2 &&
         checkpoint->native_pair.audio.accepted_sequence == 2 &&
         checkpoint->native_pair.audio.applied_application_ordinal == 1);
  bool retained_future = false;
  for (const auto &input : checkpoint->bindings.inputs)
    retained_future |= input.active && input.input_ref == future_input &&
                       input.press_sequence == 2 && !input.press_applied;
  assert(retained_future); // retained + fenced, not silently discarded
  auto wire = management_checkpoint_transport::checkpoint_wire(*checkpoint);
  auto saved =
      management_checkpoint_transport::read_checkpoint_wire(wire.get());
  auto other = prepare(fixture);
  auto reopened = std::make_unique<PerformanceManagement>(
      other.native, reference("expression:native-nine/current-manager"));
  TransportAcknowledgement ack;
  assert(reopened->stopped_restore(*saved, 0,
                                   reference("native:future/restore"),
                                   reference("native:future/checkpoint"), ack));
  std::array<float, 128> restored{};
  bool explicit_refusal = false, original_journal_refusal = false;
  while (owner->native().engine->samples_elapsed() < 48128) {
    const auto cursor = owner->native().engine->samples_elapsed();
    assert(owner->offline_advance(pcm.data(), pcm.size(), cursor));
    assert(reopened->offline_advance(restored.data(), restored.size(), cursor));
    assert(pcm == restored);
    auto applied = owner->pulse(), copied = reopened->pulse();
    assert(applied->reading.samples_elapsed == copied->reading.samples_elapsed);
    assert(applied->reading.active_touches == 0 &&
           applied->reading.active_tails == 0 &&
           applied->reading.routes_suspended);
    assert(applied->applications.size() == copied->applications.size());
    for (const auto &a : applied->applications) {
      explicit_refusal |=
          a.sequence == 2 && !a.applied && a.kind == Kind::NoteOn &&
          a.has_note && a.note.touch_ref == future.note.touch_ref &&
          a.admitted_sample == 48000 && a.applied_sample == 48000 &&
          a.applied_application_ordinal == 2;
      assert(!a.applied);
    }
    for (const auto &h : applied->input_history)
      original_journal_refusal |= h.native_sequence == 2 &&
                                  h.change == InputBindingChange::Refused &&
                                  h.input_ref == future_input;
    assert(owner->pop_audio_capture(capture));
    for (std::size_t i = 0; i < pcm.size(); ++i) {
      assert(capture.force_newtons[i] == 0);
      for (std::size_t route = 0; route < 9; ++route)
        assert(capture.route_force_newtons[route][i] == 0);
    }
    assert(reopened->pop_audio_capture(capture));
  }
  assert(explicit_refusal && original_journal_refusal);
  auto final_checkpoint = owner->stopped_checkpoint();
  for (const auto &input : final_checkpoint->bindings.inputs)
    assert(!input.active || input.input_ref != future_input);
  assert(final_checkpoint->native_pair.audio.applied_application_ordinal == 2);
}
static void genuine_carrier_detecting_trials(J *carrier) {
  const std::string bytes =
      json_object_to_json_string_ext(carrier, JSON_C_TO_STRING_PLAIN);
  auto copy = [&] { return ql_test_native_carrier::parse(bytes); };
  auto missing = copy();
  auto *parts = packet::field(missing.get(), "parts");
  assert(json_object_array_length(parts) > 1);
  assert(json_object_array_del_idx(parts, json_object_array_length(parts) - 1,
                                   1) == 0);
  refused([&] { ql_test_native_carrier::decode(missing.get()); });
  auto corrupt = copy();
  auto *part =
      json_object_array_get_idx(packet::field(corrupt.get(), "parts"), 0);
  json_object_object_add(part, "fingerprint",
                         json_object_new_string("fnv1a64:0000000000000000"));
  refused([&] { ql_test_native_carrier::decode(corrupt.get()); });
  auto unknown = copy();
  json_object_object_add(unknown.get(), "unexpected",
                         json_object_new_boolean(true));
  refused([&] { ql_test_native_carrier::decode(unknown.get()); });
  auto duplicate = copy();
  parts = packet::field(duplicate.get(), "parts");
  json_object_array_add(parts,
                        json_object_get(json_object_array_get_idx(parts, 0)));
  refused([&] { ql_test_native_carrier::decode(duplicate.get()); });
  auto restored = ql_test_native_carrier::decode(carrier);
  auto *original = packet::field(
      packet::field(restored.get(), "native_admission"), "native_basis");
  auto *current =
      packet::field(packet::field(restored.get(), "current_native_admission"),
                    "native_basis");
  const std::string current_bytes =
      json_object_to_json_string_ext(current, JSON_C_TO_STRING_PLAIN);
  json_object_object_add(original, "independent_mutation",
                         json_object_new_boolean(true));
  assert(current_bytes ==
         json_object_to_json_string_ext(current, JSON_C_TO_STRING_PLAIN));
  assert(original != current);
}
#include "../test_support/independent_retained_body_revision_cases.hpp"
int main() {
  std::string bytes;
  std::getline(std::cin, bytes);
  assert(!bytes.empty() && bytes.size() < 8 * 1024 * 1024);
  auto token = std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(
      json_tokener_new_ex(64), json_tokener_free);
  json_tokener_set_flags(token.get(),
                         JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  auto fixture = ql::physical_wire::own(
      json_tokener_parse_ex(token.get(), bytes.data(), int(bytes.size())));
  assert(json_tokener_get_error(token.get()) == json_tokener_success &&
         json_tokener_get_parse_end(token.get()) == bytes.size());
  ql_test_independent_carrier::genuine_scalar_roundtrip(fixture.get());
  genuine_carrier_detecting_trials(fixture.get());
  fixture = ql_test_native_carrier::decode(fixture.get());
  ql::physical_wire::keys(fixture.get(),
                          {"preparation", "current_m3", "operation",
                           "current_operation", "source_basis", "program_refs",
                           "world_operation", "world_current_operation",
                           "world_source_basis", "after_material",
                           "performance_preparation", "native_admission",
                           "current_native_admission", "world_native_admission",
                           "world_current_native_admission", "after_pose",
                           "overload", "native_catalog"});
  source_and_independent_routes(fixture.get());
  genuine_note_release_panic(fixture.get());
  checkpoint_and_partitions(fixture.get());
  qualification_and_atomic_refusal(fixture.get());
  neutral_world(fixture.get());
  independent_retained_body_revision_cases(fixture.get());
  retained_material_revision(fixture.get());
  stopped_parameter_admission_cases(fixture.get());
  stopped_force_savecut_artifacts(fixture.get());
  allroute_management_hold(fixture.get());
  committed_v2_scalar_import(fixture.get());
  accepted_future_attack_after_hold(fixture.get());
  high_force_reserves_original_routes(packet::field(fixture.get(), "overload"));
  assert(allocations == 0 && releases == 0);
  std::cout
      << "actual current native N9 -> one P/A callback -> Janko -> manager "
         "allroute512-force-zero/ringdown -> exact1024 continuation passed\n";
}
