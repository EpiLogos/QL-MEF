#include "../test_support/allocation_hooks.hpp"
#include <cassert>
#include <cstdlib>
#include <iostream>
#include <new>
#include <ql/physical_receiving_transition.hpp>
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
static PhysicalBodyInput bar() {
  PhysicalBodyInput in;
  in.event_ref = "controlled:receiving/event";
  in.subject_ref = "controlled:receiving/subject";
  in.source_coordinate = "#3-2-1-1-1";
  in.source_revision =
      "907c46bc8a65b47e12f14aa4d8b444263dc956a1a7b4b6d038e57223d6073288";
  in.geometry_ref = "controlled:receiving/bar";
  in.geometry_revision = "1";
  in.geometry_source_ref = "controlled:analytic-bar";
  in.geometry_standing = "reference";
  in.preparation_ref = "controlled:receiving/preparation";
  in.state_ref = "controlled:receiving/state";
  in.source_generation = 7;
  in.body_revision = 1;
  in.sample_rate = 48000;
  in.pratibimba = true;
  in.family = BodyFamily::AxialTruss;
  in.material = {"controlled:receiving/material",
                 "1",
                 "controlled:analytic-elasticity",
                 "reference",
                 1e6,
                 2,
                 .4,
                 0};
  in.nodes = {{1, "#3-2-1-1-1", {0, 0, 0}, 0, {true, true, true}},
              {2, "#3-0", {1, 0, 0}, 0, {false, true, true}}};
  in.edges = {{0, 1, 1e-4, 0}};
  in.exciter = {{1, 0, 0}, {0, 1}};
  in.pickup = in.exciter;
  in.pickup_linear_per_metre = 1000;
  in.max_force_newtons = 10;
  in.max_impulse_newton_seconds = .01;
  in.max_displacement_metres = .1;
  return in;
}
static SpatialReceivingInput receiver(double distance, bool delay = true) {
  SpatialReceivingInput in;
  in.receiver_ref = "controlled:receiving/receiver";
  in.context_ref = "controlled:neutral/world";
  in.source_ref = "QL-MEF@abf209a6:docs/"
                  "M-PRIME-PHYSICAL-MUSIC-RESEARCH-ACCEPTANCE.md:moving-source";
  in.policy_ref = "controlled:point-source/inverse-distance-linear-delay";
  in.policy_revision = "1";
  in.standing = "reference";
  in.receiver_position_metres = {1 + distance, 0, 0};
  in.receiver_forward = {-1, 0, 0};
  in.propagation_delay = delay;
  return in;
}

