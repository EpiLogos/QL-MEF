// Included only by the genuine Rust-source -> A/P/Management paired trial.
// No mock port, source permission, fabricated application or duplicate clock.
struct FreshReceiverRoutes {
  PhysicalPort port;
  std::unique_ptr<NativeRouteProgramSet> seed;
};
static FreshReceiverRoutes fresh_receiver_routes(SegmentSession &session,
                                                 J *fixture, J *current,
                                                 std::uint64_t cursor) {
  auto &native = session.owner->native();
  auto *basis = packet::field(fixture, "native_basis");
  auto *admission = packet::field(current, "native_admission");
  auto *sources =
      packet::field(packet::field(admission, "operation"), "sources");
  std::vector<std::string> refs;
  for (std::size_t i = 0; i < json_object_array_length(sources); ++i)
    refs.push_back(packet::string(packet::field(
                       json_object_array_get_idx(sources, i), "driver_ref")) +
                   "/m1-excitation-program");
  auto typed = std::make_shared<const AdmittedNativeReceivingSource>(
      read_native_receiving_admission(admission, admission, native, basis,
                                      *session.immutable, refs, cursor));
  auto binding = std::make_shared<PhysicalRoutesPortBinding>(
      native.body, typed, *session.immutable, native.determination, basis,
      native.determination.m1_face == 1, cursor);
  auto seed = std::make_unique<NativeRouteProgramSet>();
  seed->manifest = binding->manifest();
  seed->program_count = seed->manifest.route_count;
  seed->scalar_note_enabled = seed->manifest.scalar_note_enabled;
  seed->scalar_note_gain = seed->manifest.scalar_note_gain;
  for (std::size_t i = 0; i < seed->program_count; ++i) {
    seed->programs[i].handle = seed->manifest.programs[i];
    seed->programs[i].phase_source_ref = seed->manifest.m1_coordinate;
    seed->programs[i].sine = native.notes.front().phase_sin;
    seed->programs[i].cosine = native.notes.front().phase_cos;
  }
  return {physical_routes_port(physical_port(native.body), binding),
          std::move(seed)};
}
static std::vector<KeyboardCell> fresh_receiver_catalog(J *fixture) {
  std::vector<KeyboardCell> cells;
  auto *catalog = packet::field(fixture, "native_catalog");
  for (std::size_t i = 0; i < json_object_array_length(catalog); ++i)
    cells.push_back(
        management_transport::read_key(json_object_array_get_idx(catalog, i)));
  return cells;
}
static Json fresh_saved_receiver_segment(J *fixture) {
  auto original = std::make_unique<SegmentSession>(fixture);
  original->advance(4096, 128);
  original->reserve_future();
  auto before = original->owner->stopped_checkpoint();
  auto *after = packet::field(fixture, "after_acoustic");
  auto *packet_value = packet::field(after, "packet");
  auto *body_packet = packet::field(
      packet::field(fixture, "native_preparation"), "physical_body");
  auto prepared = acoustic_wire::read_prepared_acoustic(
      packet_value, packet_value, body_packet, *original->immutable, 4096,
      acoustic_wire::AcousticReadKind::RetainedReceiverSegment);
  auto installed = std::make_shared<MovingReceivingPortBinding>(
      original->owner->native().body, *original->immutable, 4096,
      std::move(prepared), before->native_pair.audio.receiving);
  {
    auto guard = original->owner->native().engine->acquire_stopped_custody();
    auto token =
        std::make_unique<PerformanceManagement::PreparedReceivingReplacement>();
    require(guard && original->owner->prepare_stopped_receiving_replacement(
                         installed->port(installed), guard, 4096, *token),
            "genuine saved receiver segment preparation refused");
    original->owner->commit_stopped_receiving_replacement(*token, guard);
    original->receiving = installed;
    original->owner->refresh_stopped_reading(guard);
  }
  auto saved = original->owner->stopped_checkpoint();
  require(saved->native_pair.audio.cursor == 4096 &&
              saved->native_pair.audio.receiving.manifest.origin_sample ==
                  4096 &&
              saved->native_pair.audio.receiving.history_start_sample == 0 &&
              saved->native_pair.audio.accepted_sequence == 3,
          "original later segment/source/queue/birth not retained");
  const auto expected = original->advance(13000, 128);
  auto final_original = original->owner->stopped_checkpoint();
  auto reopened = std::make_unique<SegmentSession>(fixture, false);
  auto fresh = fresh_receiver_routes(
      *reopened, fixture, packet::field(after, "current_receiving"), 4096);
  // Rebuild the ORIGINAL segment at4096, not the fresh resident cursor0.
  // The true native producer supplies its complete motion/context manifest.
  auto numerical = acoustic_wire::read_prepared_acoustic(
      packet_value, packet_value, body_packet, *reopened->immutable, 4096,
      acoustic_wire::AcousticReadKind::RetainedReceiverSegment);
  auto candidate = MovingReceivingPortBinding::from_saved_preparation(
      reopened->owner->native().body, *reopened->immutable, 4096,
      std::move(numerical),
      wire::decimal(packet::field(packet_value, "history_origin_sample")),
      saved->native_pair.audio.receiving);
  auto receiver = candidate->port(candidate);
  require(receiver.cursor(receiver.owner) == 4096 &&
              reopened->owner->native().engine->samples_elapsed() == 0 &&
              reopened->owner->native().body->samples_elapsed() == 0,
          "pure saved receiving candidate advanced resident P/audio");
  auto pristine = reopened->owner->stopped_checkpoint();
  auto pristine_wire =
      management_checkpoint_transport::checkpoint_wire(*pristine);
  {
    auto guard = reopened->owner->native().engine->acquire_stopped_custody();
    auto missing =
        std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
    require(guard && !reopened->owner->preflight_stopped_receiving_restore(
                         *saved, fresh.port, *fresh.seed,
                         reopened->owner->native().notes,
                         fresh_receiver_catalog(fixture), guard, 0, *missing),
            "saved receiving continuation silently omitted original receiver");
    auto token =
        std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
    require(reopened->owner->preflight_stopped_receiving_restore(
                *saved, fresh.port, *fresh.seed,
                reopened->owner->native().notes,
                fresh_receiver_catalog(fixture), guard, 0, *token, &receiver) &&
                reopened->owner->receiving_restore_current(*token, guard),
            "actual original saved receiver candidate refused before P commit");
    auto altered = std::make_unique<NativeReceivingCheckpoint>(
        saved->native_pair.audio.receiving);
    altered->history_linear[17] =
        std::nextafter(altered->history_linear[17], 1.f);
    require(receiver.validate_checkpoint(receiver.owner, *altered, 4096),
            "real candidate ring mutation is not a valid same-cursor state");
    receiver.restore_checkpoint(receiver.owner, *altered);
    require(!reopened->owner->receiving_restore_current(*token, guard),
            "fresh restore token accepted changed prepared pickup history");
    receiver.restore_checkpoint(receiver.owner,
                                saved->native_pair.audio.receiving);
    require(reopened->owner->receiving_restore_current(*token, guard),
            "return to original true saved history did not recover token");
  }
  auto refusal = reopened->owner->stopped_checkpoint();
  auto refusal_wire =
      management_checkpoint_transport::checkpoint_wire(*refusal);
  require(json_object_equal(pristine_wire.get(), refusal_wire.get()),
          "pure saved receiving preflight/refusal changed full fresh owner");
  // A numerically valid different context/birth/source manifest cannot be
  // installed merely because its cursor and finite history are compatible.
  require(
      saved->native_pair.audio.receiving.version == 2 &&
          saved->native_pair.audio.receiving.source_history.explicit_history &&
          saved->native_pair.audio.receiving.source_history.count == 2 &&
          saved->native_pair.audio.receiving.source_history.first == 0 &&
          saved->native_pair.audio.receiving.source_history.segments[0]
                  .effective_sample == 0 &&
          saved->native_pair.audio.receiving.source_history.segments[1]
                  .effective_sample == 4096,
      "wrong receiver trials lack the complete genuine dated source corpus");
  auto detecting_receivers = wire::array();
  for (unsigned variant = 0; variant < 3; ++variant) {
    auto bad = std::make_unique<NativeReceivingCheckpoint>(
        saved->native_pair.audio.receiving);
    if (variant == 0)
      bad->manifest.context = candidate->manifest().receiver;
    else if (variant == 1) {
      bad->history_start_sample = 1;
      bad->manifest.history_origin_sample = 1;
      // Keep the mutated v2 numerical history internally consistent. Its real
      // first emitter remains dated independently from the receiver birth.
      bad->source_history.segments[0].effective_sample = 1;
    } else {
      bad->manifest.source_revision = candidate->manifest().policy_revision;
      auto &tail = bad->source_history.segments[bad->source_history.count - 1];
      require(ql::intern_receiving_source_reference(
                  bad->source_history, bad->manifest.source_revision.data(),
                  tail.references[1]),
              "actual wrong-source negative exceeds native reference bound");
    }
    require(
        !same_receiving_checkpoint(*bad, saved->native_pair.audio.receiving) &&
            valid_receiving_checkpoint(*bad, bad->manifest),
        "wrong native receiver negative is not changed/numerically "
        "well-formed");
    bool refused = false;
    std::string actual_reason;
    try {
      auto other = acoustic_wire::read_prepared_acoustic(
          packet_value, packet_value, body_packet, *reopened->immutable, 4096,
          acoustic_wire::AcousticReadKind::RetainedReceiverSegment);
      // Birth is supplied by the original native producer, independently of CP.
      (void)MovingReceivingPortBinding::from_saved_preparation(
          reopened->owner->native().body, *reopened->immutable, 4096,
          std::move(other),
          wire::decimal(packet::field(packet_value, "history_origin_sample")),
          *bad);
    } catch (const std::invalid_argument &failure) {
      refused = true;
      actual_reason = failure.what();
    }
    require(refused,
            "original receiving producer admitted a different manifest");
    auto actual_after_refusal = reopened->owner->stopped_checkpoint();
    auto after_refusal_wire =
        management_checkpoint_transport::checkpoint_wire(*actual_after_refusal);
    require(
        std::strcmp(json_object_to_json_string_ext(pristine_wire.get(),
                                                   JSON_C_TO_STRING_PLAIN),
                    json_object_to_json_string_ext(
                        after_refusal_wire.get(), JSON_C_TO_STRING_PLAIN)) == 0,
        "wrong receiver/source/birth refusal mutated the fresh native owner");
    auto detecting = wire::object();
    wire::u64(detecting.get(), "variant", variant);
    wire::flag(detecting.get(), "numerically_valid", true);
    wire::text(detecting.get(), "actual_refusal", actual_reason);
    wire::put(detecting.get(), "original_mutated_checkpoint",
              wire::receiving_checkpoint(*bad).release());
    wire::append(detecting_receivers.get(), detecting.release());
  }
  std::unique_ptr<PerformanceManagement::PreparedReceivingRestore> retained;
  TransportAcknowledgement ack{};
  require(restore_current_receiving_checkpoint(
              *reopened->owner, *saved, fresh.port, *fresh.seed,
              reopened->owner->native().notes, fresh_receiver_catalog(fixture),
              0, reference("native:acoustic/fresh-receiver-restore"),
              reference("native:acoustic/saved-receiver-4096"), retained, ack,
              &receiver) &&
              retained && ack.previous_cursor == 0 &&
              ack.target_sample == 4096 && ack.previous_epoch == 1 &&
              ack.epoch == 2,
          "fresh same-body receiving/P/audio atomic restore refused");
  reopened->receiving = candidate;
  auto after_restore = reopened->owner->stopped_checkpoint();
  require(same_receiving_checkpoint(after_restore->native_pair.audio.receiving,
                                    saved->native_pair.audio.receiving),
          "fresh restore lost original complete receiver/history/birth");
  const auto actual = reopened->advance(13000, 512);
  require(
      actual.pickup == expected.pickup &&
          actual.received == expected.received && actual.pcm == expected.pcm,
      "fresh restore changed actual pickup/receiving/PCM future continuation");
  auto final_reopened = reopened->owner->stopped_checkpoint();
  require(
      same_receiving_checkpoint(final_original->native_pair.audio.receiving,
                                final_reopened->native_pair.audio.receiving),
      "fresh continuation lost full exact receiver ring");
  auto first_physical = ql::physical_wire::checkpoint_wire(
           final_original->native_pair.physical),
       last_physical = ql::physical_wire::checkpoint_wire(
           final_reopened->native_pair.physical);
  require(json_object_equal(first_physical.get(), last_physical.get()) &&
              reopened->applications.size() == 2 &&
              reopened->applications[0].sequence == 2 &&
              reopened->applications[1].sequence == 3 &&
              reopened->applications[0].applied_sample == 9000 &&
              reopened->applications[1].applied_sample == 10000 &&
              reopened->applications[0].applied_application_ordinal == 2 &&
              reopened->applications[1].applied_application_ordinal == 3,
          "fresh receiving continuation changed P or original future work");
  auto output = wire::object();
  wire::text(output.get(), "schema",
             "ql.native-acoustic-fresh-segment-component/v1");
  wire::text(output.get(), "standing",
             "actual native producer/paired stopped numerical restore; private "
             "Scene/Act lease not exercised");
  wire::put(
      output.get(), "saved_receiving",
      wire::receiving_checkpoint(saved->native_pair.audio.receiving).release());
  wire::put(
      output.get(), "restored_receiving",
      wire::receiving_checkpoint(after_restore->native_pair.audio.receiving)
          .release());
  wire::put(
      output.get(), "final_receiving",
      wire::receiving_checkpoint(final_reopened->native_pair.audio.receiving)
          .release());
  wire::put(output.get(), "physical", last_physical.release());
  auto acknowledgement = wire::object();
  wire::u64(acknowledgement.get(), "previous_epoch", ack.previous_epoch);
  wire::u64(acknowledgement.get(), "epoch", ack.epoch);
  wire::u64(acknowledgement.get(), "previous_cursor", ack.previous_cursor);
  wire::u64(acknowledgement.get(), "previous_sequence", ack.previous_sequence);
  wire::u64(acknowledgement.get(), "target_sample", ack.target_sample);
  wire::u64(acknowledgement.get(), "accepted_sequence", ack.accepted_sequence);
  wire::ref(acknowledgement.get(), "transaction_ref", ack.transaction);
  wire::ref(acknowledgement.get(), "checkpoint_ref", ack.checkpoint);
  wire::put(output.get(), "transport_ack", acknowledgement.release());
  auto apps = wire::array(), history = wire::array(), pcm = wire::array();
  for (const auto &app : reopened->applications)
    wire::append(apps.get(), wire::application(app).release());
  for (const auto &row : reopened->journal)
    wire::append(history.get(), management_transport::history(row).release());
  for (float value : actual.pcm)
    wire::append(pcm.get(), json_object_new_double(value));
  wire::put(output.get(), "applications", apps.release());
  wire::put(output.get(), "input_history", history.release());
  wire::put(output.get(), "continued_pcm", pcm.release());
  wire::put(output.get(), "detecting_wrong_receivers",
            detecting_receivers.release());
  wire::u64(output.get(), "end_cursor", 13000);
  return output;
}
