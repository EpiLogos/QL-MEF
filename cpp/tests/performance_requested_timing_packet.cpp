// Original native requested time is distinct from resolved queue admission and
// committed application. Actual producer -> Manager -> P q/v -> checkpoint.
#include <array>
#include <cassert>
#include <fstream>
#include <iostream>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_management.hpp>
#include <string>
#include <vector>
using namespace ql::performance;
namespace wire = checkpoint_transport;
static std::string source_file(const std::string &path) {
  std::ifstream in(path, std::ios::binary);
  if (!in)
    throw std::invalid_argument("actual native producer absent");
  in.seekg(0, std::ios::end);
  auto size = in.tellg();
  if (size < 1 || size > 4 * 1024 * 1024)
    throw std::invalid_argument("bounded source required");
  std::string out(std::size_t(size), '\0');
  in.seekg(0);
  if (!in.read(out.data(), size))
    throw std::invalid_argument("source read failed");
  return out;
}
static auto manager(const std::string &directory) {
  auto basis = ql::physical_wire::parse_native(
      source_file(directory + "/baseline.basis.json").c_str());
  auto native = prepare_performance_packet(
      source_file(directory + "/baseline.packet.json"), basis.get(), true,
      true);
  return std::make_unique<PerformanceManagement>(
      std::move(native),
      reference("expression:original-requested-time/session"));
}
static Operation event(const Determination &source, Kind kind,
                       std::uint64_t sequence, std::uint64_t sample) {
  Operation out{};
  out.kind = kind;
  out.identity = source.identity;
  out.sequence = sequence;
  out.sample = sample;
  return out;
}
static void erase_new_provenance(json_object *value) {
  if (!value)
    return;
  if (json_object_is_type(value, json_type_array)) {
    for (std::size_t i = 0; i < json_object_array_length(value); ++i)
      erase_new_provenance(json_object_array_get_idx(value, i));
  } else if (json_object_is_type(value, json_type_object)) {
    json_object_object_del(value, "requested_sample");
    json_object_object_foreach(value, key, child) {
      (void)key;
      erase_new_provenance(child);
    }
  }
}
static void trial(const std::string &directory, bool panic) {
  auto owner = manager(directory);
  const auto &source = owner->native().determination;
  auto attack = event(source, Kind::NoteOn, 1, 37);
  attack.note = owner->native().notes.at(4);
  attack.value = .8;
  const Ref original_input = reference("native:original-input/requested-time");
  assert(owner->enqueue_score_input(attack, original_input) ==
         Result::Accepted);
  auto future = event(source, Kind::Parameter, 2, 48000);
  future.parameter = Parameter::MasterLinear;
  future.value = .2;
  assert(owner->enqueue_score_input(future) == Result::Accepted);
  std::array<float, 512> pcm{}, restored_pcm{}, legacy_pcm{};
  assert(owner->offline_advance(pcm.data(), 128, 0));
  auto release = event(source, panic ? Kind::Panic : Kind::NoteOff, 3, 0);
  release.touch = attack.note.touch;
  assert(owner->enqueue_score_input(release, panic ? Ref{} : original_input) ==
         Result::Accepted);
  auto pending = owner->stopped_checkpoint();
  const auto &ring = pending->native_pair.audio.releases;
  assert(ring.write == ring.read + 1);
  const auto &queued = ring.storage[ring.read % 64];
  assert(queued.has_requested_sample && queued.requested_sample == 0 &&
         queued.sample == 128 && queued.late_admitted);
  auto saved = management_checkpoint_transport::checkpoint_wire(*pending);
  auto restored_state =
      management_checkpoint_transport::read_checkpoint_wire(saved.get());
  auto reopened = manager(directory);
  TransportAcknowledgement ack{};
  assert(reopened->stopped_restore(*restored_state, 0,
                                   reference("native:timing/restore"),
                                   reference("native:timing/checkpoint"), ack));
  assert(ack.target_sample == 128 && ack.epoch == 2);
  // Real old-format scalar v2 state: remove only the new optional provenance.
  // Dynamic voices, queues, body, input refs, ordinals and epochs stay exact.
  auto old_wire = ql::physical_wire::parse_native(
      json_object_to_json_string_ext(saved.get(), JSON_C_TO_STRING_PLAIN));
  erase_new_provenance(old_wire.get());
  auto old_state =
      management_checkpoint_transport::read_checkpoint_wire(old_wire.get());
  const auto &old_ring = old_state->native_pair.audio.releases;
  assert(!old_ring.storage[old_ring.read % 64].has_requested_sample);
  auto historical = manager(directory);
  TransportAcknowledgement old_ack{};
  assert(historical->stopped_restore(
      *old_state, 0, reference("native:timing/historical"),
      reference("native:timing/original-checkpoint"), old_ack));
  // A changed requested time beyond the resolved deadline refuses atomically.
  auto corrupt =
      management_checkpoint_transport::read_checkpoint_wire(saved.get());
  auto &bad = corrupt->native_pair.audio.releases;
  bad.storage[bad.read % 64].requested_sample = 129;
  auto rejected = manager(directory);
  TransportAcknowledgement denied{};
  assert(!rejected->stopped_restore(*corrupt, 0, reference("native:timing/bad"),
                                    reference("native:timing/bad-checkpoint"),
                                    denied));
  assert(rejected->native().body->samples_elapsed() == 0 &&
         rejected->native().engine->samples_elapsed() == 0);
  assert(owner->offline_advance(pcm.data(), 128, 128));
  assert(reopened->offline_advance(restored_pcm.data(), 128, 128));
  assert(historical->offline_advance(legacy_pcm.data(), 128, 128));
  assert(std::equal(pcm.begin(), pcm.begin() + 128, restored_pcm.begin()));
  assert(std::equal(pcm.begin(), pcm.begin() + 128, legacy_pcm.begin()));
  auto actual = owner->pulse(), restored = reopened->pulse(),
       old = historical->pulse();
  assert(actual->applications.size() == 2 &&
         restored->applications.size() == 2 && old->applications.size() == 2);
  const auto &on = actual->applications[0], &off = actual->applications[1];
  assert(on.has_requested_sample && on.requested_sample == 37 &&
         on.admitted_sample == 37 && on.applied_sample == 37);
  assert(off.sequence == 3 && off.applied_application_ordinal == 2 &&
         off.has_requested_sample && off.requested_sample == 0 &&
         off.admitted_sample == 128 && off.applied_sample == 128 &&
         off.late_admitted);
  const auto &reopened_off = restored->applications[1];
  assert(reopened_off.has_requested_sample &&
         reopened_off.requested_sample == 0 &&
         reopened_off.admitted_sample == 128 &&
         reopened_off.applied_sample == 128);
  assert(!old->applications[1].has_requested_sample);
  auto applied_wire = wire::application(off);
  auto applied_roundtrip = wire::read_application(applied_wire.get());
  assert(applied_roundtrip.has_requested_sample &&
         applied_roundtrip.requested_sample == 0 &&
         applied_roundtrip.admitted_sample == 128 &&
         applied_roundtrip.applied_sample == 128);
  auto imported_wire = wire::application(old->applications[1]);
  json_object *unknown = nullptr;
  assert(!json_object_object_get_ex(imported_wire.get(), "requested_sample",
                                    &unknown));
  for (const auto &h : actual->input_history) {
    assert(h.input_ref == original_input);
    assert(h.target.touch == attack.note.touch);
  }
  for (unsigned block = 0; block < 2; ++block) {
    const auto start = std::uint64_t(256 + block * 512);
    assert(owner->offline_advance(pcm.data(), 512, start));
    assert(reopened->offline_advance(restored_pcm.data(), 512, start));
    assert(historical->offline_advance(legacy_pcm.data(), 512, start));
    assert(pcm == restored_pcm && pcm == legacy_pcm);
    auto a = owner->native().body->checkpoint(),
         b = reopened->native().body->checkpoint(),
         c = historical->native().body->checkpoint();
    assert(a.displacement_modal_metres == b.displacement_modal_metres &&
           a.displacement_modal_metres == c.displacement_modal_metres &&
           a.velocity_modal_metres_per_second ==
               b.velocity_modal_metres_per_second &&
           a.velocity_modal_metres_per_second ==
               c.velocity_modal_metres_per_second);
  }
  // Fresh callers cannot inject provenance which only native admission owns.
  auto forged = event(source, Kind::Parameter, 4, 1280);
  forged.parameter = Parameter::MasterLinear;
  forged.value = .5;
  forged.has_requested_sample = true;
  forged.requested_sample = 0;
  assert(owner->enqueue_score_input(forged) == Result::Invalid);
  assert(owner->native().engine->accepted_sequence() == 3);
}
int main(int argc, char **argv) {
  try {
    if (argc != 2)
      throw std::invalid_argument(
          "actual Rust native producer fixture directory required");
    trial(argv[1], false);
    trial(argv[1], true);
    std::cout << "native requested0 admitted128 applied128 release+panic "
                 "pendingCP exact-next1024 legacy-v2-requested-unavailable "
                 "injection-refusal device=unexecuted\n";
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
