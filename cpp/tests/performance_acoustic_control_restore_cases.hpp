// Actual source-frozen Control worker restoration, separate from private C31
// authority and from the original direct Manager trials. Same native source,
// P/audio owner and queue; no fabricated applications or input history.
static std::vector<double> control_advance_partition(Resident &resident,
                                                     unsigned end,
                                                     unsigned partition) {
  std::vector<double> pcm;
  for (;;) {
    const auto cursor = wire::decimal(packet::field(
        packet::field(resident.last.get(), "reading"), "samples_elapsed"));
    if (cursor == end)
      break;
    require(cursor < end && partition && partition <= 512,
            "finite exact Control continuation partition required");
    auto next = resident.render(unsigned(
        std::min<std::uint64_t>(partition, std::uint64_t(end) - cursor)));
    pcm.insert(pcm.end(), next.begin(), next.end());
  }
  return pcm;
}
static Json control_saved_receiving_request(Resident &resident, J *fixture,
                                            J *saved, J *original_packet,
                                            J *current) {
  auto request = resident.command("restore-current-receiving");
  wire::text(request.get(), "original_checkpoint_wire",
             json_object_to_json_string_ext(saved, JSON_C_TO_STRING_PLAIN));
  wire::u64(request.get(), "expected_cursor", 0);
  wire::text(request.get(), "transaction_ref",
             "native:acoustic/control-fresh-restore");
  wire::text(request.get(), "checkpoint_ref",
             "native:acoustic/control-original-saved");
  wire::put(request.get(), "current_source_packet",
            json_object_get(packet::field(fixture, "native_preparation")));
  wire::put(request.get(), "actual_native_basis",
            json_object_get(packet::field(fixture, "native_basis")));
  auto *admission = packet::field(current, "native_admission");
  wire::put(request.get(), "receiving_admission", json_object_get(admission));
  wire::put(request.get(), "current_receiving_admission",
            json_object_get(admission));
  wire::put(request.get(), "current_receiving", json_object_get(current));
  wire::put(request.get(), "native_catalog",
            json_object_get(packet::field(fixture, "native_catalog")));
  if (original_packet)
    wire::put(request.get(), "original_saved_acoustic",
              json_object_get(original_packet));
  return request;
}
static Json control_text_checkpoint(J *value) {
  require(json_object_is_type(value, json_type_string),
          "complete genuine Control checkpoint text required");
  const std::string original = json_object_get_string(value);
  require(!original.empty() && original.size() <= 32 * 1024 * 1024,
          "complete genuine Control checkpoint text exceeds original bound");
  return ql::physical_wire::parse_native(original.c_str());
}
static Json control_fresh_saved_receiver(J *fixture, unsigned saved_cursor) {
  require(saved_cursor == 4096 || saved_cursor == 8192,
          "closed actual saved receiver trial required");
  auto original = std::make_unique<Resident>(fixture);
  auto *initial = packet::field(packet::field(fixture, "acoustic"), "packet");
  auto *segment =
      packet::field(packet::field(fixture, "after_acoustic"), "packet");
  auto *current = packet::field(fixture, "saved_current_receiving");
  install(*original, initial);
  original->attack();
  (void)control_advance_partition(*original, 4096, 128);
  control_reserve_future(*original);
  auto initial_checkpoint = original->checkpoint();
  (void)control_commit_receiver(*original, initial, segment, 4096,
                                initial_checkpoint.get());
  if (saved_cursor != 4096)
    (void)control_advance_partition(*original, saved_cursor, 128);
  auto saved = original->checkpoint();
  auto native_saved =
      management_checkpoint_transport::read_checkpoint_wire(saved.get());
  require(
      native_saved->native_pair.audio.cursor == saved_cursor &&
          native_saved->native_pair.audio.has_receiving &&
          native_saved->native_pair.audio.receiving.manifest.origin_sample ==
              4096 &&
          native_saved->native_pair.audio.receiving.history_start_sample == 0 &&
          native_saved->native_pair.audio.accepted_sequence == 3,
      "real original Control saved segment/queues/birth differs");
  const auto expected = control_advance_partition(*original, 13000, 128);
  auto original_final = original->checkpoint();
  auto fresh = std::make_unique<Resident>(fixture);
  auto pristine = fresh->checkpoint();
  unsigned refusals = 0;
  for (unsigned variant = 0; variant < 9; ++variant) {
    auto request = control_saved_receiving_request(*fresh, fixture, saved.get(),
                                                   segment, current);
    if (variant == 0)
      json_object_object_del(request.get(), "original_saved_acoustic");
    else if (variant == 1)
      wire::text(request.get(), "unknown_acoustic_grant", "not-authority");
    else if (variant == 2)
      wire::u64(request.get(), "expected_cursor", 1);
    else if (variant == 3)
      wire::u64(request.get(), "expected_transport_epoch", 2);
    else if (variant == 4 || variant == 5) {
      auto altered = copy(segment);
      if (variant == 4)
        wire::text(
            packet::field(packet::field(altered.get(), "context"), "context"),
            "reference", "native:acoustic/different-context");
      else
        wire::u64(altered.get(), "history_origin_sample", 1);
      wire::put(request.get(), "original_saved_acoustic", altered.release());
    } else if (variant == 6) {
      auto altered = copy(saved.get());
      auto *ring = packet::field(
          packet::field(
              packet::field(packet::field(altered.get(), "native_pair"),
                            "audio"),
              "receiving"),
          "history_linear");
      // A finite binary64 one-ULP change that is NOT exactly representable as
      // native f32 must fail the real existing checkpoint decoder. A valid
      // f32 signed-zero edit needs C31's selected-byte authority; no fake
      // numerical grant rejection is asserted here.
      const double sample = packet::number(json_object_array_get_idx(ring, 17));
      const double bad =
          std::nextafter(sample, std::numeric_limits<double>::infinity());
      require(double(float(bad)) != bad,
              "exact-f32 negative did not discriminate native wire");
      json_object_array_put_idx(ring, 17, json_object_new_double(bad));
      wire::text(request.get(), "original_checkpoint_wire",
                 json_object_to_json_string_ext(altered.get(),
                                                JSON_C_TO_STRING_PLAIN));
    } else if (variant == 7) {
      auto altered = copy(segment);
      wire::u64(packet::field(altered.get(), "configuration"), "revision", 3);
      wire::put(request.get(), "original_saved_acoustic", altered.release());
    } else {
      // Genuine no-receiver CP plus an extra original source operand refuses
      // strict discriminant presence, without modifying its original bytes.
      wire::text(request.get(), "original_checkpoint_wire",
                 json_object_to_json_string_ext(pristine.get(),
                                                JSON_C_TO_STRING_PLAIN));
    }
    control_refusal_unchanged(*fresh, request.get(), pristine.get());
    ++refusals;
  }
  auto request = control_saved_receiving_request(*fresh, fixture, saved.get(),
                                                 segment, current);
  fresh->apply(request.get());
  auto receipt = copy(packet::field(packet::field(fresh->last.get(), "payload"),
                                    "receiving_readmission"));
  auto *ack = packet::field(receipt.get(), "transport_ack");
  auto *reading = packet::field(fresh->last.get(), "reading");
  require(
      packet::string(packet::field(receipt.get(), "schema")) ==
              "ql.native-receiving-readmission/v1" &&
          std::string(json_object_get_string(
              packet::field(receipt.get(), "original_checkpoint_wire"))) ==
              json_object_to_json_string_ext(saved.get(),
                                             JSON_C_TO_STRING_PLAIN) &&
          json_object_equal(
              packet::field(receipt.get(), "original_saved_acoustic"),
              segment) &&
          json_object_equal(
              packet::field(receipt.get(), "current_source_packet"),
              packet::field(fixture, "native_preparation")) &&
          json_object_equal(packet::field(receipt.get(), "actual_native_basis"),
                            packet::field(fixture, "native_basis")) &&
          json_object_equal(packet::field(receipt.get(), "current_receiving"),
                            current) &&
          wire::decimal(packet::field(ack, "previous_cursor")) == 0 &&
          wire::decimal(packet::field(ack, "previous_epoch")) == 1 &&
          wire::decimal(packet::field(ack, "epoch")) == 2 &&
          wire::decimal(packet::field(ack, "target_sample")) == saved_cursor &&
          wire::decimal(packet::field(ack, "accepted_sequence")) == 3 &&
          wire::decimal(packet::field(reading, "samples_elapsed")) ==
              saved_cursor &&
          wire::decimal(packet::field(packet::field(reading, "physical"),
                                      "samples_elapsed")) == saved_cursor,
      "actual Control lost original packet/source/full saved restore ACK");
  auto before = control_text_checkpoint(
      packet::field(receipt.get(), "before_checkpoint_wire"));
  auto operative = control_text_checkpoint(
      packet::field(receipt.get(), "operative_checkpoint_wire"));
  auto after = control_text_checkpoint(
      packet::field(receipt.get(), "after_checkpoint_wire"));
  require(json_object_equal(before.get(), pristine.get()),
          "fresh Control changed owner before atomic restore");
  auto native_operative =
      management_checkpoint_transport::read_checkpoint_wire(operative.get());
  auto native_after =
      management_checkpoint_transport::read_checkpoint_wire(after.get());
  require(
      same_receiving_checkpoint(
          native_saved->native_pair.audio.receiving,
          native_operative->native_pair.audio.receiving) &&
          same_receiving_checkpoint(native_saved->native_pair.audio.receiving,
                                    native_after->native_pair.audio.receiving),
      "Control changed complete original receiver/ring/sign/birth");
  auto saved_physical = physical(saved.get()),
       restored_physical = physical(after.get());
  require(json_object_equal(saved_physical.get(), restored_physical.get()),
          "Control fresh restore changed original actual P q/v state");
  const auto actual = control_advance_partition(*fresh, 13000, 512);
  require(
      actual == expected,
      "Control fresh restore lost exact differently partitioned future PCM");
  auto final = fresh->checkpoint();
  auto native_final =
      management_checkpoint_transport::read_checkpoint_wire(final.get());
  auto native_expected = management_checkpoint_transport::read_checkpoint_wire(
      original_final.get());
  auto final_physical = physical(final.get()),
       expected_physical = physical(original_final.get());
  require(json_object_equal(final_physical.get(), expected_physical.get()) &&
              same_receiving_checkpoint(
                  native_final->native_pair.audio.receiving,
                  native_expected->native_pair.audio.receiving) &&
              json_object_array_length(fresh->applications.get()) == 2,
          "Control fresh future continuation lost original body/ring/work");
  for (std::size_t i = 0; i < 2; ++i) {
    auto application = wire::read_application(
        json_object_array_get_idx(fresh->applications.get(), i));
    const auto sample = i == 0 ? 9000 : 10000;
    require(application.applied && application.sequence == i + 2 &&
                application.applied_application_ordinal == i + 2 &&
                application.has_requested_sample &&
                application.requested_sample == std::uint64_t(sample) &&
                application.admitted_sample == std::uint64_t(sample) &&
                application.applied_sample == std::uint64_t(sample),
            "Control fresh receiver lost original future application/timing");
  }
  auto out = wire::object();
  wire::text(out.get(), "schema",
             "ql.native-acoustic-control-fresh-restore/v1");
  wire::text(
      out.get(), "standing",
      "actual numerical Control/P/audio source trial; private C31 origin "
      "and device acceptance remain distinct");
  wire::u64(out.get(), "saved_cursor", saved_cursor);
  wire::u64(out.get(), "original_segment_origin", 4096);
  wire::u64(out.get(), "original_history_birth", 0);
  wire::put(out.get(), "context_kind",
            json_object_get(packet::field(fixture, "context_kind")));
  wire::put(out.get(), "readmission", receipt.release());
  wire::put(
      out.get(), "original_receiving",
      wire::receiving_checkpoint(native_saved->native_pair.audio.receiving)
          .release());
  wire::put(
      out.get(), "final_receiving",
      wire::receiving_checkpoint(native_final->native_pair.audio.receiving)
          .release());
  wire::put(out.get(), "physical", final_physical.release());
  wire::put(out.get(), "applications",
            json_object_get(fresh->applications.get()));
  wire::put(out.get(), "input_history",
            json_object_get(fresh->input_history.get()));
  wire::put(out.get(), "actual_reading",
            json_object_get(packet::field(fresh->last.get(), "reading")));
  auto pcm = wire::array();
  for (double sample : actual)
    wire::append(pcm.get(), json_object_new_double(sample));
  wire::put(out.get(), "continued_pcm", pcm.release());
  wire::put(out.get(), "refusal_count", json_object_new_uint64(refusals));
  wire::u64(out.get(), "end_cursor", 13000);
  return out;
}
