#include <algorithm>
#include <array>
#include <cassert>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_management.hpp>
#include <stdexcept>
#include <string>
#include <vector>
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
static void cancelled(const std::string &dir,
                      const std::filesystem::path &out) {
  auto owner = manager(dir);
  auto initial = owner->stopped_checkpoint();
  assert(initial->native_pair.audio.cursor == 0 &&
         initial->native_pair.audio.applied_application_ordinal == 0);
  auto initial_wire = management_checkpoint_transport::checkpoint_wire(*initial);
  write_json(out / "cancel.origin.json", initial_wire.get());
  auto future = op(owner->native().determination, Kind::Parameter, 1, 1000);
  future.parameter = Parameter::MasterLinear;
  future.value = .2;
  assert(owner->enqueue_score_input(future) == Result::Accepted);
  std::array<float, 128> pcm{};
  assert(owner->offline_advance(pcm.data(), 128, 0));
  auto before = owner->stopped_checkpoint();
  TransportAcknowledgement ack{};
  assert(owner->stopped_restore(*initial, 128,
                                reference("native:reservation/cancel"),
                                reference("native:reservation/initial"), ack));
  assert(ack.previous_epoch == 1 && ack.epoch == 2 &&
         ack.previous_cursor == 128 && ack.previous_sequence == 1 &&
         ack.target_sample == 0 && ack.accepted_sequence == 0);
  auto after = owner->stopped_checkpoint();
  auto b = management_checkpoint_transport::checkpoint_wire(*before);
  auto a = management_checkpoint_transport::checkpoint_wire(*after);
  auto proof = checkpoint_transport::object();
  checkpoint_transport::u64(proof.get(), "previous_epoch", ack.previous_epoch);
  checkpoint_transport::u64(proof.get(), "epoch", ack.epoch);
  checkpoint_transport::u64(proof.get(), "previous_cursor",
                            ack.previous_cursor);
  checkpoint_transport::u64(proof.get(), "previous_sequence",
                            ack.previous_sequence);
  checkpoint_transport::u64(proof.get(), "target_sample", ack.target_sample);
  checkpoint_transport::u64(proof.get(), "accepted_sequence",
                            ack.accepted_sequence);
  checkpoint_transport::ref(proof.get(), "transaction_ref", ack.transaction);
  checkpoint_transport::ref(proof.get(), "checkpoint_ref", ack.checkpoint);
  write_json(out / "cancel.before.json", b.get());
  write_json(out / "cancel.after.json", a.get());
  write_json(out / "cancel.ack.json", proof.get());
}
static void lost(const std::string &dir, const std::filesystem::path &out) {
  auto owner = manager(dir);
  auto initial = owner->stopped_checkpoint();
  assert(initial->native_pair.audio.cursor == 0 &&
         initial->native_pair.audio.applied_application_ordinal == 0);
  auto initial_wire = management_checkpoint_transport::checkpoint_wire(*initial);
  write_json(out / "lost.origin.json", initial_wire.get());
  auto future = op(owner->native().determination, Kind::Parameter, 1, 1000);
  future.parameter = Parameter::MasterLinear;
  future.value = .2;
  assert(owner->enqueue_score_input(future) == Result::Accepted);
  auto before = owner->stopped_checkpoint();
  float sample = 0;
  // Preserve actual overflow; do not drain or invent an application receipt.
  for (std::uint64_t i = 0; i < 256; ++i) {
    auto event = op(owner->native().determination, Kind::Parameter, i + 2, i);
    event.parameter = Parameter::MasterLinear;
    event.value = .5;
    assert(owner->enqueue_score_input(event) == Result::Accepted);
    assert(owner->offline_advance(&sample, 1, i));
  }
  std::array<float, 128> pcm{};
  while (owner->native().engine->samples_elapsed() < 1001) {
    auto cursor = owner->native().engine->samples_elapsed();
    auto frames = std::size_t(std::min<std::uint64_t>(128, 1001 - cursor));
    assert(owner->offline_advance(pcm.data(), frames, cursor));
  }
  auto after = owner->stopped_checkpoint();
  const auto &state = after->native_pair.audio;
  assert(state.applied_application_ordinal == 257 &&
         state.recording.dropped_applications == 1 &&
         state.recording.first_failed_sequence == 1 &&
         state.recording.first_failed_sample == 1000);
  auto b = management_checkpoint_transport::checkpoint_wire(*before);
  auto a = management_checkpoint_transport::checkpoint_wire(*after);
  write_json(out / "lost.before.json", b.get());
  write_json(out / "lost.after.json", a.get());
}
int main(int argc, char **argv) {
  try {
    if (argc != 3)
      throw std::invalid_argument(
          "usage: performance_score_reservation_packet-test "
          "ACTUAL_NATIVE_DIRECTORY NEW_OUTPUT_DIRECTORY");
    std::filesystem::path out(argv[2]);
    if (std::filesystem::exists(out) || !std::filesystem::create_directory(out))
      throw std::invalid_argument(
          "reservation artifact destination must be new");
    cancelled(argv[1], out);
    lost(argv[1], out);
    auto basis = parse(file(std::string(argv[1]) + "/baseline.basis.json"));
    write_json(out / "basis.json", basis.get());
    std::cout << "actual-reservation native-cancel-restore=1->2 "
                 "lost-native-ID=1 committed-ordinal=257 drop=1 "
                 "fabricated-applications=0 device=unexecuted\n";
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
