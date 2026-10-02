#include <cassert>
#include <iostream>
#include <ql/physical_contact.hpp>
using namespace ql;
static PhysicalBodyInput reference_bar() {
  PhysicalBodyInput in;
  in.event_ref = "controlled:contact/event";
  in.subject_ref = "controlled:contact/subject";
  in.source_coordinate = "#3-2-1-1-1";
  in.source_revision =
      "907c46bc8a65b47e12f14aa4d8b444263dc956a1a7b4b6d038e57223d6073288";
  in.geometry_ref = "controlled:contact/bar";
  in.geometry_revision = "1";
  in.geometry_source_ref = "controlled:analytic-bar";
  in.geometry_standing = "reference";
  in.preparation_ref = "controlled:contact/preparation";
  in.state_ref = "controlled:contact/state";
  in.source_generation = 7;
  in.body_revision = 1;
  in.sample_rate = 48000;
  in.pratibimba = true;
  in.family = BodyFamily::AxialTruss;
  in.material = {"controlled:contact/material",
                 "1",
                 "controlled:analytic-elasticity",
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
  in.max_impulse_newton_seconds = .01;
  in.max_displacement_metres = .1;
  return in;
}
static GravityContactInput gravity() {
  GravityContactInput in;
  in.contact_ref = "controlled:gravity/contact1";
  in.particle_ref = "controlled:gravity/particle1";
  in.collider_ref = "controlled:gravity/plane1";
  in.route_ref = "controlled:gravity/body-exciter";
  in.source_ref =
      "QL-MEF@abf209a6:docs/"
      "M-PRIME-PHYSICAL-MUSIC-RESEARCH-ACCEPTANCE.md:collision-articulation";
  in.policy_ref = "controlled:gravity/finite-pulse";
  in.policy_revision = "1";
  in.standing = "reference";
  in.plane_position_metres = {1, 0, 0};
  in.normal = {-1, 0, 0};
  in.height_metres = .5;
  in.mass_kg = 1e-6;
  in.duration_samples = 32;
  return in;
}
static void near(double actual, double expected, double tolerance = 1e-10) {
  assert(std::isfinite(actual) && std::abs(actual - expected) <= tolerance);
}
template <class F> static void rejected(F f) {
  bool failure = false;
  try {
    f();
  } catch (const std::invalid_argument &) {
    failure = true;
  }
  assert(failure);
}
static std::vector<float> causal_render(PhysicalBody &body,
                                        const PreparedContactForce &contact,
                                        std::size_t partition,
                                        std::uint64_t end) {
  std::vector<float> result;
  result.reserve(end);
  while (body.samples_elapsed() < end) {
    const auto start = body.samples_elapsed();
    const auto frames = std::min<std::uint64_t>(partition, end - start);
    std::array<double, physical_max_frames> forces{};
    std::array<float, physical_max_frames> pcm{};
    for (std::size_t i = 0; i < frames; ++i) {
      const auto sample = start + i;
      if (sample >= contact.start_sample &&
          sample - contact.start_sample < contact.frames)
        forces[i] = contact.force_newtons[sample - contact.start_sample];
    }
    assert(
        body.advance_force_block(forces.data(), pcm.data(), frames, 1, start));
    result.insert(result.end(), pcm.begin(), pcm.begin() + frames);
  }
  return result;
}
int main() {
  PhysicalBody body{PreparedPhysicalBody(reference_bar())};
  const auto source = gravity();
  const auto contact =
      prepare_gravity_contact(PreparedPhysicalBody(reference_bar()), 0, source);
  // Independently known free-fall solution, not a renderer event or clock.
  near(contact.impact_seconds, 0.3192754284070505, 1e-14);
  near(contact.impact_speed_metres_per_second, 3.132091952673165, 1e-13);
  assert(contact.start_sample == 15326 && contact.frames == 32);
  near(contact.impulse_newton_seconds, 3.132091952673165e-6, 1e-18);
  near(contact.force_newtons[0], contact.impulse_newton_seconds * 1500, 1e-17);
  assert(contact.position_metres == source.plane_position_metres &&
         contact.dedup_ref == source.contact_ref &&
         contact_matches_body(contact, body));
  assert(contact.seed_standing == "deterministic-no-randomness" &&
         contact.seed == 0);
  const auto end = contact.start_sample + contact.frames + 67;
  const auto pcm = causal_render(body, contact, 128, end);
  PhysicalBody replay{PreparedPhysicalBody(reference_bar())};
  const auto alternate = causal_render(replay, contact, 256, end);
  assert(pcm == alternate);
  assert(std::all_of(pcm.begin(), pcm.begin() + contact.start_sample,
                     [](float x) { return x == 0; }));
  // Finite rectangular force response of an undamped M=.0001kg/K=100N/m bar.
  const double since_on = double(end - contact.start_sample) / 48000,
               since_off =
                   double(end - contact.start_sample - contact.frames) / 48000;
  const double expected =
      contact.force_newtons[0] / 100 *
      (std::cos(1000 * since_off) - std::cos(1000 * since_on));
  std::array<Vec3, 2> displacement{};
  assert(body.write_displacements(displacement.data(), 2, 1, end));
  near(displacement[1][0], expected, 1e-14);
  near(pcm.back(), expected * 1000, 1e-8);
  assert(body.mechanical_energy_joules() > 0 &&
         !contact_matches_body(contact, body));
  auto altered = source;
  altered.initial_normal_velocity_metres_per_second = -1000;
  // A control-side stale cursor is refused without touching live q/v/time.
  rejected([&] {
    prepare_gravity_contact(PreparedPhysicalBody(reference_bar()), 1, source);
  });
  assert(contact_matches_preparation(contact,
                                     PreparedPhysicalBody(reference_bar()), 0));
  assert(!contact_matches_preparation(contact,
                                      PreparedPhysicalBody(reference_bar()),
                                      contact.start_sample + 1));
  const auto fast = prepare_gravity_contact(
      PreparedPhysicalBody(reference_bar()), 0, altered);
  near(fast.impact_seconds, 0.0004999987737560147,
       2e-14); // Independent stable quadratic evaluation.
  altered = source;
  altered.normal = {0, 1, 0};
  const auto orthogonal = prepare_gravity_contact(
      PreparedPhysicalBody(reference_bar()), 0, altered);
  assert(orthogonal.impulse_newton_seconds ==
         0); // Explicit directional projection, not a hidden oscillator.
  PhysicalBody untouched{PreparedPhysicalBody(reference_bar())};
  auto bad = contact;
  bad.source_coordinate = "#3-2-1-1-2";
  assert(!contact_matches_body(bad, untouched));
  bad = contact;
  bad.pratibimba = false;
  assert(!contact_matches_body(bad, untouched));
  bad = contact;
  bad.body_revision = 2;
  assert(!contact_matches_body(bad, untouched));
  bad = contact;
  bad.body_state_ref = "controlled:other-state";
  assert(!contact_matches_body(bad, untouched));
  bad = contact;
  bad.route_ref = "";
  assert(!contact_matches_body(bad, untouched));
  bad = contact;
  bad.route_ref = std::string("route\0disconnected", 18);
  assert(!contact_matches_body(bad, untouched));
  bad = contact;
  bad.force_newtons[0] = std::numeric_limits<double>::quiet_NaN();
  assert(!contact_matches_body(bad, untouched));
  bad = contact;
  bad.force_newtons[0] *= 2;
  assert(!contact_matches_body(bad, untouched));
  bad = contact;
  bad.normal = {0, 0, 0};
  assert(!contact_matches_body(bad, untouched));
  bad = contact;
  bad.eigenbasis_identity = "controlled:foreign-basis";
  assert(!contact_matches_body(bad, untouched));
  assert(untouched.samples_elapsed() == 0 &&
         untouched.mechanical_energy_joules() == 0);
  altered = source;
  altered.duration_samples = 0;
  rejected([&] {
    prepare_gravity_contact(PreparedPhysicalBody(reference_bar()), 0, altered);
  });
  altered = source;
  altered.height_metres = -.1;
  rejected([&] {
    prepare_gravity_contact(PreparedPhysicalBody(reference_bar()), 0, altered);
  });
  altered = source;
  altered.mass_kg = 1;
  rejected([&] {
    prepare_gravity_contact(PreparedPhysicalBody(reference_bar()), 0, altered);
  });
  altered = source;
  altered.start_sample = 1;
  rejected([&] {
    prepare_gravity_contact(PreparedPhysicalBody(reference_bar()), 0, altered);
  });
  altered = source;
  altered.minimum_impact_speed_metres_per_second = 4;
  assert(
      prepare_gravity_contact(PreparedPhysicalBody(reference_bar()), 0, altered)
          .impulse_newton_seconds == 0);
  std::cout << "source-body-bound physical gravity contact tests passed\n";
}
