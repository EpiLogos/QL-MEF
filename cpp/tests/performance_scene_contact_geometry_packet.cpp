// Actual Rust native packets construct P. This leaf proves geometry/force/P
// and generic ingress refusal. It deliberately cannot claim the C Scene lease;
// C's normal closed Kernel activity separately exercises the real issuer.
#include "../src/native_scene_contact_channel.hpp"
#include "../test_support/allocation_hooks.hpp"
#include <cassert>
#include <fstream>
#include <iostream>
#include <new>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_scene_contact_owner.hpp>
#include <type_traits>

static bool callback = false;
static std::uint64_t callback_allocations = 0, callback_releases = 0;
void *operator new(std::size_t size) {
  if (callback)
    ++callback_allocations;
  if (auto *value = ql_test_allocate(size))
    return value;
  throw std::bad_alloc();
}
void *operator new[](std::size_t size) { return ::operator new(size); }
void operator delete(void *value) noexcept {
  if (callback && value)
    ++callback_releases;
  ql_test_release(value);
}
void operator delete[](void *value) noexcept { ::operator delete(value); }
void operator delete(void *value, std::size_t) noexcept {
  ::operator delete(value);
}
void operator delete[](void *value, std::size_t) noexcept {
  ::operator delete(value);
}

using namespace ql::performance;
namespace sw = ql::performance::scene_contact_transport;
namespace wire = ql::performance::checkpoint_transport;
using J = json_object;
using Json = ql::physical_wire::Json;
static_assert(
    !std::is_default_constructible_v<NativeSceneContactOccurrenceOwner>);
static_assert(!std::is_copy_constructible_v<NativeSceneContactOccurrenceOwner>);
static_assert(!std::is_default_constructible_v<NativeContactOccurrenceWitness>);
static_assert(!std::is_copy_constructible_v<NativeContactOccurrenceWitness>);

