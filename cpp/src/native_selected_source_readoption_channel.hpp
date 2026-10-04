#ifndef QL_NATIVE_SELECTED_SOURCE_READOPTION_CHANNEL_HPP
#define QL_NATIVE_SELECTED_SOURCE_READOPTION_CHANNEL_HPP
#include <cstring>
#include <ql/performance_management_wire.hpp>
#include <ql/performance_source_readoption.hpp>
#include <ql/physical_scene_contact_replay.hpp>
#include <string_view>

namespace ql::performance::selected_source_transport {
namespace mt = management_transport;
namespace wire = checkpoint_transport;
using J = json_object;
using Json = wire::Json;
inline constexpr const char *request_schema =
    "ql.native-selected-source-readoption-request/v1";

inline bool exact(J *a, J *b) {
  if (!a || !b)
    return a == b;
  const auto type = json_object_get_type(a);
  if (type != json_object_get_type(b))
    return false;
  if (type == json_type_array) {
    if (json_object_array_length(a) != json_object_array_length(b))
      return false;
    for (std::size_t i = 0; i < json_object_array_length(a); ++i)
      if (!exact(json_object_array_get_idx(a, i),
                 json_object_array_get_idx(b, i)))
        return false;
    return true;
  }
  if (type == json_type_object) {
    if (json_object_object_length(a) != json_object_object_length(b))
      return false;
    json_object_object_foreach(a, key, value) {
      J *other = nullptr;
      if (!json_object_object_get_ex(b, key, &other) || !exact(value, other))
        return false;
    }
    return true;
  }
  return scene_contact_transport::exact(a, b);
}
inline Json parse_checkpoint(const std::string &text) {
  require(!text.empty() && text.size() <= 32 * 1024 * 1024,
          "original selected-source checkpoint exceeds native envelope");
  auto parser = std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(
      json_tokener_new_ex(64), json_tokener_free);
  require(bool(parser), "selected-source checkpoint parser unavailable");
  json_tokener_set_flags(parser.get(),
                         JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  auto value = wire::own(
      json_tokener_parse_ex(parser.get(), text.data(), int(text.size())));
  require(value &&
              json_tokener_get_error(parser.get()) == json_tokener_success &&
              json_tokener_get_parse_end(parser.get()) == text.size(),
          "selected-source complete original checkpoint is malformed");
  return value;
}

// Borrowed JSON counting: no whole-value stringify/deep-copy precedes a bound.
// This private worker uses ONLY json-c's standard parser/default native codecs;
// parsed doubles' documented new_double_s userdata holds their original token.
// It is read unchanged here, never replaced with a shorter canonical number.
class Budget {
  std::size_t limit_, bytes_ = 0;

public:
  explicit Budget(std::size_t limit) : limit_(limit) {}
  void add(std::size_t bytes) {
    require(bytes <= limit_ - bytes_,
            "selected-source complete retained byte bound exceeded");
    bytes_ += bytes;
  }
  std::size_t bytes() const noexcept { return bytes_; }
  void string(std::string_view value) {
    add(2);
    for (unsigned char byte : value) {
      if (byte == '"' || byte == '\\' || byte == '/' || byte == '\b' ||
          byte == '\f' || byte == '\n' || byte == '\r' || byte == '\t')
        add(2);
      else
        add(byte < 32 ? 6 : 1);
    }
  }
  void value(J *input, unsigned depth = 0) {
    require(depth <= 64, "selected-source retained JSON depth exceeded");
    if (!input) {
      add(4);
      return;
    }
    switch (json_object_get_type(input)) {
    case json_type_null:
      add(4);
      return;
    case json_type_boolean:
      add(5);
      return;
    case json_type_int:
      add(21);
      return;
    case json_type_double: {
      require(std::isfinite(json_object_get_double(input)),
              "nonfinite selected-source retained operand");
      if (const auto *original =
              static_cast<const char *>(json_object_get_userdata(input))) {
        // json_tokener/default new_double_s is the only input producer here.
        // Bound the borrowed token BEFORE json-c can allocate an output buffer.
        const auto size = strnlen(original, limit_ + 1);
        add(size);
      } else
        add(128); // bounded default finite f64 formatting, incl signed zero
      return;
    }
    case json_type_string:
      string({json_object_get_string(input),
              std::size_t(json_object_get_string_len(input))});
      return;
    case json_type_array:
      add(2);
      for (std::size_t i = 0; i < json_object_array_length(input); ++i) {
        if (i)
          add(1);
        value(json_object_array_get_idx(input, i), depth + 1);
      }
      return;
    case json_type_object:
      add(2);
      {
        bool first = true;
        json_object_object_foreach(input, key, child) {
          if (!first)
            add(1);
          first = false;
          string(key);
          add(1);
          value(child, depth + 1);
        }
      }
      return;
    }
    throw std::invalid_argument(
        "unsupported selected-source retained JSON type");
  }
};
inline std::size_t charged(J *value, std::size_t limit) {
  Budget budget(limit);
  budget.value(value);
  return budget.bytes();
}
inline std::size_t charged_string(std::string_view value, std::size_t limit) {
  Budget budget(limit);
  budget.string(value);
  return budget.bytes();
}
inline constexpr std::size_t message_limit = 32 * 1024 * 1024,
                             source_limit = 8 * 1024 * 1024,
                             sidecar_limit = 4 * 1024 * 1024;

// The existing worker alone owns this private channel. Root's genuine closed
// Act/source caller retains the actual lease; this numerical reader cannot
// construct it. Control::execute rejects this schema and operation.
class Channel {
  std::unique_ptr<NativeStoppedSourceReadoption::Retained> retained_;
  Json original_request_ = wire::own(nullptr),
       original_reply_ = wire::own(nullptr);
  std::unique_ptr<ManagementPulse> original_pulse_;
  bool last_committed_ = false, failed_ = false, held_ = false;

  void hold_once(mt::Control &control) noexcept {
    if (last_committed_ && !held_ && control.active()) {
      control.hold();
      held_ = true;
    }
  }
  std::size_t normal_pulse_bound(const ManagementCheckpoint &saved,
                                 const std::vector<KeyboardCell> &catalog,
                                 const mt::Control &control) {
    // Actual original saved application/input feedback, without a new owner,
    // pulse or numerical clock. Counting codecs cannot admit a performed event.
    Budget normal(sidecar_limit);
    NativeInputBindings bindings;
    bindings.restore_validated(saved.bindings);
    const auto &audio = saved.native_pair.audio;
    for (auto i = audio.applications.read; i < audio.applications.write; ++i) {
      const auto &application = audio.applications.storage[i % 256];
      if (application.applied_application_ordinal >
          audio.applied_application_ordinal)
        break;
      normal.value(wire::application(application).get());
      require(bindings.application(application),
              "selected-source original application feedback exceeds native "
              "journal");
    }
    Readback retirement{};
    retirement.last_sequence = audio.applied_sequence;
    for (std::size_t i = 0; i < audio.touches.size(); ++i)
      retirement.held_touch_tokens[i] = audio.touches[i].token;
    require(bindings.retire_absent(retirement),
            "selected-source original retirement journal unavailable");
    InputBindingRecord row{};
    while (bindings.pop_history(row))
      normal.value(mt::history(row).get());
    for (const auto &cell : catalog)
      normal.value(mt::key(cell).get());
    require(control.devices_.size() <= 64,
            "selected-source native device catalog exceeds original bound");
    normal.add(2);
    for (const auto &device : control.devices_) {
      // Charge borrowed dynamic native strings before a serializer copies them.
      normal.string(device.uid);
      normal.string(device.name);
      normal.add(1024); // complete fixed device fields and punctuation
    }
    const auto actual_device = control.owner_->device_receipt();
    normal.string(actual_device.backend);
    normal.string(actual_device.error);
    // Reading+three resident registrations+physical/receiving observations have
    // fixed native Ref/array layouts.16 bytes per native byte bounds escaped
    // refs, finite numeric text, field names/punctuation; device/catalog and
    // all unbounded authored strings are charged individually above. This is an
    // envelope reservation, never an observation or accepted/native source.
    normal.add(16 * (sizeof(Readback) + sizeof(NativeResidentRegistration) * 3 +
                     sizeof(DeviceReceipt) + sizeof(TransportAcknowledgement)) +
               65536);
    // The authentic BEFORE came AFTER the original stopped checkpoint pulse:
    // its same-pulse capture codec drained BOTH bounded capture queues through
    // current cursor. No callback can add a block while the device is stopped;
    // restore also clears Engine observer queues. Original blocks remain in
    // that retained BEFORE reply, never silently appended to a new source
    // epoch.
    return normal.bytes();
  }

public:
  bool last_source_committed() const noexcept { return last_committed_; }
  void protect_after_exception(mt::Control &control) noexcept {
    if (last_committed_)
      failed_ = true;
    hold_once(control);
  }
  // Original evidence remains owned even if reply serialization/outer delivery
  // fails. This error path never obtains a second pulse or rebuilds feedback.
  Json retained_failure() const {
    auto out = wire::object();
    wire::text(out.get(), "schema",
               "ql.native-selected-source-readoption-failure/v1");
    wire::flag(out.get(), "source_committed", last_committed_);
    wire::put(out.get(), "original_request",
              json_object_get(original_request_.get()));
    wire::put(out.get(), "original_reply",
              json_object_get(original_reply_.get()));
    if (last_committed_ && retained_ && !original_reply_) {
      for (const auto &entry :
           {std::pair{"before_checkpoint", &retained_->before_checkpoint()},
            std::pair{"original_checkpoint", &retained_->original_checkpoint()},
            std::pair{"pre_pulse_checkpoint",
                      &retained_->pre_pulse_checkpoint()},
            std::pair{"operative_checkpoint",
                      &retained_->operative_checkpoint()}})
        wire::put(
            out.get(), entry.first,
            management_checkpoint_transport::checkpoint_wire(*entry.second)
                .release());
    }
    if (last_committed_ && original_pulse_ && !original_reply_) {
      auto applications = wire::array();
      for (const auto &row : original_pulse_->applications)
        wire::append(applications.get(), wire::application(row).release());
      wire::put(out.get(), "applications", applications.release());
      mt::put_input_history(out.get(), *original_pulse_);
    }
    return out;
  }
  Json execute(mt::Control &control, J *request) {
    require(!failed_,
            "selected-source committed failure retained; native owner held");
    last_committed_ = false;
    original_request_ = wire::own(json_object_get(request));
    original_reply_.reset();
    original_pulse_.reset();
    const auto request_charge = charged(request, message_limit);
    for (const auto &group :
         {std::array{"before_source_packet", "before_native_basis"},
          std::array{"current_source_packet", "actual_native_basis"}}) {
      Budget source(source_limit);
      for (const auto *key : group)
        source.value(packet::field(request, key));
    }
    for (const auto *key : {"current_receiving", "receiving_admission",
                            "current_receiving_admission"})
      charged(packet::field(request, key), sidecar_limit);
    J *saved_acoustic = nullptr;
    const bool has_acoustic = json_object_object_get_ex(
        request, "original_saved_acoustic", &saved_acoustic);
    std::set<std::string> keys = {"schema",
                                  "session_ref",
                                  "transport_epoch",
                                  "expected_cursor",
                                  "expected_accepted_sequence",
                                  "transaction_ref",
                                  "checkpoint_ref",
                                  "original_before_checkpoint_wire",
                                  "original_checkpoint_wire",
                                  "before_source_packet",
                                  "before_native_basis",
                                  "packet",
                                  "actual_native_basis",
                                  "m1_pratibimba",
                                  "physical_pratibimba",
                                  "body_source",
                                  "current_source_packet",
                                  "receiving_admission",
                                  "current_receiving_admission",
                                  "current_receiving",
                                  "native_catalog"};
    if (has_acoustic)
      keys.insert("original_saved_acoustic");
    require(json_object_is_type(request, json_type_object) &&
                json_object_object_length(request) == int(keys.size()),
            "private selected-source original request shape differs");
    json_object_object_foreach(request, key, value) {
      (void)value;
      require(keys.count(key) != 0, "foreign selected-source operand");
    }
    require(packet::string(packet::field(request, "schema")) ==
                    request_schema &&
                control.owner_ && !control.released_ &&
                packet::ref(request, "session_ref") ==
                    control.owner_->session_ref(),
            "actual retained selected-source owner absent");
    require(
        control.prepared_source_ && control.prepared_basis_ &&
            exact(control.prepared_source_.get(),
                  packet::field(request, "before_source_packet")) &&
            exact(control.prepared_basis_.get(),
                  packet::field(request, "before_native_basis")),
        "selected-source request detached its full original resident producer");
    const auto epoch = wire::decimal(packet::field(request, "transport_epoch"));
    const auto cursor =
        wire::decimal(packet::field(request, "expected_cursor"));
    const auto sequence =
        wire::decimal(packet::field(request, "expected_accepted_sequence"));
    require(epoch == control.owner_->transport_epoch(),
            "selected-source original transport epoch changed");
    // Original escaped wire strings are charged while still borrowed, before
    // checkpoint_text makes its complete native parsing copies.
    charged(packet::field(request, "original_before_checkpoint_wire"),
            sidecar_limit);
    charged(packet::field(request, "original_checkpoint_wire"), sidecar_limit);
    const auto before_text = mt::checkpoint_text(
        packet::field(request, "original_before_checkpoint_wire"));
    const auto saved_text =
        mt::checkpoint_text(packet::field(request, "original_checkpoint_wire"));
    charged_string(before_text, sidecar_limit);
    charged_string(saved_text, sidecar_limit);
    if (has_acoustic)
      charged(saved_acoustic, sidecar_limit);
    auto before_value = parse_checkpoint(before_text),
         saved_value = parse_checkpoint(saved_text);
    auto before = management_checkpoint_transport::read_checkpoint_wire(
        before_value.get());
    auto saved = management_checkpoint_transport::read_checkpoint_wire(
        saved_value.get());
    require(
        has_acoustic == saved->native_pair.audio.has_receiving,
        "selected-source saved receiver requires its original native producer");
    auto *packet_value = packet::field(request, "packet");
    auto *basis = packet::field(request, "actual_native_basis");
    require(
        exact(packet_value, packet::field(request, "current_source_packet")),
        "selected-source prepared packet differs from genuine current "
        "producer");
    const std::string encoded =
        json_object_to_json_string_ext(packet_value, JSON_C_TO_STRING_PLAIN);
    J *sparse = nullptr;
    auto native =
        json_object_object_get_ex(packet_value, "source_key_admission", &sparse)
            ? prepare_source_performance_packet(
                  encoded, packet::field(request, "current_source_packet"),
                  basis,
                  packet::boolean(packet::field(request, "m1_pratibimba")),
                  packet::boolean(
                      packet::field(request, "physical_pratibimba")))
            : prepare_performance_packet(
                  encoded, basis,
                  packet::boolean(packet::field(request, "m1_pratibimba")),
                  packet::boolean(
                      packet::field(request, "physical_pratibimba")));
    const auto immutable = native.body->preparation();
    mt::BodyStanding standing{};
    auto *stamp = packet::field(request, "body_source");
    wire::keys(stamp, {"kind", "recipe_ref", "validated_m3_generation"});
    require(packet::string(packet::field(stamp, "kind")) == "sourceForm" &&
                packet::field(packet_value, "source_form_recipe"),
            "selected-source physical form standing absent");
    standing.source_form = true;
    standing.recipe = packet::ref(stamp, "recipe_ref");
    standing.validated_generation =
        wire::decimal(packet::field(stamp, "validated_m3_generation"));
    require(standing.validated_generation ==
                immutable.input().source_generation,
            "selected-source original physical generation differs");
    const auto saved_cursor = saved->native_pair.audio.cursor;
    auto *current = packet::field(request, "current_receiving_admission");
    auto *candidate = packet::field(request, "receiving_admission");
    auto *complete = packet::field(request, "current_receiving");
    require(exact(packet::field(complete, "native_admission"), current),
            "selected-source full N9 admission differs");
    auto selected_body = native.body;
    std::shared_ptr<PhysicalRoutesPortBinding> routes;
    std::unique_ptr<NativeRouteProgramSet> seed;
    auto selected_port = physical_port(selected_body);
    if (saved->native_pair.audio.has_route_programs) {
      auto *sources =
          packet::field(packet::field(current, "operation"), "sources");
      require(json_object_is_type(sources, json_type_array) &&
                  json_object_array_length(sources) <= 9 &&
                  !native.notes.empty(),
              "selected-source complete native route/phase producer absent");
      std::vector<std::string> refs;
      for (std::size_t i = 0; i < json_object_array_length(sources); ++i)
        refs.push_back(
            packet::string(packet::field(json_object_array_get_idx(sources, i),
                                         "driver_ref")) +
            "/m1-excitation-program");
      auto admitted = std::make_shared<const AdmittedNativeReceivingSource>(
          read_native_receiving_admission(candidate, current, native, basis,
                                          immutable, refs, saved_cursor));
      routes = std::make_shared<PhysicalRoutesPortBinding>(
          native.body, admitted, immutable, native.determination, basis,
          packet::boolean(packet::field(request, "m1_pratibimba")),
          saved_cursor);
      selected_port = physical_routes_port(selected_port, routes);
      seed = std::make_unique<NativeRouteProgramSet>();
      seed->manifest = routes->manifest();
      seed->program_count = seed->manifest.route_count;
      seed->scalar_note_enabled = seed->manifest.scalar_note_enabled;
      seed->scalar_note_gain = seed->manifest.scalar_note_gain;
      for (std::size_t i = 0; i < seed->program_count; ++i) {
        auto &program = seed->programs[i];
        program.handle = seed->manifest.programs[i];
        program.phase_source_ref = seed->manifest.m1_coordinate;
        program.sine = native.notes.front().phase_sin;
        program.cosine = native.notes.front().phase_cos;
      }
    }
    std::shared_ptr<MovingReceivingPortBinding> receiver_owner;
    ReceivingPort receiver{};
    Json acoustic = wire::own(nullptr);
    if (has_acoustic) {
      const auto origin =
          wire::decimal(packet::field(saved_acoustic, "origin_sample"));
      const auto birth =
          wire::decimal(packet::field(saved_acoustic, "history_origin_sample"));
      auto prepared = acoustic_wire::read_prepared_acoustic(
          saved_acoustic, saved_acoustic,
          packet::field(packet_value, "physical_body"), immutable, origin,
          acoustic_wire::AcousticReadKind::RetainedReceiverSegment);
      receiver_owner = MovingReceivingPortBinding::from_saved_preparation(
          native.body, immutable, saved_cursor, std::move(prepared), birth,
          saved->native_pair.audio.receiving);
      receiver = receiver_owner->port(receiver_owner);
      acoustic = wire::own(json_object_get(saved_acoustic));
    }
    auto *cells = packet::field(request, "native_catalog");
    require(json_object_is_type(cells, json_type_array) &&
                json_object_array_length(cells) <= 1024,
            "selected-source full native catalog bound differs");
    std::vector<KeyboardCell> catalog;
    for (std::size_t i = 0; i < json_object_array_length(cells); ++i)
      catalog.push_back(mt::read_key(json_object_array_get_idx(cells, i)));
    // Charge the complete prospective retained request+reply BEFORE mutation.
    // Preview is a bounded typed numerical copy, never an Act/source grant.
    auto preview = std::make_unique<ManagementCheckpoint>(*saved);
    require(epoch != std::numeric_limits<std::uint64_t>::max(),
            "selected-source epoch exhausted");
    preview->transport_epoch = epoch + 1;
    if (seed) {
      auto &programmes = preview->native_pair.audio.route_programs;
      require(programmes.program_count == seed->program_count,
              "selected-source route count differs before retention");
      programmes.manifest.admitted_cursor = seed->manifest.admitted_cursor;
      programmes.manifest.source_basis_seal = seed->manifest.source_basis_seal;
      for (std::size_t i = 0; i < programmes.program_count; ++i) {
        programmes.programs[i].handle.preparation_seal =
            seed->programs[i].handle.preparation_seal;
        programmes.programs[i].handle.program_seal =
            seed->programs[i].handle.program_seal;
        programmes.manifest.programs[i].preparation_seal =
            seed->programs[i].handle.preparation_seal;
        programmes.manifest.programs[i].program_seal =
            seed->programs[i].handle.program_seal;
      }
    }
    auto preview_wire =
        management_checkpoint_transport::checkpoint_wire(*preview);
    charged(preview_wire.get(), sidecar_limit);
    const std::string preview_text = json_object_to_json_string_ext(
        preview_wire.get(), JSON_C_TO_STRING_PLAIN);
    const auto pre_charge = charged_string(preview_text, sidecar_limit);
    const auto normal_charge = normal_pulse_bound(*saved, catalog, control);
    Budget retained(message_limit);
    retained.add(request_charge);
    retained.add(normal_charge);
    retained.add(charged_string(before_text, sidecar_limit));
    retained.add(charged_string(saved_text, sidecar_limit));
    // Pre-pulse and both complete post-feedback operative/after copies. Native
    // feedback drains journals; bounded release-proof/flags can grow<=64bytes.
    retained.add(pre_charge);
    retained.add(pre_charge + 64);
    retained.add(pre_charge + 64);
    for (J *value : {packet_value, basis, complete,
                     packet::field(packet_value, "physical_body")})
      retained.value(value);
    if (has_acoustic)
      retained.value(saved_acoustic);
    retained.add(
        65536); // exact field names/ACK/descriptor numeric-mode wrappers

    auto held_request = wire::own(json_object_get(request));
    auto source = wire::own(json_object_get(packet_value)),
         source_basis = wire::own(json_object_get(basis));
    std::unique_ptr<NativeStoppedSourceReadoption::Retained> next;
    TransportAcknowledgement ack{};
    const bool restored = NativeStoppedSourceReadoption::restore(
        *control.owner_, std::move(native), *before, *saved, selected_port,
        seed.get(), std::move(catalog), epoch, cursor, sequence,
        packet::ref(request, "transaction_ref"),
        packet::ref(request, "checkpoint_ref"), next, ack,
        has_acoustic ? &receiver : nullptr);
    // NO allocating statement can occur between commit and retained custody.
    original_request_ = std::move(held_request);
    original_reply_.reset();
    original_pulse_.reset();
    if (restored) {
      retained_.swap(next);
      last_committed_ = true;
      held_ = false;
      control.prepared_source_ = std::move(source);
      control.prepared_basis_ = std::move(source_basis);
      control.retained_acoustic_ = std::move(acoustic);
      control.standing_ = standing;
      control.admitted_route_count_ = seed ? seed->program_count : 0;
    }
    try {
      // The sole actual normal feedback is retained before further allocation.
      // Root's Management dependency reserves BOTH vectors before any drain.
      original_pulse_ = control.owner_->pulse();
      control.commit_private_channel_pulse(*original_pulse_);
      bool accepted = restored;
      if (restored && !NativeStoppedSourceReadoption::complete_pulse(
                          *control.owner_, *original_pulse_, *retained_)) {
        accepted = false;
        failed_ = true;
        hold_once(control);
      }
      auto payload = wire::object();
      if (restored) {
        auto acknowledgement = wire::object();
        wire::u64(acknowledgement.get(), "previous_epoch", ack.previous_epoch);
        wire::u64(acknowledgement.get(), "epoch", ack.epoch);
        wire::u64(acknowledgement.get(), "previous_cursor",
                  ack.previous_cursor);
        wire::u64(acknowledgement.get(), "previous_sequence",
                  ack.previous_sequence);
        wire::u64(acknowledgement.get(), "target_sample", ack.target_sample);
        wire::u64(acknowledgement.get(), "accepted_sequence",
                  ack.accepted_sequence);
        wire::ref(acknowledgement.get(), "transaction_ref", ack.transaction);
        wire::ref(acknowledgement.get(), "checkpoint_ref", ack.checkpoint);
        auto original = wire::object();
        wire::text(original.get(), "schema",
                   "ql.native-selected-source-readoption/v1");
        wire::text(original.get(), "before_checkpoint_wire", before_text);
        wire::text(original.get(), "original_checkpoint_wire", saved_text);
        for (const auto &entry :
             {std::pair{"pre_pulse_checkpoint_wire",
                        &retained_->pre_pulse_checkpoint()},
              std::pair{"operative_checkpoint_wire",
                        &retained_->operative_checkpoint()}}) {
          auto cp =
              management_checkpoint_transport::checkpoint_wire(*entry.second);
          wire::text(
              original.get(), entry.first,
              json_object_to_json_string_ext(cp.get(), JSON_C_TO_STRING_PLAIN));
        }
        wire::put(original.get(), "current_source_packet",
                  json_object_get(packet_value));
        wire::put(original.get(), "actual_native_basis",
                  json_object_get(basis));
        wire::put(original.get(), "current_receiving",
                  json_object_get(complete));
        wire::put(original.get(), "transport_ack",
                  json_object_get(acknowledgement.get()));
        if (has_acoustic)
          wire::put(original.get(), "original_saved_acoustic",
                    json_object_get(saved_acoustic));
        wire::put(payload.get(), "transport_ack", acknowledgement.release());
        auto descriptor = wire::object();
        wire::text(descriptor.get(), "schema",
                   "ql.native-physical-descriptor/v1");
        wire::put(
            descriptor.get(), "physical_preparation",
            json_object_get(packet::field(packet_value, "physical_body")));
        const auto &actual_body = control.owner_->native().body->preparation();
        wire::text(descriptor.get(), "eigenbasis_identity",
                   actual_body.eigenbasis_identity());
        auto frequencies = wire::array();
        for (std::size_t i = 0; i < actual_body.mode_count(); ++i)
          wire::append(frequencies.get(),
                       json_object_new_double(actual_body.frequency_hz(i)));
        wire::put(descriptor.get(), "mode_frequencies_hz",
                  frequencies.release());
        wire::u64(descriptor.get(), "native_cursor", saved_cursor);
        wire::put(payload.get(), "body_descriptor", descriptor.release());
        auto after = control.owner_->stopped_checkpoint();
        auto after_wire =
            management_checkpoint_transport::checkpoint_wire(*after);
        wire::text(original.get(), "after_checkpoint_wire",
                   json_object_to_json_string_ext(after_wire.get(),
                                                  JSON_C_TO_STRING_PLAIN));
        wire::put(payload.get(), "source_readoption", original.release());
      }
      auto reply = control.serialize_pulse(
          "selected-source-readoption", accepted,
          accepted ? ""
          : restored
              ? "actual committed source feedback qualification refused; owner "
                "held"
              : "actual selected native source restore preflight refused",
          std::move(payload), *original_pulse_);
      original_reply_ = wire::own(json_object_get(reply.get()));
      Budget actual(message_limit);
      actual.add(request_charge);
      // The original complete request+reply also fits its retained failure
      // wrapper if delivery fails; no evidence needs trimming after commit.
      actual.add(65536);
      actual.value(reply.get());
      return reply;
    } catch (...) {
      if (last_committed_)
        failed_ = true;
      hold_once(control);
      throw;
    }
  }
};
} // namespace ql::performance::selected_source_transport
#endif