static void near(double a, double b, double tol = 1e-8) {
  assert(std::isfinite(a) && std::abs(a - b) <= tol);
}
static void step(PhysicalBody &body, SpatialReceiving &receiving,
                 std::size_t frames, std::array<float, 512> &out,
                 std::array<float, 512> &raw) {
  const auto start = body.samples_elapsed();
  std::array<double, 512> force{};
  assert(body.advance_force_block(force.data(), raw.data(), frames,
                                  body.body_revision(), start));
  assert(receiving.process_pickup_block(body, raw.data(), out.data(), frames,
                                        start));
}
template <class F> static void rejected(F f) {
  bool failed = false;
  try {
    f();
  } catch (const std::invalid_argument &) {
    failed = true;
  }
  assert(failed);
}
static void movement_preserves_history_and_has_no_callback_allocation() {
  const PreparedPhysicalBody immutable(bar());
  PhysicalBody body{PreparedPhysicalBody(bar())};
  const PreparedSpatialReceiving before(immutable, receiver(1, false));
  SpatialReceiving receiving(immutable, 0, before);
  auto move = receiver(2, false);
  move.revision = 2;
  move.transition_samples = 64;
  move.policy_revision = "2";
  PreparedReceivingTransition update(
      before, PreparedSpatialReceiving(immutable, move), 101, 128,
      "controlled:actual-context-receiver-move");
  assert(body.apply_impulse_newton_seconds(1e-6, 1, 0));
  std::array<float, 512> raw{}, out{};
  step(body, receiving, 128, out, raw);
  const auto saved = receiving.checkpoint(body);
  const auto physical = body.checkpoint();
  ReceivingLiveTransitionReceipt receipt{};
  callback_probe = true;
  assert(update.apply(body, receiving, receipt));
  step(body, receiving, 32, out, raw);
  callback_probe = false;
  assert(allocations == 0 && releases == 0 &&
         receiving.samples_elapsed() == 160 && body.samples_elapsed() == 160);
  for (std::size_t i = 0; i < 32; ++i)
    near(out[i], raw[i] * (1 - 0.5 * (i + 1.0) / 64));
  assert(receipt.transaction == 101 && receipt.samples_elapsed == 128 &&
         receipt.before_effective_gain == 1 && receipt.target_gain == 0.5);
  const auto midway = receiving.checkpoint(body);
  assert(midway.transition_remaining == 32);
  assert(midway.history_linear[0] == saved.history_linear[0] &&
         midway.history_linear[127] == saved.history_linear[127]);
  assert(body.state_ref() == physical.state_ref);
  receipt.transaction = 999;
  assert(!update.apply(body, receiving, receipt));
  assert(receipt.transaction == 999);
  // Pending immutable metadata suffices to reprepare after the actual body
  // and delay-history checkpoints reopen, without replaying from a seed.
  PhysicalBody reopened{PreparedPhysicalBody(bar())};
  assert(reopened.restore_checkpoint(physical, 1, 0));
  SpatialReceiving recovered(immutable, 128, before);
  assert(recovered.restore_checkpoint(reopened, saved, 128));
  PreparedReceivingTransition pending(
      update.before_preparation(), update.after_preparation(),
      update.transaction(), update.sample(), update.cause_ref());
  assert(pending.apply(reopened, recovered, receipt));
  std::array<float, 512> replay{}, rraw{};
  step(reopened, recovered, 32, replay, rraw);
  for (std::size_t i = 0; i < 32; ++i)
    assert(replay[i] == out[i]);
  assert(recovered.checkpoint(reopened).history_linear ==
         midway.history_linear);
}
static void paired_body_requalification_keeps_ring_and_transport() {
  const auto input = bar();
  const PreparedPhysicalBody old_body(input);
  PhysicalBody body{PreparedPhysicalBody(input)};
  const PreparedSpatialReceiving old_receiver(old_body, receiver(1, false));
  SpatialReceiving receiving(old_body, 0, old_receiver);
  auto changed = input;
  changed.body_revision = 2;
  changed.preparation_ref = "controlled:receiving/preparation2";
  changed.material.revision = "2";
  changed.material.young_modulus_pa *= 4;
  const PreparedPhysicalBody new_body(changed);
  PreparedPhysicalTransition material(
      old_body, PreparedPhysicalBody(changed), 201, 128,
      PhysicalLiveUpdateKind::Material,
      PhysicalFormTransition::ProjectCorrespondingNodes,
      "controlled:actual-m2-material");
  PreparedReceivingTransition qualification(
      old_receiver, PreparedSpatialReceiving(new_body, receiver(1, false)), 202,
      128, "controlled:body-receiving-requalification");
  assert(body.apply_impulse_newton_seconds(1e-6, 1, 0));
  std::array<float, 512> raw{}, out{};
  step(body, receiving, 128, out, raw);
  const auto saved = body.checkpoint();
  ReceivingLiveTransitionReceipt rr{};
  rr.transaction = 999;
  assert(!qualification.apply(body, receiving, rr));
  assert(rr.transaction == 999 &&
         receiving.preparation().identity() == old_receiver.identity());
  // A's one callback transaction checks BOTH before either mutation. A stale
  // receiver must not strand an already changed body in a disconnected state.
  auto wrong_move = receiver(1, false);
  wrong_move.revision = 2;
  SpatialReceiving stale(old_body, 128,
                         PreparedSpatialReceiving(old_body, wrong_move));
  const auto before_pair = body.checkpoint();
  assert(material.preflight(body));
  assert(!qualification.preflight_after_body(
      material.pending_after_preparation(), stale, body.samples_elapsed()));
  assert(body.checkpoint().eigenbasis_identity ==
             before_pair.eigenbasis_identity &&
         body.body_revision() == 1 && !material.committed());
  PhysicalLiveTransitionReceipt pr{};
  callback_probe = true;
  assert(material.preflight(body));
  assert(qualification.preflight_after_body(
      material.pending_after_preparation(), receiving, body.samples_elapsed()));
  assert(material.apply(body, pr));
  assert(qualification.apply(body, receiving, rr));
  step(body, receiving, 128, out, raw);
  callback_probe = false;
  assert(allocations == 0 && releases == 0);
  for (std::size_t i = 0; i < 128; ++i)
    assert(out[i] == raw[i]);
  assert(body.samples_elapsed() == 256 && receiving.samples_elapsed() == 256);
  assert(body.state_ref() == saved.state_ref &&
         body.mechanical_energy_joules() > 0 && body.body_revision() == 2);
  assert(rr.before_body_revision == 1 && rr.after_body_revision == 2 &&
         rr.before_receiving_revision == rr.after_receiving_revision);
}
static void stale_foreign_receiver_body_and_cursor_are_atomic() {
  const PreparedPhysicalBody immutable(bar());
  const PreparedSpatialReceiving before(immutable, receiver(1));
  auto move = receiver(2);
  move.revision = 2;
  const PreparedSpatialReceiving after(immutable, move);
  PreparedReceivingTransition update(before, after, 301, 128,
                                     "controlled:metric-move");
  PhysicalBody body{PreparedPhysicalBody(bar())};
  SpatialReceiving receiving(immutable, 0, before);
  std::array<float, 512> raw{}, out{};
  step(body, receiving, 127, out, raw);
  auto saved = receiving.checkpoint(body);
  ReceivingLiveTransitionReceipt receipt{};
  receipt.transaction = 999;
  assert(!update.apply(body, receiving, receipt));
  assert(receipt.transaction == 999 &&
         receiving.checkpoint(body).history_linear == saved.history_linear);
  step(body, receiving, 1, out, raw);
  auto wrong = bar();
  wrong.material.young_modulus_pa *= 2;
  PhysicalBody wrong_body{PreparedPhysicalBody(wrong)};
  std::array<double, 128> f{};
  std::array<float, 128> p{};
  assert(wrong_body.advance_force_block(f.data(), p.data(), 128, 1, 0));
  saved = receiving.checkpoint(body);
  assert(!update.apply(wrong_body, receiving, receipt));
  assert(receiving.checkpoint(body).history_linear == saved.history_linear);
  auto third = move;
  third.revision = 3;
  SpatialReceiving wrong_receiver(immutable, 128,
                                  PreparedSpatialReceiving(immutable, third));
  assert(!update.apply(body, wrong_receiver, receipt));
  assert(!update.committed());
  assert(update.apply(body, receiving, receipt));
  auto foreign = move;
  foreign.context_ref = "controlled:foreign-context";
  rejected([&] {
    PreparedReceivingTransition invalid(
        before, PreparedSpatialReceiving(immutable, foreign), 302, 128,
        "controlled:act");
  });
  rejected([&] {
    PreparedReceivingTransition invalid(before, before, 302, 128,
                                        "controlled:act");
  });
  auto tampered = bar();
  tampered.material.young_modulus_pa *= 2;
  rejected([&] {
    PreparedReceivingTransition invalid(
        before, PreparedSpatialReceiving(PreparedPhysicalBody(tampered), move),
        302, 128, "controlled:act");
  });
}
int main() {
  callback_probe = true;
  void *probe = ::operator new(32);
  ::operator delete(probe);
  callback_probe = false;
  assert(allocations == 1 && releases == 1);
  allocations = releases = 0;
  movement_preserves_history_and_has_no_callback_allocation();
  paired_body_requalification_keeps_ring_and_transport();
  stale_foreign_receiver_body_and_cursor_are_atomic();
  std::cout << "native live receiving movement, body requalification and "
               "custody tests passed\n";
}
