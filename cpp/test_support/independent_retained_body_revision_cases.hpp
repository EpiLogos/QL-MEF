#ifndef QL_INDEPENDENT_RETAINED_BODY_REVISION_CASES_HPP
#define QL_INDEPENDENT_RETAINED_BODY_REVISION_CASES_HPP
// Included by the existing genuine Rust-produced route-management CPP driver,
// after its prepare/native_catalog/revision_source helpers. No substituted P,
// route, source, input binding or callback implementation is introduced.
static void independent_retained_body_revision_cases(J *fixture) {
  enum class Mutation { Catalog, Queue, Emergency, RouteReinstall, Restore };
  for (const auto mutation : {Mutation::Catalog, Mutation::Queue,
                              Mutation::Emergency, Mutation::RouteReinstall,
                              Mutation::Restore}) {
    auto joined = prepare(fixture);
    auto owner = std::make_unique<PerformanceManagement>(
        joined.native, reference("expression:native-nine/independent-revision"));
    owner->admit_catalog(native_catalog(fixture));
    std::array<float, 128> pcm{};
    for (unsigned block = 0; block < 4; ++block) {
      assert(owner->offline_advance(pcm.data(), pcm.size(), block * 128));
      owner->pulse();
    }
    const auto saved = owner->stopped_checkpoint();
    const auto resident_before = owner->native().body->checkpoint();
    auto *after_fixture = packet::field(fixture, "after_material");
    auto source = revision_source(after_fixture, owner->native().body);
    auto physical = std::make_unique<PreparedPhysicalTransition>(
        owner->native().body->preparation(),
        source.prepared.body->preparation(), 991, 512,
        PhysicalLiveUpdateKind::Material,
        PhysicalFormTransition::ProjectCorrespondingNodes,
        "native:independent/revision/exact-original-material");
    std::unique_ptr<PreparedRetainedBodyRevision> candidate;
    {
      auto guard = owner->native().engine->acquire_stopped_custody();
      candidate = std::make_unique<PreparedRetainedBodyRevision>(
          *owner, std::move(physical), source.admitted,
          source.prepared.determination,
          packet::field(packet::field(after_fixture, "performance_preparation"),
                        "native_basis"),
          source.seed, source.prepared.notes, native_catalog(after_fixture),
          guard);
      assert(candidate->current(guard));
      if (mutation == Mutation::Catalog) {
        // A genuine public Manager admission under the same stopped guard.
        // Numeric targets/provenance remain native; a later visible catalog
        // edit must not be silently overwritten by a formerly prepared token.
        auto changed = native_catalog(fixture);
        changed.front().label += " independent edit";
        owner->admit_catalog(std::move(changed));
      } else if (mutation == Mutation::Emergency) {
        assert(owner->native().engine->request_panic() != 0);
      } else if (mutation == Mutation::RouteReinstall) {
        // The current original resident port at current cursor, not AFTER P.
        auto programmes = saved->native_pair.audio.route_programs;
        assert(owner->native().engine->install_route_programs(
            programmes, guard, 512) == false);
        // Original manifest cursor is 0. This refusal must not invalidate an
        // otherwise current token: it installed nothing.
        assert(candidate->current(guard));
        continue;
      }
    }
    if (mutation == Mutation::Queue) {
      auto future = operation(owner->native().determination, Kind::Parameter,
                              owner->native().engine->accepted_sequence() + 1,
                              600);
      future.parameter = Parameter::MasterLinear;
      future.value = .25;
      assert(owner->enqueue_score_input(future) == Result::Accepted);
    }
    if (mutation == Mutation::Restore) {
      TransportAcknowledgement acknowledgement;
      assert(owner->stopped_restore(
          *saved, 512, reference("native:independent/revision/real-restore"),
          reference("native:independent/revision/original-manager-checkpoint"),
          acknowledgement));
      assert(acknowledgement.target_sample == 512);
    }
    auto guard = owner->native().engine->acquire_stopped_custody();
    assert(!candidate->current(guard));
    PhysicalLiveTransitionReceipt receipt;
    receipt.transaction = 777; // refusal preserves caller output too
    assert(!candidate->commit(guard, receipt));
    assert(!candidate->committed() && receipt.transaction == 777);
    const auto unchanged = owner->native().body->checkpoint();
    assert(unchanged.body_revision == resident_before.body_revision &&
           unchanged.samples_elapsed == resident_before.samples_elapsed &&
           unchanged.displacement_modal_metres ==
               resident_before.displacement_modal_metres &&
           unchanged.velocity_modal_metres_per_second ==
               resident_before.velocity_modal_metres_per_second);
    assert(owner->native().engine->owns_physical_owner(
        owner->native().body.get()));
  }
}
#endif
