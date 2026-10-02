// Real sole-body numerical transactions, with independently calculated response
// and complete native allocation/free counters around callback-owned
// operations.
#include "../test_support/allocation_hooks.hpp"
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <new>
#include <ql/physical_snapshot.hpp>
#include <ql/physical_transition.hpp>
using namespace ql;
static bool callback_probe = false;
static std::size_t allocations = 0, releases = 0;
void *operator new(std::size_t n) {
  if (callback_probe)
    ++allocations;
  if (void *p = ql_test_allocate(n))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t n) { return ::operator new(n); }
void operator delete(void *p) noexcept {
  if (p && callback_probe)
    ++releases;
  ql_test_release(p);
}
void operator delete[](void *p) noexcept { ::operator delete(p); }
void operator delete(void *p, std::size_t) noexcept { ::operator delete(p); }
void operator delete[](void *p, std::size_t) noexcept { ::operator delete(p); }
void *operator new(std::size_t n, std::align_val_t a) {
  if (callback_probe)
    ++allocations;
  if (void *p = ql_test_allocate_aligned(n, static_cast<std::size_t>(a)))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t n, std::align_val_t a) {
  return ::operator new(n, a);
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
static void near(double a, double b, double absolute = 1e-12,
                 double relative = 1e-9) {
  assert(std::isfinite(a));
  assert(std::abs(a - b) <= absolute + relative * std::abs(b));
}
static PhysicalBodyInput bar() {
  PhysicalBodyInput in;
  in.event_ref = "controlled:live-physical/event";
  in.subject_ref = "controlled:live-physical/subject";
  in.source_coordinate = "#3-2-1-1-1";
  in.source_revision =
      "907c46bc8a65b47e12f14aa4d8b444263dc956a1a7b4b6d038e57223d6073288";
  in.geometry_ref = "controlled:metric-axial-bar";
  in.geometry_revision = "1";
  in.geometry_source_ref = "controlled:analytic-bar";
  in.geometry_standing = "reference";
  in.preparation_ref = "controlled:live-body/preparation1";
  in.state_ref = "controlled:live-body/state";
  in.source_generation = 7;
  in.body_revision = 1;
  in.sample_rate = 48000;
  in.pratibimba = true;
  in.family = BodyFamily::AxialTruss;
  in.material = {"controlled:elastic",
                 "1",
                 "controlled:analytic-material",
                 "reference",
                 1e6,
                 2,
                 0,
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
static PhysicalBodyInput next(PhysicalBodyInput in) {
  in.body_revision++;
  in.preparation_ref = "controlled:live-body/preparation2";
  in.material.revision = "2";
  return in;
}
static void advance(PhysicalBody &body, std::size_t frames, double force = 0) {
  std::array<double, 512> f{};
  f.fill(force);
  std::array<float, 512> p{};
  while (frames) {
    const auto n = std::min(frames, f.size());
    assert(body.advance_force_block(f.data(), p.data(), n, body.body_revision(),
                                    body.samples_elapsed()));
    frames -= n;
  }
}
static double displacement(const PhysicalBody &b, std::size_t node = 1) {
  std::array<Vec3, physical_max_nodes> x{};
  assert(b.write_displacements(x.data(), b.preparation().input().nodes.size(),
                               b.body_revision(), b.samples_elapsed()));
  return x[node][0];
}
static double velocity(const PhysicalBody &b, std::size_t node = 1) {
  const auto state = b.checkpoint();
  double v = 0;
  for (std::size_t m = 0; m < state.velocity_modal_metres_per_second.size();
       ++m)
    v += state.velocity_modal_metres_per_second[m] *
         b.preparation().mode_shape(m)[node][0];
  return v;
}
static void equal_state(const PhysicalBodyCheckpoint &a,
                        const PhysicalBodyCheckpoint &b) {
  assert(a.body_revision == b.body_revision &&
         a.samples_elapsed == b.samples_elapsed &&
         a.eigenbasis_identity == b.eigenbasis_identity);
  assert(a.displacement_modal_metres == b.displacement_modal_metres &&
         a.velocity_modal_metres_per_second ==
             b.velocity_modal_metres_per_second &&
         a.last_pickup_linear == b.last_pickup_linear);
}
template <class F> static void reject(F f) {
  bool rejected = false;
  try {
    f();
  } catch (const std::invalid_argument &) {
    rejected = true;
  }
  assert(rejected);
}
static void material_ringing_work_and_allocation_custody() {
  const auto in = bar();
  const PreparedPhysicalBody retained(in);
  PhysicalBody body{PreparedPhysicalBody(in)};
  auto changed = next(in);
  changed.material.young_modulus_pa *= 4;
  PreparedPhysicalTransition prepared(
      retained, PreparedPhysicalBody(changed), 11, 123,
      PhysicalLiveUpdateKind::Material,
      PhysicalFormTransition::ProjectCorrespondingNodes,
      "controlled:actual-m2-material-act");
  // Preparation precedes the callback's ringing state; no captured q/v can
  // substitute for the actual state at sample123.
  assert(body.apply_impulse_newton_seconds(1e-6, 1, 0));
  advance(body, 123);
  const double t0 = 123.0 / 48000, q0 = 1e-5 * std::sin(1000 * t0),
               v0 = 0.01 * std::cos(1000 * t0);
  near(displacement(body), q0);
  near(velocity(body), v0);
  const auto before = body.checkpoint();
  const PhysicalBody *sole = &body;
  PhysicalLiveTransitionReceipt receipt{};
  PhysicalSnapshot visible{};
  callback_probe = true;
  assert(prepared.preflight(body));
  assert(prepared.apply(body, receipt));
  assert(write_physical_snapshot(body, visible, 2, 123));
  callback_probe = false;
  assert(allocations == 0 && releases == 0 && &body == sole &&
         prepared.committed());
  near(displacement(body), q0);
  near(velocity(body), v0);
  near(body.preparation().frequency_hz(0), 2000 / (2 * pi));
  assert(body.samples_elapsed() == 123 && body.state_ref() == in.state_ref &&
         visible.samples_elapsed == 123);
  near(receipt.external_work_joules, 0.5 * (400 - 100) * q0 * q0, 1e-16);
  near(receipt.before_energy_joules, 0.5 * 1e-4 * v0 * v0 + 0.5 * 100 * q0 * q0,
       1e-16);
  assert(receipt.transaction == 11 && receipt.before_revision == 1 &&
         receipt.after_revision == 2 && receipt.samples_elapsed == 123);
  std::array<double, 257> f{};
  std::array<float, 257> pcm{};
  callback_probe = true;
  assert(body.advance_force_block(f.data(), pcm.data(), f.size(), 2, 123));
  callback_probe = false;
  assert(allocations == 0 && releases == 0);
  for (std::size_t i = 0; i < pcm.size(); ++i) {
    const double t = (i + 1.0) / 48000;
    near(pcm[i],
         1000 * (q0 * std::cos(2000 * t) + v0 / 2000 * std::sin(2000 * t)),
         1e-8, 1e-6);
  }
  const auto resident = body.checkpoint();
  receipt.transaction = 999;
  assert(!prepared.apply(body, receipt));
  assert(receipt.transaction == 999);
  equal_state(body.checkpoint(), resident);
  // The old prepared payload remains immutable; the committed transaction
  // owns retired allocations until the control owner acknowledges/reclaims.
  assert(retained.input().body_revision == 1 &&
         prepared.before_input().preparation_ref == before.preparation_ref);
  assert(prepared.after_basis_identity() ==
         body.preparation().eigenbasis_identity());
}
static void changed_mass_geometry_and_boundary_project_actual_state() {
  const auto in = bar();
  const PreparedPhysicalBody retained(in);
  PhysicalBody body{PreparedPhysicalBody(in)};
  advance(body, 321, 0.01);
  const double q = displacement(body), v = velocity(body),
               energy = body.mechanical_energy_joules();
  auto changed = next(in);
  changed.nodes[1].rest_metres[0] = 2;
  changed.geometry_revision = "2";
  changed.source_generation = 8;
  PreparedPhysicalTransition update(
      retained, PreparedPhysicalBody(changed), 12, 321,
      PhysicalLiveUpdateKind::FormOrBoundary,
      PhysicalFormTransition::ProjectCorrespondingNodes,
      "controlled:actual-m3-form-act");
  PhysicalLiveTransitionReceipt receipt{};
  assert(update.apply(body, receipt));
  near(displacement(body), q);
  near(velocity(body), v);
  near(body.preparation().node_mass_kg(1), 2e-4);
  near(body.preparation().frequency_hz(0), 500 / (2 * pi));
  near(receipt.external_work_joules,
       0.5 * (2e-4 - 1e-4) * v * v + 0.5 * (50 - 100) * q * q, 1e-16);
  PhysicalSnapshot snapshot{};
  assert(write_physical_snapshot(body, snapshot, 2, 321));
  near(snapshot.visible_positions_metres[1][0], 2 + q);
  assert(snapshot.source_generation == 8 &&
         receipt.before_energy_joules == energy);
  // Same labels with changed determinant must yield different future PCM.
  PhysicalBody unmodified{PreparedPhysicalBody(in)};
  advance(unmodified, 321, 0.01);
  advance(body, 113);
  advance(unmodified, 113);
  assert(std::abs(displacement(body) - displacement(unmodified)) > 1e-8);
  auto chain = in;
  chain.nodes.push_back({3, "#3-1", {2, 0, 0}, 0, {false, true, true}});
  chain.edges.push_back({1, 2, 1e-4, 0});
  chain.exciter.node_weights = {0, 0, 1};
  chain.pickup = chain.exciter;
  const PreparedPhysicalBody chain_before(chain);
  PhysicalBody constrained{PreparedPhysicalBody(chain)};
  advance(constrained, 211, 0.01);
  const double tip = displacement(constrained, 2),
               tipv = velocity(constrained, 2);
  auto boundary = next(chain);
  boundary.nodes[1].fixed = {true, true, true};
  boundary.geometry_revision = "boundary2";
  PreparedPhysicalTransition constrain(
      chain_before, PreparedPhysicalBody(boundary), 13, 211,
      PhysicalLiveUpdateKind::FormOrBoundary,
      PhysicalFormTransition::ProjectCorrespondingNodes,
      "controlled:actual-m2-nodal-boundary-act");
  assert(constrain.apply(constrained, receipt));
  assert(displacement(constrained, 1) == 0);
  near(displacement(constrained, 2), tip);
  near(velocity(constrained, 2), tipv);
  assert(constrained.preparation().mode_count() == 1 &&
         constrained.state_ref() == chain.state_ref);
}
static void stale_disconnected_bounds_and_reset_are_explicit() {
  const auto in = bar();
  const PreparedPhysicalBody retained(in);
  auto changed = next(in);
  changed.material.young_modulus_pa *= 4;
  PreparedPhysicalTransition update(
      retained, PreparedPhysicalBody(changed), 14, 20,
      PhysicalLiveUpdateKind::Material,
      PhysicalFormTransition::ProjectCorrespondingNodes,
      "controlled:admitted-act");
  PhysicalBody body{PreparedPhysicalBody(in)};
  advance(body, 19, 0.01);
  auto saved = body.checkpoint();
  PhysicalLiveTransitionReceipt receipt{};
  receipt.transaction = 999;
  assert(!update.apply(body, receipt));
  equal_state(saved, body.checkpoint());
  assert(receipt.transaction == 999);
  advance(body, 1, 0.01);
  auto wrong = in;
  wrong.material.young_modulus_pa *= 2;
  PhysicalBody wrong_basis{PreparedPhysicalBody(wrong)};
  advance(wrong_basis, 20, 0.01);
  saved = wrong_basis.checkpoint();
  assert(!update.apply(wrong_basis, receipt));
  equal_state(saved, wrong_basis.checkpoint());
  auto wrong_face = in;
  wrong_face.pratibimba = false;
  PhysicalBody foreign{PreparedPhysicalBody(wrong_face)};
  advance(foreign, 20, 0.01);
  saved = foreign.checkpoint();
  assert(!update.apply(foreign, receipt));
  equal_state(saved, foreign.checkpoint());
  auto tiny = changed;
  tiny.max_displacement_metres = 1e-12;
  PreparedPhysicalTransition bounded(
      retained, PreparedPhysicalBody(tiny), 15, 20,
      PhysicalLiveUpdateKind::Material,
      PhysicalFormTransition::ProjectCorrespondingNodes,
      "controlled:bound-act");
  saved = body.checkpoint();
  assert(!bounded.apply(body, receipt));
  equal_state(saved, body.checkpoint());
  assert(receipt.transaction == 999);
  assert(update.apply(body, receipt));
  auto invalid = changed;
  invalid.event_ref = "controlled:foreign";
  reject([&] {
    PreparedPhysicalTransition x(
        retained, PreparedPhysicalBody(invalid), 16, 20,
        PhysicalLiveUpdateKind::FormOrBoundary,
        PhysicalFormTransition::ProjectCorrespondingNodes, "controlled:act");
  });
  invalid = changed;
  invalid.pratibimba = false;
  reject([&] {
    PreparedPhysicalTransition x(
        retained, PreparedPhysicalBody(invalid), 16, 20,
        PhysicalLiveUpdateKind::FormOrBoundary,
        PhysicalFormTransition::ProjectCorrespondingNodes, "controlled:act");
  });
  invalid = changed;
  invalid.nodes[1].identity = 3;
  reject([&] {
    PreparedPhysicalTransition x(
        retained, PreparedPhysicalBody(invalid), 16, 20,
        PhysicalLiveUpdateKind::FormOrBoundary,
        PhysicalFormTransition::ProjectCorrespondingNodes, "controlled:act");
  });
  invalid = changed;
  invalid.nodes[1].rest_metres[0] = 2;
  reject([&] {
    PreparedPhysicalTransition x(
        retained, PreparedPhysicalBody(invalid), 16, 20,
        PhysicalLiveUpdateKind::Material,
        PhysicalFormTransition::ProjectCorrespondingNodes, "controlled:act");
  });
  reject([&] {
    PreparedPhysicalTransition x(retained, PreparedPhysicalBody(changed), 16,
                                 20, PhysicalLiveUpdateKind::Material,
                                 PhysicalFormTransition::ExplicitReset,
                                 "controlled:act");
  });
  reject([&] {
    PreparedPhysicalTransition x(
        retained, PreparedPhysicalBody(changed), 0, 20,
        PhysicalLiveUpdateKind::Material,
        PhysicalFormTransition::ProjectCorrespondingNodes, "controlled:act");
  });
  const PreparedPhysicalBody second(changed);
  auto reset = next(changed);
  reset.nodes[1].identity = 3;
  reset.body_revision = 3;
  reset.preparation_ref = "controlled:explicit-reset";
  PreparedPhysicalTransition explicit_reset(
      second, PreparedPhysicalBody(reset), 17, 20,
      PhysicalLiveUpdateKind::FormOrBoundary,
      PhysicalFormTransition::ExplicitReset, "controlled:explicit-reset-act");
  const double energy = body.mechanical_energy_joules();
  assert(explicit_reset.apply(body, receipt));
  assert(body.mechanical_energy_joules() == 0 && displacement(body) == 0 &&
         body.samples_elapsed() == 20);
  near(receipt.external_work_joules, -energy);
  assert(receipt.policy == PhysicalFormTransition::ExplicitReset);
}
static void seek_reopen_preserves_pending_transition_and_future_pcm() {
  const auto in = bar();
  const PreparedPhysicalBody retained(in);
  auto changed = next(in);
  changed.material.young_modulus_pa *= 4;
  PhysicalBody original{PreparedPhysicalBody(in)};
  advance(original, 37, 0.01);
  const auto seek = original.checkpoint();
  PreparedPhysicalTransition pending(
      retained, PreparedPhysicalBody(changed), 18, 73,
      PhysicalLiveUpdateKind::Material,
      PhysicalFormTransition::ProjectCorrespondingNodes,
      "controlled:persisted-material-act");
  PhysicalBody reopen{PreparedPhysicalBody(in)};
  assert(reopen.restore_checkpoint(seek, 1, 0));
  // C stores the admitted before/after source-qualified payload plus these
  // primitive transaction fields; reprepare off callback, not seed-restart.
  PreparedPhysicalTransition restored(
      PreparedPhysicalBody(pending.before_input()),
      PreparedPhysicalBody(pending.after_input()), pending.transaction(),
      pending.sample(), pending.kind(), pending.policy(), pending.cause_ref());
  assert(restored.before_basis_identity() == pending.before_basis_identity() &&
         restored.after_basis_identity() == pending.after_basis_identity());
  advance(original, 36, 0.01);
  advance(reopen, 36, 0.01);
  PhysicalLiveTransitionReceipt r1{}, r2{};
  assert(pending.apply(original, r1));
  assert(restored.apply(reopen, r2));
  equal_state(original.checkpoint(), reopen.checkpoint());
  std::array<double, 256> force{};
  for (std::size_t i = 0; i < force.size(); ++i)
    force[i] = 0.01 * std::sin(2 * pi * 73 * i / 48000);
  std::array<float, 256> pcm1{}, pcm2{};
  assert(original.advance_force_block(force.data(), pcm1.data(), 256, 2, 73));
  assert(reopen.advance_force_block(force.data(), pcm2.data(), 256, 2, 73));
  assert(pcm1 == pcm2);
  equal_state(original.checkpoint(), reopen.checkpoint());
  const auto final = original.checkpoint();
  PhysicalBody another{PreparedPhysicalBody(changed)};
  assert(another.restore_checkpoint(final, 2, 0));
  assert(another.samples_elapsed() == 329 &&
         another.state_ref() == in.state_ref);
}
int main() {
  callback_probe = true;
  void *probe = ::operator new(32);
  ::operator delete(probe);
  callback_probe = false;
  assert(allocations == 1 && releases == 1);
  allocations = releases = 0;
  material_ringing_work_and_allocation_custody();
  changed_mass_geometry_and_boundary_project_actual_state();
  stale_disconnected_bounds_and_reset_are_explicit();
  seek_reopen_preserves_pending_transition_and_future_pcm();
  std::cout << "native live body projection, custody, atomic admission and "
               "reopen tests passed\n";
}
