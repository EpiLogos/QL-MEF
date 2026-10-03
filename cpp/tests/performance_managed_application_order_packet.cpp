#include <array>
#include <cassert>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_management.hpp>
#include <ql/performance_management_wire.hpp>
using namespace ql::performance;
static std::string file(const std::string &path) {
  std::ifstream in(path, std::ios::binary);
  if (!in)
    throw std::invalid_argument("actual producer fixture absent");
  in.seekg(0, std::ios::end);
  const auto size = in.tellg();
  if (size < 1 || size > 4 * 1024 * 1024)
    throw std::invalid_argument("fixture size refused");
  std::string out(std::size_t(size), '\0');
  in.seekg(0);
  if (!in.read(out.data(), size))
    throw std::invalid_argument("fixture read refused");
  return out;
}
static ql::physical_wire::Json parse(const std::string &text) {
  auto *tok = json_tokener_new_ex(64);
  if (!tok)
    throw std::bad_alloc();
  json_tokener_set_flags(tok, JSON_TOKENER_STRICT);
  auto out = ql::physical_wire::own(
      json_tokener_parse_ex(tok, text.data(), int(text.size())));
  const auto error = json_tokener_get_error(tok);
  const auto end = json_tokener_get_parse_end(tok);
  json_tokener_free(tok);
  if (error != json_tokener_success || !out ||
      text.find_first_not_of(" \r\n\t", end) != std::string::npos)
    throw std::invalid_argument("strict fixture JSON refused");
  return out;
}
static Operation op(const Determination &d, Kind kind, std::uint64_t id,
                    std::uint64_t sample) {
  Operation out{};
  out.identity = d.identity;
  out.kind = kind;
  out.sequence = id;
  out.sample = sample;
  return out;
}
static NativePerformance native(const std::string &dir) {
  auto basis = parse(file(dir + "/baseline.basis.json"));
  return prepare_performance_packet(file(dir + "/baseline.packet.json"),
                                    basis.get(), true, true);
}
static auto manager(const std::string &dir) {
  return std::make_unique<PerformanceManagement>(
      native(dir), reference("expression:managed-order/session"));
}
static void write_json(const std::filesystem::path &path, json_object *value) {
  if (std::filesystem::exists(path))
    throw std::invalid_argument("refuse overwriting managed native artifacts");
  std::ofstream out(path, std::ios::binary);
  if (!out)
    throw std::invalid_argument("artifact destination unavailable");
  out << json_object_to_json_string_ext(value, JSON_C_TO_STRING_PLAIN) << '\n';
  if (!out)
    throw std::invalid_argument("artifact write failed");
}
static void append_pulse(const ManagementPulse &pulse,
                         std::vector<NativeGestureApplication> &applications,
                         std::vector<InputBindingRecord> &history) {
  assert(pulse.has_readback);
  for (const auto &application : pulse.applications) {
    assert(application.applied_application_ordinal <=
           pulse.reading.last_applied_application_ordinal);
    assert(application.committed_cursor <= pulse.reading.samples_elapsed);
    applications.push_back(application);
  }
  history.insert(history.end(), pulse.input_history.begin(),
                 pulse.input_history.end());
}
static auto artifacts(const std::vector<NativeGestureApplication> &applications,
                      const std::vector<InputBindingRecord> &history) {
  using namespace checkpoint_transport;
  auto output = object();
  text(output.get(), "schema", "ql.performance-managed-application-history/v1");
  auto actual = array();
  for (const auto &a : applications)
    append(actual.get(), application(a).release());
  put(output.get(), "applications", actual.release());
  auto journal = array();
  for (const auto &h : history) {
    auto entry = object();
    u64(entry.get(), "ordinal", h.ordinal);
    u64(entry.get(), "native_sequence", h.native_sequence);
    put(entry.get(), "change", json_object_new_uint64(unsigned(h.change)));
    put(entry.get(), "operation",
        json_object_new_uint64(unsigned(h.operation)));
    ref(entry.get(), "input_ref", h.input_ref);
    put(entry.get(), "target", note(h.target).release());
    append(journal.get(), entry.release());
  }
  put(output.get(), "input_history", journal.release());
  return output;
}
// A second actual owner reopens the genuine pending snapshot. The original
// un-restored overtaking trial below remains unchanged; no epoch is relabelled.
static void pending_restore_trial(const std::string &dir, bool panic,
                                  const ManagementCheckpoint &saved,
                                  const std::filesystem::path &output_dir) {
  using namespace checkpoint_transport;
  auto owner = manager(dir);
  auto before = owner->stopped_checkpoint();
  assert(before->native_pair.audio.cursor == 0);
  TransportAcknowledgement ack{};
  assert(owner->stopped_restore(
      saved, 0, reference("native:restore/pending-order"),
      reference("native:checkpoint/pending-order"), ack));
  auto after = owner->stopped_checkpoint();
  assert(saved.transport_epoch == 1 && before->transport_epoch == 1 &&
         after->transport_epoch == 2 && ack.previous_epoch == 1 &&
         ack.epoch == 2 && ack.previous_cursor == 0 &&
         ack.target_sample == 128 && ack.accepted_sequence == 3);
  assert(after->native_pair.audio.cursor == 128 &&
         after->native_pair.audio.applied_application_ordinal == 1);
  auto original = management_checkpoint_transport::checkpoint_wire(saved);
  auto preceding = management_checkpoint_transport::checkpoint_wire(*before);
  auto current = management_checkpoint_transport::checkpoint_wire(*after);
  for (const char *key :
       {"native_pair", "inputs", "input_history", "recording_failed",
        "release_pending", "panic_applied", "release_request",
        "release_sequence", "release_proof_cursor"})
    assert(json_object_equal(packet::field(original.get(), key),
                             packet::field(current.get(), key)));
  if (!output_dir.empty()) {
    auto proof = object();
    put(proof.get(), "saved", json_object_get(original.get()));
    put(proof.get(), "before", json_object_get(preceding.get()));
    put(proof.get(), "after", json_object_get(current.get()));
    auto acknowledgement = object();
    u64(acknowledgement.get(), "previous_epoch", ack.previous_epoch);
    u64(acknowledgement.get(), "epoch", ack.epoch);
    u64(acknowledgement.get(), "previous_cursor", ack.previous_cursor);
    u64(acknowledgement.get(), "previous_sequence", ack.previous_sequence);
    u64(acknowledgement.get(), "target_sample", ack.target_sample);
    u64(acknowledgement.get(), "accepted_sequence", ack.accepted_sequence);
    ref(acknowledgement.get(), "transaction_ref", ack.transaction);
    ref(acknowledgement.get(), "checkpoint_ref", ack.checkpoint);
    put(proof.get(), "transport_ack", acknowledgement.release());
    // No observer pulse has run after restore. The checkpoint contains every
    // original unread application/history entry exactly; neither is dropped.
    put(proof.get(), "restored_applications", array().release());
    put(proof.get(), "restored_input_history", array().release());
    const std::string prefix = panic ? "panic" : "release";
    write_json(output_dir / (prefix + ".restore.json"), proof.get());
  }
  std::vector<NativeGestureApplication> applications;
  std::vector<InputBindingRecord> history;
  auto first = owner->pulse();
  append_pulse(*first, applications, history);
  std::array<float, 512> pcm{};
  while (owner->native().engine->samples_elapsed() < 48128) {
    const auto cursor = owner->native().engine->samples_elapsed();
    const auto frames =
        std::size_t(std::min<std::uint64_t>(512, 48128 - cursor));
    assert(owner->offline_advance(pcm.data(), frames, cursor));
    auto pulse = owner->pulse();
    append_pulse(*pulse, applications, history);
    assert(pulse->recording.failure == RecordingFailure::None &&
           pulse->recording.dropped_applications == 0);
  }
  assert(applications.size() == 3);
  assert(applications[0].sequence == 1 && applications[1].sequence == 3 &&
         applications[2].sequence == 2);
  assert(applications[1].has_requested_sample &&
         applications[1].requested_sample == 0 &&
         applications[1].admitted_sample == 128 &&
         applications[1].applied_sample == 128);
  assert(applications[2].requested_sample == 48000 &&
         applications[2].admitted_sample == 48000 &&
         applications[2].applied_sample == 48000);
  for (std::size_t i = 0; i < applications.size(); ++i)
    assert(applications[i].applied &&
           applications[i].applied_application_ordinal == i + 1);
  assert(owner->transport_epoch() == 2 && owner->recording_available());
  if (!output_dir.empty()) {
    const std::string prefix = panic ? "panic" : "release";
    auto records = artifacts(applications, history);
    write_json(output_dir / (prefix + ".restored-history.json"), records.get());
    auto final = owner->stopped_checkpoint();
    assert(final->transport_epoch == 2 &&
           final->native_pair.audio.cursor == 48128 &&
           final->native_pair.audio.applied_application_ordinal == 3);
    auto final_wire = management_checkpoint_transport::checkpoint_wire(*final);
    write_json(output_dir / (prefix + ".restored-checkpoint.json"),
               final_wire.get());
  }
}
static void managed_trial(const std::string &dir, bool panic,
                          const std::filesystem::path &output_dir) {
  auto owner = manager(dir);
  const auto &source = owner->native().determination;
  auto attack = op(source, Kind::NoteOn, 1, 37);
  attack.note = owner->native().notes.at(4);
  attack.value = .8;
  const auto original_input = reference("native-score:original-pointer/42");
  assert(original_input != attack.note.touch_ref);
  const auto attack_admission =
      owner->enqueue_score_input_admission(attack, original_input);
  assert(attack_admission.result() == Result::Accepted &&
         attack_admission.queue().queued());
  assert(attack_admission.queue().operation().requested_sample == 37 &&
         attack_admission.queue().operation().sample == 37 &&
         attack_admission.queue().queue_cursor() == 0 &&
         attack_admission.transport_epoch() == 1 &&
         attack_admission.input_ref() == original_input);
  if (!output_dir.empty()) {
    // Actual original reservation BEFORE ID1 is applied. No C-authored
    // operation/checkpoint is reconstructed from later callback history.
    auto original_queued = owner->stopped_checkpoint();
    assert(original_queued->transport_epoch == 1 &&
           original_queued->native_pair.audio.cursor == 0 &&
           original_queued->native_pair.audio.accepted_sequence == 1 &&
           original_queued->native_pair.audio.applied_application_ordinal == 0);
    const std::string prefix = panic ? "panic" : "release";
    auto original_wire =
        management_checkpoint_transport::checkpoint_wire(*original_queued);
    write_json(output_dir / (prefix + ".original-queued-checkpoint.json"),
               original_wire.get());
    auto admitted_wire =
        management_transport::score_admission(attack_admission);
    write_json(output_dir / (prefix + ".original-score-admission.json"),
               admitted_wire.get());
  }
  auto future = op(source, Kind::Parameter, 2, 48000);
  future.parameter = Parameter::MasterLinear;
  future.value = .2;
  const auto future_admission = owner->enqueue_score_input_admission(future);
  assert(future_admission.result() == Result::Accepted &&
         future_admission.queue().operation().requested_sample == 48000 &&
         future_admission.queue().operation().sample == 48000 &&
         future_admission.queue().operation().sequence == 2);
  std::array<float, 512> pcm{}, reopened_pcm{};
  assert(owner->offline_advance(pcm.data(), 128, 0));
  auto release = op(source, panic ? Kind::Panic : Kind::NoteOff, 3, 0);
  release.touch = attack.note.touch;
  const auto release_admission = owner->enqueue_score_input_admission(
      release, panic ? Ref{} : original_input);
  assert(release_admission.result() == Result::Accepted &&
         release_admission.queue().queued());
  const auto &queued_release = release_admission.queue().operation();
  assert(queued_release.has_requested_sample &&
         queued_release.requested_sample == 0 && queued_release.sample == 128 &&
         queued_release.sequence == 3 && queued_release.late_admitted);
  assert(release_admission.queue().queue_cursor() == 128 &&
         release_admission.queue().queue_horizon() == 128 &&
         release_admission.transport_epoch() == 1);
  assert(release_admission.queue().source().identity == source.identity &&
         release_admission.session_ref() == owner->session_ref());
  auto pending = owner->stopped_checkpoint();
  assert(pending->native_pair.audio.cursor == 128 &&
         pending->native_pair.audio.applied_application_ordinal == 1);
  const auto &release_ring = pending->native_pair.audio.releases;
  assert(release_ring.write == release_ring.read + 1);
  const auto &pending_release = release_ring.storage[release_ring.read % 64];
  assert(
      pending_release.sequence == 3 && pending_release.has_requested_sample &&
      pending_release.requested_sample == 0 && pending_release.sample == 128);
  auto pending_wire =
      management_checkpoint_transport::checkpoint_wire(*pending);
  auto admission_wire =
      management_transport::score_admission(release_admission);
  assert(checkpoint_transport::decimal(
             packet::field(admission_wire.get(), "queue_cursor")) == 128);
  if (!output_dir.empty()) {
    const auto prefix = panic ? "panic" : "release";
    write_json(output_dir / (std::string(prefix) + ".pending-checkpoint.json"),
               pending_wire.get());
    auto receipts = checkpoint_transport::array();
    checkpoint_transport::append(
        receipts.get(),
        management_transport::score_admission(attack_admission).release());
    checkpoint_transport::append(
        receipts.get(),
        management_transport::score_admission(future_admission).release());
    checkpoint_transport::append(receipts.get(), admission_wire.release());
    write_json(output_dir / (std::string(prefix) + ".score-admissions.json"),
               receipts.get());
  }
  pending_restore_trial(dir, panic, *pending, output_dir);
  assert(owner->offline_advance(pcm.data(), 128, 128));
  auto saved = owner->stopped_checkpoint();
  assert(saved->native_pair.audio.applied_application_ordinal == 2);
  assert(saved->native_pair.audio.accepted_sequence == 3);
  assert(saved->native_pair.audio.heap_size == 1);
  assert(saved->bindings.inputs[0].input_ref == original_input);
  auto encoded = management_checkpoint_transport::checkpoint_wire(*saved);
  auto decoded =
      management_checkpoint_transport::read_checkpoint_wire(encoded.get());
  auto reopened = manager(dir);
  TransportAcknowledgement acknowledged{};
  assert(reopened->stopped_restore(
      *decoded, 0, reference("native:restore/order"),
      reference("native:checkpoint/order"), acknowledged));
  assert(acknowledged.epoch == 2 && acknowledged.target_sample == 256);
  std::vector<NativeGestureApplication> applications;
  std::vector<InputBindingRecord> history;
  auto pulse = owner->pulse();
  auto restored_pulse = reopened->pulse();
  assert(pulse->applications.size() == 2 &&
         restored_pulse->applications.size() == 2);
  assert(pulse->reading.samples_elapsed == 256 &&
         pulse->reading.last_applied_application_ordinal == 2);
  assert(pulse->applications[0].sequence == 1 &&
         pulse->applications[1].sequence == 3);
  assert(pulse->applications[1].applied_sample == 128);
  append_pulse(*pulse, applications, history);
  for (std::size_t i = 0; i < history.size(); ++i) {
    assert(history[i].input_ref == original_input);
    assert(history[i].target.touch == attack.note.touch);
  }
  while (owner->native().engine->samples_elapsed() < 48128) {
    auto cursor = owner->native().engine->samples_elapsed();
    auto frames = std::size_t(std::min<std::uint64_t>(512, 48128 - cursor));
    assert(owner->offline_advance(pcm.data(), frames, cursor));
    assert(reopened->offline_advance(reopened_pcm.data(), frames, cursor));
    assert(std::equal(pcm.begin(), pcm.begin() + frames, reopened_pcm.begin()));
    pulse = owner->pulse();
    restored_pulse = reopened->pulse();
    assert(pulse->reading.samples_elapsed ==
           restored_pulse->reading.samples_elapsed);
    assert(pulse->reading.last_applied_application_ordinal ==
           restored_pulse->reading.last_applied_application_ordinal);
    assert(pulse->applications.size() == restored_pulse->applications.size());
    append_pulse(*pulse, applications, history);
  }
  assert(applications.size() == 3);
  assert(applications[2].sequence == 2 &&
         applications[2].applied_sample == 48000);
  for (std::size_t i = 0; i < applications.size(); ++i)
    assert(applications[i].applied &&
           applications[i].applied_application_ordinal == i + 1);
  assert(owner->recording_available() && reopened->recording_available());
  assert(pulse->recording.failure == RecordingFailure::None &&
         pulse->recording.dropped_applications == 0);
  if (!output_dir.empty()) {
    auto prefix = panic ? "panic" : "release";
    write_json(output_dir / (std::string(prefix) + ".checkpoint.json"),
               encoded.get());
    auto records = artifacts(applications, history);
    write_json(output_dir / (std::string(prefix) + ".history.json"),
               records.get());
    auto basis = parse(file(dir + "/baseline.basis.json"));
    write_json(output_dir / (std::string(prefix) + ".basis.json"), basis.get());
    auto final = owner->stopped_checkpoint();
    auto final_wire = management_checkpoint_transport::checkpoint_wire(*final);
    write_json(output_dir /
                   (std::string(prefix) + ".continued-checkpoint.json"),
               final_wire.get());
  }
}
static void readback_cutoff(const std::string &dir) {
  auto owner = manager(dir);
  float pcm = 0;
  for (std::uint64_t i = 0; i < 65; ++i) {
    auto parameter =
        op(owner->native().determination, Kind::Parameter, i + 1, i);
    parameter.parameter = Parameter::MasterLinear;
    parameter.value = i % 2 ? .5 : .6;
    assert(owner->enqueue_score_input(parameter) == Result::Accepted);
    assert(owner->offline_advance(&pcm, 1, i));
  }
  // Callback65 actually commits after its preceding64 readbacks filled the
  // ring. Manager must not attach that newer application to older q/v64.
  auto older = owner->pulse();
  assert(older->reading.samples_elapsed == 64);
  assert(older->reading.physical.samples_elapsed == 64);
  assert(older->reading.last_applied_application_ordinal == 64);
  assert(older->applications.size() == 64);
  assert(older->applications.back().applied_application_ordinal == 64);
  assert(owner->offline_advance(&pcm, 1, 65));
  auto current = owner->pulse();
  assert(current->reading.samples_elapsed == 66 &&
         current->reading.physical.samples_elapsed == 66);
  assert(current->reading.last_applied_application_ordinal == 65);
  assert(current->reading.dropped_readbacks == 1);
  assert(current->applications.size() == 1 &&
         current->applications[0].applied_application_ordinal == 65);
  assert(current->applications[0].committed_cursor == 65);
  assert(current->recording.failure == RecordingFailure::None);
}
static void trailing_application_loss(const std::string &dir) {
  auto owner = manager(dir);
  float pcm = 0;
  for (std::uint64_t i = 0; i < 257; ++i) {
    auto parameter =
        op(owner->native().determination, Kind::Parameter, i + 1, i);
    parameter.parameter = Parameter::MasterLinear;
    parameter.value = i % 2 ? .5 : .6;
    assert(owner->enqueue_score_input(parameter) == Result::Accepted);
    assert(owner->offline_advance(&pcm, 1, i));
  }
  auto first = owner->pulse();
  assert(first->recording.failure ==
         RecordingFailure::ApplicationQueueOverflow);
  assert(first->recording.dropped_applications == 1);
  assert(first->recording.first_failed_sequence == 257 &&
         first->recording.first_failed_sample == 256);
  assert(!owner->recording_available());
  assert(first->applications.size() == 64);
  assert(owner->offline_advance(&pcm, 1, 257));
  auto latest = owner->pulse();
  assert(latest->reading.last_applied_application_ordinal == 257);
  assert(latest->applications.size() == 192);
  assert(latest->applications.back().applied_application_ordinal == 256);
  assert(latest->recording.dropped_applications == 1 &&
         !owner->recording_available());
  // The actual committed highwater257 vs last retained256 proves a trailing
  // missing fact even though all available history is internally contiguous.
}
int main(int argc, char **argv) {
  std::cout << "sizeof(NativeQueueAdmission)=" << sizeof(NativeQueueAdmission)
            << " sizeof(NativeScoreAdmission)=" << sizeof(NativeScoreAdmission)
            << '\n';
  try {
    if (argc < 2 || argc > 3)
      throw std::invalid_argument(
          "usage: performance_managed_application_order_packet-test "
          "ACTUAL_NATIVE_PRODUCER_FIXTURE_DIRECTORY [NEW_ARTIFACT_DIRECTORY]");
    std::filesystem::path output;
    if (argc == 3) {
      output = argv[2];
      if (std::filesystem::exists(output))
        throw std::invalid_argument("native artifact destination must be new");
      if (!std::filesystem::create_directory(output))
        throw std::invalid_argument("native artifact destination refused");
    }
    managed_trial(argv[1], false, output);
    managed_trial(argv[1], true, output);
    readback_cutoff(argv[1]);
    trailing_application_loss(argv[1]);
    std::cout << "real-native-manager overtaking=2 same-body-highwater=65 "
                 "trailing-loss=257 reopen-next47872-exact device=unexecuted\n";
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
