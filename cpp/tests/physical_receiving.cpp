#include <cassert>
#include <iostream>
#include <ql/physical_receiving.hpp>
using namespace ql;
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
static void near(double a, double b, double tolerance = 1e-8) {
  assert(std::isfinite(a) && std::abs(a - b) <= tolerance);
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
static void step(PhysicalBody &body, SpatialReceiving &receiving,
                 std::size_t frames, std::array<float, 512> &output) {
  const auto start = body.samples_elapsed();
  std::array<double, 512> force{};
  std::array<float, 512> pickup{};
  assert(
      body.advance_force_block(force.data(), pickup.data(), frames, 1, start));
  assert(receiving.process_pickup_block(body, pickup.data(), output.data(),
                                        frames, start));
}
static std::vector<float> render(std::size_t partition) {
  const PreparedPhysicalBody immutable(bar());
  PhysicalBody body{PreparedPhysicalBody(bar())};
  SpatialReceiving receiving(immutable, 0,
                             PreparedSpatialReceiving(immutable, receiver(1)));
  assert(body.apply_impulse_newton_seconds(1e-6, 1, 0));
  std::vector<float> out;
  while (body.samples_elapsed() < 1024) {
    const auto frames =
        std::min<std::uint64_t>(partition, 1024 - body.samples_elapsed());
    std::array<float, 512> block{};
    step(body, receiving, frames, block);
    out.insert(out.end(), block.begin(), block.begin() + frames);
  }
  return out;
}
int main() {
  const PreparedPhysicalBody immutable(bar());
  PhysicalBody body{PreparedPhysicalBody(bar())};
  PreparedSpatialReceiving prepared1(immutable, receiver(1)),
      prepared2(immutable, receiver(2));
  near(prepared1.distance_metres(), 1, 1e-15);
  near(prepared2.distance_metres(), 2, 1e-15);
  near(prepared1.delay_samples(), 141.1764705882353, 1e-12);
  near(prepared2.delay_samples() - prepared1.delay_samples(), 141.1764705882353,
       1e-12);
  near(prepared2.gain() / prepared1.gain(), .5, 1e-15);
  const Vec3 expected_source{1, 0, 0};
  assert(prepared1.source_position_metres() == expected_source);
  SpatialReceiving first(immutable, 0, prepared1),
      second(immutable, 0, prepared2);
  SpatialReceiving immediate1(
      immutable, 0, PreparedSpatialReceiving(immutable, receiver(1, false))),
      immediate2(immutable, 0,
                 PreparedSpatialReceiving(immutable, receiver(2, false)));
  auto backwards = receiver(1, false);
  backwards.directivity = ReceivingDirectivity::Cardioid;
  backwards.receiver_forward = {1, 0, 0};
  SpatialReceiving away(immutable, 0,
                        PreparedSpatialReceiving(immutable, backwards));
  assert(body.apply_impulse_newton_seconds(1e-6, 1, 0));
  std::array<double, 512> force{};
  std::array<float, 512> raw{}, out1{}, out2{}, now1{}, now2{}, muted{};
  assert(body.advance_force_block(force.data(), raw.data(), 512, 1, 0));
  const auto actual = body.checkpoint();
  assert(first.process_pickup_block(body, raw.data(), out1.data(), 512, 0));
  assert(second.process_pickup_block(body, raw.data(), out2.data(), 512, 0));
  assert(
      immediate1.process_pickup_block(body, raw.data(), now1.data(), 512, 0));
  assert(
      immediate2.process_pickup_block(body, raw.data(), now2.data(), 512, 0));
  assert(away.process_pickup_block(body, raw.data(), muted.data(), 512, 0));
  for (std::size_t n = 0; n < 512; ++n) {
    assert(now1[n] == raw[n] && now2[n] == raw[n] * .5f && muted[n] == 0);
    for (const auto &pair :
         {std::pair{&prepared1, &out1}, std::pair{&prepared2, &out2}}) {
      const auto delay = pair.first->delay_samples();
      const auto k = std::size_t(std::floor(delay));
      const double f = delay - k;
      const double a = n >= k ? raw[n - k] : 0, b = n > k ? raw[n - k - 1] : 0;
      near((*pair.second)[n], pair.first->gain() * ((1 - f) * a + f * b));
    }
  }
  assert(out1[140] == 0 && out1[141] != 0 && out2[281] == 0 && out2[282] != 0);
  // All observations used one unchanged body; receiving cannot advance q/v.
  const auto after = body.checkpoint();
  assert(after.displacement_modal_metres == actual.displacement_modal_metres &&
         after.velocity_modal_metres_per_second ==
             actual.velocity_modal_metres_per_second &&
         after.samples_elapsed == 512);
  assert(render(1) == render(128) && render(128) == render(256));
  auto move = receiver(2);
  move.revision = 2;
  move.policy_revision = "2";
  assert(first.replace_receiving(
      body, PreparedSpatialReceiving(immutable, move), 1, 512));
  std::array<float, 512> progressed{};
  step(body, first, 64, progressed);
  const auto body_saved = body.checkpoint();
  const auto transport_saved = first.checkpoint(body);
  assert(transport_saved.transition_remaining == 64);
  std::array<float, 512> future{}, replayed{};
  step(body, first, 128, future);
  assert(body.restore_checkpoint(body_saved, 1, 704));
  assert(first.restore_checkpoint(body, transport_saved, 704));
  step(body, first, 128, replayed);
  assert(future == replayed); // Delay history and in-flight receiver transition
                              // survive seek.
  auto bad = first.checkpoint(body);
  bad.receiving_identity = "controlled:wrong-receiver";
  assert(!first.restore_checkpoint(body, bad, 704));
  bad = first.checkpoint(body);
  bad.history_linear[0] = std::numeric_limits<float>::quiet_NaN();
  assert(!first.restore_checkpoint(body, bad, 704));
  bad = first.checkpoint(body);
  bad.delay_unit = "s";
  assert(!first.restore_checkpoint(body, bad, 704));
  bad = first.checkpoint(body);
  bad.effective_gain = 2;
  assert(!first.restore_checkpoint(body, bad, 704));
  const auto retained = first.checkpoint(body);
  assert(retained.history_linear == first.checkpoint(body).history_linear);
  std::array<float, 512> sentinel{};
  sentinel.fill(73);
  assert(
      !first.process_pickup_block(body, raw.data(), sentinel.data(), 128, 576));
  assert(std::all_of(sentinel.begin(), sentinel.end(),
                     [](float x) { return x == 73; }));
  std::array<float, 512> aliased{}, recovered{};
  assert(body.advance_force_block(force.data(), aliased.data(), 128, 1, 704));
  assert(!first.process_pickup_block(body, aliased.data(), aliased.data(), 128,
                                     704));
  assert(first.samples_elapsed() == 704);
  assert(first.process_pickup_block(body, aliased.data(), recovered.data(), 128,
                                    704));
  assert(first.samples_elapsed() == 832);
  auto other = bar();
  other.state_ref = "controlled:foreign-body";
  PhysicalBody foreign{PreparedPhysicalBody(other)};
  rejected([&] {
    SpatialReceiving wrong(PreparedPhysicalBody(other), 0, prepared1);
  });
  assert(!prepared1.matches_body(foreign)); // actual stopped exclusive custody
  assert(prepared1.matches_preparation(immutable));
  assert(!prepared1.matches_preparation(PreparedPhysicalBody(other)));
  auto invalid = receiver(1000);
  rejected([&] { PreparedSpatialReceiving wrong(immutable, invalid); });
  invalid = receiver(1);
  invalid.receiver_forward = {0, 0, 0};
  rejected([&] { PreparedSpatialReceiving wrong(immutable, invalid); });
  invalid = receiver(1);
  invalid.source_ref = std::string("source\0wrong", 12);
  rejected([&] { PreparedSpatialReceiving wrong(immutable, invalid); });
  std::cout
      << "physical pickup receiving, propagation and checkpoint tests passed\n";
}
