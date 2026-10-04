// Actual original Rust N9/P -> normal retained body Form/material revision ->
// SAME native Manager/A stopped earlier-prefix restore -> complete future
// note/control/release and bit-exact PCM/P/M4 continuation. No C lease/source
// grant is constructed by this detecting numerical activity.
#include <ql/performance_source_readoption.hpp>
namespace ql_test_native_source_readoption {
static std::string complete(PerformanceManagement &manager) {
  const auto cp = manager.stopped_checkpoint();
  auto wire = management_checkpoint_transport::checkpoint_wire(*cp);
  return json_object_to_json_string_ext(wire.get(), JSON_C_TO_STRING_PLAIN);
}
static std::shared_ptr<MovingReceivingPortBinding> saved_receiver(
    const NativePerformance &native, const ManagementCheckpoint &saved,
    const SpatialReceivingInput &spatial, const SpatialMotionInput &motion) {
  return MovingReceivingPortBinding::from_saved_preparation(
      native.body, native.body->preparation(), saved.native_pair.audio.cursor,
      PreparedMovingSpatialReceiving(native.body->preparation(), spatial,
                                     motion),
      saved.native_pair.audio.receiving.history_start_sample,
      saved.native_pair.audio.receiving);
}
struct Selected {
  NativePerformance native;
  std::shared_ptr<PhysicalRoutesPortBinding> binding;
  NativeRouteProgramSet seed;
};
static Selected select(J *original, J *saved_fixture, std::uint64_t cursor) {
  if (cursor == 0) {
    auto real = prepare(original);
    return {std::move(real.native), std::move(real.binding), real.programmes};
  }
  // The paired actual Rust before_at512 producer owns this admission. A date
  // edited into the old fixture would be a disconnected producer and refuse.
  assert(cursor == 512 && saved_fixture);
  auto real = revision_source(saved_fixture, {});
  return {std::move(real.prepared), std::move(real.binding), real.seed};
}
static void activity(PerformanceManagement &owner, J *fixture, J *saved_fixture,
                     const ManagementCheckpoint &saved,
                     const SpatialReceivingInput &spatial,
                     const SpatialMotionInput &motion, bool with_receiving) {
  const auto old_manager = owner.resident_token();
  const auto old_engine = owner.native().engine.get();
  const auto old_audio_resident = owner.native().engine->resident_token();
  const auto saved_cursor = saved.native_pair.audio.cursor;
  assert((saved_cursor == 0 || saved_cursor == 512) &&
         saved.native_pair.physical.body_revision == 1);
  assert(owner.native().engine->samples_elapsed() > saved_cursor);
  const auto original_before = owner.stopped_checkpoint();
  const auto before = complete(owner);
  // Refusals use the real selected source producer and entire original CP.
  // No fake occurrence/source token or a synthesized positive is introduced.
  for (unsigned variant = 0; variant < 6; ++variant) {
    auto selected = select(fixture, saved_fixture, saved_cursor);
    const auto selected_port = physical_routes_port(
        physical_port(selected.native.body), selected.binding);
    auto changed = std::make_unique<ManagementCheckpoint>(saved);
    auto seed = selected.seed;
    std::shared_ptr<MovingReceivingPortBinding> receiver;
    ReceivingPort port{};
    if (with_receiving) {
      receiver = saved_receiver(selected.native, *changed, spatial, motion);
      port = receiver->port(receiver);
    }
    auto cursor = original_before->native_pair.audio.cursor;
    auto epoch = owner.transport_epoch();
    auto sequence = original_before->native_pair.audio.accepted_sequence;
    if (variant == 0)
      ++cursor;
    if (variant == 1)
      ++sequence;
    if (variant == 2)
      ++epoch;
    if (variant == 3)
      changed->native_pair.physical.body_revision = 2;
    if (variant == 4)
      seed.manifest.source_revision[0] =
          seed.manifest.source_revision[0] == 'x' ? 'y' : 'x';
    if (variant == 5)
      changed->native_pair.audio.determination.audio_octet_hz[0] =
          std::nextafter(
              changed->native_pair.audio.determination.audio_octet_hz[0],
              std::numeric_limits<double>::infinity());
    TransportAcknowledgement refused{};
    std::unique_ptr<NativeStoppedSourceReadoption::Retained> hold;
    assert(!NativeStoppedSourceReadoption::restore(
        owner, std::move(selected.native), *original_before, *changed,
        selected_port, &seed,
        native_catalog(saved_fixture ? saved_fixture : fixture), epoch, cursor,
        sequence, reference("native-test:readoption/refused"),
        reference("native-test:readoption/original-cut"), hold, refused,
        with_receiving ? &port : nullptr));
    assert(!hold && owner.native().engine.get() == old_engine &&
           owner.resident_token() == old_manager && complete(owner) == before);
  }
  auto reference_source = select(fixture, saved_fixture, saved_cursor);
  auto independent_owner = std::make_unique<PerformanceManagement>(
      reference_source.native, owner.session_ref());
  std::shared_ptr<MovingReceivingPortBinding> independent_receiver;
  ReceivingPort independent_receiver_port{};
  if (with_receiving) {
    independent_receiver =
        saved_receiver(reference_source.native, saved, spatial, motion);
    independent_receiver_port =
        independent_receiver->port(independent_receiver);
  }
  TransportAcknowledgement reference_ack{};
  std::unique_ptr<PerformanceManagement::PreparedReceivingRestore>
      independent_restore;
  const auto reference_port = physical_routes_port(
      physical_port(reference_source.native.body), reference_source.binding);
  assert(restore_current_receiving_checkpoint(
      *independent_owner, saved, reference_port, reference_source.seed,
      reference_source.native.notes,
      native_catalog(saved_fixture ? saved_fixture : fixture), 0,
      reference("native-test:readoption/independent-replay"),
      reference("native-test:readoption/original-cut"), independent_restore,
      reference_ack, with_receiving ? &independent_receiver_port : nullptr));

  auto selected = select(fixture, saved_fixture, saved_cursor);
  const auto selected_body = selected.native.body;
  std::shared_ptr<MovingReceivingPortBinding> receiver;
  ReceivingPort receiver_port{};
  if (with_receiving) {
    receiver = saved_receiver(selected.native, saved, spatial, motion);
    receiver_port = receiver->port(receiver);
  }
  const auto selected_port =
      physical_routes_port(physical_port(selected_body), selected.binding);
  std::unique_ptr<NativeStoppedSourceReadoption::Retained> hold;
  TransportAcknowledgement ack{};
  assert(NativeStoppedSourceReadoption::restore(
      owner, std::move(selected.native), *original_before, saved, selected_port,
      &selected.seed, native_catalog(saved_fixture ? saved_fixture : fixture),
      owner.transport_epoch(), original_before->native_pair.audio.cursor,
      original_before->native_pair.audio.accepted_sequence,
      reference("native-test:readoption/selected-original"),
      reference("native-test:readoption/original-cut"), hold, ack,
      with_receiving ? &receiver_port : nullptr));
  assert(hold && hold->committed() && owner.resident_token() == old_manager &&
         owner.native().engine.get() == old_engine &&
         owner.native().engine->resident_token() == old_audio_resident &&
         owner.native().body.get() == selected_body.get() &&
         owner.native().body->body_revision() == 1 &&
         ack.previous_epoch + 1 == ack.epoch &&
         ack.target_sample == saved_cursor &&
         Engine::same_prepared_determination(
             owner.native().engine->current_source(),
             saved.native_pair.audio.determination));
  auto original_wire = management_checkpoint_transport::checkpoint_wire(
      hold->original_checkpoint());
  auto saved_wire = management_checkpoint_transport::checkpoint_wire(saved);
  assert(
      std::string(json_object_to_json_string_ext(original_wire.get(),
                                                 JSON_C_TO_STRING_PLAIN)) ==
      json_object_to_json_string_ext(saved_wire.get(), JSON_C_TO_STRING_PLAIN));
  // This is the genuine original normal feedback, not an extra observation
  // clock. Pending birth0 retains admitted input journal; saved applications
  // may also return. Compare both independent native owners before completion.
  const auto actual_feedback = owner.pulse();
  const auto independent_feedback = independent_owner->pulse();
  const auto feedback = [](const ManagementPulse &pulse) {
    auto out = checkpoint_transport::object();
    auto applications = checkpoint_transport::array();
    for (const auto &app : pulse.applications)
      checkpoint_transport::append(
          applications.get(), checkpoint_transport::application(app).release());
    checkpoint_transport::put(out.get(), "applications",
                              applications.release());
    management_transport::put_input_history(out.get(), pulse);
    return std::string(
        json_object_to_json_string_ext(out.get(), JSON_C_TO_STRING_PLAIN));
  };
  assert(feedback(*actual_feedback) == feedback(*independent_feedback));
  // Derive a detecting refusal from the actual original native pulse. The
  // full saved journal/application feedback is never removed to make a pass.
  auto wrong_feedback = *actual_feedback;
  assert(wrong_feedback.last_input_ordinal !=
         std::numeric_limits<std::uint64_t>::max());
  ++wrong_feedback.last_input_ordinal;
  const auto actual_after_feedback = complete(owner);
  assert(!NativeStoppedSourceReadoption::complete_pulse(owner, wrong_feedback,
                                                        *hold));
  assert(!hold->pulse_completed() && complete(owner) == actual_after_feedback);
  assert(NativeStoppedSourceReadoption::complete_pulse(owner, *actual_feedback,
                                                       *hold));
  assert(hold->pulse_completed());
  // Native pulse completion is a one-time transaction acknowledgement.
  assert(!NativeStoppedSourceReadoption::complete_pulse(owner, *actual_feedback,
                                                        *hold));
  assert(complete(owner) == actual_after_feedback);
  // The derivative native route admissions and the newly acknowledged epoch
  // belong ONLY to the operative copy. Complete original saved/pre-pulse wire
  // stays separate; SAME original feedback drains the actual native journals.
  // This complete post-feedback operative state equals native SAME-owner AFTER.
  const auto after_checkpoint = owner.stopped_checkpoint();
  auto operative_wire = management_checkpoint_transport::checkpoint_wire(
      hold->operative_checkpoint());
  auto after_wire =
      management_checkpoint_transport::checkpoint_wire(*after_checkpoint);
  assert(
      std::string(json_object_to_json_string_ext(operative_wire.get(),
                                                 JSON_C_TO_STRING_PLAIN)) ==
      json_object_to_json_string_ext(after_wire.get(), JSON_C_TO_STRING_PLAIN));
  auto before_wire = management_checkpoint_transport::checkpoint_wire(
      hold->before_checkpoint());
  assert(std::string(json_object_to_json_string_ext(
             before_wire.get(), JSON_C_TO_STRING_PLAIN)) == before);
  unsigned parameter = 0, released = 0;
  for (unsigned block = 0; block < 12; ++block) {
    std::array<float, 128> actual{}, independent{};
    callback_probe = true;
    const bool a = owner.offline_advance(actual.data(), actual.size(),
                                         saved_cursor + block * 128);
    const bool b = independent_owner->offline_advance(
        independent.data(), independent.size(), saved_cursor + block * 128);
    callback_probe = false;
    assert(a && b && !allocations && !releases &&
           std::memcmp(actual.data(), independent.data(), sizeof(actual)) == 0);
    const auto x = owner.pulse(), y = independent_owner->pulse();
    assert(x->applications.size() == y->applications.size());
    for (std::size_t i = 0; i < x->applications.size(); ++i) {
      const auto &app = x->applications[i];
      auto first = checkpoint_transport::application(app);
      auto second = checkpoint_transport::application(y->applications[i]);
      assert(
          std::string(json_object_to_json_string_ext(first.get(),
                                                     JSON_C_TO_STRING_PLAIN)) ==
          json_object_to_json_string_ext(second.get(), JSON_C_TO_STRING_PLAIN));
      parameter += app.applied && app.kind == Kind::Parameter &&
                   app.applied_sample == 600;
      released +=
          app.applied && app.kind == Kind::NoteOff && app.applied_sample == 900;
    }
    Capture first{}, second{};
    assert(owner.pop_audio_capture(first) &&
           independent_owner->pop_audio_capture(second));
    const auto bits = [](const auto &a, const auto &b) {
      assert(std::memcmp(a.data(), b.data(), sizeof(a)) == 0);
    };
    bits(first.force_newtons, second.force_newtons);
    bits(first.note_force_newtons, second.note_force_newtons);
    bits(first.contact_force_newtons, second.contact_force_newtons);
    bits(first.body_gain_linear, second.body_gain_linear);
    bits(first.monitor_gain_linear, second.monitor_gain_linear);
    bits(first.force_scale_newtons, second.force_scale_newtons);
    bits(first.pickup_linear, second.pickup_linear);
    bits(first.received_linear, second.received_linear);
    bits(first.output_linear, second.output_linear);
    bits(first.route_force_newtons, second.route_force_newtons);
    auto original_p =
        physical_wire::checkpoint_wire(owner.native().body->checkpoint());
    auto independent_p = physical_wire::checkpoint_wire(
        independent_owner->native().body->checkpoint());
    assert(std::string(json_object_to_json_string_ext(
               original_p.get(), JSON_C_TO_STRING_PLAIN)) ==
           json_object_to_json_string_ext(independent_p.get(),
                                          JSON_C_TO_STRING_PLAIN));
    if (with_receiving) {
      const auto a = owner.stopped_checkpoint(),
                 b = independent_owner->stopped_checkpoint();
      assert(same_receiving_checkpoint(a->native_pair.audio.receiving,
                                       b->native_pair.audio.receiving));
    }
  }
  assert(parameter == 1 && released == 1);
}
} // namespace ql_test_native_source_readoption
