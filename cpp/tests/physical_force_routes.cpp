#include "../test_support/allocation_hooks.hpp"
#include <cassert>
#include <iostream>
#include <new>
#include <ql/physical_force_routes.hpp>
#include <ql/physical_snapshot.hpp>
using namespace ql;
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
void *operator new(std::size_t bytes, std::align_val_t alignment) {
  if (callback_probe)
    ++allocations;
  if (void *p = ql_test_allocate_aligned(bytes, static_cast<std::size_t>(alignment)))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t bytes, std::align_val_t alignment) {
  return ::operator new(bytes, alignment);
}
void operator delete(void *p, std::align_val_t) noexcept { ::operator delete(p); }
void operator delete[](void *p, std::align_val_t a) noexcept {
  ::operator delete(p, a);
}
void operator delete(void *p, std::size_t, std::align_val_t a) noexcept {
  ::operator delete(p, a);
}
void operator delete[](void *p, std::size_t, std::align_val_t a) noexcept {
  ::operator delete(p, a);
}
static void near(double actual, double expected, double absolute = 1e-12,
                 double relative = 1e-9) {
  assert(std::isfinite(actual));
  assert(std::abs(actual - expected) <= absolute + relative * std::abs(expected));
}
template <class F> static void refused(F test) {
  bool rejected = false;
  try {
    test();
  } catch (const std::invalid_argument &) {
    rejected = true;
  }
  assert(rejected);
}
// Two actual axial members in one numerical body. Their independent metric
// axes and lengths give omega_x=1000/s, omega_y=500/s. Reference metric
// controls supply units; no source name, anatomy label or ordinal supplies m.
static PhysicalBodyInput two_axes() {
  PhysicalBodyInput in;
  in.event_ref = "controlled:force-routes/event";
  in.subject_ref = "controlled:force-routes/subject";
  in.source_coordinate = "#3-2-1-1-1";
  in.source_revision = ql_m_live_source_revision();
  in.geometry_ref = "controlled:two-axial-members";
  in.geometry_revision = "1";
  in.geometry_source_ref = "controlled:metric-bar-reference";
  in.geometry_standing = "reference";
  in.preparation_ref = "controlled:force-routes/preparation";
  in.state_ref = "controlled:force-routes/state";
  in.source_generation = 7;
  in.body_revision = 1;
  in.sample_rate = 48000;
  in.pratibimba = true;
  in.family = BodyFamily::AxialTruss;
  in.material = {"controlled:elastic-material", "1", "controlled:elastic-law",
                 "reference", 1e6, 2, 0, 0};
  in.nodes = {{1, "#3-2-1-1-1", {0, 0, 0}, 0, {true, true, true}},
              {2, "#3-0", {1, 0, 0}, 0, {false, true, true}},
              {3, "#3-0", {0, 2, 0}, 0, {true, false, true}}};
  in.edges = {{0, 1, 1e-4, 0}, {0, 2, 1e-4, 0}};
  in.exciter = {{1, 0, 0}, {0, 1, 0}};
  const double r = 1 / std::sqrt(2.0);
  in.pickup = {{r, r, 0}, {0, .5, .5}};
  in.pickup_linear_per_metre = 1000;
  in.max_force_newtons = 10;
  in.max_impulse_newton_seconds = .01;
  in.max_displacement_metres = .1;
  return in;
}
static PhysicalForceSourceBasis source(const PreparedPhysicalBody &body) {
  PhysicalForceSourceBasis result;
  result.event_ref = body.input().event_ref;
  result.subject_ref = body.input().subject_ref;
  result.registry_revision = ql_m_live_registry_revision();
  result.source_revision = ql_m_live_source_revision();
  result.definition_ref = "controlled:native-receiving-definition";
  result.source_instance_ref = "controlled:actual-source-program-instance";
  result.determination_ref = "controlled:admitted-vimarsha-determination";
  result.m2_writer_coordinate = "#2-1";
  result.m2_pratibimba = true;
  result.m1_revision = 17;
  result.m2_generation = 91;
  result.m3_generation = body.input().source_generation;
  return result;
}
// Actual current source identities/relations, read through the real C owner.
// These reference numerical tests do not claim personal consent/currentness;
// the A/N integration tests must construct that original native definition.
static std::vector<PhysicalForceRouteInput>
native_routes(const PreparedPhysicalBody &body) {
  std::vector<PhysicalForceRouteInput> result;
  for (unsigned planet_index = 0; planet_index < 10; ++planet_index) {
    QL_M2_PlanetChakraRoute native{};
    const auto status = ql_m2_planet_chakra_route(planet_index, &native);
    if (planet_index == 7) {
      assert(status == QL_M2_UNAVAILABLE);
      continue;
    }
    assert(status == QL_M2_OK);
    PhysicalForceRouteInput route;
    route.driver_ref = "controlled:original-driver/" + std::to_string(planet_index);
    route.target_ref = source(body).definition_ref + "#driver/" +
                       std::to_string(planet_index);
    route.program_ref = "controlled:independent-force-program/" +
                        std::to_string(planet_index);
    route.native_planet_index = planet_index;
    route.centre_ordinal = native.chakra_index - 1;
    route.planet_node_id = native.planet_id;
    route.chakra_node_id = native.chakra_id;
    const auto *planet = ql_m_live_node_by_id(native.planet_id);
    const auto *chakra = ql_m_live_node_by_id(native.chakra_id);
    assert(planet && chakra);
    route.planet_coordinate = planet->source_ref;
    route.chakra_coordinate = chakra->source_ref;
    for (std::size_t relation = 0; relation < native.relation_count; ++relation) {
      const auto *actual = ql_m2_planet_chakra_relation(planet_index, relation);
      assert(actual);
      route.relation_refs.push_back(actual->relation_ref);
    }
    route.share_numerator = std::to_string(planet_index + 1);
    route.share_denominator = "55"; // all ten reference contributor weights
    route.calibration_ref = "controlled:seven-explicit-metric-maps";
    route.calibration_revision = "1";
    route.calibration_source_ref = "controlled:metric-force-calibration";
    route.calibration_standing = "reference";
    route.projection_ref = route.calibration_ref + "#centre/" +
                           std::to_string(route.centre_ordinal);
    for (const auto &node : body.input().nodes)
      route.metric_node_ids.push_back(node.identity);
    // Seven explicitly calibrated maps, not seven anatomy measurements.
    const double x = native.chakra_index / 8.0, r = 1 / std::sqrt(2.0);
    route.projection = {{r, r, 0}, {0, x, 1 - x}};
    route.peak_force_newtons = .5;
    result.push_back(std::move(route));
  }
  assert(result.size() == 9);
  return result;
}
static std::vector<PhysicalForceRouteInput>
orthogonal_routes(const PreparedPhysicalBody &body) {
  auto routes = native_routes(body);
  auto distinct = std::find_if(routes.begin() + 1, routes.end(), [&](const auto &r) {
    return r.centre_ordinal != routes.front().centre_ordinal;
  });
  assert(distinct != routes.end());
  routes[1] = *distinct;
  routes.resize(2);
  routes[0].projection = {{1, 0, 0}, {0, 1, 0}};
  routes[1].projection = {{0, 1, 0}, {0, 0, 1}};
  return routes;
}
static std::array<Vec3, 3> displacements(const PhysicalBody &body) {
  std::array<Vec3, 3> result{};
  assert(body.write_displacements(result.data(), result.size(), body.body_revision(),
                                  body.samples_elapsed()));
  return result;
}
static void independent_metric_forces_have_analytic_effects() {
  PreparedPhysicalBody prepared(two_axes());
  auto inputs = orthogonal_routes(prepared);
  PreparedPhysicalForceRoutes routes(prepared, source(prepared), inputs, 0);
  PhysicalBody body(prepared);
  std::array<double, 257> fx{}, fy{}, m1{};
  fx.fill(.02);
  fy.fill(.03);
  m1.fill(.004);
  std::array<PhysicalRouteForceBlock, 2> forces{{
      {0, routes.route_seal(0), fx.data(), .5, true},
      {1, routes.route_seal(1), fy.data(), -1, true}}};
  std::array<float, 257> pcm{};
  PhysicalForceRouteReceipt receipt;
  assert(routes.preflight(prepared, source(prepared), 0));
  assert(routes.advance_force_block(body, {m1.data(), .25, true}, forces.data(),
                                    forces.size(), pcm.data(), pcm.size(), 1, 0,
                                    &receipt));
  const double time = 257.0 / 48000;
  const double x = (.02 * .5 + .004 * .25) / 100 * (1 - std::cos(1000 * time));
  const double y = -.03 / 50 * (1 - std::cos(500 * time));
  const auto actual = displacements(body);
  near(actual[1][0], x);
  near(actual[2][1], y);
  near(pcm.back(), 1000 * .5 / std::sqrt(2.0) * (x + y), 1e-8, 1e-6);
  assert(receipt.start_sample == 0 && receipt.end_sample == 257 &&
         receipt.body_revision == 1 && receipt.route_count == 2);
  assert(receipt.earth_frame_node_id == ql_m_live_resolve("#2-5-0/1-0")->id);
  near(receipt.peak_total_absolute_force_newtons, .041);
  assert(receipt.scalar_m1_enabled && receipt.scalar_m1_gain == .25);
  for (std::size_t r = 0; r < 2; ++r) {
    assert(receipt.routes[r].planet_node_id == inputs[r].planet_node_id);
    assert(receipt.routes[r].chakra_node_id == inputs[r].chakra_node_id);
  }
  PhysicalSnapshot snapshot;
  assert(write_physical_snapshot(body, snapshot, 1, 257));
  assert(snapshot.samples_elapsed == receipt.end_sample);
  near(snapshot.visible_positions_metres[1][0], 1 + x);
  near(snapshot.visible_positions_metres[2][1], 2 + y);
  assert(!write_physical_snapshot(body, snapshot, 1, 256));
  // A relabelled common scalar misses the independent y-axis physical cause.
  inputs[1].projection = inputs[0].projection;
  PreparedPhysicalForceRoutes collapsed(prepared, source(prepared), inputs, 0);
  PhysicalBody wrong(prepared);
  forces[0].preparation_seal = collapsed.route_seal(0);
  forces[1].preparation_seal = collapsed.route_seal(1);
  std::array<float, 257> wrong_pcm{};
  assert(collapsed.advance_force_block(wrong, {m1.data(), .25, true}, forces.data(),
                                       2, wrong_pcm.data(), wrong_pcm.size(), 1, 0));
  assert(displacements(wrong)[2][1] == 0 && y != 0);
  assert(wrong_pcm != pcm);
}
static void scalar_world_compatibility_and_explicit_off() {
  PreparedPhysicalBody prepared(two_axes());
  PhysicalBody world(prepared), scalar(prepared), off(prepared);
  // The ordinary scalar World path requires no receiving definition at all.
  std::array<double, 512> force{};
  for (std::size_t i = 0; i < force.size(); ++i)
    force[i] = .01 * std::sin(2 * pi * 173 * i / 48000.0);
  std::array<float, 512> a{}, b{}, c{};
  assert(world.advance_force_block(force.data(), a.data(), a.size(), 1, 0));
  PreparedPhysicalForceRoutes empty(prepared, source(prepared), {}, 0);
  assert(empty.advance_force_block(scalar, {force.data(), 1, true}, nullptr, 0,
                                   b.data(), b.size(), 1, 0));
  assert(a == b);
  assert(world.checkpoint().displacement_modal_metres ==
         scalar.checkpoint().displacement_modal_metres);
  assert(world.checkpoint().velocity_modal_metres_per_second ==
         scalar.checkpoint().velocity_modal_metres_per_second);
  assert(empty.advance_force_block(off, {}, nullptr, 0, c.data(), c.size(), 1, 0));
  assert(off.mechanical_energy_joules() == 0);
  for (float sample : c)
    assert(sample == 0);
}
static void nine_sources_keep_identity_program_and_projection() {
  PreparedPhysicalBody prepared(two_axes());
  const auto inputs = native_routes(prepared);
  PreparedPhysicalForceRoutes routes(prepared, source(prepared), inputs, 0);
  std::array<std::array<double, 512>, 9> waveforms{};
  std::array<PhysicalRouteForceBlock, 9> blocks{};
  std::array<std::array<float, 512>, 9> isolated_pcm{};
  std::array<std::array<Vec3, 3>, 9> isolated_displacements{};
  std::array<std::vector<double>, 9> isolated_q{};
  std::array<bool, 7> centres{};
  for (std::size_t r = 0; r < 9; ++r) {
    centres[inputs[r].centre_ordinal] = true;
    for (std::size_t sample = 0; sample < 512; ++sample)
      waveforms[r][sample] = .02 * std::sin(2 * pi * (83 + 37 * r) * sample /
                                         48000.0 + .17 * r);
    blocks[r] = {r, routes.route_seal(r), waveforms[r].data(), .1 * (r + 1), true};
  }
  for (bool centre : centres)
    assert(centre);
  PhysicalBody combined(prepared);
  std::array<float, 512> combined_pcm{};
  PhysicalForceRouteReceipt receipt;
  assert(routes.advance_force_block(combined, {}, blocks.data(), 9,
                                    combined_pcm.data(), 512, 1, 0, &receipt));
  for (std::size_t r = 0; r < 9; ++r) {
    PhysicalBody isolated(prepared);
    auto one = blocks;
    for (std::size_t other = 0; other < 9; ++other)
      one[other].enabled = other == r;
    assert(routes.advance_force_block(isolated, {}, one.data(), 9,
                                      isolated_pcm[r].data(), 512, 1, 0));
    assert(isolated.mechanical_energy_joules() > 0);
    isolated_displacements[r] = displacements(isolated);
    isolated_q[r] = isolated.checkpoint().displacement_modal_metres;
    for (std::size_t previous = 0; previous < r; ++previous) {
      assert(inputs[previous].driver_ref != inputs[r].driver_ref);
      assert(inputs[previous].program_ref != inputs[r].program_ref);
      assert(isolated_q[previous] != isolated_q[r]);
    }
    assert(receipt.routes[r].enabled &&
           receipt.routes[r].preparation_seal == routes.route_seal(r) &&
           receipt.routes[r].peak_applied_force_newtons > 0);
    auto omitted = blocks;
    omitted[r].enabled = false;
    PhysicalBody without(prepared);
    std::array<float, 512> without_pcm{};
    assert(routes.advance_force_block(without, {}, omitted.data(), 9,
                                      without_pcm.data(), 512, 1, 0));
    assert(without_pcm != combined_pcm);
    assert(displacements(without) != displacements(combined));
  }
  const auto actual = displacements(combined);
  for (std::size_t node = 0; node < 3; ++node)
    for (unsigned axis = 0; axis < 3; ++axis) {
      double expected = 0;
      for (const auto &one : isolated_displacements)
        expected += one[node][axis];
      near(actual[node][axis], expected);
    }
  for (std::size_t sample = 0; sample < 512; ++sample) {
    double expected = 0;
    for (const auto &one : isolated_pcm)
      expected += one[sample];
    near(combined_pcm[sample], expected, 1e-7, 1e-6);
  }
  assert(routes.route_count() == 9 && combined.samples_elapsed() == 512);
  // Two sources can share one of seven maps while their force programs and
  // source IDs remain distinct; the original ten-way denominator stays 55.
  bool shared = false;
  for (std::size_t a = 0; a < 9; ++a)
    for (std::size_t b = a + 1; b < 9; ++b)
      if (inputs[a].projection_ref == inputs[b].projection_ref) {
        shared = true;
        assert(routes.route_seal(a) != routes.route_seal(b));
      }
  assert(shared);
  for (std::size_t r = 0; r < 9; ++r)
    assert(routes.route_input(r).share_denominator == "55");
}
static void source_projection_and_basis_admission() {
  PreparedPhysicalBody prepared(two_axes());
  auto inputs = native_routes(prepared);
  const auto basis = source(prepared);
  const auto reject_basis = [&](const PhysicalForceSourceBasis &wrong) {
    refused([&] { PreparedPhysicalForceRoutes candidate(prepared, wrong, inputs, 0); });
  };
  auto wrong = basis;
  wrong.registry_revision += ":stale";
  reject_basis(wrong);
  wrong = basis;
  wrong.m2_writer_coordinate = "#2-5-5";
  reject_basis(wrong);
  wrong = basis;
  wrong.m2_pratibimba = false;
  reject_basis(wrong);
  wrong = basis;
  wrong.event_ref += ":disconnected";
  reject_basis(wrong);
  wrong = basis;
  ++wrong.m3_generation;
  reject_basis(wrong);
  wrong = basis;
  wrong.definition_ref.push_back('\0');
  reject_basis(wrong);
  const auto reject_routes = [&](const std::vector<PhysicalForceRouteInput> &bad) {
    refused([&] { PreparedPhysicalForceRoutes candidate(prepared, basis, bad, 0); });
  };
  auto bad = inputs;
  bad[0].planet_coordinate = "#2-5-5";
  reject_routes(bad);
  bad = inputs;
  bad[0].relation_refs.front() += ":disconnected";
  reject_routes(bad);
  bad = inputs;
  bad[0].metric_node_ids[1] += 1000;
  reject_routes(bad);
  bad = inputs;
  bad[0].projection.node_weights = {1, 0, 0};
  reject_routes(bad);
  bad = inputs;
  bad[0].projection.axis = {0, 0, 1};
  reject_routes(bad);
  bad = inputs;
  bad[0].projection.node_weights[1] = std::numeric_limits<double>::quiet_NaN();
  reject_routes(bad);
  bad = inputs;
  bad[0].target_ref = "arbitrary:nonempty-target";
  reject_routes(bad);
  bad = inputs;
  bad[1] = bad[0];
  reject_routes(bad);
  bad = inputs;
  bad.push_back(inputs[0]);
  reject_routes(bad);
  PreparedPhysicalForceRoutes routes(prepared, basis, inputs, 0);
  assert(routes.preflight(prepared, basis, 123));
  // Unequal producer generations are intentional. Currentness is qualified
  // through the owner's actual basis, never by equating these generations.
  assert(basis.m1_revision != basis.m2_generation &&
         basis.m2_generation != basis.m3_generation);
  wrong = basis;
  ++wrong.m1_revision;
  assert(!routes.preflight(prepared, wrong, 0));
  wrong = basis;
  ++wrong.m2_generation;
  assert(!routes.preflight(prepared, wrong, 0));
  auto initial = two_axes();
  initial.source_generation = 0;
  PreparedPhysicalBody initial_preparation(initial);
  auto initial_source = source(initial_preparation);
  initial_source.m1_revision = 0;
  initial_source.m2_generation = 0;
  PreparedPhysicalForceRoutes initial_routes(initial_preparation, initial_source,
                                             native_routes(initial_preparation), 0);
  assert(initial_routes.preflight(initial_preparation, initial_source, 0));
  auto changed = two_axes();
  changed.material.young_modulus_pa *= 4; // held labels/revision still stale basis
  assert(!routes.preflight(PreparedPhysicalBody(changed), basis, 0));
  changed = two_axes();
  changed.pratibimba = false;
  assert(!routes.preflight(PreparedPhysicalBody(changed), basis, 0));
}
static void callback_refusal_is_atomic_and_allocation_free() {
  PreparedPhysicalBody prepared(two_axes());
  const auto inputs = native_routes(prepared);
  PreparedPhysicalForceRoutes routes(prepared, source(prepared), inputs, 0);
  PhysicalBody body(prepared);
  std::array<double, 512> force{};
  force.fill(.01);
  std::array<PhysicalRouteForceBlock, 9> blocks{};
  for (std::size_t r = 0; r < 9; ++r)
    blocks[r] = {r, routes.route_seal(r), force.data(), 1, true};
  std::array<float, 512> pcm{};
  PhysicalForceRouteReceipt receipt;
  PhysicalSnapshot snapshot;
  callback_probe = true;
  assert(routes.advance_force_block(body, {}, blocks.data(), 9, pcm.data(), 512,
                                    1, 0, &receipt));
  assert(write_physical_snapshot(body, snapshot, 1, 512));
  callback_probe = false;
  assert(allocations == 0 && releases == 0);
  assert(snapshot.samples_elapsed == receipt.end_sample &&
         snapshot.body_revision == receipt.body_revision &&
         snapshot.pickup_linear == pcm.back());
  const auto saved = body.checkpoint();
  std::array<float, 512> unchanged;
  unchanged.fill(123);
  const auto check = [&] {
    const auto after = body.checkpoint();
    assert(after.samples_elapsed == saved.samples_elapsed &&
           after.displacement_modal_metres == saved.displacement_modal_metres &&
           after.velocity_modal_metres_per_second == saved.velocity_modal_metres_per_second);
    for (float sample : unchanged)
      assert(sample == 123);
    assert(receipt.start_sample == 0 && receipt.end_sample == 512);
  };
  const auto reject = [&](const auto &candidate, std::size_t count,
                          std::uint64_t revision, std::uint64_t cursor) {
    callback_probe = true;
    assert(!routes.advance_force_block(body, {}, candidate.data(), count,
                                       unchanged.data(), 512, revision, cursor,
                                       &receipt));
    callback_probe = false;
    assert(allocations == 0 && releases == 0);
    check();
  };
  reject(blocks, 9, 2, 512);
  reject(blocks, 9, 1, 511);
  reject(blocks, 8, 1, 512);
  auto bad = blocks;
  ++bad[0].preparation_seal;
  reject(bad, 9, 1, 512);
  auto recalibrated = inputs;
  for (auto &route : recalibrated)
    route.calibration_revision = "2";
  PreparedPhysicalForceRoutes other_calibration(prepared, source(prepared), recalibrated, 0);
  bad = blocks;
  bad[0].preparation_seal = other_calibration.route_seal(0);
  assert(bad[0].preparation_seal != blocks[0].preparation_seal);
  reject(bad, 9, 1, 512);
  bad = blocks;
  bad[0].route_index = 9;
  reject(bad, 9, 1, 512);
  bad = blocks;
  bad[1] = bad[0];
  reject(bad, 9, 1, 512);
  bad = blocks;
  bad[0].gain = 1.01;
  reject(bad, 9, 1, 512);
  bad = blocks;
  bad[0].force_newtons = nullptr;
  reject(bad, 9, 1, 512);
  force.back() = std::numeric_limits<double>::infinity();
  reject(blocks, 9, 1, 512);
  force.back() = .51;
  reject(blocks, 9, 1, 512);
  force.fill(.01);
  assert(!routes.advance_force_block(body, {}, blocks.data(), 9, unchanged.data(),
                                     0, 1, 512, &receipt));
  assert(!routes.advance_force_block(body, {}, blocks.data(), 9, unchanged.data(),
                                     513, 1, 512, &receipt));
  check();
  std::array<double, 512> scalar{};
  scalar.fill(10);
  assert(!routes.advance_force_block(body, {scalar.data(), 1, true}, blocks.data(),
                                     9, unchanged.data(), 512, 1, 512, &receipt));
  check();
  auto changed = two_axes();
  changed.material.density_kg_per_m3 *= 4;
  PhysicalBody stale{PreparedPhysicalBody(changed)};
  assert(!routes.advance_force_block(stale, {}, blocks.data(), 9, unchanged.data(),
                                     512, 1, 0, &receipt));
  assert(stale.samples_elapsed() == 0 && stale.mechanical_energy_joules() == 0);
  check();
  auto tiny = two_axes();
  tiny.max_displacement_metres = 1e-15;
  PreparedPhysicalBody tiny_preparation(tiny);
  PreparedPhysicalForceRoutes bounded(tiny_preparation, source(tiny_preparation),
                                      native_routes(tiny_preparation), 0);
  PhysicalBody limited(tiny_preparation);
  auto bounded_blocks = blocks;
  for (std::size_t r = 0; r < 9; ++r)
    bounded_blocks[r].preparation_seal = bounded.route_seal(r);
  assert(!bounded.advance_force_block(limited, {}, bounded_blocks.data(), 9,
                                      unchanged.data(), 512, 1, 0));
  assert(limited.samples_elapsed() == 0 && limited.mechanical_energy_joules() == 0);
  check();
}
static void partitions_and_checkpoint_preserve_one_native_cursor() {
  PreparedPhysicalBody prepared(two_axes());
  PreparedPhysicalForceRoutes routes(prepared, source(prepared), native_routes(prepared), 0);
  std::array<std::array<double, 2048>, 9> waves{};
  for (std::size_t r = 0; r < 9; ++r)
    for (std::size_t sample = 0; sample < 2048; ++sample)
      waves[r][sample] = .01 * std::sin(2 * pi * (107 + 29 * r) * sample /
                                     48000.0 + .13 * r);
  std::array<float, 2048> whole{}, split{}, resumed{};
  PhysicalBody a(prepared), b(prepared), reopen(prepared);
  const auto render = [&](PhysicalBody &body, float *output, std::size_t end,
                          std::size_t partition) {
    while (body.samples_elapsed() < end) {
      const auto cursor = body.samples_elapsed();
      const auto frames = std::min<std::size_t>(partition, end - cursor);
      std::array<PhysicalRouteForceBlock, 9> blocks{};
      for (std::size_t r = 0; r < 9; ++r)
        blocks[r] = {r, routes.route_seal(r), waves[r].data() + cursor, .7, true};
      assert(routes.advance_force_block(body, {}, blocks.data(), 9,
                                        output + cursor, frames, 1, cursor));
    }
  };
  render(a, whole.data(), 1024, 512);
  const auto checkpoint = a.checkpoint();
  assert(reopen.restore_checkpoint(checkpoint, 1, 0));
  render(a, whole.data(), 2048, 512);
  render(b, split.data(), 2048, 1);
  render(reopen, resumed.data(), 2048, 128);
  assert(whole == split);
  for (std::size_t sample = 1024; sample < 2048; ++sample)
    assert(whole[sample] == resumed[sample]);
  assert(displacements(a) == displacements(b) && displacements(a) == displacements(reopen));
  assert(a.checkpoint().displacement_modal_metres == reopen.checkpoint().displacement_modal_metres);
  assert(a.checkpoint().velocity_modal_metres_per_second == reopen.checkpoint().velocity_modal_metres_per_second);
  PhysicalSnapshot observed;
  assert(write_physical_snapshot(reopen, observed, 1, 2048));
  assert(observed.samples_elapsed == 2048);
}
int main() {
  independent_metric_forces_have_analytic_effects();
  scalar_world_compatibility_and_explicit_off();
  nine_sources_keep_identity_program_and_projection();
  source_projection_and_basis_admission();
  callback_refusal_is_atomic_and_allocation_free();
  partitions_and_checkpoint_preserve_one_native_cursor();
  std::cout << "native physical force routes analytic, nine-source and causal tests passed\n";
}