static std::string file(const std::string &path) {
  std::ifstream input(path, std::ios::binary);
  ql::require(bool(input), "actual Rust native contact input absent");
  input.seekg(0, std::ios::end);
  const auto bytes = input.tellg();
  ql::require(bytes > 0 && bytes < 4 * 1024 * 1024,
              "actual contact input exceeds the existing source bound");
  std::string out(std::size_t(bytes), '\0');
  input.seekg(0);
  ql::require(bool(input.read(out.data(), bytes)),
              "actual contact input truncated");
  return out;
}
static Json parse(const std::string &bytes) {
  auto *tokener = json_tokener_new_ex(64);
  ql::require(tokener, "native contact test decoder unavailable");
  json_tokener_set_flags(tokener,
                         JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  auto out = ql::physical_wire::own(
      json_tokener_parse_ex(tokener, bytes.data(), int(bytes.size())));
  const auto error = json_tokener_get_error(tokener);
  const auto end = json_tokener_get_parse_end(tokener);
  json_tokener_free(tokener);
  ql::require(out && error == json_tokener_success &&
                  bytes.find_first_not_of(" \t\r\n", end) == std::string::npos,
              "complete actual native contact source required");
  return out;
}
static NativePerformance native(const std::string &directory) {
  auto basis = parse(file(directory + "/baseline.basis.json"));
  return prepare_performance_packet(file(directory + "/baseline.packet.json"),
                                    basis.get(), true, true);
}
template <class F> static void refused(F action) {
  bool rejected = false;
  try {
    action();
  } catch (const std::invalid_argument &) {
    rejected = true;
  }
  assert(rejected);
}
static ql::PhysicalSceneContactDefinition
definition(const ql::PreparedPhysicalBody &body) {
  ql::PhysicalSceneContactDefinition out{};
  out.contact_ref = "native-test:geometry/contact";
  out.particle_ref = "native-test:geometry/particle";
  out.collider_ref = "native-test:geometry/body-exciter-plane";
  out.policy_ref = "native-test:analytic-gravity-reference";
  out.policy_revision = "1";
  out.standing = "reference";
  out.mass_kg = 1e-6;
  out.duration_samples = 512;
  for (std::size_t node = 0; node < body.input().nodes.size(); ++node)
    for (unsigned axis = 0; axis < 3; ++axis)
      out.plane_position_metres[axis] +=
          body.input().exciter.node_weights[node] *
          body.input().nodes[node].rest_metres[axis];
  for (unsigned axis = 0; axis < 3; ++axis) {
    out.outward_normal[axis] = -body.input().exciter.axis[axis];
    out.particle_position_metres[axis] =
        out.plane_position_metres[axis] + .5 * out.outward_normal[axis];
    out.gravity_metres_per_second_squared[axis] =
        -9.81 * out.outward_normal[axis];
  }
  return out;
}
static std::vector<float>
actual_force_passage(ql::PhysicalBody &body,
                     const ql::PreparedPhysicalSceneContact &contact,
                     std::uint64_t end, std::size_t partition,
                     double &maximum_energy, double &maximum_displacement) {
  std::vector<float> pcm;
  pcm.reserve(end - body.samples_elapsed());
  while (body.samples_elapsed() < end) {
    const auto cursor = body.samples_elapsed();
    const auto frames = std::min<std::uint64_t>(partition, end - cursor);
    std::array<double, 512> force{};
    std::array<float, 512> output{};
    ql::PhysicalSnapshot visible{};
    for (std::size_t frame = 0; frame < frames; ++frame) {
      const auto sample = cursor + frame;
      if (sample >= contact.force.start_sample &&
          sample - contact.force.start_sample < contact.force.frames)
        force[frame] =
            contact.force.force_newtons[sample - contact.force.start_sample];
    }
    callback = true;
    assert(body.advance_force_block(force.data(), output.data(), frames,
                                    body.body_revision(), cursor));
    assert(ql::write_physical_snapshot(body, visible, body.body_revision(),
                                       cursor + frames));
    callback = false;
    maximum_energy = std::max(maximum_energy, visible.mechanical_energy_joules);
    for (std::size_t node = 0; node < visible.node_count; ++node)
      for (unsigned axis = 0; axis < 3; ++axis)
        maximum_displacement = std::max(
            maximum_displacement,
            std::abs(visible.visible_positions_metres[node][axis] -
                     body.preparation().input().nodes[node].rest_metres[axis]));
    for (std::size_t frame = 0; frame < frames; ++frame) {
      assert(std::isfinite(output[frame]));
      if (cursor + frame < contact.force.start_sample)
        assert(output[frame] == 0);
    }
    pcm.insert(pcm.end(), output.begin(), output.begin() + frames);
  }
  return pcm;
}
static std::vector<float> body_and_reopen(const std::string &directory,
                                          std::size_t partition, double &energy,
                                          double &displacement) {
  auto original = native(directory);
  const auto geometry = definition(original.body->preparation());
  const auto contact = ql::prepare_physical_scene_contact(
      original.body->preparation(), original.body->samples_elapsed(), geometry);
  assert(contact.force.start_sample == 15326);
  assert(std::abs(contact.force.impulse_newton_seconds - 3.132091952673165e-6) <
         1e-18);
  const auto cut = contact.force.start_sample + 17;
  auto before = actual_force_passage(*original.body, contact, cut, partition,
                                     energy, displacement);
  const auto checkpoint = original.body->checkpoint();
  auto reopened = native(directory);
  assert(reopened.body->restore_checkpoint(checkpoint,
                                           reopened.body->body_revision(),
                                           reopened.body->samples_elapsed()));
  double reopened_energy = energy, reopened_displacement = displacement;
  const auto end = contact.force.start_sample + contact.force.frames + 2048;
  const auto continuing = actual_force_passage(*original.body, contact, end,
                                               partition, energy, displacement);
  const auto returned =
      actual_force_passage(*reopened.body, contact, end, partition,
                           reopened_energy, reopened_displacement);
  assert(continuing == returned);
  assert(original.body->checkpoint().displacement_modal_metres ==
         reopened.body->checkpoint().displacement_modal_metres);
  assert(original.body->checkpoint().velocity_modal_metres_per_second ==
         reopened.body->checkpoint().velocity_modal_metres_per_second);
  before.insert(before.end(), continuing.begin(), continuing.end());
  return before;
}
static void complete_original_programme_refusals(const std::string &directory) {
  auto owner = native(directory);
  const auto &body = owner.body->preparation();
  auto derived = ql::prepare_physical_scene_contact(body, 0, definition(body));
  PreparedContactProgram programme(body, 0, derived.native_input);
  auto original = sw::ct::original(programme.original());
  auto original_body = sw::ct::body_input(programme.original_body());
  auto operands = sw::ct::operands(programme.operands());
  auto forces = sw::wire::array();
  for (double value : programme.force().force_newtons)
    sw::wire::append(forces.get(), json_object_new_double(value));
  const auto before = owner.body->checkpoint();
  sw::verify_original_programme(programme, original.get(), original_body.get(),
                                operands.get(), forces.get());
  for (std::size_t i = 0; i < ql::physical_max_frames; ++i) {
    auto changed = sw::copy(forces.get());
    assert(json_object_array_put_idx(
               changed.get(), i,
               json_object_new_double(programme.force().force_newtons[i] +
                                      1e-9)) == 0);
    refused([&] {
      sw::verify_original_programme(programme, original.get(),
                                    original_body.get(), operands.get(),
                                    changed.get());
    });
  }
  for (const char *key :
       {"route_ref", "source_ref", "policy_ref", "policy_revision"}) {
    auto changed = sw::copy(original.get());
    sw::wire::text(changed.get(), key, "native-test:changed-original");
    refused([&] {
      sw::verify_original_programme(programme, changed.get(),
                                    original_body.get(), operands.get(),
                                    forces.get());
    });
  }
  auto stale = sw::copy(original_body.get());
  sw::wire::u64(stale.get(), "body_revision", body.input().body_revision + 1);
  auto wrong_face = sw::copy(original_body.get());
  sw::wire::flag(wrong_face.get(), "pratibimba", !body.input().pratibimba);
  auto lost = sw::copy(original_body.get());
  auto *nodes = packet::field(lost.get(), "nodes");
  assert(json_object_array_del_idx(nodes, json_object_array_length(nodes) - 1,
                                   1) == 0);
  for (auto *changed : {stale.get(), wrong_face.get(), lost.get()})
    refused([&] {
      sw::verify_original_programme(programme, original.get(), changed,
                                    operands.get(), forces.get());
    });
  auto no_unused_suffix = sw::copy(forces.get());
  assert(json_object_array_del_idx(no_unused_suffix.get(), 511, 1) == 0);
  refused([&] {
    sw::verify_original_programme(programme, original.get(),
                                  original_body.get(), operands.get(),
                                  no_unused_suffix.get());
  });
  auto reservation = parse(
      R"({"schema":"ql.native-contact-retention-reservation/v1","source_record_bytes_limit":"4194304","native_pulse_bytes_limit":"4194304"})");
  const auto allowance = sw::read_retention_reservation(reservation.get());
  assert(sw::retained_json_upper_bound(original_body.get()) <=
         allowance.source_record_bytes_limit);
  for (const char *bad : {"0", "01", "4194305"}) {
    auto changed = sw::copy(reservation.get());
    sw::wire::text(changed.get(), "native_pulse_bytes_limit", bad);
    refused([&] { (void)sw::read_retention_reservation(changed.get()); });
  }
  assert(before.displacement_modal_metres ==
         owner.body->checkpoint().displacement_modal_metres);
  assert(before.velocity_modal_metres_per_second ==
         owner.body->checkpoint().velocity_modal_metres_per_second);
  assert(owner.body->samples_elapsed() == 0 &&
         owner.engine->accepted_sequence() == 0);
}
static void geometry_refusals(const std::string &directory) {
  auto owner = native(directory);
  const auto original = definition(owner.body->preparation());
  auto detached_plane = original;
  auto detached_particle = original;
  auto wrong_gravity = original;
  auto tangential_velocity = original;
  const unsigned dominant =
      std::max_element(
          original.outward_normal.begin(), original.outward_normal.end(),
          [](double a, double b) { return std::abs(a) < std::abs(b); }) -
      original.outward_normal.begin();
  const unsigned tangent = (dominant + 1) % 3;
  for (unsigned axis = 0; axis < 3; ++axis)
    detached_plane.plane_position_metres[axis] +=
        .001 * original.outward_normal[axis];
  detached_particle.particle_position_metres[tangent] += .01;
  wrong_gravity.gravity_metres_per_second_squared[tangent] += .1;
  tangential_velocity.particle_velocity_metres_per_second[tangent] = .1;
  auto below = original;
  for (unsigned axis = 0; axis < 3; ++axis)
    below.particle_position_metres[axis] -= original.outward_normal[axis];
  for (const auto &bad : {detached_plane, detached_particle, wrong_gravity,
                          tangential_velocity, below})
    refused([&] {
      (void)ql::prepare_physical_scene_contact(owner.body->preparation(), 0,
                                               bad);
    });
  auto excessive = original;
  excessive.mass_kg = 1000;
  refused([&] {
    (void)ql::prepare_physical_scene_contact(owner.body->preparation(), 0,
                                             excessive);
  });
  auto borrowed = owner.body->preparation().input();
  borrowed.exciter.axis = {};
  borrowed.exciter.axis[tangent] = 1;
  const ql::PreparedPhysicalBody disconnected(borrowed);
  refused([&] {
    (void)ql::prepare_physical_scene_contact(disconnected, 0, original);
  });
  auto fixed_only = owner.body->preparation().input();
  auto fixed = std::find_if(
      fixed_only.nodes.begin(), fixed_only.nodes.end(), [](const auto &node) {
        return node.fixed[0] && node.fixed[1] && node.fixed[2];
      });
  ql::require(fixed != fixed_only.nodes.end(),
              "actual anchored body must retain its fixed source node");
  std::fill(fixed_only.exciter.node_weights.begin(),
            fixed_only.exciter.node_weights.end(), 0);
  fixed_only.exciter.node_weights[fixed - fixed_only.nodes.begin()] = 1;
  const ql::PreparedPhysicalBody fixed_exciter(fixed_only);
  const auto fixed_definition = definition(fixed_exciter);
  refused([&] {
    (void)ql::prepare_physical_scene_contact(fixed_exciter, 0,
                                             fixed_definition);
  });
}
static void generic_ingress_refusals(const std::string &directory) {
  auto actual_packet = parse(file(directory + "/baseline.packet.json"));
  auto actual_basis = parse(file(directory + "/baseline.basis.json"));
  management_transport::Control control;
  auto prepare = wire::object();
  wire::text(prepare.get(), "schema", "ql.performance-control/v1");
  wire::text(prepare.get(), "operation", "prepare");
  wire::text(prepare.get(), "session_ref",
             "native-test:retained-contact-session");
  wire::put(prepare.get(), "packet", json_object_get(actual_packet.get()));
  wire::put(prepare.get(), "actual_native_basis",
            json_object_get(actual_basis.get()));
  wire::flag(prepare.get(), "m1_pratibimba", true);
  wire::flag(prepare.get(), "physical_pratibimba", true);
  auto body_source = wire::object();
  wire::text(body_source.get(), "kind", "referenceMetric");
  management_transport::null(body_source.get(), "recipe_ref");
  management_transport::null(body_source.get(), "validated_m3_generation");
  wire::put(prepare.get(), "body_source", body_source.release());
  management_transport::null(prepare.get(), "current_source_packet");
  auto born = control.execute(prepare.get());
  assert(packet::boolean(packet::field(born.get(), "accepted")));
  auto request = wire::object();
  wire::text(request.get(), "schema", "ql.performance-control/v1");
  wire::text(request.get(), "operation", "checkpoint");
  wire::text(request.get(), "session_ref",
             "native-test:retained-contact-session");
  wire::put(request.get(), "expected_transport_epoch",
            json_object_get(packet::field(packet::field(born.get(), "reading"),
                                          "transport_epoch")));
  wire::put(
      request.get(), "expected_source",
      json_object_get(packet::field(
          packet::field(actual_packet.get(), "determination"), "identity")));
  wire::put(request.get(), "expected_body_revision",
            json_object_get(packet::field(
                packet::field(packet::field(born.get(), "reading"), "scope"),
                "body_revision")));
  const auto before = control.execute(request.get());
  for (const char *operation :
       {"contact-prepare", "contact-apply", "contact-trigger"}) {
    auto bad = sw::copy(request.get());
    wire::text(bad.get(), "operation", operation);
    refused([&] { (void)control.execute(bad.get()); });
    wire::text(bad.get(), "schema", sw::request_schema);
    refused([&] { (void)control.execute(bad.get()); });
  }
  const auto after = control.execute(request.get());
  assert(json_object_equal(packet::field(before.get(), "payload"),
                           packet::field(after.get(), "payload")));
}
int main(int argc, char **argv) {
  assert(argc == 2 || argc == 3);
  const std::string directory = argv[1];
  geometry_refusals(directory);
  complete_original_programme_refusals(directory);
  generic_ingress_refusals(directory);
  double energy = 0, displacement = 0;
  const auto single = body_and_reopen(directory, 1, energy, displacement);
  double other_energy = 0, other_displacement = 0;
  assert(single ==
         body_and_reopen(directory, 128, other_energy, other_displacement));
  assert(single ==
         body_and_reopen(directory, 512, other_energy, other_displacement));
  double peak = 0, squared = 0;
  for (float value : single) {
    peak = std::max(peak, std::abs(double(value)));
    squared += double(value) * value;
  }
  auto receipt = wire::object();
  wire::text(receipt.get(), "schema",
             "ql.actual-native-scene-contact-geometry/v1");
  wire::u64(receipt.get(), "impact_sample", 15326);
  wire::u64(receipt.get(), "sample_count", single.size());
  wire::real(receipt.get(), "peak_linear", peak);
  wire::real(receipt.get(), "rms_linear", std::sqrt(squared / single.size()));
  wire::real(receipt.get(), "maximum_mechanical_energy_joules", energy);
  wire::real(receipt.get(), "maximum_displacement_metres", displacement);
  wire::u64(receipt.get(), "callback_allocations", callback_allocations);
  wire::u64(receipt.get(), "callback_releases", callback_releases);
  wire::text(receipt.get(), "scope",
             "actual native geometry/force/P/cold continuation; C private "
             "issuer receipt required separately");
  if (argc == 3) {
    std::ofstream out(argv[2], std::ios::binary | std::ios::trunc);
    assert(out);
    out << json_object_to_json_string_ext(receipt.get(), JSON_C_TO_STRING_PLAIN)
        << '\n';
    assert(out.good());
  }
  assert(peak > 0 && energy > 0 && displacement > 0);
  assert(callback_allocations == 0 && callback_releases == 0);
  std::cout << "actual source body contact geometry/force/visible/P/reopen and "
               "generic ingress refusal passed\n";
}
