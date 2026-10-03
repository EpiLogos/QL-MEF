// Genuine Rust current source/receiving -> actual P/A/Management continuation.
// This native component test grants no installed Act/lease or consent
// authority.
#include <iostream>
#include <ql/performance_management_wire.hpp>
#include <ql/performance_receiving_restore.hpp>
using namespace ql::performance;
using ql::require;
namespace wire = checkpoint_transport;
using J = json_object;
using Json = wire::Json;

static std::vector<KeyboardCell> catalog(J *fixture) {
  auto *cells = packet::field(fixture, "native_catalog");
  packet::array(cells, 36);
  std::vector<KeyboardCell> result;
  for (std::size_t i = 0; i < 36; ++i)
    result.push_back(
        management_transport::read_key(json_object_array_get_idx(cells, i)));
  return result;
}
static std::unique_ptr<PerformanceManagement> owner(J *fixture) {
  auto *p = packet::field(fixture, "native_preparation");
  auto *d = packet::field(p, "determination");
  auto *coordinate =
      packet::field(packet::field(p, "physical_body"), "source_coordinate");
  auto native = prepare_source_performance_packet(
      json_object_to_json_string_ext(p, JSON_C_TO_STRING_PLAIN), p,
      packet::field(fixture, "native_basis"),
      packet::integer(packet::field(d, "m1_face")) == 1,
      packet::string(packet::field(coordinate, "face")) == "pratibimba");
  auto result = std::make_unique<PerformanceManagement>(
      std::move(native),
      reference("native:receiving-restore/retained-session"));
  result->admit_catalog(catalog(fixture));
  return result;
}
struct Receiving {
  std::shared_ptr<PhysicalRoutesPortBinding> binding;
  std::unique_ptr<NativeRouteProgramSet> seed;
  PhysicalPort port;
};
static Receiving receiving(PerformanceManagement &manager, J *fixture,
                           J *current, std::uint64_t cursor) {
  auto *basis = packet::field(fixture, "native_basis");
  auto *admission = packet::field(current, "native_admission");
  auto *sources =
      packet::field(packet::field(admission, "operation"), "sources");
  std::vector<std::string> refs;
  for (std::size_t i = 0; i < json_object_array_length(sources); ++i)
    refs.push_back(packet::string(packet::field(
                       json_object_array_get_idx(sources, i), "driver_ref")) +
                   "/m1-excitation-program");
  // Body is stopped here. Numerical preparation is immutable; no control
  // query of the live q/v cursor provides the admitted saved cursor.
  const auto immutable = manager.native().body->preparation();
  auto typed = std::make_shared<const AdmittedNativeReceivingSource>(
      read_native_receiving_admission(admission, admission, manager.native(),
                                      basis, immutable, refs, cursor));
  const auto &d = manager.native().determination;
  auto binding = std::make_shared<PhysicalRoutesPortBinding>(
      manager.native().body, typed, immutable, d, basis, d.m1_face == 1,
      cursor);
  auto seed = std::make_unique<NativeRouteProgramSet>();
  seed->manifest = binding->manifest();
  seed->program_count = seed->manifest.route_count;
  seed->scalar_note_enabled = seed->manifest.scalar_note_enabled;
  seed->scalar_note_gain = seed->manifest.scalar_note_gain;
  for (std::size_t i = 0; i < seed->program_count; ++i) {
    auto &program = seed->programs[i];
    program.handle = seed->manifest.programs[i];
    program.phase_source_ref = seed->manifest.m1_coordinate;
    program.sine = manager.native().notes.front().phase_sin;
    program.cosine = manager.native().notes.front().phase_cos;
  }
  auto port =
      physical_routes_port(physical_port(manager.native().body), binding);
  return {std::move(binding), std::move(seed), std::move(port)};
}
static std::vector<float> render(PerformanceManagement &manager,
                                 unsigned frames) {
  std::vector<float> result(frames);
  require(manager.offline_advance(result.data(), frames,
                                  manager.native().engine->samples_elapsed()),
          "actual native restored callback refused");
  auto pulse = manager.pulse();
  require(pulse->has_readback && pulse->reading.available &&
              pulse->reading.samples_elapsed ==
                  pulse->reading.physical.samples_elapsed &&
              pulse->recording.failure == RecordingFailure::None,
          "actual restored callback/readback/recording disconnected");
  return result;
}
static bool same_json(Json a, Json b) {
  return json_object_equal(a.get(), b.get());
}
static Json trial(J *fixture) {
  auto original = owner(fixture);
  auto initial =
      receiving(*original, fixture, packet::field(fixture, "initial"), 0);
  if (initial.seed->program_count) {
    initial.seed->programs[0].enabled = false;
    initial.seed->programs[0].target_gain = .42;
    initial.seed->programs[0].effective_gain = .11;
    initial.seed->programs[1].target_gain = .63;
    initial.seed->programs[1].effective_gain = .29;
  }
  require(original->admit_distinct_receiving(initial.port, *initial.seed, 0)
                  .result == Result::Accepted,
          "actual initial receiving refused");
  Operation attack{};
  attack.kind = Kind::NoteOn;
  attack.identity = original->native().determination.identity;
  attack.sequence = 1;
  attack.note = original->native().notes.front();
  attack.value = .7;
  require(original->enqueue_score_input(
              attack, reference("native:receiving-restore/original-input")) ==
              Result::Accepted,
          "real native note admission refused");
  Operation future{};
  future.kind = Kind::Parameter;
  future.identity = attack.identity;
  future.sequence = 2;
  future.sample = 48000;
  future.parameter = Parameter::MasterLinear;
  future.value = .8;
  require(original->enqueue_score_input(future) == Result::Accepted,
          "real future reservation refused");
  render(*original, 128);
  render(*original, 512);
  auto saved = original->stopped_checkpoint();
  const auto original_wire =
      management_checkpoint_transport::checkpoint_wire(*saved);
  require(saved->native_pair.audio.cursor == 640 &&
              saved->native_pair.audio.accepted_sequence == 2 &&
              saved->native_pair.audio.applied_application_ordinal == 1,
          "real saved source cursor/queue/application differs");
  auto resumed = owner(fixture);
  auto fresh = receiving(*resumed, fixture,
                         packet::field(fixture, "saved_current"), 640);
  auto wrong = receiving(*resumed, fixture,
                         packet::field(fixture, "other_valid_context"), 640);
  auto before = resumed->stopped_checkpoint();
  {
    auto guard = resumed->native().engine->acquire_stopped_custody();
    auto rejected =
        std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
    require(!resumed->preflight_stopped_receiving_restore(
                *saved, wrong.port, *wrong.seed, resumed->native().notes,
                catalog(fixture), guard, 0, *rejected),
            "other valid native context replaced saved original receiving");
  }
  auto after_wrong = resumed->stopped_checkpoint();
  require(
      same_json(management_checkpoint_transport::checkpoint_wire(*before),
                management_checkpoint_transport::checkpoint_wire(*after_wrong)),
      "other valid context refusal changed native state");
  {
    auto guard = resumed->native().engine->acquire_stopped_custody();
    auto corrupted = std::make_unique<ManagementCheckpoint>(*saved);
    // A genuine differently produced receiving definition is a NONderivative
    // identity. Admission seals cannot launder it into the saved source.
    // World has zero routes, so this trial discriminates that path too.
    const auto other_definition = wrong.binding->manifest().definition_ref;
    require(other_definition !=
                saved->native_pair.audio.route_programs.manifest.definition_ref,
            "true distinct native definition missing from refusal trial");
    corrupted->native_pair.audio.route_programs.manifest.definition_ref =
        other_definition;
    auto rejected =
        std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
    require(!resumed->preflight_stopped_receiving_restore(
                *corrupted, fresh.port, *fresh.seed, resumed->native().notes,
                catalog(fixture), guard, 0, *rejected),
            "derivative route requalification accepted a changed native "
            "definition");
    if (saved->native_pair.audio.route_programs.program_count) {
      *corrupted = *saved;
      ++corrupted->native_pair.audio.route_programs.programs[0]
            .handle.share_denominator;
      ++corrupted->native_pair.audio.route_programs.manifest.programs[0]
            .share_denominator;
      require(!resumed->preflight_stopped_receiving_restore(
                  *corrupted, fresh.port, *fresh.seed, resumed->native().notes,
                  catalog(fixture), guard, 0, *rejected),
              "derivative route requalification altered original all-ten "
              "denominator");
    }
  }
  auto after_nonderivative = resumed->stopped_checkpoint();
  require(same_json(management_checkpoint_transport::checkpoint_wire(*before),
                    management_checkpoint_transport::checkpoint_wire(
                        *after_nonderivative)),
          "nonderivative manifest/handle refusal changed native state");
  auto stale_guard =
      std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
  {
    auto guard = resumed->native().engine->acquire_stopped_custody();
    require(resumed->preflight_stopped_receiving_restore(
                *saved, fresh.port, *fresh.seed, resumed->native().notes,
                catalog(fixture), guard, 0, *stale_guard),
            "actual stopped restore preflight refused");
  }
  {
    auto guard = resumed->native().engine->acquire_stopped_custody();
    require(!resumed->receiving_restore_current(*stale_guard, guard),
            "reacquired unchanged guard reused prior private restore token");
  }
  auto stale_catalog =
      std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
  {
    auto guard = resumed->native().engine->acquire_stopped_custody();
    require(resumed->preflight_stopped_receiving_restore(
                *saved, fresh.port, *fresh.seed, resumed->native().notes,
                catalog(fixture), guard, 0, *stale_catalog),
            "real catalog preflight refused");
    auto changed = catalog(fixture);
    changed.front().label += " changed";
    resumed->admit_catalog(std::move(changed));
    require(!resumed->receiving_restore_current(*stale_catalog, guard),
            "same-source catalog mutation left private restore token current");
  }
  resumed->admit_catalog(catalog(fixture));
  std::unique_ptr<PerformanceManagement::PreparedReceivingRestore> candidate;
  TransportAcknowledgement ack{};
  const auto numerical_owner = resumed->native().body.get();
  {
    auto before_physical_refusal = resumed->stopped_checkpoint();
    auto corrupted = std::make_unique<ManagementCheckpoint>(*saved);
    corrupted->native_pair.physical.eigenbasis_identity += "/different";
    require(
        !restore_current_receiving_checkpoint(
            *resumed, *corrupted, fresh.port, *fresh.seed,
            resumed->native().notes, catalog(fixture), 0,
            reference("native:receiving-restore/transaction"),
            reference("native:receiving-restore/saved-checkpoint"), candidate,
            ack),
        "actual paired owner accepted a different saved physical eigenbasis");
    require(
        !candidate && ack.epoch == 0,
        "refused physical checkpoint published a candidate or transport ACK");
    auto after_physical_refusal = resumed->stopped_checkpoint();
    require(same_json(management_checkpoint_transport::checkpoint_wire(
                          *before_physical_refusal),
                      management_checkpoint_transport::checkpoint_wire(
                          *after_physical_refusal)),
            "physical preflight refusal changed actual A/P/Management state");
  }
  require(restore_current_receiving_checkpoint(
              *resumed, *saved, fresh.port, *fresh.seed,
              resumed->native().notes, catalog(fixture), 0,
              reference("native:receiving-restore/transaction"),
              reference("native:receiving-restore/saved-checkpoint"), candidate,
              ack),
          "actual paired owner fresh receiving restore refused");
  require(resumed->native().body.get() == numerical_owner &&
              resumed->native().engine->owns_physical_owner(numerical_owner) &&
              ack.previous_epoch == 1 && ack.epoch == 2 &&
              ack.previous_cursor == 0 && ack.target_sample == 640 &&
              ack.accepted_sequence == 2,
          "native stopped restore changed owner or inferred epoch/cursor");
  auto restored = resumed->stopped_checkpoint();
  require(
      same_json(
          ql::physical_wire::checkpoint_wire(saved->native_pair.physical),
          ql::physical_wire::checkpoint_wire(restored->native_pair.physical)),
      "saved qv/body/source changed during receiving requalification");
  require(
      same_json(management_checkpoint_transport::checkpoint_wire(*saved),
                management_checkpoint_transport::checkpoint_wire(
                    candidate->original_checkpoint())) &&
          json_object_equal(
              original_wire.get(),
              management_checkpoint_transport::checkpoint_wire(*saved).get()),
      "original saved source/checkpoint bytes changed");
  // Compare complete dynamic audio/queues/history after replacing ONLY the
  // declared derivative manifest/handles in a separate test comparison copy.
  auto expected = std::make_unique<ManagementCheckpoint>(*saved);
  expected->transport_epoch = restored->transport_epoch;
  auto &expected_programmes = expected->native_pair.audio.route_programs;
  const auto &actual_programmes = restored->native_pair.audio.route_programs;
  expected_programmes.manifest.admitted_cursor =
      actual_programmes.manifest.admitted_cursor;
  expected_programmes.manifest.source_basis_seal =
      actual_programmes.manifest.source_basis_seal;
  require(saved->native_pair.audio.route_programs.manifest.admitted_cursor ==
                  0 &&
              actual_programmes.manifest.admitted_cursor == 640,
          "native saved-to-operative admission cursor relation absent");
  for (std::size_t i = 0; i < expected_programmes.program_count; ++i) {
    const auto &actual_handle = actual_programmes.programs[i].handle;
    expected_programmes.programs[i].handle.preparation_seal =
        actual_handle.preparation_seal;
    expected_programmes.programs[i].handle.program_seal =
        actual_handle.program_seal;
    expected_programmes.manifest.programs[i].preparation_seal =
        actual_handle.preparation_seal;
    expected_programmes.manifest.programs[i].program_seal =
        actual_handle.program_seal;
  }
  require(
      same_json(management_checkpoint_transport::checkpoint_wire(*expected),
                management_checkpoint_transport::checkpoint_wire(*restored)),
      "saved queues/touch/input/journal/phase/gains were reset or redated");
  for (unsigned block = 0; block < 2; ++block)
    require(render(*original, 512) == render(*resumed, 512),
            "fresh receiving restore lost exact native PCM continuation");
  auto final_original = original->stopped_checkpoint();
  auto final_resumed = resumed->stopped_checkpoint();
  require(same_json(ql::physical_wire::checkpoint_wire(
                        final_original->native_pair.physical),
                    ql::physical_wire::checkpoint_wire(
                        final_resumed->native_pair.physical)),
          "fresh restore changed same causal modal q/v");
  auto out = wire::object();
  for (const char *key :
       {"same_body", "original_checkpoint_unchanged", "exact_pcm_continuation",
        "exact_modal_qv", "original_queues_inputs_preserved",
        "nondefault_programmes_preserved", "valid_other_context_refused",
        "guard_reacquisition_refused", "catalog_replacement_refused",
        "physical_eigenbasis_refused"})
    wire::flag(out.get(), key, true);
  wire::u64(out.get(), "saved_cursor", 640);
  wire::u64(out.get(), "resumed_cursor", 1664);
  wire::put(out.get(), "native_routes",
            json_object_new_uint64(fresh.seed->program_count));
  wire::u64(out.get(), "saved_route_admitted_cursor",
            saved->native_pair.audio.route_programs.manifest.admitted_cursor);
  wire::u64(
      out.get(), "fresh_route_admitted_cursor",
      restored->native_pair.audio.route_programs.manifest.admitted_cursor);
  wire::u64(out.get(), "engine_bytes", sizeof(Engine));
  wire::u64(out.get(), "checkpoint_bytes", sizeof(Engine::Checkpoint));
  wire::u64(out.get(), "manager_checkpoint_bytes",
            sizeof(ManagementCheckpoint));
  wire::u64(out.get(), "engine_token_bytes",
            sizeof(Engine::PreparedReceivingRestore));
  wire::u64(out.get(), "manager_token_bytes",
            sizeof(PerformanceManagement::PreparedReceivingRestore));
  return out;
}
int main() {
  try {
    std::string input;
    char ch;
    while (std::cin.get(ch)) {
      input.push_back(ch);
      require(input.size() < 16 * 1024 * 1024, "native fixture exceeds bound");
    }
    auto root = ql::physical_wire::parse_native(input.c_str());
    require(packet::string(packet::field(root.get(), "schema")) ==
                "ql.receiving-restore-native-fixture/v1",
            "genuine native source fixture required");
    auto out = wire::object();
    wire::text(out.get(), "schema", "ql.receiving-restore-native-receipt/v1");
    J *context_kind = nullptr;
    if (json_object_object_get_ex(root.get(), "context_kind", &context_kind)) {
      // Closed test carrier only. Full actual native source/context validators
      // still execute inside trial; selecting a name grants no native work.
      ql::physical_wire::keys(root.get(),
                              {"schema", "context_kind", "context"});
      const auto kind = packet::string(context_kind);
      require(kind == "world" || kind == "personal" || kind == "shared",
              "unknown named native receiving context carrier");
      auto *fixture = packet::field(root.get(), "context");
      auto *actual_context = packet::field(
          packet::field(packet::field(packet::field(fixture, "initial"),
                                      "native_admission"),
                        "operation"),
          "context");
      require(packet::string(packet::field(actual_context, "kind")) == kind,
              "named context carrier differs from actual native producer");
      wire::put(out.get(), kind.c_str(), trial(fixture).release());
    } else {
      // Preserve the original complete-three fixture when it fits the same cap.
      for (const char *kind : {"world", "personal", "shared"})
        wire::put(out.get(), kind,
                  trial(packet::field(root.get(), kind)).release());
    }
    std::cout << json_object_to_json_string_ext(out.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &failure) {
    std::cerr << failure.what() << '\n';
    return 1;
  }
}
