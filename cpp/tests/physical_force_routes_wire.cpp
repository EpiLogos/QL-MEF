// Stdin is the actual Rust N/A/P producer and its independently replayed
// operation. The existing native floor supplies the current C/C++ owner.
#include <cassert>
#include <iostream>
#include <ql/physical_force_routes_wire.hpp>
#include <ql/physical_snapshot.hpp>
#include <ql/physical_transition.hpp>
using namespace ql;
using namespace ql::physical_wire;
static Json copy(J *source) {
  J *value = nullptr;
  assert(json_object_deep_copy(source, &value, nullptr) == 0);
  return own(value);
}
static void put(J *object, const char *name, J *value) {
  assert(value && json_object_object_add(object, name, value) == 0);
}
template <class F> static void refused(F operation) {
  bool no = false;
  try {
    operation();
  } catch (const std::invalid_argument &) {
    no = true;
  }
  assert(no);
}
static PhysicalForceSourceBasis source_basis(J *value) {
  keys(value, {"event_ref", "subject_ref", "registry_revision", "source_revision",
               "definition_ref", "source_instance_ref", "determination_ref",
               "m2_writer_coordinate", "m2_pratibimba", "m1_revision", "m2_generation", "m3_generation"});
  PhysicalForceSourceBasis basis;
  basis.event_ref = text(field(value, "event_ref"));
  basis.subject_ref = text(field(value, "subject_ref"));
  basis.registry_revision = text(field(value, "registry_revision"));
  basis.source_revision = text(field(value, "source_revision"));
  basis.definition_ref = text(field(value, "definition_ref"));
  basis.source_instance_ref = text(field(value, "source_instance_ref"));
  basis.determination_ref = text(field(value, "determination_ref"));
  basis.m2_writer_coordinate = text(field(value, "m2_writer_coordinate"));
  basis.m2_pratibimba = boolean(field(value, "m2_pratibimba"));
  basis.m1_revision = checkpoint_decimal(field(value, "m1_revision"));
  basis.m2_generation = checkpoint_decimal(field(value, "m2_generation"));
  basis.m3_generation = checkpoint_decimal(field(value, "m3_generation"));
  return basis;
}
static std::vector<std::string> programs(J *value) {
  std::vector<std::string> refs;
  const auto n = count(value, physical_max_personal_force_routes);
  for (std::size_t i = 0; i < n; ++i)
    refs.push_back(text(at(value, i)));
  return refs;
}
int main() {
  std::string line;
  std::getline(std::cin, line);
  assert(!line.empty() && line.size() <= 8 * 1024 * 1024);
  auto tok = std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(
      json_tokener_new_ex(64), json_tokener_free);
  json_tokener_set_flags(tok.get(), JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  auto input = own(json_tokener_parse_ex(tok.get(), line.data(), int(line.size())));
  assert(json_tokener_get_error(tok.get()) == json_tokener_success &&
         json_tokener_get_parse_end(tok.get()) == line.size());
  keys(input.get(), {"preparation", "current_m3", "operation", "current_operation",
                     "source_basis", "program_refs", "world_operation",
                     "world_current_operation", "world_source_basis", "after_material"});
  auto operation = field(input.get(), "operation");
  auto current = field(input.get(), "current_operation");
  const auto basis = source_basis(field(input.get(), "source_basis"));
  const auto program_refs = programs(field(input.get(), "program_refs"));
  PreparedPhysicalBody prepared(read_prepared_physical_body(field(input.get(), "preparation"),
                                                            field(input.get(), "current_m3"), true));
  auto joined = read_prepared_physical_force_routes(operation, current, basis, prepared, program_refs, 0);
  assert(joined.routes.route_count() == 9 && joined.routes.preflight(prepared, basis, 0));
  const auto world_basis = source_basis(field(input.get(), "world_source_basis"));
  auto world_routes = read_prepared_physical_force_routes(
      field(input.get(), "world_operation"), field(input.get(), "world_current_operation"),
      world_basis, prepared, {}, 0);
  assert(world_routes.routes.route_count() == 0 && world_routes.routes.preflight(prepared, world_basis, 0));
  const auto reject_mutation = [&](auto mutate, bool both = false) {
    auto changed = copy(operation);
    mutate(changed.get());
    auto other = both ? copy(changed.get()) : copy(current);
    refused([&] { read_prepared_physical_force_routes(changed.get(), other.get(), basis, prepared, program_refs, 0); });
  };
  reject_mutation([](J *packet) {
    put(packet, "source_instance", json_object_new_string("source:disconnected"));
  });
  reject_mutation([](J *packet) {
    put(packet, "native_sample", json_object_new_string("01"));
  }, true);
  reject_mutation([](J *packet) {
    put(packet, "m3_generation", json_object_new_string("9000"));
  }, true);
  reject_mutation([](J *packet) {
    put(at(field(packet, "sources"), 0), "source_ref", json_object_new_string("#2-5-5"));
  }, true);
  reject_mutation([](J *packet) {
    auto relations = field(at(field(packet, "sources"), 0), "relation_refs");
    assert(json_object_array_put_idx(relations, 0, json_object_new_string("relation:detached")) == 0);
  }, true);
  reject_mutation([](J *packet) {
    auto ids = field(at(field(packet, "projections"), 0), "native_node_ids");
    assert(json_object_array_put_idx(ids, 1, json_object_new_string("9000")) == 0);
  }, true);
  reject_mutation([](J *packet) {
    put(at(field(packet, "sources"), 0), "share_denominator", json_object_new_string("9"));
  }, true);
  reject_mutation([](J *packet) {
    put(at(field(packet, "sources"), 0), "peak_force_newtons", json_object_new_double(1));
  }, true);
  auto stale_basis = basis;
  ++stale_basis.m1_revision;
  refused([&] { read_prepared_physical_force_routes(operation, current, stale_basis, prepared, program_refs, 0); });
  stale_basis = basis;
  stale_basis.m2_pratibimba = false;
  refused([&] { read_prepared_physical_force_routes(operation, current, stale_basis, prepared, program_refs, 0); });
  std::array<std::array<double, 1024>, 9> waveforms{};
  for (std::size_t r = 0; r < 9; ++r) {
    assert(joined.programs[r].source_hertz > 0 && joined.programs[r].peak_force_newtons > 0);
    for (std::size_t sample = 0; sample < 1024; ++sample)
      waveforms[r][sample] = joined.programs[r].peak_force_newtons * .2 *
          std::sin(2 * pi * joined.programs[r].source_hertz * sample / prepared.input().sample_rate + .19 * r);
  }
  const auto blocks_for = [&](const PreparedPhysicalForceRoutes &routes, std::size_t cursor) {
    std::array<PhysicalRouteForceBlock, 9> blocks{};
    for (std::size_t r = 0; r < 9; ++r)
      blocks[r] = {r, routes.route_seal(r), waveforms[r].data() + cursor, .5, true};
    return blocks;
  };
  PhysicalBody body(prepared);
  std::array<float, 512> pcm{}, resumed_pcm{};
  auto blocks = blocks_for(joined.routes, 0);
  PhysicalForceRouteReceipt force_receipt;
  assert(joined.routes.advance_force_block(body, {}, blocks.data(), 9, pcm.data(), 512,
                                           body.body_revision(), 0, &force_receipt));
  assert(body.mechanical_energy_joules() > 0 && body.samples_elapsed() == 512);
  PhysicalSnapshot snapshot;
  assert(write_physical_snapshot(body, snapshot, body.body_revision(), 512));
  assert(snapshot.pickup_linear == pcm.back() && snapshot.samples_elapsed == force_receipt.end_sample);
  auto witness = force_routes_preparation_witness(joined.routes);
  qualify_force_routes_preparation_witness(witness.get(), joined.routes);
  auto wrong_witness = copy(witness.get());
  put(wrong_witness.get(), "source_basis_seal", json_object_new_string("0"));
  refused([&] { qualify_force_routes_preparation_witness(wrong_witness.get(), joined.routes); });
  // Full saved P state and freshly replayed route preparation, one cursor.
  auto saved = checkpoint_wire(body.checkpoint());
  PhysicalBody reopened(prepared);
  assert(reopened.restore_checkpoint(read_checkpoint_wire(saved.get()), body.body_revision(), 0));
  auto replayed = read_prepared_physical_force_routes(operation, current, basis, prepared, program_refs, 0);
  qualify_force_routes_preparation_witness(witness.get(), replayed.routes);
  blocks = blocks_for(joined.routes, 512);
  assert(joined.routes.advance_force_block(body, {}, blocks.data(), 9, pcm.data(), 512,
                                           body.body_revision(), 512));
  blocks = blocks_for(replayed.routes, 512);
  assert(replayed.routes.advance_force_block(reopened, {}, blocks.data(), 9, resumed_pcm.data(), 512,
                                             reopened.body_revision(), 512));
  assert(pcm == resumed_pcm && body.checkpoint().displacement_modal_metres ==
         reopened.checkpoint().displacement_modal_metres);
  // Actual Rust producer prepares a changed material and separately recompiles
  // N's full original definition/calibration against the after preparation.
  auto after = field(input.get(), "after_material");
  keys(after, {"preparation", "current_m3", "operation", "current_operation", "source_basis", "program_refs",
               "world_operation", "world_current_operation", "world_source_basis"});
  PreparedPhysicalBody after_preparation(read_prepared_physical_body(field(after, "preparation"),
                                                                    field(after, "current_m3"), true));
  const auto after_world_basis = source_basis(field(after, "world_source_basis"));
  auto after_world_routes = read_prepared_physical_force_routes(
      field(after, "world_operation"), field(after, "world_current_operation"),
      after_world_basis, after_preparation, {}, 512);
  assert(after_world_routes.routes.route_count() == 0 &&
         after_world_routes.routes.preflight(after_preparation, after_world_basis, 512));
  // A real independent World operation for the original material is still
  // source-current at the M3 generation. It must not acquire the after body.
  assert(prepared.input().body_revision != after_preparation.input().body_revision);
  assert(world_basis.m3_generation == after_world_basis.m3_generation);
  std::cerr << "checking genuine native World against a changed material body\n";
  refused([&] {
    read_prepared_physical_force_routes(
        field(input.get(), "world_operation"), field(input.get(), "world_current_operation"),
        world_basis, after_preparation, {}, 0);
  });
  const auto after_basis = source_basis(field(after, "source_basis"));
  const auto after_refs = programs(field(after, "program_refs"));
  auto after_routes = read_prepared_physical_force_routes(field(after, "operation"), field(after, "current_operation"),
                                                          after_basis, after_preparation, after_refs, 512);
  assert(program_refs == after_refs);
  assert(after_routes.routes.eigenbasis_identity() != joined.routes.eigenbasis_identity());
  assert(!joined.routes.preflight(after_preparation, after_basis, 512));
  PhysicalBody ringing(prepared);
  blocks = blocks_for(joined.routes, 0);
  assert(joined.routes.advance_force_block(ringing, {}, blocks.data(), 9, pcm.data(), 512,
                                           ringing.body_revision(), 0));
  const auto before_displacement = ringing.checkpoint().displacement_modal_metres;
  PreparedPhysicalTransition transition(prepared, after_preparation, 1, 512, PhysicalLiveUpdateKind::Material,
                                        PhysicalFormTransition::ProjectCorrespondingNodes, "reference:actual-material-act");
  assert(transition.preflight(ringing));
  assert(after_routes.routes.preflight(transition.pending_after_preparation(), after_basis, 512));
  PhysicalLiveTransitionReceipt transition_receipt;
  assert(transition.apply(ringing, transition_receipt));
  assert(ringing.samples_elapsed() == 512 && ringing.state_ref() == prepared.input().state_ref);
  assert(ringing.checkpoint().displacement_modal_metres == before_displacement);
  refused([&] { qualify_force_routes_preparation_witness(witness.get(), after_routes.routes); });
  blocks = blocks_for(joined.routes, 512);
  pcm.fill(99);
  assert(!joined.routes.advance_force_block(ringing, {}, blocks.data(), 9, pcm.data(), 512,
                                            ringing.body_revision(), 512));
  for (float sample : pcm)
    assert(sample == 99);
  blocks = blocks_for(after_routes.routes, 512);
  assert(after_routes.routes.advance_force_block(ringing, {}, blocks.data(), 9, pcm.data(), 512,
                                                 ringing.body_revision(), 512));
  assert(ringing.samples_elapsed() == 1024 && pcm != resumed_pcm);
  std::cout << "actual N operation -> nine P routes -> same-body PCM/snapshot/checkpoint/material requalification passed\n";
}
