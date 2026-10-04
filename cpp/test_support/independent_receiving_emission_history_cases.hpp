// Independent bounded native P/form/retarded receiving experiment. Numerical
// geometry has declared architecture-model standing; it issues no Scene/Act
// authority. The actual public Form producer is tested in the joined host.
static Vec3 independent_pickup_anchor(const PreparedPhysicalBody &body) {
  Vec3 p{};
  const auto &in = body.input();
  for (std::size_t node = 0; node < in.nodes.size(); ++node)
    for (unsigned axis = 0; axis < 3; ++axis)
      p[axis] +=
          in.pickup.node_weights[node] * in.nodes[node].rest_metres[axis];
  return p;
}
static double independent_stationary_received(
    const std::vector<float> &pickup, std::uint64_t sample, std::uint64_t begin,
    std::uint64_t end, const Vec3 &anchor, const Vec3 &receiver,
    const SpatialReceivingInput &spatial) {
  double square = 0;
  for (unsigned axis = 0; axis < 3; ++axis) {
    const double d = anchor[axis] - receiver[axis];
    square += d * d;
  }
  const double distance = std::sqrt(square),
               delay = distance / spatial.speed_metres_per_second * 48000;
  const auto whole = std::uint64_t(std::floor(delay));
  const double fraction = delay - whole;
  const auto read = [&](std::uint64_t lag) {
    if (lag > sample)
      return 0.0;
    const auto at = sample - lag;
    return at < begin || at >= end || at >= pickup.size() ? 0.0
                                                          : double(pickup[at]);
  };
  return spatial.minimum_distance_metres /
         std::max(distance, spatial.minimum_distance_metres) *
         ((1 - fraction) * read(whole) + fraction * read(whole + 1));
}
static void independent_receiving_emission_form_case(J *fixture, double shift) {
  auto actual = prepare(fixture);
  auto body = actual.native.body;
  const auto original = body->preparation();
  assert(original.input().sample_rate == 48000);
  const auto old_anchor = independent_pickup_anchor(original);
  SpatialReceivingInput spatial;
  spatial.receiver_ref = "native-test:emission/receiver";
  spatial.context_ref = "native-test:emission/context";
  spatial.source_ref = "native-test:emission/body-pickup";
  spatial.policy_ref = "native-test:emission/dated-retarded-point";
  spatial.policy_revision = "1";
  spatial.standing = "architecture-model";
  spatial.receiver_position_metres = old_anchor;
  spatial.receiver_position_metres[0] += .37;
  SpatialMotionInput motion;
  motion.source_motion_ref = "native-test:emission/source-motion";
  motion.receiver_motion_ref = "native-test:emission/receiver-motion";
  motion.policy_ref = spatial.policy_ref;
  motion.policy_revision = spatial.policy_revision;
  motion.standing = spatial.standing;
  motion.end_sample = 48000 * 60;
  auto receiving = std::make_shared<MovingReceivingPortBinding>(
      body, original, 0,
      PreparedMovingSpatialReceiving(original, spatial, motion));
  auto port = receiving->port(receiving);
  std::vector<float> all_pickup, all_received;
  std::array<double, 128> force{};
  std::array<float, 128> pickup{}, received{};
  for (unsigned block = 0; block < 4; ++block) {
    for (unsigned i = 0; i < 128; ++i)
      force[i] = .005 * std::sin(.037 * (block * 128 + i));
    callback_probe = true;
    const bool pre = port.preflight(port.owner, 128, block * 128);
    const bool p =
        body->advance_force_block(force.data(), pickup.data(), 128,
                                  original.input().body_revision, block * 128);
    const bool r = p && port.process(port.owner, pickup.data(), received.data(),
                                     128, block * 128);
    callback_probe = false;
    assert(pre && p && r && allocations == 0 && releases == 0);
    all_pickup.insert(all_pickup.end(), pickup.begin(), pickup.end());
    all_received.insert(all_received.end(), received.begin(), received.end());
  }
  auto before = std::make_unique<NativeReceivingCheckpoint>();
  assert(port.write_checkpoint(port.owner, *before, 512));
  assert(before->version == 1 && before->history_start_sample == 0 &&
         before->source_history.count == 1);
  auto old_wire = checkpoint_transport::receiving_checkpoint(*before);
  J *unexpected_history = nullptr;
  assert(json_object_object_length(old_wire.get()) == 7 &&
         !json_object_object_get_ex(old_wire.get(), "source_history",
                                    &unexpected_history));
  auto in = original.input();
  ++in.body_revision;
  in.preparation_ref = "native-test:emission/after-form-preparation";
  in.geometry_ref = "native-test:emission/translated-form";
  in.geometry_revision = "2";
  in.geometry_source_ref =
      "native-test:emission/bounded-metric-form-experiment";
  in.geometry_standing = "architecture-model";
  for (auto &node : in.nodes)
    node.rest_metres[0] += shift;
  const PreparedPhysicalBody after(in);
  const auto new_anchor = independent_pickup_anchor(after);
  auto transition = std::make_unique<PreparedPhysicalTransition>(
      original, after, original.input().body_revision, 512,
      PhysicalLiveUpdateKind::FormOrBoundary,
      PhysicalFormTransition::ProjectCorrespondingNodes,
      "native-test:emission/actual-form-transition");
  assert(transition->preflight(*body));
  auto candidate = MovingReceivingPortBinding::from_prepared_body_transition(
      body, *transition, PreparedMovingSpatialReceiving(after, spatial, motion),
      *before);
  auto next_port = candidate->port(candidate);
  auto dated = std::make_unique<NativeReceivingCheckpoint>();
  assert(next_port.write_transport_checkpoint(next_port.owner, *dated, 512) &&
         dated->version == 2 && dated->source_history.count == 2 &&
         dated->source_history.segments[0].effective_sample == 0 &&
         dated->source_history.segments[1].effective_sample == 512 &&
         dated->history_linear == before->history_linear);
  PhysicalLiveTransitionReceipt receipt;
  callback_probe = true;
  const bool committed = transition->apply(*body, receipt);
  callback_probe = false;
  assert(committed && allocations == 0 && releases == 0 &&
         body->samples_elapsed() == 512 &&
         body->body_revision() == after.input().body_revision);
  receiving = candidate;
  port = next_port;
  bool detects_backdating = false, overlap = false, gap = false;
  for (unsigned block = 4; block < 8; ++block) {
    for (unsigned i = 0; i < 128; ++i)
      force[i] = .005 * std::sin(.037 * (block * 128 + i));
    callback_probe = true;
    const bool pre = port.preflight(port.owner, 128, block * 128);
    const bool p =
        body->advance_force_block(force.data(), pickup.data(), 128,
                                  after.input().body_revision, block * 128);
    const bool r = p && port.process(port.owner, pickup.data(), received.data(),
                                     128, block * 128);
    callback_probe = false;
    assert(pre && p && r && allocations == 0 && releases == 0);
    all_pickup.insert(all_pickup.end(), pickup.begin(), pickup.end());
    for (unsigned i = 0; i < 128; ++i) {
      const auto at = block * 128 + i;
      const double old = independent_stationary_received(
          all_pickup, at, 0, 512, old_anchor, spatial.receiver_position_metres,
          spatial);
      const double next = independent_stationary_received(
          all_pickup, at, 512, std::numeric_limits<std::uint64_t>::max(),
          new_anchor, spatial.receiver_position_metres, spatial);
      const float expected = float(old + next);
      assert(received[i] == expected &&
             std::signbit(received[i]) == std::signbit(expected));
      const float backdated = float(independent_stationary_received(
          all_pickup, at, 0, std::numeric_limits<std::uint64_t>::max(),
          new_anchor, spatial.receiver_position_metres, spatial));
      detects_backdating |= backdated != received[i];
      overlap |= old != 0 && next != 0;
      gap |= old == 0 && next == 0;
    }
    all_received.insert(all_received.end(), received.begin(), received.end());
  }
  assert(detects_backdating && (shift > 0 ? overlap : gap));
  auto saved = std::make_unique<NativeReceivingCheckpoint>();
  assert(port.write_checkpoint(port.owner, *saved, 1024));
  auto wire = checkpoint_transport::receiving_checkpoint(*saved);
  auto imported = std::make_unique<NativeReceivingCheckpoint>();
  checkpoint_transport::read_receiving_checkpoint(wire.get(), *imported);
  assert(same_receiving_checkpoint(*saved, *imported));
  auto lossy = checkpoint_transport::receiving_checkpoint(*saved);
  auto rows =
      packet::field(packet::field(lossy.get(), "source_history"), "segments");
  assert(json_object_array_del_idx(rows, 0, 1) == 0);
  refused([&] {
    auto bad = std::make_unique<NativeReceivingCheckpoint>();
    checkpoint_transport::read_receiving_checkpoint(lossy.get(), *bad);
  });
  auto bad = std::make_unique<NativeReceivingCheckpoint>(*imported);
  bad->source_history.segments[1].anchor_metres[0] += .01;
  assert(!port.validate_checkpoint(port.owner, *bad, 1024));
  const auto p_saved = body->checkpoint();
  auto cold = std::make_shared<PhysicalBody>(after);
  assert(cold->restore_checkpoint(p_saved, after.input().body_revision, 0));
  auto resumed = MovingReceivingPortBinding::from_saved_preparation(
      cold, after, 1024, PreparedMovingSpatialReceiving(after, spatial, motion),
      0, *imported);
  auto resumed_port = resumed->port(resumed);
  std::array<float, 128> cold_pickup{}, cold_received{};
  for (unsigned block = 8; block < 12; ++block) {
    for (unsigned i = 0; i < 128; ++i)
      force[i] = .005 * std::sin(.037 * (block * 128 + i));
    callback_probe = true;
    const bool pre =
        port.preflight(port.owner, 128, block * 128) &&
        resumed_port.preflight(resumed_port.owner, 128, block * 128);
    const bool p =
        body->advance_force_block(force.data(), pickup.data(), 128,
                                  after.input().body_revision, block * 128) &&
        cold->advance_force_block(force.data(), cold_pickup.data(), 128,
                                  after.input().body_revision, block * 128);
    const bool r = p &&
                   port.process(port.owner, pickup.data(), received.data(), 128,
                                block * 128) &&
                   resumed_port.process(resumed_port.owner, cold_pickup.data(),
                                        cold_received.data(), 128, block * 128);
    callback_probe = false;
    assert(pre && p && r && allocations == 0 && releases == 0 &&
           pickup == cold_pickup && received == cold_received);
    auto a = std::make_unique<NativeReceivingCheckpoint>(),
         b = std::make_unique<NativeReceivingCheckpoint>();
    assert(port.write_checkpoint(port.owner, *a, (block + 1) * 128) &&
           resumed_port.write_checkpoint(resumed_port.owner, *b,
                                         (block + 1) * 128) &&
           same_receiving_checkpoint(*a, *b));
    assert(body->checkpoint().displacement_modal_metres ==
               cold->checkpoint().displacement_modal_metres &&
           body->checkpoint().velocity_modal_metres_per_second ==
               cold->checkpoint().velocity_modal_metres_per_second);
  }
  // Capacity is concurrent retained emission history, not a lifetime edit cap.
  auto history = std::make_unique<ReceivingSourceHistory>();
  PreparedMovingSpatialReceiving numerical(after, spatial, motion);
  assert(append_receiving_source(*history, after, numerical, 0, 0));
  for (unsigned i = 1; i < receiving_source_segments; ++i)
    assert(append_receiving_source(*history, after, numerical, i, 0));
  auto full = std::make_unique<ReceivingSourceHistory>(*history);
  assert(!append_receiving_source(*history, after, numerical,
                                  receiving_source_segments, 0) &&
         same_receiving_source_history(*history, *full));
  for (unsigned i = 0; i < 600; ++i)
    assert(append_receiving_source(*history, after, numerical,
                                   receiving_history_samples + i * 128, 0));
  assert(history->count < receiving_source_segments &&
         valid_receiving_source_history(
             *history, receiving_history_samples + 599 * 128, 0));
  // Full historical references retain their admitted 2048-byte bound rather
  // than silently taking the short musical Ref parser path.
  auto long_motion = motion;
  long_motion.source_motion_ref =
      "native-test:emission/full-source-descendant/" + std::string(1800, 'x');
  PreparedMovingSpatialReceiving long_source(after, spatial, long_motion);
  auto long_history = std::make_unique<ReceivingSourceHistory>();
  assert(append_receiving_source(*long_history, after, long_source, 0, 0) &&
         append_receiving_source(*long_history, after, long_source, 1, 0));
  auto long_wire =
      checkpoint_transport::receiving_source_history(*long_history);
  auto decoded_history = std::make_unique<ReceivingSourceHistory>();
  checkpoint_transport::read_receiving_source_history(long_wire.get(),
                                                      *decoded_history, 1, 0);
  assert(same_receiving_source_history(*long_history, *decoded_history) &&
         decoded_history->segments[1].references[5].length > 256);
  auto wrong_units =
      checkpoint_transport::receiving_source_history(*long_history);
  json_object_object_add(wrong_units.get(), "velocity_units",
                         json_object_new_string("arbitrary"));
  refused([&] {
    checkpoint_transport::read_receiving_source_history(wrong_units.get(),
                                                        *decoded_history, 1, 0);
  });
}
static void independent_receiving_emission_history_cases(J *fixture) {
  independent_receiving_emission_form_case(fixture, .2);
  independent_receiving_emission_form_case(fixture, -.2);
}
