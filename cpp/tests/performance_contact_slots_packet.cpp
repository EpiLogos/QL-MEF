// Actual Rust M1/K/M2/B/P packets supply the physical constructor. The numeric
// slot test exercises real native force/P/q-v/PCM and full continuation. It
// deliberately creates no private Scene witness or host/source authority.
#include "../test_support/allocation_hooks.hpp"
#include <cassert>
#include <fstream>
#include <iostream>
#include <new>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_management.hpp>
static std::atomic<bool> in_callback{false};
static std::atomic<std::uint64_t> allocations{0}, releases{0};
void *operator new(std::size_t n) {
  if (in_callback.load(std::memory_order_relaxed))
    ++allocations;
  if (auto *p = ql_test_allocate(n))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t n) { return ::operator new(n); }
void operator delete(void *p) noexcept {
  if (in_callback.load(std::memory_order_relaxed) && p)
    ++releases;
  ql_test_release(p);
}
void operator delete[](void *p) noexcept { ::operator delete(p); }
void operator delete(void *p, std::size_t) noexcept { ::operator delete(p); }
void operator delete[](void *p, std::size_t) noexcept { ::operator delete(p); }
using namespace ql::performance;
static std::string file(const std::string &path) {
  std::ifstream in(path, std::ios::binary);
  ql::require(bool(in), "actual native packet absent");
  in.seekg(0, std::ios::end);
  const auto n = in.tellg();
  ql::require(n > 0 && n < 4 * 1024 * 1024, "actual packet budget differs");
  std::string s(std::size_t(n), '\0');
  in.seekg(0);
  ql::require(bool(in.read(s.data(), n)), "actual packet incomplete");
  return s;
}
static ql::physical_wire::Json parse(const std::string &s) {
  auto *tok = json_tokener_new_ex(64);
  if (!tok)
    throw std::bad_alloc();
  json_tokener_set_flags(tok, JSON_TOKENER_STRICT);
  auto out = ql::physical_wire::own(
      json_tokener_parse_ex(tok, s.data(), int(s.size())));
  const auto error = json_tokener_get_error(tok);
  const auto end = json_tokener_get_parse_end(tok);
  json_tokener_free(tok);
  ql::require(error == json_tokener_success && out &&
                  s.find_first_not_of(" \t\r\n", end) == std::string::npos,
              "strict complete native packet required");
  return out;
}
static NativePerformance native(const std::string &directory) {
  auto basis = parse(file(directory + "/baseline.basis.json"));
  return prepare_performance_packet(file(directory + "/baseline.packet.json"),
                                    basis.get(), true, true);
}
static NativeContactSourceIdentity source(const Determination &d) {
  const auto &i = d.identity;
  return {i.instance, i.event, i.subject, i.m1_revision, i.m2_generation};
}
static NativeContactOccurrence occurrence(std::uint64_t n) {
  return {reference("controlled:numerical-slot-owner-not-Scene-authority"), n};
}
static ql::GravityContactInput gravity(const ql::PreparedPhysicalBody &body,
                                       std::uint64_t trigger = 0) {
  ql::GravityContactInput v{};
  v.contact_ref = "controlled:actual-packet/gravity-occurrence";
  v.particle_ref = "controlled:actual-packet/particle";
  v.collider_ref = "controlled:actual-packet/plane";
  v.route_ref = "controlled:actual-packet/declared-body-exciter";
  v.source_ref = "QL-MEF:#284/R-D3/analytic-contact";
  v.policy_ref = "QL-MEF:finite-original-Newton-pulse";
  v.policy_revision = "1";
  v.standing = "reference";
  v.normal = body.input().exciter.axis;
  for (auto &x : v.normal)
    x = -x;
  v.height_metres = .5;
  v.mass_kg = 1e-6;
  v.start_sample = trigger;
  v.duration_samples = 512;
  return v;
}
template <class F> static void rejected(F f) {
  bool refused = false;
  try {
    f();
  } catch (const std::invalid_argument &) {
    refused = true;
  }
  assert(refused);
}
static std::vector<float>
passage(ql::PhysicalBody &body, NativeContactSlots &slots,
        const NativeContactSourceIdentity &current, NativeContactHandle handle,
        std::uint64_t sequence, std::uint64_t impact, std::uint64_t end,
        std::size_t partition, bool start = true) {
  std::vector<float> pcm;
  pcm.reserve(end - body.samples_elapsed());
  while (body.samples_elapsed() < end) {
    const auto cursor = body.samples_elapsed();
    const auto frames = std::min<std::uint64_t>(partition, end - cursor);
    std::array<double, 512> force{};
    std::array<float, 512> output{};
    ql::PhysicalSnapshot physical{};
    NativeContactOperands operands{};
    NativeContactOccurrence original{};
    in_callback.store(true);
    slots.begin_block();
    slots.requalify(current, body.preparation());
    for (std::size_t i = 0; i < frames; ++i) {
      if (start && cursor + i == impact) {
        assert(slots.begin(handle, sequence, current, body.preparation(),
                           impact, 0, operands, original));
        slots.application_ordinal(handle, 1);
      }
      force[i] = slots.force_at(cursor + i);
    }
    assert(body.advance_force_block(force.data(), output.data(), frames,
                                    body.body_revision(), cursor));
    assert(ql::write_physical_snapshot(body, physical, body.body_revision(),
                                       cursor + frames));
    for (std::size_t i = 0; i < frames; ++i)
      assert(std::isfinite(output[i]));
    slots.commit(cursor + frames);
    slots.acknowledge_terminals();
    in_callback.store(false);
    assert(physical.samples_elapsed == body.samples_elapsed());
    pcm.insert(pcm.end(), output.begin(), output.begin() + frames);
  }
  return pcm;
}
static void continuation(const std::string &dir, std::size_t partition,
                         bool before_impact) {
  auto owner = native(dir);
  auto slots = std::make_unique<NativeContactSlots>();
  auto p = std::make_shared<const PreparedContactProgram>(
      owner.body->preparation(), 0, gravity(owner.body->preparation()));
  assert(p->operands().impact_sample == 15326);
  assert(std::abs(p->operands().impact_seconds - 0.3192754284070505) < 1e-14);
  assert(std::abs(p->operands().planned_impulse_newton_seconds -
                  3.132091952673165e-6) < 1e-18);
  NativeContactHandle h{};
  const auto id = source(owner.determination);
  assert(slots->reserve(p, occurrence(10), id, 1, owner.body->preparation(), 0,
                        owner.body->preparation().input().max_force_newtons,
                        h) == NativeContactReservation::Ready);
  slots->accept(h);
  assert(slots->observe_queue(h));
  const auto save_at = before_impact ? 15300ULL : 15343ULL;
  const auto initial =
      passage(*owner.body, *slots, id, h, 1, 15326, save_at, partition);
  assert(std::all_of(initial.begin(),
                     initial.begin() +
                         std::min<std::size_t>(15326, initial.size()),
                     [](float x) { return x == 0; }));
  auto cp = std::make_unique<NativeContactCheckpoint>();
  slots->write_checkpoint(*cp);
  assert(cp->slots[h.slot].delivery.delivered_frames ==
         (before_impact ? 0 : 17));
  const auto original_p = owner.body->checkpoint();
  auto wire = contact_transport::checkpoint(*cp);
  const auto bytes = std::string(
      json_object_to_json_string_ext(wire.get(), JSON_C_TO_STRING_PLAIN));
  auto parsed = parse(bytes);
  auto restored_cp = std::make_unique<NativeContactCheckpoint>(
      contact_transport::read_checkpoint(parsed.get()));
  assert(NativeContactSlots::same_checkpoint(*cp, *restored_cp));
  auto rebuilt = NativeContactSlots::prepare_restore(
      *restored_cp, owner.body->preparation(), save_at, 1);
  auto replay = native(dir);
  assert(replay.body->restore_checkpoint(original_p,
                                         replay.body->body_revision(), 0));
  auto replay_slots = std::make_unique<NativeContactSlots>();
  replay_slots->restore(*rebuilt);
  const auto end = 15326ULL + 512 + 67;
  const auto expected = passage(*owner.body, *slots, id, h, 1, 15326, end,
                                partition, before_impact);
  const auto resumed = passage(*replay.body, *replay_slots, id, h, 1, 15326,
                               end, partition == 1 ? 512 : 1, before_impact);
  assert(expected == resumed);
  assert(owner.body->checkpoint().displacement_modal_metres ==
         replay.body->checkpoint().displacement_modal_metres);
  assert(owner.body->checkpoint().velocity_modal_metres_per_second ==
         replay.body->checkpoint().velocity_modal_metres_per_second);
  const auto d = replay_slots->deliveries()[h.slot];
  assert(d.status == NativeContactStatus::Completed &&
         d.delivered_frames == 512);
  assert(std::abs(d.delivered_impulse_newton_seconds - 3.132091952673165e-6) <
         1e-18);
  assert(d.start_application_ordinal == 1 && d.actual_impact_sample == 15326);
  NativeContactHandle unused{};
  assert(replay_slots->reserve(
             p, occurrence(10), id, 2, replay.body->preparation(), 0,
             replay.body->preparation().input().max_force_newtons,
             unused) == NativeContactReservation::Duplicate);
  auto corrupt = *cp;
  corrupt.slots[h.slot].force_newtons[0] *= 2;
  rejected([&] {
    NativeContactSlots::prepare_restore(corrupt, owner.body->preparation(),
                                        save_at, 1);
  });
  corrupt = *cp;
  ++corrupt.slots[h.slot].delivery.delivered_frames;
  rejected([&] {
    NativeContactSlots::prepare_restore(corrupt, owner.body->preparation(),
                                        save_at, 1);
  });
  // A public imported handle reaches the actual resident Engine and is refused.
  Operation imported{};
  imported.kind = Kind::Contact;
  imported.contact = h;
  imported.identity = replay.determination.identity;
  imported.sequence = 1;
  imported.sample = 15326;
  assert(replay.engine->enqueue(imported) == Result::Invalid);
  assert(replay.engine->samples_elapsed() == 0);
}
static void changed_body(const std::string &dir) {
  auto native_owner = native(dir);
  auto slots = std::make_unique<NativeContactSlots>();
  const auto id = source(native_owner.determination);
  auto p = std::make_shared<const PreparedContactProgram>(
      native_owner.body->preparation(), 0,
      gravity(native_owner.body->preparation()));
  NativeContactHandle h{};
  assert(slots->reserve(p, occurrence(31), id, 1,
                        native_owner.body->preparation(), 0, 10,
                        h) == NativeContactReservation::Ready);
  slots->accept(h);
  assert(slots->observe_queue(h));
  passage(*native_owner.body, *slots, id, h, 1, 15326, 15343, 128);
  const auto before = slots->deliveries()[h.slot];
  auto changed = native_owner.body->preparation().input();
  ++changed.body_revision;
  changed.preparation_ref += "/after-material";
  changed.material.revision += "/after-material";
  changed.material.young_modulus_pa *= 1.25;
  assert(native_owner.body->replace_material(ql::PreparedPhysicalBody(changed),
                                             changed.body_revision - 1));
  const auto cp_before = native_owner.body->checkpoint();
  const auto ring =
      passage(*native_owner.body, *slots, id, h, 1, 15326, 15471, 128, false);
  const auto after = slots->deliveries()[h.slot];
  assert(after.status == NativeContactStatus::Interrupted &&
         after.refusal == NativeContactRefusal::PhysicalPreparationChanged &&
         after.delivered_frames == 17 &&
         after.delivered_impulse_newton_seconds ==
             before.delivered_impulse_newton_seconds);
  assert(std::any_of(ring.begin(), ring.end(), [](float x) { return x != 0; }));
  auto cp = std::make_unique<NativeContactCheckpoint>();
  slots->write_checkpoint(*cp);
  assert(cp->slots[h.slot].operands.body_revision == changed.body_revision - 1);
  NativeContactSlots::prepare_restore(*cp, native_owner.body->preparation(),
                                      15471, 1);
  assert(cp_before.samples_elapsed == 15343 &&
         native_owner.body->samples_elapsed() == 15471);
}
static void recycle(const std::string &dir) {
  auto owner = native(dir);
  auto slots = std::make_unique<NativeContactSlots>();
  const auto id = source(owner.determination);
  for (std::uint64_t n = 1; n <= 64; ++n) {
    auto original =
        gravity(owner.body->preparation(), owner.body->samples_elapsed());
    original.height_metres = 0;
    original.initial_normal_velocity_metres_per_second = -1;
    original.duration_samples = 32;
    original.contact_ref += "/" + std::to_string(n);
    auto p = std::make_shared<const PreparedContactProgram>(
        owner.body->preparation(), original.start_sample, original);
    NativeContactHandle h{};
    assert(slots->reserve(p, occurrence(n), id, n, owner.body->preparation(),
                          original.start_sample, 10,
                          h) == NativeContactReservation::Ready);
    slots->accept(h);
    assert(slots->observe_queue(h));
    assert(h.slot == 0 && h.generation == n);
    const auto use_count = p.use_count();
    passage(*owner.body, *slots, id, h, n, original.start_sample,
            original.start_sample + 32, 32);
    assert(p.use_count() == use_count);
  }
  auto cp = std::make_unique<NativeContactCheckpoint>();
  slots->write_checkpoint(*cp);
  assert(cp->original_request_high_water == 64 &&
         cp->slot_generations[0] == 64);
  assert(cp->slots[0].occurrence.original_request_id == 64);
}
static void refusals(const std::string &dir) {
  auto owner = native(dir);
  auto slots = std::make_unique<NativeContactSlots>();
  const auto id = source(owner.determination);
  auto p = std::make_shared<const PreparedContactProgram>(
      owner.body->preparation(), 0, gravity(owner.body->preparation()));
  NativeContactHandle h{};
  const auto physical_before = owner.body->checkpoint();
  assert(slots->reserve(p, occurrence(1), id, 1, owner.body->preparation(), 0,
                        std::abs(p->operands().force_newtons) * .5,
                        h) == NativeContactReservation::Budget);
  auto wrong = owner.body->preparation().input();
  wrong.pratibimba = !wrong.pratibimba;
  const ql::PreparedPhysicalBody wrong_face(wrong);
  assert(slots->reserve(p, occurrence(1), id, 1, wrong_face, 0, 10, h) ==
         NativeContactReservation::Stale);
  wrong = owner.body->preparation().input();
  ++wrong.body_revision;
  const ql::PreparedPhysicalBody wrong_revision(wrong);
  assert(slots->reserve(p, occurrence(1), id, 1, wrong_revision, 0, 10, h) ==
         NativeContactReservation::Stale);
  assert(slots->reserve(p, occurrence(1), id, 1, owner.body->preparation(),
                        15327, 10, h) == NativeContactReservation::Stale);
  auto empty = std::make_unique<NativeContactCheckpoint>();
  slots->write_checkpoint(*empty);
  assert(!empty->history_present && empty->original_request_high_water == 0);
  // A failed publication rolls back the exact programme and generation.
  assert(slots->reserve(p, occurrence(1), id, 1, owner.body->preparation(), 0,
                        10, h) == NativeContactReservation::Ready);
  slots->rollback_unpublished(h);
  auto rolled_back = std::make_unique<NativeContactCheckpoint>();
  slots->write_checkpoint(*rolled_back);
  assert(NativeContactSlots::same_checkpoint(*empty, *rolled_back));
  std::array<NativeContactHandle, 16> handles{};
  for (std::uint64_t n = 1; n <= 16; ++n) {
    auto original = gravity(owner.body->preparation());
    original.contact_ref += "/" + std::to_string(n);
    auto q = std::make_shared<const PreparedContactProgram>(
        owner.body->preparation(), 0, original);
    assert(slots->reserve(q, occurrence(n), id, n, owner.body->preparation(), 0,
                          10,
                          handles[n - 1]) == NativeContactReservation::Ready);
    slots->accept(handles[n - 1]);
    assert(slots->observe_queue(handles[n - 1]));
  }
  auto full = std::make_unique<NativeContactCheckpoint>();
  slots->write_checkpoint(*full);
  assert(slots->reserve(p, occurrence(17), id, 17, owner.body->preparation(), 0,
                        10, h) == NativeContactReservation::Exhausted);
  auto unchanged = std::make_unique<NativeContactCheckpoint>();
  slots->write_checkpoint(*unchanged);
  assert(NativeContactSlots::same_checkpoint(*full, *unchanged));
  assert(owner.body->checkpoint().samples_elapsed ==
             physical_before.samples_elapsed &&
         owner.body->checkpoint().displacement_modal_metres ==
             physical_before.displacement_modal_metres);
  // Every delivery/refusal observation is committed only after the actual P
  // owner advances. These numerical negatives issue no private Scene witness.
  passage(*owner.body, *slots, id, handles[0], 1, 15326, 15326, 512, false);
  slots->begin_block();
  NativeContactOperands original{};
  NativeContactOccurrence occurred{};
  assert(!slots->begin(handles[0], 1, id, owner.body->preparation(), 15326, 1,
                       original, occurred));
  assert(original.impact_sample == 15326 && occurred.original_request_id == 1);
  assert(slots->begin(handles[1], 2, id, owner.body->preparation(), 15326, 0,
                      original, occurred));
  auto stale_source = id;
  ++stale_source.m2_generation;
  assert(!slots->begin(handles[2], 3, stale_source, owner.body->preparation(),
                       15326, 0, original, occurred));
  double force = slots->force_at(15326);
  float pickup = 0;
  assert(force == p->force().force_newtons[0]);
  assert(owner.body->advance_force_block(&force, &pickup, 1,
                                         owner.body->body_revision(), 15326));
  assert(std::isfinite(pickup));
  slots->commit(15327);
  slots->acknowledge_terminals();
  assert(slots->deliveries()[0].status == NativeContactStatus::Refused &&
         slots->deliveries()[0].refusal ==
             NativeContactRefusal::EmergencyFenced &&
         slots->deliveries()[0].delivered_frames == 0);
  assert(slots->deliveries()[2].refusal ==
         NativeContactRefusal::CurrentSourceChanged);
  // Active interruption retains the exact real physical impulse prefix and
  // advances the same P q/v with zero new excitation on the next sample.
  slots->begin_block();
  slots->interrupt(2);
  force = slots->force_at(15327);
  assert(force == 0);
  assert(owner.body->advance_force_block(&force, &pickup, 1,
                                         owner.body->body_revision(), 15327));
  assert(std::isfinite(pickup));
  slots->commit(15328);
  slots->acknowledge_terminals();
  assert(slots->deliveries()[1].status == NativeContactStatus::Interrupted &&
         slots->deliveries()[1].delivered_frames == 1 &&
         slots->deliveries()[1].delivered_impulse_newton_seconds ==
             p->force().force_newtons[0] / p->force().sample_rate);
}
int main(int argc, char **argv) {
  if (argc != 2)
    return 2;
  try {
    static_assert(sizeof(Engine) < 16 * 1024 * 1024);
    static_assert(sizeof(Engine::Checkpoint) < 8 * 1024 * 1024);
    for (auto partition : {1U, 128U, 512U}) {
      continuation(argv[1], partition, true);
      continuation(argv[1], partition, false);
    }
    changed_body(argv[1]);
    recycle(argv[1]);
    refusals(argv[1]);
    assert(allocations == 0 && releases == 0);
    std::cout << "{\"schema\":\"ql.native-contact-component-result/"
                 "v1\",\"partitions\":[1,128,512],\"impact_sample\":15326,"
                 "\"continuations\":6,\"reused_original_occurrences\":64,"
                 "\"callback_allocations\":"
              << allocations << ",\"callback_releases\":" << releases
              << ",\"Engine\":" << sizeof(Engine)
              << ",\"Operation\":" << sizeof(Operation)
              << ",\"NativeGestureApplication\":"
              << sizeof(NativeGestureApplication)
              << ",\"EngineCheckpoint\":" << sizeof(Engine::Checkpoint)
              << "}\n";
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
