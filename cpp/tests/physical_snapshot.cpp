#include <cassert>
#include <iostream>
#include <ql/physical_snapshot.hpp>
using namespace ql;
static PhysicalBodyInput bar() {
  PhysicalBodyInput in;
  in.event_ref = "controlled:snapshot/event";
  in.subject_ref = "controlled:snapshot/subject";
  in.source_coordinate = "#3-2-1-1-1";
  in.source_revision =
      "907c46bc8a65b47e12f14aa4d8b444263dc956a1a7b4b6d038e57223d6073288";
  in.geometry_ref = "controlled:snapshot/bar";
  in.geometry_revision = "1";
  in.geometry_source_ref = "controlled:analytic-bar";
  in.geometry_standing = "reference";
  in.preparation_ref = "controlled:snapshot/preparation";
  in.state_ref = "controlled:snapshot/state";
  in.source_generation = 7;
  in.body_revision = 1;
  in.sample_rate = 48000;
  in.pratibimba = true;
  in.family = BodyFamily::AxialTruss;
  in.material = {"controlled:snapshot/material",
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
int main() {
  const auto source = bar();
  PhysicalBody body{PreparedPhysicalBody(source)};
  std::array<double, 128> force{};
  force.fill(.001);
  std::array<float, 128> pcm{};
  assert(body.advance_force_block(force.data(), pcm.data(), 128, 1, 0));
  PhysicalSnapshot snapshot;
  assert(write_physical_snapshot(body, snapshot, 1, 128));
  assert(snapshot.version == 1 && snapshot.node_count == 2 &&
         snapshot.body_revision == 1 && snapshot.samples_elapsed == 128);
  assert(snapshot.source_generation == 7 && snapshot.pratibimba &&
         snapshot.sample_rate == 48000);
  assert(std::string(snapshot.source_coordinate.data()) ==
             source.source_coordinate &&
         std::string(snapshot.source_revision.data()) ==
             source.source_revision);
  assert(std::string(snapshot.preparation_ref.data()) ==
             source.preparation_ref &&
         std::string(snapshot.state_ref.data()) == source.state_ref);
  assert(std::string(snapshot.geometry_ref.data()) == source.geometry_ref &&
         std::string(snapshot.material_ref.data()) ==
             source.material.reference);
  assert(std::string(snapshot.eigenbasis_identity.data()) ==
         body.preparation().eigenbasis_identity());
  assert(snapshot.node_identity[0] == 1 && snapshot.node_identity[1] == 2);
  const double t = 128.0 / 48000, gamma = .2, omega = 1000,
               wd = std::sqrt(omega * omega - gamma * gamma);
  const double expected =
      .001 / 100 *
      (1 - std::exp(-gamma * t) *
               (std::cos(wd * t) + gamma / wd * std::sin(wd * t)));
  assert(std::abs(snapshot.visible_positions_metres[1][0] - (1 + expected)) <
         1e-13);
  assert(snapshot.visible_positions_metres[0] == source.nodes[0].rest_metres &&
         snapshot.pickup_linear == pcm.back());
  assert(std::abs(snapshot.pickup_linear - expected * 1000) < 1e-8 &&
         snapshot.mechanical_energy_joules > 0);
  const auto copied = snapshot;
  assert(body.advance_force_block(force.data(), pcm.data(), 128, 1, 128));
  // An immutable old snapshot remains an honest observation at128; a direct
  // current-body read claiming128 must refuse, without overwriting it.
  assert(!write_physical_snapshot(body, snapshot, 1, 128));
  assert(snapshot.visible_positions_metres == copied.visible_positions_metres &&
         snapshot.samples_elapsed == copied.samples_elapsed &&
         snapshot.preparation_ref == copied.preparation_ref);
  assert(!write_physical_snapshot(body, snapshot, 2, 256));
  assert(snapshot.samples_elapsed == 128);
  assert(write_physical_snapshot(body, snapshot, 1, 256));
  assert(snapshot.samples_elapsed == 256 &&
         snapshot.visible_positions_metres != copied.visible_positions_metres);
  std::cout << "callback-owned physical snapshot causal tests passed\n";
}
