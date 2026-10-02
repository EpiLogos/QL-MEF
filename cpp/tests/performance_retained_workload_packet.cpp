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

// Mandatory real 15 minute producer. This controlled Reference body proves
// retained A/P applications/cursor/tails; it grants no AUHAL, personal or H
// claim.
int main(int argc, char **argv) {
  try {
    if (argc != 3)
      throw std::invalid_argument(
          "usage: performance_retained_workload_packet-test "
          "ACTUAL_NATIVE_DIRECTORY NEW_OUTPUT_DIRECTORY");
    const std::filesystem::path output(argv[2]);
    if (std::filesystem::exists(output) ||
        !std::filesystem::create_directory(output))
      throw std::invalid_argument("workload destination must be new");
    auto owner = manager(argv[1]);
    const auto source = owner->native().determination;
    const auto target = owner->native().notes.at(4);
    const std::uint64_t rate = 48000, interval = 5 * rate;
    std::uint64_t sequence = 0, committed = 0, journal_ordinal = 0;
    std::array<float, 128> pcm{};
    auto manifest = checkpoint_transport::object();
    checkpoint_transport::text(manifest.get(), "schema",
                               "ql.retained-native-workload/v1");
    checkpoint_transport::u64(manifest.get(), "sample_rate", rate);
    checkpoint_transport::u64(manifest.get(), "duration_samples", 900 * rate);
    checkpoint_transport::u64(manifest.get(), "voice_count", 24);
    checkpoint_transport::u64(manifest.get(), "application_count", 45000);
    checkpoint_transport::u64(manifest.get(), "edition_count", 180);
    auto parts = checkpoint_transport::array();
    for (std::uint64_t block = 0; block < 180; ++block) {
      const auto base = block * interval;
      std::array<Ref, 24> inputs{};
      std::array<NoteTarget, 24> targets{};
      auto enqueue = [&](Operation event, Ref input = Ref{}) {
        event.sequence = ++sequence;
        if (owner->enqueue_score_input(event, input) != Result::Accepted)
          throw std::runtime_error("actual native workload admission refused");
      };
      for (std::size_t voice = 0; voice < 24; ++voice) {
        targets[voice] = target;
        targets[voice].touch = targets[voice].member = block * 24 + voice + 1;
        inputs[voice] =
            reference(("native-score:retained/" + std::to_string(block) + "/" +
                       std::to_string(voice))
                          .c_str());
        auto attack = op(source, Kind::NoteOn, 0, base + 37);
        attack.note = targets[voice];
        attack.value = .4;
        enqueue(attack, inputs[voice]);
      }
      for (std::uint64_t pass = 0; pass < 5; ++pass)
        for (std::size_t voice = 0; voice < 24; ++voice) {
          auto expression = op(source, Kind::Expression, 0,
                               base + 1000 + pass * 4000 + voice);
          expression.touch = targets[voice].touch;
          expression.value = .3 + pass * .1;
          enqueue(expression, inputs[voice]);
        }
      for (std::uint64_t i = 0; i < 24; ++i) {
        auto force = op(source, Kind::Parameter, 0, base + 22000 + i * 100);
        force.parameter = Parameter::ForceNewtons;
        force.value = .05 + (i % 4) * .01;
        enqueue(force);
      }
      for (std::uint64_t i = 0; i < 10; ++i) {
        auto cutoff = op(source, Kind::Parameter, 0, base + 26000 + i * 100);
        cutoff.parameter = Parameter::CutoffHertz;
        cutoff.value = 1000 + i * 100;
        enqueue(cutoff);
      }
      // Accepted BEFORE later-ID critical releases but callback-applied AFTER.
      for (std::uint64_t i = 0; i < 48; ++i) {
        auto automation =
            op(source, Kind::Parameter, 0, base + 180000 + i * 100);
        automation.parameter = Parameter::MasterLinear;
        automation.value = .4 + (i % 3) * .1;
        enqueue(automation);
      }
      for (std::size_t voice = 0; voice < 24; ++voice) {
        auto release = op(source, Kind::NoteOff, 0, base + 40000 + voice);
        release.touch = targets[voice].touch;
        enqueue(release, inputs[voice]);
      }
      std::vector<NativeGestureApplication> applications;
      std::vector<InputBindingRecord> journal;
      while (owner->native().engine->samples_elapsed() < base + interval) {
        const auto cursor = owner->native().engine->samples_elapsed();
        const auto frames =
            std::size_t(std::min<std::uint64_t>(128, base + interval - cursor));
        if (!owner->offline_advance(pcm.data(), frames, cursor))
          throw std::runtime_error("actual A/P workload advance refused");
        auto pulse = owner->pulse();
        if (!pulse->has_readback ||
            pulse->recording.failure != RecordingFailure::None ||
            pulse->recording.dropped_applications != 0 ||
            !owner->recording_available())
          throw std::runtime_error("actual native workload recording loss");
        append_pulse(*pulse, applications, journal);
      }
      if (applications.size() != 250)
        throw std::runtime_error("native workload application count differs");
      bool overtook = false;
      for (std::size_t i = 0; i < applications.size(); ++i) {
        const auto &a = applications[i];
        if (!a.applied || a.applied_application_ordinal != ++committed ||
            a.physical_sample_rate != rate ||
            a.committed_cursor > base + interval)
          throw std::runtime_error(
              "native workload application/body/cursor differs");
        if (i && a.sequence < applications[i - 1].sequence)
          overtook = true;
      }
      if (!overtook)
        throw std::runtime_error(
            "native workload did not exercise critical overtaking");
      for (const auto &entry : journal)
        if (entry.ordinal != ++journal_ordinal)
          throw std::runtime_error("native workload original input gap");
      const auto name = "edition-" + std::to_string(block + 1);
      auto history = artifacts(applications, journal);
      write_json(output / (name + ".history.json"), history.get());
      auto checkpoint = owner->stopped_checkpoint();
      if (checkpoint->native_pair.audio.cursor != base + interval ||
          checkpoint->native_pair.physical.samples_elapsed != base + interval ||
          checkpoint->native_pair.audio.applied_application_ordinal !=
              committed)
        throw std::runtime_error("workload paired checkpoint cursor differs");
      auto wire = management_checkpoint_transport::checkpoint_wire(*checkpoint);
      write_json(output / (name + ".checkpoint.json"), wire.get());
      auto part = checkpoint_transport::object();
      checkpoint_transport::text(part.get(), "history", name + ".history.json");
      checkpoint_transport::text(part.get(), "checkpoint",
                                 name + ".checkpoint.json");
      checkpoint_transport::append(parts.get(), part.release());
    }
    if (sequence != 45000 || committed != 45000)
      throw std::runtime_error("full workload was truncated");
    checkpoint_transport::put(manifest.get(), "editions", parts.release());
    auto basis = parse(file(std::string(argv[1]) + "/baseline.basis.json"));
    write_json(output / "basis.json", basis.get());
    write_json(output / "manifest.json", manifest.get());
    std::cout
        << "actual-native-workload duration=900 voices=24 applications=45000 "
           "editions=180 force=24x180 automation=58x180 "
           "original-inputs=preserved overtaking=180 device=unexecuted\n";
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
