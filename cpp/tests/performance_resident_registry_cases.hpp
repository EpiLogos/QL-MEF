// Same real Control/P/A source producers; no synthetic observations or ports.
static Json resident_registry_copy(Resident &resident) {
  auto *registry = packet::field(resident.last.get(), "resident_consumers");
  require(registry && packet::string(packet::field(registry, "schema")) ==
                          "ql.native-resident-consumer-registry/v1",
          "actual native constructor/same-pulse registry absent");
  auto *snapshot = packet::field(
      packet::field(registry, "physical_observation"), "snapshot");
  auto *ui =
      packet::field(packet::field(resident.last.get(), "reading"), "physical");
  require(packet::integer(packet::field(snapshot, "version")) == 1 &&
              packet::integer(packet::field(snapshot, "node_count")) == 12 &&
              wire::decimal(packet::field(snapshot, "samples_elapsed")) ==
                  wire::decimal(packet::field(registry, "sample")),
          "actual original physical snapshot lost its boundary or node count");
  for (const auto *key :
       {"event_ref", "subject_ref", "preparation_ref", "state_ref",
        "source_coordinate", "source_revision", "eigenbasis_identity",
        "source_generation", "body_revision", "samples_elapsed", "sample_rate",
        "pratibimba", "pickup_linear"})
    require(
        json_object_equal(packet::field(snapshot, key), packet::field(ui, key)),
        "registered snapshot differs from original same-pulse physical copy");
  require(
      json_object_equal(packet::field(snapshot, "node_identity"),
                        packet::field(ui, "node_ids")) &&
          json_object_equal(packet::field(snapshot, "visible_positions_metres"),
                            packet::field(ui, "positions_metres")) &&
          json_object_equal(packet::field(snapshot, "mechanical_energy_joules"),
                            packet::field(ui, "energy_joules")),
      "registered snapshot dropped original physical positions/energy");
  return copy(registry);
}
static std::string resident_role_token(J *registry, const char *role) {
  auto *roles = packet::field(registry, "required_consumers");
  const auto count = json_object_array_length(roles);
  require(count == 2 || count == 3, "native role set is not bounded");
  J *found = nullptr;
  for (std::size_t i = 0; i < count; ++i) {
    auto *item = json_object_array_get_idx(roles, i);
    require(packet::string(packet::field(item, "generation_domain")) ==
                    "native-resident-construction" &&
                wire::decimal(packet::field(item, "generation")) != 0,
            "resident construction lifetime became source-generation alias");
    if (packet::string(packet::field(item, "role")) == role) {
      require(!found, "duplicate actual consumer role");
      found = item;
    }
  }
  if (!found)
    return {};
  const auto token = packet::string(packet::field(found, "instance_ref"));
  require(token.rfind("native-resident:v1:", 0) == 0,
          "source/state reference masquerades as native lifetime");
  return token;
}
static Json actual_resident_timing(Resident &resident, const char *moment,
                                   std::uint64_t ordinal) {
  auto request = resident.command("timing");
  wire::text(request.get(), "moment", moment);
  wire::u64(request.get(), "ordinal", ordinal);
  resident.apply(request.get());
  auto *fact = packet::field(packet::field(resident.last.get(), "payload"),
                             "timing_fact");
  require(packet::string(packet::field(fact, "schema")) ==
              "ql.native-performance-timing-fact/v1",
          "actual native timing producer absent");
  J *mapping = nullptr;
  require(json_object_object_get_ex(packet::field(fact, "binding"),
                                    "time_mapping_ref", &mapping) &&
              !mapping,
          "native timing fabricated an authored mapping");
  return copy(fact);
}
static Json actual_resident_registry(J *fixture) {
  auto first = std::make_unique<Resident>(fixture);
  auto initial = resident_registry_copy(*first);
  J *absent_receiving = nullptr;
  require(json_object_object_get_ex(initial.get(), "receiving_observation",
                                    &absent_receiving) &&
              !absent_receiving,
          "uninstalled receiver must retain an explicit native null");
  auto boundary_timing = actual_resident_timing(*first, "boundary", 0);
  J *unapplied = nullptr;
  require(
      json_object_object_get_ex(boundary_timing.get(), "applied_cursor",
                                &unapplied) &&
          !unapplied &&
          !packet::boolean(packet::field(boundary_timing.get(), "queued")) &&
          wire::decimal(
              packet::field(boundary_timing.get(), "committed_cursor")) == 0,
      "native prepared boundary became an applied event");
  const auto audio = resident_role_token(initial.get(), "audio_engine"),
             body = resident_role_token(initial.get(), "physical_body");
  require(
      audio != body &&
          resident_role_token(initial.get(), "acoustic_receiving").empty() &&
          json_object_array_length(
              packet::field(initial.get(), "native_nodes")) == 12 &&
          !packet::boolean(
              packet::field(packet::field(initial.get(), "audio_observation"),
                            "callback_output_committed")),
      "prepared native state fabricated receiving or played output");
  auto second = std::make_unique<Resident>(fixture);
  auto other = resident_registry_copy(*second);
  require(resident_role_token(other.get(), "audio_engine") != audio &&
              resident_role_token(other.get(), "physical_body") != body &&
              json_object_equal(packet::field(initial.get(), "source"),
                                packet::field(other.get(), "source")) &&
              json_object_equal(packet::field(initial.get(), "state_ref"),
                                packet::field(other.get(), "state_ref")),
          "two real same-definition residents collapsed native lifetimes");
  auto *input =
      packet::field(packet::field(packet::field(fixture, "native_preparation"),
                                  "physical_body"),
                    "request");
  auto *native_nodes = packet::field(packet::field(input, "geometry"), "nodes");
  auto *observed_nodes = packet::field(initial.get(), "native_nodes");
  auto *full = packet::field(
      packet::field(initial.get(), "physical_observation"), "snapshot");
  auto *prepared = packet::field(packet::field(fixture, "native_preparation"),
                                 "physical_body");
  for (const auto *key :
       {"event_ref", "subject_ref", "source_coordinate", "source_revision"})
    require(json_object_equal(packet::field(full, key),
                              packet::field(prepared, key)),
            "full snapshot lost native source provenance");
  for (const auto *key : {"preparation_ref", "state_ref"})
    require(
        json_object_equal(packet::field(full, key), packet::field(input, key)),
        "full snapshot lost native preparation/state provenance");
  require(
      wire::decimal(packet::field(full, "source_generation")) ==
              packet::integer(packet::field(prepared, "source_generation")) &&
          wire::decimal(packet::field(full, "body_revision")) ==
              packet::integer(packet::field(input, "body_revision")),
      "full snapshot changed independent source/body generations");
  for (const auto *role : {"geometry", "material"}) {
    auto *provenance = packet::field(packet::field(input, role), "provenance");
    const std::string ref = std::string(role) + "_ref";
    const std::string rev = std::string(role) + "_revision";
    require(json_object_equal(packet::field(full, ref.c_str()),
                              packet::field(provenance, "reference")) &&
                json_object_equal(packet::field(full, rev.c_str()),
                                  packet::field(provenance, "revision")),
            "full snapshot lost exact geometry/material source revision");
  }
  auto *rest = packet::field(full, "rest_positions_metres");
  packet::array(rest, 12);
  for (std::size_t n = 0; n < 12; ++n)
    require(json_object_equal(
                json_object_array_get_idx(rest, n),
                packet::field(json_object_array_get_idx(native_nodes, n),
                              "rest_metres")),
            "full snapshot rest source was filled or reordered");
  packet::array(native_nodes, 12);
  for (std::size_t i = 0; i < 12; ++i) {
    auto *node = json_object_array_get_idx(native_nodes, i);
    auto *observed = json_object_array_get_idx(observed_nodes, i);
    require(wire::decimal(packet::field(observed, "native_node_id")) ==
                packet::integer(packet::field(node, "identity")),
            "registered node correspondence was invented or reordered");
    for (std::size_t axis = 0; axis < 3; ++axis) {
      const auto actual = packet::number(json_object_array_get_idx(
          packet::field(observed, "rest_metres"), axis));
      const auto expected = packet::number(
          json_object_array_get_idx(packet::field(node, "rest_metres"), axis));
      require(std::memcmp(&actual, &expected, sizeof(double)) == 0,
              "native node rest correspondence lost exact metric source");
    }
  }
  auto pristine = first->checkpoint();
  auto injected = first->command("inspect");
  wire::put(injected.get(), "resident_consumers", json_object_get(other.get()));
  control_refusal_unchanged(*first, injected.get(), pristine.get());
  auto *original = packet::field(packet::field(fixture, "acoustic"), "packet");
  auto *after_packet =
      packet::field(packet::field(fixture, "after_acoustic"), "packet");
  install(*first, original);
  auto installed = resident_registry_copy(*first);
  const auto receiving =
      resident_role_token(installed.get(), "acoustic_receiving");
  require(!receiving.empty() && receiving != audio && receiving != body &&
              resident_role_token(installed.get(), "audio_engine") == audio &&
              resident_role_token(installed.get(), "physical_body") == body,
          "actual receiving installation aliased owner/source references");
  first->attack();
  auto queued = resident_registry_copy(*first);
  auto score_timing = actual_resident_timing(*first, "score", 1);
  require(json_object_object_get_ex(score_timing.get(), "applied_cursor",
                                    &unapplied) &&
              !unapplied &&
              packet::boolean(packet::field(score_timing.get(), "queued")) &&
              wire::decimal(
                  packet::field(score_timing.get(), "requested_cursor")) == 0 &&
              wire::decimal(
                  packet::field(score_timing.get(), "admitted_cursor")) == 0,
          "actual native score queue lost null application standing");
  require(!packet::boolean(
              packet::field(packet::field(queued.get(), "audio_observation"),
                            "callback_output_committed")) &&
              wire::decimal(packet::field(
                  packet::field(queued.get(), "audio_observation"),
                  "last_applied_application_ordinal")) == 0 &&
              json_object_array_length(first->applications.get()) == 0,
          "genuine queue admission became a played receiving fact");
  (void)control_advance_partition(*first, 4096, 128);
  auto played = resident_registry_copy(*first);
  require(wire::decimal(packet::field(played.get(), "sample")) == 4096 &&
              packet::boolean(packet::field(
                  packet::field(played.get(), "audio_observation"),
                  "callback_output_committed")) &&
              wire::decimal(packet::field(
                  packet::field(played.get(), "audio_observation"),
                  "last_applied_application_ordinal")) == 1 &&
              json_object_array_length(first->applications.get()) == 1,
          "real A/P callback failed independent output/application custody");
  for (const char *key :
       {"audio_observation", "physical_observation", "receiving_observation"})
    require(wire::decimal(packet::field(packet::field(played.get(), key),
                                        "sample")) == 4096,
            "independent consumer observation has detached output cursor");
  auto applied_timing = actual_resident_timing(*first, "applied", 1);
  require(wire::decimal(
              packet::field(applied_timing.get(), "applied_cursor")) == 0 &&
              wire::decimal(packet::field(applied_timing.get(),
                                          "committed_cursor")) == 4096,
          "actual native callback application lost its original sample");
  auto saved = first->checkpoint();
  (void)control_commit_receiver(*first, original, after_packet, 4096,
                                saved.get());
  auto replaced = resident_registry_copy(*first);
  const auto next_receiving =
      resident_role_token(replaced.get(), "acoustic_receiving");
  require(next_receiving != receiving && next_receiving != audio &&
              next_receiving != body &&
              resident_role_token(replaced.get(), "audio_engine") == audio &&
              resident_role_token(replaced.get(), "physical_body") == body &&
              wire::decimal(packet::field(replaced.get(), "sample")) == 4096 &&
              !packet::boolean(packet::field(
                  packet::field(replaced.get(), "audio_observation"),
                  "callback_output_committed")),
          "receiver replacement retired wrong owner or fabricated output");
  (void)control_advance_partition(*first, 4608, 512);
  auto continued = resident_registry_copy(*first);
  require(resident_role_token(continued.get(), "acoustic_receiving") ==
                  next_receiving &&
              wire::decimal(packet::field(
                  packet::field(continued.get(), "receiving_observation"),
                  "sample")) == 4608 &&
              packet::boolean(packet::field(
                  packet::field(continued.get(), "audio_observation"),
                  "callback_output_committed")),
          "retired receiver supplied current native observation");
  // Genuine existing paired Management/P/audio checkpoint restore. A
  // numerical validation candidate is not the installed body registration.
  auto paired = first->checkpoint();
  auto before_restore_pcm = control_advance_partition(*first, 5632, 128);
  auto restore = first->command("restore");
  wire::put(restore.get(), "checkpoint", json_object_get(paired.get()));
  wire::u64(restore.get(), "expected_cursor", 5632);
  wire::text(restore.get(), "transaction_ref",
             "native:resident/paired-restore");
  wire::text(restore.get(), "checkpoint_ref",
             "native:resident/paired-saved4608");
  first->apply(restore.get());
  auto restored = resident_registry_copy(*first);
  require(resident_role_token(restored.get(), "audio_engine") == audio &&
              resident_role_token(restored.get(), "physical_body") == body &&
              resident_role_token(restored.get(), "acoustic_receiving") ==
                  next_receiving &&
              wire::decimal(packet::field(restored.get(), "sample")) == 4608 &&
              !packet::boolean(packet::field(
                  packet::field(restored.get(), "audio_observation"),
                  "callback_output_committed")),
          "paired numerical restore replaced an actual resident lifetime");
  auto after_restore_pcm = control_advance_partition(*first, 5632, 512);
  require(before_restore_pcm.size() == after_restore_pcm.size() &&
              std::memcmp(before_restore_pcm.data(), after_restore_pcm.data(),
                          before_restore_pcm.size() * sizeof(float)) == 0,
          "same-resident paired restore lost exact A/P/receiving continuation");
  auto replayed = resident_registry_copy(*first);
  require(resident_role_token(replayed.get(), "physical_body") == body &&
              wire::decimal(packet::field(replayed.get(), "sample")) == 5632 &&
              packet::boolean(packet::field(
                  packet::field(replayed.get(), "audio_observation"),
                  "callback_output_committed")),
          "temporary checkpoint candidate supplied resident observation");
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-resident-registry-component/v1");
  wire::put(out.get(), "initial", initial.release());
  wire::put(out.get(), "boundary_timing", boundary_timing.release());
  wire::put(out.get(), "score_timing", score_timing.release());
  wire::put(out.get(), "applied_timing", applied_timing.release());
  wire::put(out.get(), "other_same_source", other.release());
  wire::put(out.get(), "installed", installed.release());
  wire::put(out.get(), "queued", queued.release());
  wire::put(out.get(), "played", played.release());
  wire::put(out.get(), "replaced", replaced.release());
  wire::put(out.get(), "continued", continued.release());
  wire::put(out.get(), "restored", restored.release());
  wire::put(out.get(), "replayed", replayed.release());
  wire::u64(out.get(), "exact_restore_continuation_frames", 1024);
  wire::put(out.get(), "applications",
            json_object_get(first->applications.get()));
  wire::put(out.get(), "input_history",
            json_object_get(first->input_history.get()));
  auto sizes = wire::object();
  for (const auto &entry :
       {std::pair<const char *, std::size_t>{"Engine", sizeof(Engine)},
        {"Readback", sizeof(Readback)},
        {"PhysicalSnapshot", sizeof(ql::PhysicalSnapshot)},
        {"NativeResidentRegistry", sizeof(NativeResidentRegistry)},
        {"NativeResidentToken", sizeof(ql::NativeResidentToken)}})
    wire::put(sizes.get(), entry.first, json_object_new_uint64(entry.second));
  wire::put(out.get(), "resource_sizes_bytes", sizes.release());
  return out;
}
