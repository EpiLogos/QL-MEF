//! Coordinate: S0 M0 -> M0'
//! Residency: Body/S/S0/epi-lib/src/lib.rs
//! Position (#n): #0' -- compiled Anuttara verifier bridge.
//! Actualises: Track 01.T1.10 and the generic-profile M0 witness emission.
//! Public surface: m0_verifier::{bootstrap_witness_for_tick, contemplation_prompt_lut, evaluate_state, is_language_member}.
//! Does NOT own: gateway transport, profile serialization, or session evidence.
//! Contract: [[S0-SPEC]] -> [[M0'-SPEC]].

#[cfg(feature = "m0_verifier")]
pub mod m0_verifier {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int};

    const M0_VERIFIER_VIRTUE_COUNT: usize = 9;
    const M0_VERIFIER_MAX_UNSATISFIED: usize = 80;
    const M0_VERIFIER_COORDINATE_MAX: usize = 96;
    const M0_VERIFIER_MAX_TYPED_QUERIES: usize = 16;
    const M0_VERIFIER_MAX_BACKING_CHAIN: usize = 24;
    const M0_VERIFIER_MAX_ROUTE_IN: usize = 16;
    const M0_VERIFIER_MAX_ROUTE_OUT: usize = 24;
    const M0_VERIFIER_MAX_ENGAGED_COORDS: usize = 8;
    const M0_VERIFIER_QUERY_KIND_MAX: usize = 32;
    #[cfg(test)]
    const M0_VERIFIER_SYNTAX_SPEECH: u16 = 1 << 0;
    #[cfg(test)]
    const M0_VERIFIER_SYNTAX_RELATIONSHIP: u16 = 1 << 1;
    #[cfg(test)]
    const M0_VERIFIER_SYNTAX_ACTION: u16 = 1 << 2;
    #[cfg(test)]
    const M0_VERIFIER_SYNTAX_COMPLETION: u16 = 1 << 3;

    #[cfg(test)]
    const R_BAND_PRAVRITTI: u8 = 0;
    #[cfg(test)]
    const R_BAND_NIVRITTI: u8 = 1;
    #[cfg(test)]
    const R_BAND_TURN: u8 = 2;

    #[cfg(test)]
    const M0_ANUTTARA_LAW_CONTAINMENT: u8 = 2;
    #[cfg(test)]
    const M0_ANUTTARA_LAW_EIGHT_PLUS_ONE: u8 = 5;
    #[cfg(test)]
    const M0_ANUTTARA_LAW_DERIVATION: u8 = 7;

    #[cfg(test)]
    const M0_TRIAD_NOT_CLOSING: u8 = 0;
    #[cfg(test)]
    const M0_TRIAD_COMPILES: u8 = 1;

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct RFactorPathStep {
        r_factor: u8,
        base_route: u8,
        band: u8,
        position: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct KernelState {
        committed_virtue_mask: u16,
        virtue_evidence: [f32; M0_VERIFIER_VIRTUE_COUNT],
        observed_core_relation_count: u16,
        syntax_layer_mask: u16,
        active_archetype: u8,
        active_tct_position: u8,
        slot_privacy_boundary_compliance: u8,
        route_step_count: u8,
        route_steps: [RFactorPathStep; M0_VERIFIER_MAX_ROUTE_IN],
        engaged_coordinate_count: u8,
        engaged_coordinates: [[c_char; M0_VERIFIER_COORDINATE_MAX]; M0_VERIFIER_MAX_ENGAGED_COORDS],
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct M0TypedQuery {
        law_family: u8,
        query_kind: [c_char; M0_VERIFIER_QUERY_KIND_MAX],
        symbolic_coordinate_string: [c_char; M0_VERIFIER_COORDINATE_MAX],
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct M0CoordinateRef {
        coordinate: [c_char; M0_VERIFIER_COORDINATE_MAX],
        packed: u16,
    }

    #[repr(C)]
    #[cfg(test)]
    struct M0BackingChain {
        depth: u8,
        grounded: u8,
        link_count: u8,
        links: [M0CoordinateRef; M0_VERIFIER_MAX_BACKING_CHAIN],
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct M0OwlValidationReport {
        status: u8,
        live_n10s_deferred: u8,
        checked_relation_count: u16,
        violation_count: u16,
        unresolved_endpoint_count: u16,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct M0RVirtueViolation {
        virtue_index: u8,
        r_factor: u8,
        evidence: f32,
        coordinate: [c_char; M0_VERIFIER_COORDINATE_MAX],
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct M0PrincipleTriadCompilation {
        status: u8,
        triad_bits: u8,
    }

    #[repr(C)]
    struct M0VerifierReport {
        virtue_witness_vector: u16,
        virtue_scores: [f32; M0_VERIFIER_VIRTUE_COUNT],
        unsatisfied_count: u16,
        unsatisfied_constraints:
            [[c_char; M0_VERIFIER_COORDINATE_MAX]; M0_VERIFIER_MAX_UNSATISFIED],
        slot_privacy_boundary_compliance: u8,
        act_face: u8,
        witness_face: u8,
        syntax_witness_vector: u8,
        route_step_count: u8,
        band_turn_index: u8,
        r_factor_route: [RFactorPathStep; M0_VERIFIER_MAX_ROUTE_OUT],
        typed_query_count: u16,
        typed_queries: [M0TypedQuery; M0_VERIFIER_MAX_TYPED_QUERIES],
        backing_chain_count: u16,
        backing_chain: [M0CoordinateRef; M0_VERIFIER_MAX_BACKING_CHAIN],
        canonical_membership: u8,
        owl_validation: M0OwlValidationReport,
        r_virtue_violation_count: u16,
        r_virtue_violations: [M0RVirtueViolation; M0_VERIFIER_VIRTUE_COUNT],
        closure_marker: M0PrincipleTriadCompilation,
    }

    extern "C" {
        fn m0_verifier_check_state(state: *const KernelState, out: *mut M0VerifierReport) -> c_int;
        #[cfg(test)]
        fn m0_verifier_emit_query(state: *const KernelState, out: *mut M0TypedQuery) -> c_int;
        #[cfg(test)]
        fn m0_verifier_walk_backing(
            q: *const M0TypedQuery,
            anchor: *const M0CoordinateRef,
            depth: u8,
            out: *mut M0BackingChain,
        ) -> c_int;
        #[cfg(test)]
        fn m0_verifier_emit_question(
            report: *const M0VerifierReport,
            out_buf: *mut c_char,
            buf_len: usize,
        ) -> c_int;
        fn anuttara_language_is_member(coordinate_or_symbol: *const c_char) -> bool;
        static ARCHETYPE_COORDINATE_LUT: [*const c_char; 12];
        static CONTEMPLATION_PROMPT_LUT: [*const c_char; 12];
        static VIRTUE_LUT: [VirtueEntryC; 9];
    }

    /// Mirror of m0.h `Virtue_Entry` (the compiled 9-row virtue authority).
    #[repr(C)]
    struct VirtueEntryC {
        r_factor: u8,
        divine_act: u8,
        cross_branch_refs: u16,
        name: *const c_char,
        symbol: *const c_char,
    }

    /// One projected `VIRTUE_LUT` row. `r_factor` is `None` for the three
    /// meta rows (indices 0..=2, compiled as 0xFF) — the R-mapped virtues are
    /// rows 3..=8 per `VIRTUE_TO_RFACTOR`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct VirtueLutEntry {
        pub virtue_index: u8,
        pub r_factor: Option<u8>,
        pub name: String,
        pub symbol: String,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct M0VerifierBootstrapWitness {
        pub virtue_witness_vector: u16,
        pub syntax_witness_vector: u8,
        pub open_questions: Vec<String>,
        pub coherence_score: f32,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct M0VerifierStateInput {
        pub committed_virtue_mask: u16,
        pub virtue_evidence: [f32; M0_VERIFIER_VIRTUE_COUNT],
        pub observed_core_relation_count: u16,
        pub syntax_layer_mask: u16,
        pub active_archetype: u8,
        pub active_tct_position: u8,
        pub slot_privacy_boundary_compliance: bool,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct M0VerifierEvaluation {
        pub virtue_witness_vector: u16,
        pub virtue_scores: [f32; M0_VERIFIER_VIRTUE_COUNT],
        pub unsatisfied_constraints: Vec<String>,
        pub typed_queries: Vec<M0VerifierTypedQueryResult>,
        pub coherence_score: f32,
        pub slot_privacy_boundary_compliance: bool,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct M0VerifierTypedQueryResult {
        pub law_family: u8,
        pub query_kind: String,
        pub symbolic_coordinate_string: String,
    }

    /// Evaluates caller-supplied public verifier state through the compiled C
    /// substrate. Transport and request validation remain outside this crate.
    pub fn evaluate_state(input: &M0VerifierStateInput) -> Result<M0VerifierEvaluation, String> {
        let mut state: KernelState = unsafe { std::mem::zeroed() };
        state.committed_virtue_mask = input.committed_virtue_mask;
        state.virtue_evidence = input.virtue_evidence;
        state.observed_core_relation_count = input.observed_core_relation_count;
        state.syntax_layer_mask = input.syntax_layer_mask;
        state.active_archetype = input.active_archetype;
        state.active_tct_position = input.active_tct_position;
        state.slot_privacy_boundary_compliance = u8::from(input.slot_privacy_boundary_compliance);

        let mut report = zeroed_report();
        let status = unsafe { m0_verifier_check_state(&state, &mut report) };
        if status != 0 {
            return Err(format!("compiled M0 verifier failed with status {status}"));
        }

        let unsatisfied_constraints = report
            .unsatisfied_constraints
            .iter()
            .take(report.unsatisfied_count as usize)
            .map(|constraint| cstr(constraint).to_owned())
            .collect();
        let typed_queries = report
            .typed_queries
            .iter()
            .take(report.typed_query_count as usize)
            .map(|query| M0VerifierTypedQueryResult {
                law_family: query.law_family,
                query_kind: cstr(&query.query_kind).to_owned(),
                symbolic_coordinate_string: cstr(&query.symbolic_coordinate_string).to_owned(),
            })
            .collect();

        Ok(M0VerifierEvaluation {
            virtue_witness_vector: report.virtue_witness_vector,
            virtue_scores: report.virtue_scores,
            unsatisfied_constraints,
            typed_queries,
            coherence_score: report.virtue_witness_vector.count_ones() as f32
                / M0_VERIFIER_VIRTUE_COUNT as f32,
            slot_privacy_boundary_compliance: report.slot_privacy_boundary_compliance != 0,
        })
    }

    /// Uses the compiled 128-entry Anuttara registry; this does not infer
    /// membership from string shape.
    pub fn is_language_member(element: &str) -> Result<bool, String> {
        let element = CString::new(element)
            .map_err(|_| "language element must not contain a NUL byte".to_owned())?;
        Ok(unsafe { anuttara_language_is_member(element.as_ptr()) })
    }

    /// Projects the compiled C prompt authority without recreating its
    /// contents in Rust. Empty entries are intentional canonical absences.
    pub fn contemplation_prompt_lut() -> Vec<String> {
        unsafe { &CONTEMPLATION_PROMPT_LUT }
            .iter()
            .map(|prompt| {
                assert!(
                    !prompt.is_null(),
                    "compiled contemplation prompt pointers must not be null"
                );
                unsafe { CStr::from_ptr(*prompt) }
                    .to_str()
                    .expect("contemplation prompt must be UTF-8 compatible")
                    .to_owned()
            })
            .collect()
    }

    /// Projects the compiled 9-row `VIRTUE_LUT` (name, symbol, R-factor)
    /// without recreating its contents in Rust. 25.T25.23: the fretboard's
    /// virtue lamps consume these symbols over the profile wire — the one
    /// place they exist is the C authority, never a renderer-local copy.
    pub fn virtue_lut() -> Vec<VirtueLutEntry> {
        unsafe { &VIRTUE_LUT }
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                assert!(
                    !entry.name.is_null() && !entry.symbol.is_null(),
                    "compiled virtue pointers must not be null"
                );
                VirtueLutEntry {
                    virtue_index: index as u8,
                    r_factor: (entry.r_factor != 0xFF).then_some(entry.r_factor),
                    name: unsafe { CStr::from_ptr(entry.name) }
                        .to_str()
                        .expect("virtue name must be UTF-8 compatible")
                        .to_owned(),
                    symbol: unsafe { CStr::from_ptr(entry.symbol) }
                        .to_str()
                        .expect("virtue symbol must be UTF-8 compatible")
                        .to_owned(),
                }
            })
            .collect()
    }

    /// Projects the canonical graph identity of every compiled ARCHETYPE_LUT
    /// slot. S2 uses this to subtract kernel-lifted rows from the M0 graph
    /// browser without maintaining a second twelve-entry mapping.
    pub fn archetype_coordinate_lut() -> Vec<String> {
        unsafe { &ARCHETYPE_COORDINATE_LUT }
            .iter()
            .map(|coordinate| {
                assert!(
                    !coordinate.is_null(),
                    "compiled archetype coordinate pointers must not be null"
                );
                unsafe { CStr::from_ptr(*coordinate) }
                    .to_str()
                    .expect("archetype coordinate must be UTF-8 compatible")
                    .to_owned()
            })
            .collect()
    }

    /// Runs the compiled C verifier over the information a generic public
    /// kernel tick genuinely has: canonical relation coverage and its safe
    /// boundary, but no user/session virtue evidence or syntax stamps.
    pub fn bootstrap_witness_for_tick(tick12: u8, position6: u8) -> M0VerifierBootstrapWitness {
        let mut state: KernelState = unsafe { std::mem::zeroed() };
        state.observed_core_relation_count = 65;
        state.active_archetype = tick12 % 12;
        state.active_tct_position = position6 % 6;
        state.slot_privacy_boundary_compliance = 1;

        let mut report = zeroed_report();
        let status = unsafe { m0_verifier_check_state(&state, &mut report) };
        assert_eq!(
            status, 0,
            "the compiled M0 verifier accepts its bootstrap state"
        );

        let open_questions = report
            .typed_queries
            .iter()
            .take(report.typed_query_count as usize)
            .map(|query| cstr(&query.symbolic_coordinate_string).to_owned())
            .collect();
        let coherence_score =
            report.virtue_witness_vector.count_ones() as f32 / M0_VERIFIER_VIRTUE_COUNT as f32;

        M0VerifierBootstrapWitness {
            virtue_witness_vector: report.virtue_witness_vector,
            syntax_witness_vector: report.syntax_witness_vector,
            open_questions,
            coherence_score,
        }
    }

    fn zeroed_report() -> M0VerifierReport {
        unsafe { std::mem::zeroed() }
    }

    fn cstr(field: &[c_char]) -> &str {
        unsafe { CStr::from_ptr(field.as_ptr()) }
            .to_str()
            .expect("verifier string must be UTF-8 compatible")
    }

    /// Closing act-route: an operative pravritti act (R1 at O#, fret 0 per
    /// the R-distribution matrix), a nivritti step (R4 at O#, fret 5), then
    /// the R5/Samavesa positionless return-to-matrix.
    #[cfg(test)]
    fn closing_route() -> ([RFactorPathStep; M0_VERIFIER_MAX_ROUTE_IN], u8) {
        let mut steps = [RFactorPathStep::default(); M0_VERIFIER_MAX_ROUTE_IN];
        steps[0] = RFactorPathStep {
            r_factor: 1,
            base_route: 0,
            band: R_BAND_PRAVRITTI,
            position: 0,
        };
        steps[1] = RFactorPathStep {
            r_factor: 4,
            base_route: 0,
            band: R_BAND_NIVRITTI,
            position: 5,
        };
        steps[2] = RFactorPathStep {
            r_factor: 5,
            base_route: 6,
            band: R_BAND_NIVRITTI,
            position: 7,
        };
        (steps, 3)
    }

    #[cfg(test)]
    fn fully_witnessed_state() -> KernelState {
        let (route_steps, route_step_count) = closing_route();
        let mut state: KernelState = unsafe { std::mem::zeroed() };
        state.committed_virtue_mask = 0x01ff;
        state.virtue_evidence = [1.0; M0_VERIFIER_VIRTUE_COUNT];
        state.observed_core_relation_count = 65;
        state.syntax_layer_mask = M0_VERIFIER_SYNTAX_SPEECH
            | M0_VERIFIER_SYNTAX_RELATIONSHIP
            | M0_VERIFIER_SYNTAX_ACTION
            | M0_VERIFIER_SYNTAX_COMPLETION;
        state.active_archetype = 7;
        state.active_tct_position = 0;
        state.slot_privacy_boundary_compliance = 1;
        state.route_steps = route_steps;
        state.route_step_count = route_step_count;
        state
    }

    #[cfg(test)]
    fn set_engaged(state: &mut KernelState, index: usize, coordinate: &str) {
        let bytes = coordinate.as_bytes();
        for (i, b) in bytes.iter().enumerate() {
            state.engaged_coordinates[index][i] = *b as c_char;
        }
        state.engaged_coordinates[index][bytes.len()] = 0;
        if state.engaged_coordinate_count <= index as u8 {
            state.engaged_coordinate_count = index as u8 + 1;
        }
    }

    #[test]
    fn checks_against_virtue_lut() {
        let state = fully_witnessed_state();
        let mut report = zeroed_report();

        let status = unsafe { m0_verifier_check_state(&state, &mut report) };

        assert_eq!(status, 0);
        assert_eq!(report.virtue_witness_vector & 0x01ff, 0x01ff);
        assert_eq!(report.unsatisfied_count, 0);
        assert_eq!(report.typed_query_count, 0);
        assert_eq!(report.slot_privacy_boundary_compliance, 1);
        assert_eq!(report.act_face, 0);
        assert_eq!(report.witness_face, 1);
        assert_eq!(report.syntax_witness_vector, 0x0f);
        assert_eq!(report.canonical_membership, 1);
        for score in report.virtue_scores {
            assert!(score > 0.99);
        }
        // One trajectory, two faces: the witnessed virtues ground through
        // the identity chains into the report-level backing trace.
        assert!(report.backing_chain_count >= 9);
        let first_backing = cstr(&report.backing_chain[0].coordinate);
        assert_eq!(first_backing, "M0-2-9-0");
        // Kernel-static ontology validation covers the full 65 skeleton.
        assert_eq!(report.owl_validation.checked_relation_count, 65);
        assert_eq!(report.owl_validation.violation_count, 0);
        assert_eq!(report.owl_validation.live_n10s_deferred, 1);
        // Archetype-7 closure: pravritti -> (@#) -> nivritti -> R5 compiles
        // back to the principle triad (##) and (R#) and (#R).
        assert_eq!(report.closure_marker.status, M0_TRIAD_COMPILES);
        assert_eq!(report.closure_marker.triad_bits, 0b111);
        assert_ne!(report.band_turn_index, 0xff);
        assert_eq!(
            report.r_factor_route[report.band_turn_index as usize].band,
            R_BAND_TURN
        );
    }

    #[test]
    fn reports_slot_privacy_boundary_violation() {
        let mut state = fully_witnessed_state();
        state.slot_privacy_boundary_compliance = 0;
        let mut report = zeroed_report();

        let status = unsafe { m0_verifier_check_state(&state, &mut report) };

        assert_eq!(status, 0);
        assert_eq!(report.slot_privacy_boundary_compliance, 0);
        assert!(report.unsatisfied_count > 0);
        let first_constraint = cstr(&report.unsatisfied_constraints[0]);
        assert_eq!(first_constraint, "#R0-0/1/P-T0-slot-privacy-boundary?");
        assert_eq!(report.typed_query_count, 1);
        // Law 2 — the slot privacy boundary is a Frame; breaching it is a
        // containment breach, not a generic Law-6 interrogative.
        assert_eq!(
            report.typed_queries[0].law_family,
            M0_ANUTTARA_LAW_CONTAINMENT
        );
        let first_query = cstr(&report.typed_queries[0].symbolic_coordinate_string);
        assert_eq!(first_query, first_constraint);
    }

    #[test]
    fn emits_symbolic_coordinate_string() {
        let mut state = fully_witnessed_state();
        state.virtue_evidence[8] = 0.0;
        state.syntax_layer_mask &= !M0_VERIFIER_SYNTAX_COMPLETION;
        let mut report = zeroed_report();
        unsafe { m0_verifier_check_state(&state, &mut report) };
        let mut question = [0 as c_char; 128];

        let status =
            unsafe { m0_verifier_emit_question(&report, question.as_mut_ptr(), question.len()) };

        assert_eq!(status, 0);
        let question = unsafe { CStr::from_ptr(question.as_ptr()) }
            .to_str()
            .expect("verifier question must be UTF-8 compatible");
        assert_eq!(question, "#R5-0/1/A-T9-unwitnessed?");
    }

    /// Round-trip per the Tranche 1.10 verification line: a state with
    /// unwitnessed Archetype-9 wholeness emits a typed query whose
    /// symbolic-coordinate-string carries the T9/unwitnessed address, and
    /// the query's anchor walks a grounded backing chain to the M0-0 root.
    #[test]
    fn round_trip_unwitnessed_wholeness_grounds_backing_chain() {
        let mut state = fully_witnessed_state();
        state.virtue_evidence[8] = 0.0;

        let mut query: M0TypedQuery = unsafe { std::mem::zeroed() };
        let status = unsafe { m0_verifier_emit_query(&state, &mut query) };
        assert_eq!(status, 0);
        assert_eq!(query.law_family, M0_ANUTTARA_LAW_EIGHT_PLUS_ONE);
        let symbolic = cstr(&query.symbolic_coordinate_string);
        assert!(
            symbolic.starts_with('#'),
            "EBNF: coordinate-string starts with #"
        );
        assert!(
            symbolic.ends_with('?'),
            "DR-MP-3: the verifier raises questions"
        );
        assert!(
            symbolic.contains("T9"),
            "Archetype-9 wholeness address: {symbolic}"
        );
        assert!(symbolic.contains("unwitnessed"), "state-marker: {symbolic}");

        let mut anchor: M0CoordinateRef = unsafe { std::mem::zeroed() };
        for (i, b) in b"M0-2-9-8".iter().enumerate() {
            anchor.coordinate[i] = *b as c_char;
        }
        let mut chain: M0BackingChain = unsafe { std::mem::zeroed() };
        let status = unsafe { m0_verifier_walk_backing(&query, &anchor, 4, &mut chain) };
        assert_eq!(status, 0);
        assert_eq!(chain.grounded, 1);
        assert_eq!(chain.link_count, 2);
        assert_eq!(cstr(&chain.links[0].coordinate), "M0-(4.5/0)-0");
        assert_eq!(cstr(&chain.links[1].coordinate), "M0-0");

        // The emitted vocabulary is bound to the 128 registry.
        for member in ["M0-2-9-8", "M0-1-(0/1)", "O#", "(@#)", "L'"] {
            let c = CString::new(member).unwrap();
            assert!(
                unsafe { anuttara_language_is_member(c.as_ptr()) },
                "{member} must be a registry member"
            );
        }
        let absent = CString::new("M9-NOT-A-COORDINATE").unwrap();
        assert!(!unsafe { anuttara_language_is_member(absent.as_ptr()) });
    }

    #[test]
    fn inserts_band_turn_marker_and_reports_non_closing_route() {
        // Band flip without an explicit (@#) step: the verifier inserts the
        // marker at the flip position (Beauty -> Life pivot).
        let mut state = fully_witnessed_state();
        let mut steps = [RFactorPathStep::default(); M0_VERIFIER_MAX_ROUTE_IN];
        steps[0] = RFactorPathStep {
            r_factor: 2,
            base_route: 1,
            band: R_BAND_PRAVRITTI,
            position: 0,
        };
        steps[1] = RFactorPathStep {
            r_factor: 3,
            base_route: 1,
            band: R_BAND_NIVRITTI,
            position: 5,
        };
        state.route_steps = steps;
        state.route_step_count = 2;

        let mut report = zeroed_report();
        unsafe { m0_verifier_check_state(&state, &mut report) };

        assert_eq!(report.route_step_count, 3, "marker step inserted");
        assert_eq!(report.band_turn_index, 1);
        assert_eq!(report.r_factor_route[1].band, R_BAND_TURN);
        assert_eq!(
            report.r_factor_route[1].position, 0,
            "Siva-instruction-0 seed"
        );
        // No R5 return-to-matrix: the trajectory does not close (None).
        assert_eq!(report.closure_marker.status, M0_TRIAD_NOT_CLOSING);
        assert_eq!(report.closure_marker.triad_bits, 0);
    }

    #[test]
    fn flags_non_member_engaged_coordinate() {
        let mut state = fully_witnessed_state();
        set_engaged(&mut state, 0, "M0-2-9-4");
        set_engaged(&mut state, 1, "M9-77-UNKNOWN");

        let mut report = zeroed_report();
        unsafe { m0_verifier_check_state(&state, &mut report) };

        assert_eq!(report.canonical_membership, 0);
        assert_eq!(report.typed_query_count, 1);
        assert_eq!(
            report.typed_queries[0].law_family,
            M0_ANUTTARA_LAW_DERIVATION
        );
        let symbolic = cstr(&report.typed_queries[0].symbolic_coordinate_string);
        assert!(symbolic.contains("violated"), "{symbolic}");
    }

    #[test]
    fn archetype_coordinate_lut_projects_the_compiled_twelve_slot_authority() {
        assert_eq!(
            archetype_coordinate_lut(),
            vec![
                "M0-3-(0/1)",
                "M0-3-4",
                "M0-3-2",
                "M0-3-3",
                "M0-3-5",
                "M0-3-6",
                "M0-3-7",
                "M0-3-8",
                "M0-3-9",
                "M0-3-10",
                "M0-3-11",
                "M0-2-9",
            ]
        );
    }
}

/// Tranche 19.T19.1 — the ARCHETYPE_LUT ordering fix, pinned behaviorally.
/// Dataset law (docs/m0-archetype-lut-ordering-fix.md): archetypal numbers
/// 0-9 live at LUT index number+2; the odd archetypes route to their
/// canonical sub-tables.
#[cfg(test)]
mod m0_archetype_routing {
    const SUB_TABLE_NONE: u8 = 0;
    const SUB_TABLE_ZODIACAL: u8 = 1;
    const SUB_TABLE_MONOPOLY: u8 = 2;
    const SUB_TABLE_DIVINE: u8 = 3;
    const SUB_TABLE_VIRTUE: u8 = 4;

    const POLARITY_ADAM: u8 = 0;
    const POLARITY_EVE: u8 = 1;
    const POLARITY_NEUTRAL: u8 = 2;

    /// Prefix mirror of Archetype_Entry (m0.h): the eight leading u8 fields.
    /// Read through the resolver's pointer, never by array stride.
    #[repr(C)]
    struct ArchetypeEntryHead {
        index: u8,
        dimensionality: u8,
        polarity: u8,
        complement_idx: u8,
        weave_anchor: [u8; 2],
        sub_table_type: u8,
        sub_table_size: u8,
    }

    extern "C" {
        fn m0_resolve_archetypal_number(number: u8) -> *const ArchetypeEntryHead;
    }

    fn head(number: u8) -> &'static ArchetypeEntryHead {
        let ptr = unsafe { m0_resolve_archetypal_number(number) };
        assert!(!ptr.is_null(), "number {number} must resolve");
        unsafe { &*ptr }
    }

    #[test]
    fn odd_archetypes_route_to_canonical_sub_tables() {
        // Archetype 3 (Vak) -> ZODIACAL_LUT[12]
        assert_eq!(head(3).sub_table_type, SUB_TABLE_ZODIACAL);
        assert_eq!(head(3).sub_table_size, 12);
        // Archetype 5 (Dynamic Harmony / Mono-Poly) -> MONOPOLY_LUT[7]
        assert_eq!(head(5).sub_table_type, SUB_TABLE_MONOPOLY);
        assert_eq!(head(5).sub_table_size, 7);
        // Archetype 7 (Divine Action / Ananda-Tandava) -> DIVINE_ACT_LUT[7]
        assert_eq!(head(7).sub_table_type, SUB_TABLE_DIVINE);
        assert_eq!(head(7).sub_table_size, 7);
        // Archetype 9 (Paramesvara / Wholeness) -> VIRTUE_LUT[9]
        assert_eq!(head(9).sub_table_type, SUB_TABLE_VIRTUE);
        assert_eq!(head(9).sub_table_size, 9);
    }

    #[test]
    fn even_archetypes_carry_no_sub_table() {
        for number in [0u8, 1, 2, 4, 6, 8] {
            let entry = head(number);
            assert_eq!(entry.sub_table_type, SUB_TABLE_NONE, "number {number}");
            assert_eq!(entry.sub_table_size, 0, "number {number}");
        }
    }

    #[test]
    fn resolver_maps_number_to_index_plus_two_and_bounds() {
        for number in 0u8..=9 {
            assert_eq!(head(number).index, number + 2);
        }
        assert!(unsafe { m0_resolve_archetypal_number(10) }.is_null());
        assert!(unsafe { m0_resolve_archetypal_number(0xff) }.is_null());
    }

    #[test]
    fn complement_pairs_are_reciprocal_and_polarities_match_dataset() {
        // Numbers 0/1 are transcendent roots: NEUTRAL, mutually complementary.
        assert_eq!(head(0).polarity, POLARITY_NEUTRAL);
        assert_eq!(head(1).polarity, POLARITY_NEUTRAL);
        // Number 6 (Synthetic Emptiness at index 8) is the fourth ADAM;
        // number 9 (Paramesvara at index 11) is the transcendent EVE.
        assert_eq!(head(6).polarity, POLARITY_ADAM);
        assert_eq!(head(9).polarity, POLARITY_EVE);
        // Every pair is reciprocal: partner's complement points back.
        for number in 0u8..=9 {
            let entry = head(number);
            assert_ne!(entry.complement_idx, 0xff, "number {number} has a partner");
            let partner_number = entry.complement_idx - 2;
            assert_eq!(
                head(partner_number).complement_idx,
                entry.index,
                "complement of number {number} must be reciprocal"
            );
        }
    }
}

#[cfg(test)]
mod m0_m2_parity {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    const ELEMENT_ID_AKASHA: u8 = 0;
    const ELEMENT_ID_VAYU: u8 = 1;
    const ELEMENT_ID_AGNI: u8 = 2;
    const ELEMENT_ID_APAS: u8 = 3;
    const ELEMENT_ID_PRITHVI: u8 = 4;
    /// Spec name for the tattvic Akasha id (== ELEMENT_ID_AKASHA in m2.h).
    const TATTVA_AKASHA: u8 = ELEMENT_ID_AKASHA;

    const PLANET_SATURN: u8 = 6;

    /// 19.10(c)/(d) — L2' alchemical ids per the 05.16 naming canon:
    /// index 5 is SALT (the fixed body), never Mineral.
    const M_ELEM_AETHER: u8 = 0;
    const M_ELEM_SALT: u8 = 5;

    /// Classical element block -> the tattva every decan of that block
    /// carries in M2_DECAN_DESC (Fire/Earth/Air/Water order).
    const BLOCK_TATTVA: [u8; 4] = [
        ELEMENT_ID_AGNI,
        ELEMENT_ID_PRITHVI,
        ELEMENT_ID_VAYU,
        ELEMENT_ID_APAS,
    ];

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct DecanFaceDesc {
        element: u8,
        sign: u8,
        decan: u8,
        face: u8,
        ruling_planet: u8,
        pad: u8,
        meaning_id: u16,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct M0M2ZodiacalBridgeEntry {
        vak_symbol: *const c_char,
        m0_resonance_idx: u8,
        m0_successor: u8,
        element: u8,
        mode: u8,
        m2_sign_idx: u8,
        decan_planets: [u8; 3],
        first_decan_idx_72: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct PsychoidPlanetaryEntry {
        l0_prime_position: u8,
        archetypal_number: u8,
        planet_id: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct AlchemicalTattvicEntry {
        alchemical: u8,
        tattvic: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct ZodiacalEntry {
        symbol: *const c_char,
        resonance: u8,
        successor: u8,
        zodiacal_quality: u8,
    }

    extern "C" {
        static M0_M2_ZODIACAL_BRIDGE: [M0M2ZodiacalBridgeEntry; 12];
        static PSYCHOID_PLANETARY_CORRESPONDENCE: [PsychoidPlanetaryEntry; 7];
        static ALCHEMICAL_TO_TATTVIC: [AlchemicalTattvicEntry; 6];
        static M2_DECAN_DESC: [DecanFaceDesc; 72];
        static ZODIACAL_LUT: [ZodiacalEntry; 12];
    }

    #[test]
    fn parity_bridges_consistent() {
        let bridge = unsafe { M0_M2_ZODIACAL_BRIDGE };
        let psychoid = unsafe { PSYCHOID_PLANETARY_CORRESPONDENCE };
        let alchemical = unsafe { ALCHEMICAL_TO_TATTVIC };
        let decans = unsafe { M2_DECAN_DESC };
        let zodiacal_lut = unsafe { ZODIACAL_LUT };

        // (a) Each m2_sign_idx x element pair carries 3 DISTINCT
        // decan_planets equal to the first 3 (light-face) decans of the
        // corresponding sign in M2_DECAN_DESC[72]; the M0 side mirrors
        // ZODIACAL_LUT exactly.
        for (sign, entry) in bridge.iter().enumerate() {
            assert_eq!(entry.m2_sign_idx as usize, sign);
            assert_eq!(entry.m0_resonance_idx as usize, sign);
            assert_eq!(entry.m0_successor as usize, (sign + 1) % 12);

            let lut = zodiacal_lut[sign];
            let bridge_symbol = unsafe { CStr::from_ptr(entry.vak_symbol) };
            let lut_symbol = unsafe { CStr::from_ptr(lut.symbol) };
            assert_eq!(bridge_symbol, lut_symbol, "sign {sign} vak symbol");
            assert_eq!(entry.element, (lut.zodiacal_quality >> 2) & 0x03);
            assert_eq!(entry.mode, lut.zodiacal_quality & 0x03);

            let planets = entry.decan_planets;
            assert!(
                planets[0] != planets[1] && planets[1] != planets[2] && planets[0] != planets[2],
                "sign {sign} must carry 3 distinct decan planets"
            );

            let first = entry.first_decan_idx_72 as usize;
            assert_eq!(first % 6, 0, "first decan index must open a sign block");
            for decan in 0..3 {
                let face = decans[first + decan * 2];
                assert_eq!(face.face, 0, "bridge points at light faces");
                assert_eq!(face.decan as usize, decan);
                assert_eq!(
                    face.ruling_planet, planets[decan],
                    "sign {sign} decan {decan} planet parity"
                );
                assert_eq!(
                    face.element, BLOCK_TATTVA[entry.element as usize],
                    "sign {sign} element-block parity"
                );
            }
        }

        // (b) Parent-as-7th: the L0' lens parent (7th-Boundary) is Saturn,
        // closing the Sun..Saturn classical sequence.
        assert_eq!(psychoid[6].planet_id, PLANET_SATURN);
        assert_eq!(psychoid[6].archetypal_number, 7);
        for (idx, entry) in psychoid.iter().enumerate() {
            assert_eq!(entry.l0_prime_position as usize, idx);
            assert_eq!(entry.archetypal_number as usize, idx + 1);
            assert_eq!(entry.planet_id as usize, idx, "Sun..Saturn in order");
        }

        // (c) Aether and Salt both route to Akasha — prima and ultima
        // materia, the Möbius return of the elemental cycle.
        assert_eq!(alchemical[0].tattvic, TATTVA_AKASHA);
        assert_eq!(alchemical[5].tattvic, TATTVA_AKASHA);
        assert_eq!(alchemical[0].alchemical, M_ELEM_AETHER);

        // (d) The element constant at index 5 is SALT per the 05.16 naming
        // canon (the machine grep over m0.h guards the C-side name).
        assert_eq!(alchemical[M_ELEM_SALT as usize].alchemical, M_ELEM_SALT);
        // Earth/Water/Air/Fire land on their tattvas between the two poles.
        assert_eq!(alchemical[1].tattvic, ELEMENT_ID_PRITHVI);
        assert_eq!(alchemical[2].tattvic, ELEMENT_ID_APAS);
        assert_eq!(alchemical[3].tattvic, ELEMENT_ID_VAYU);
        assert_eq!(alchemical[4].tattvic, ELEMENT_ID_AGNI);
    }
}

#[cfg(test)]
mod m3_major_arcana_transcription {
    use std::os::raw::c_char;

    const M3_MAJOR_ARCANA_COUNT: usize = 22;
    const M3_STOP_CODON_AA: u8 = 10;
    const M3_NO_ARCANA: u8 = 0xFF;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct M3MajorArcanaEntry {
        card_id: u8,
        name: *const c_char,
        chromosome_pair: u8,
        amino_acid_index: u8,
    }

    extern "C" {
        static M3_CODON_TO_AA: [u8; 64];
        static M3_MAJOR_ARCANA: [M3MajorArcanaEntry; M3_MAJOR_ARCANA_COUNT];
        fn m3_major_arcana_from_codon(codon: u8) -> u8;
    }

    #[test]
    fn m3_major_arcana_from_codon_roundtrip() {
        let codon_to_aa = unsafe { M3_CODON_TO_AA };
        let major_arcana = unsafe { M3_MAJOR_ARCANA };

        for codon in 0u8..64u8 {
            let aa_index = codon_to_aa[codon as usize];
            let card = unsafe { m3_major_arcana_from_codon(codon) };

            if aa_index == M3_STOP_CODON_AA {
                assert_eq!(card, M3_NO_ARCANA, "STOP codon {codon:#04x}");
                continue;
            }

            let expected = major_arcana
                .iter()
                .position(|entry| entry.amino_acid_index == aa_index)
                .map(|idx| idx as u8)
                .unwrap_or(M3_NO_ARCANA);

            assert_eq!(
                card, expected,
                "codon {codon:#04x} should reverse-map amino acid {aa_index}"
            );
            assert_ne!(card, M3_NO_ARCANA, "non-STOP codon {codon:#04x}");
            assert_eq!(major_arcana[card as usize].amino_acid_index, aa_index);
        }

        assert_eq!(unsafe { m3_major_arcana_from_codon(64) }, M3_NO_ARCANA);
    }
}

#[cfg(test)]
mod m4_session_lifecycle {
    use std::os::raw::{c_char, c_int};

    const M3_GOVERNANCE_ROLE_NONE: u8 = 0;
    const M3_GOVERNANCE_ROLE_START: u8 = 1;
    const M3_GOVERNANCE_ROLE_STOP: u8 = 2;
    const M3_CODON_ATG_AUG_VALUE: u8 = 0x07;
    const M3_STOP_CODON_TGA_VALUE: u8 = 0x1c;
    const M4_TRANSCRIPTION_STEP_START: u8 = 1 << 0;
    const M4_TRANSCRIPTION_STEP_STOP: u8 = 1 << 1;
    const M4_TRANSCRIPTION_STEP_TAIL: u8 = 1 << 2;
    const M4_TRANSCRIPTION_TAIL_MARKER_CODON: u8 = 0xfe;
    const M4_SYMBOLIC_PROTEIN_MAX_STEPS: usize = 256;

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct NucleotideBalance {
        adenine_water: u8,
        thymine_fire: u8,
        cytosine_earth: u8,
        guanine_air: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct M4SymbolDnaProfile {
        gene_keys_activation: u64,
        nucleotide_balance: NucleotideBalance,
        sun_degree_anchor: u16,
        moon_degree_anchor: u16,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct M4NumerologicalLayer {
        numerological_key: u32,
        sixfold_difference: u8,
        sixfold_sum: u8,
        life_path: u8,
        _pad: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct M4AstrologicalLayer {
        sun_degree_anchor: u16,
        moon_degree_anchor: u16,
        asc_degree_anchor: u16,
        mc_degree_anchor: u16,
        planet_degrees: [u16; 10],
        dominant_sign: u8,
        dominant_element: u8,
        dominant_modality: u8,
        _pad: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct M4JungianLayer {
        nucleotide_balance: NucleotideBalance,
        mbti_raw: u8,
        dominant_function: u8,
        auxiliary_function: u8,
        enneagram_type: u8,
        enneagram_wing: u8,
        _pad: [u8; 3],
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct M4GeneKeysLayer {
        gene_keys_activation: u64,
        shadow_mask: u64,
        gift_mask: u64,
        siddhi_mask: u64,
        life_work_hex: u8,
        evolution_hex: u8,
        radiance_hex: u8,
        purpose_hex: u8,
        attraction_hex: u8,
        iq_hex: u8,
        eq_hex: u8,
        sq_hex: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct M4HumanDesignLayer {
        hd_type: u8,
        hd_authority: u8,
        hd_profile: [u8; 2],
        hd_definition: u8,
        incarnation_cross: u8,
        defined_channels: u16,
        defined_gates: [u32; 2],
        _pad: [u8; 4],
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct M4IdentityMatrix {
        layer_presence: u8,
        _pad_lp: [u8; 7],
        layer_0: M4NumerologicalLayer,
        layer_1: M4AstrologicalLayer,
        layer_2: M4JungianLayer,
        layer_3: M4GeneKeysLayer,
        layer_4: M4HumanDesignLayer,
        dna_profile: M4SymbolDnaProfile,
        quintessence_hash: [u8; 32],
        quintessence_preview: [c_char; 65],
        numerological_key: u32,
        jung_type: u8,
        computed: bool,
    }

    impl Default for M4IdentityMatrix {
        fn default() -> Self {
            Self {
                layer_presence: 0,
                _pad_lp: [0; 7],
                layer_0: M4NumerologicalLayer::default(),
                layer_1: M4AstrologicalLayer::default(),
                layer_2: M4JungianLayer::default(),
                layer_3: M4GeneKeysLayer::default(),
                layer_4: M4HumanDesignLayer::default(),
                dna_profile: M4SymbolDnaProfile::default(),
                quintessence_hash: [0; 32],
                quintessence_preview: [0; 65],
                numerological_key: 0,
                jung_type: 0,
                computed: false,
            }
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct M4TarotDraw {
        cards: [u8; 78],
        drawn: [u8; 12],
        draw_count: u8,
        spread_type: u8,
        cast_degree: u16,
    }

    impl Default for M4TarotDraw {
        fn default() -> Self {
            Self {
                cards: [0; 78],
                drawn: [0; 12],
                draw_count: 0,
                spread_type: 0,
                cast_degree: 0,
            }
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct M4TranscriptionStep {
        degree: u16,
        hexagram: u8,
        codon: u8,
        amino_acid: u8,
        transcript_class: u8,
        governance_role: u8,
        flags: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct M4SymbolicProtein {
        session_id: [c_char; 64],
        start_codon: u8,
        stop_codon: u8,
        sealed: u8,
        truncated: u8,
        kairos_open: u64,
        kairos_close: u64,
        identity_hash: [u8; 32],
        step_count: u32,
        capacity: u32,
        has_mythos_archetype_reading: u8,
        mythos_archetype_reading: [c_char; 256],
        steps: [M4TranscriptionStep; M4_SYMBOLIC_PROTEIN_MAX_STEPS],
    }

    impl Default for M4SymbolicProtein {
        fn default() -> Self {
            Self {
                session_id: [0; 64],
                start_codon: 0,
                stop_codon: 0,
                sealed: 0,
                truncated: 0,
                kairos_open: 0,
                kairos_close: 0,
                identity_hash: [0; 32],
                step_count: 0,
                capacity: 0,
                has_mythos_archetype_reading: 0,
                mythos_archetype_reading: [0; 256],
                steps: [M4TranscriptionStep::default(); M4_SYMBOLIC_PROTEIN_MAX_STEPS],
            }
        }
    }

    #[repr(C)]
    struct M4SessionFrame {
        kairos: u64,
        identity: *mut M4IdentityMatrix,
        tarot_psyche_anchor: M4TarotDraw,
        protein_storage: M4SymbolicProtein,
        protein: *mut M4SymbolicProtein,
        stop_codon_policy: c_int,
        opened: bool,
    }

    impl Default for M4SessionFrame {
        fn default() -> Self {
            Self {
                kairos: 0,
                identity: std::ptr::null_mut(),
                tarot_psyche_anchor: M4TarotDraw::default(),
                protein_storage: M4SymbolicProtein::default(),
                protein: std::ptr::null_mut(),
                stop_codon_policy: 0,
                opened: false,
            }
        }
    }

    extern "C" {
        fn m4_session_open(
            identity: *mut M4IdentityMatrix,
            kairos: u64,
            out: *mut M4SessionFrame,
        ) -> c_int;
        fn m4_session_close(frame: *mut M4SessionFrame, out: *mut M4SymbolicProtein) -> c_int;
        fn m4_symbolic_protein_append_step(
            protein: *mut M4SymbolicProtein,
            degree: u16,
            hexagram: u8,
            codon: u8,
            role: c_int,
        ) -> c_int;
    }

    #[test]
    fn m4_session_open_emits_start_codon() {
        let mut identity = M4IdentityMatrix::default();
        identity.numerological_key = 42;
        let mut frame = M4SessionFrame::default();

        let status = unsafe { m4_session_open(&mut identity, 7205, &mut frame) };

        assert_eq!(status, 0);
        assert!(frame.opened);
        assert!(!frame.protein.is_null());
        let protein = unsafe { &*frame.protein };
        assert_eq!(protein.step_count, 1);
        assert_eq!(protein.start_codon, M3_CODON_ATG_AUG_VALUE);
        assert_eq!(protein.steps[0].codon, M3_CODON_ATG_AUG_VALUE);
        assert_eq!(protein.steps[0].governance_role, M3_GOVERNANCE_ROLE_START);
        assert_ne!(protein.steps[0].flags & M4_TRANSCRIPTION_STEP_START, 0);
        assert_eq!(frame.tarot_psyche_anchor.draw_count, 3);
    }

    #[test]
    fn m4_session_close_seals_protein_with_kairos_derived_stop() {
        let mut identity = M4IdentityMatrix::default();
        identity.numerological_key = 19;
        let mut frame = M4SessionFrame::default();
        let mut sealed = M4SymbolicProtein::default();

        let open_status = unsafe { m4_session_open(&mut identity, 7205, &mut frame) };
        assert_eq!(open_status, 0);
        let close_status = unsafe { m4_session_close(&mut frame, &mut sealed) };

        assert_eq!(close_status, 0);
        assert_eq!(sealed.sealed, 1);
        assert_eq!(sealed.stop_codon, M3_STOP_CODON_TGA_VALUE);
        assert_eq!(sealed.step_count, 2);
        assert_eq!(sealed.steps[1].governance_role, M3_GOVERNANCE_ROLE_STOP);
        assert_ne!(sealed.steps[1].flags & M4_TRANSCRIPTION_STEP_STOP, 0);
    }

    #[test]
    fn m4_session_protein_capacity_truncates_with_tail_marker() {
        let mut identity = M4IdentityMatrix::default();
        identity.numerological_key = 7;
        let mut frame = M4SessionFrame::default();

        let status = unsafe { m4_session_open(&mut identity, 8, &mut frame) };
        assert_eq!(status, 0);

        let protein = unsafe { &mut *frame.protein };
        protein.capacity = 2;
        let fill = unsafe {
            m4_symbolic_protein_append_step(
                frame.protein,
                9,
                0,
                0,
                M3_GOVERNANCE_ROLE_NONE as c_int,
            )
        };
        let truncate = unsafe {
            m4_symbolic_protein_append_step(
                frame.protein,
                10,
                1,
                1,
                M3_GOVERNANCE_ROLE_NONE as c_int,
            )
        };

        assert_eq!(fill, 0);
        assert_eq!(truncate, 0);
        let protein = unsafe { &*frame.protein };
        assert_eq!(protein.step_count, 2);
        assert_eq!(protein.truncated, 1);
        assert_eq!(protein.steps[1].codon, M4_TRANSCRIPTION_TAIL_MARKER_CODON);
        assert_ne!(protein.steps[1].flags & M4_TRANSCRIPTION_STEP_TAIL, 0);
    }
}

#[cfg(all(test, feature = "m0_verifier"))]
mod m0_virtue_lut_projection {
    use super::m0_verifier::virtue_lut;

    /// 25.T25.23 — the projection is the C authority verbatim: 9 rows, the
    /// three meta rows carry no R-factor, rows 3..=8 map to R0..=R5 exactly
    /// as `VIRTUE_TO_RFACTOR` computes, and every row carries its symbol.
    #[test]
    fn virtue_lut_projects_the_compiled_nine_rows_with_their_r_factors() {
        let lut = virtue_lut();
        assert_eq!(lut.len(), 9);
        for (index, entry) in lut.iter().enumerate() {
            assert_eq!(entry.virtue_index, index as u8);
            let expected = if index >= 3 { Some(index as u8 - 3) } else { None };
            assert_eq!(entry.r_factor, expected, "row {index} R-factor");
            assert!(!entry.name.is_empty(), "row {index} name");
            assert!(!entry.symbol.is_empty(), "row {index} symbol");
        }
        // The 0R Joy/Play symbol is the fretboard's drone lamp — pin one
        // literal so a re-ordered LUT cannot pass as a re-labelling.
        assert!(lut[3].symbol.starts_with("0R"), "lut[3] symbol: {}", lut[3].symbol);
    }
}

#[cfg(test)]
mod m0_contemplation_prompts {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    extern "C" {
        static CONTEMPLATION_PROMPT_LUT: [*const c_char; 12];
    }

    fn prompt_at(prompts: &[*const c_char; 12], idx: usize) -> &str {
        unsafe { CStr::from_ptr(prompts[idx]) }
            .to_str()
            .expect("contemplation prompt must be UTF-8 compatible")
    }

    #[test]
    fn contemplation_prompt_lut_size() {
        let prompts = unsafe { &CONTEMPLATION_PROMPT_LUT };

        assert_eq!(prompts.len(), 12);
        assert_eq!(prompt_at(prompts, 0), "");
        assert_eq!(prompt_at(prompts, 1), "");
        assert_eq!(prompt_at(prompts, 2), "");
        assert_eq!(
            prompt_at(prompts, 3),
            "Did your speech articulate identity or just signal? Where did naming become performance?"
        );
        assert_eq!(prompt_at(prompts, 4), "");
        assert_eq!(
            prompt_at(prompts, 5),
            "Did unity-multiplicity hold or did one side eat the other? Where was the mercurial crossroads refused?"
        );
        assert_eq!(prompt_at(prompts, 6), "");
        assert_eq!(
            prompt_at(prompts, 7),
            "Did the four causes integrate or did one dominate? Which act was missing?"
        );
        assert_eq!(prompt_at(prompts, 8), "");
        assert_eq!(
            prompt_at(prompts, 9),
            "Did the cycle complete in wholeness or close prematurely? Which virtue went unwitnessed?"
        );
        assert_eq!(prompt_at(prompts, 10), "");
        assert_eq!(prompt_at(prompts, 11), "");
    }
}

/// Tranche 01.T1.11 — the Anuttara symbolic-coordinate-string emission
/// protocol: a corpus of ≥12 distinct EBNF forms parses and re-renders
/// byte-identically (round-trip is the training signal per DR-MP-3), and
/// the kernel Verifier's real emissions live inside the same grammar.
/// The EBNF is the M0'-SPEC "Anuttara Symbolic-Coordinate-String EBNF"
/// section; the LLM-Nara `anuttara-symbolic-parse` skill (Tranche 5.21)
/// is the canonical agent-side parser of the same strings.
#[cfg(all(test, feature = "m0_verifier"))]
mod m0_symbolic_coordinate_string_round_trip {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    const STATE_MARKERS: [&str; 5] = ["pending", "unwitnessed", "drift", "incoherent", "violated"];

    #[derive(Debug, PartialEq)]
    struct CoordinateString {
        namespace: String,
        coordinates: Vec<String>,
        archetype: Option<String>,
        state_marker: Option<String>,
    }

    /// Longest-match namespace per the M0'-SPEC r-namespace production:
    /// nR-digit | R-digit | R# | #R | ## | L/M/C-digit.
    fn parse_namespace(body: &str) -> Option<(String, &str)> {
        let take = |n: usize| (body[..n].to_owned(), &body[n..]);
        let bytes = body.as_bytes();
        if body.starts_with('n')
            && bytes.get(1) == Some(&b'R')
            && bytes.get(2).is_some_and(|c| (b'0'..=b'5').contains(c))
        {
            return Some(take(3));
        }
        if body.starts_with('R') {
            if bytes.get(1) == Some(&b'#') {
                return Some(take(2));
            }
            if bytes.get(1).is_some_and(|c| (b'0'..=b'5').contains(c)) {
                return Some(take(2));
            }
        }
        if body.starts_with("##") || body.starts_with("#R") {
            return Some(take(2));
        }
        if (body.starts_with('L') || body.starts_with('M') || body.starts_with('C'))
            && bytes.get(1).is_some_and(u8::is_ascii_digit)
        {
            return Some(take(2));
        }
        None
    }

    fn is_coordinate(segment: &str) -> bool {
        if segment.is_empty() {
            return false;
        }
        // ql-coordinate: atoms of digits or letters joined by '/'
        let ql = segment.split('/').all(|atom| {
            !atom.is_empty()
                && (atom.bytes().all(|b| b.is_ascii_digit())
                    || atom.bytes().all(|b| b.is_ascii_alphabetic()))
        });
        // dotted-coordinate: digits joined by '.'
        let dotted = segment.contains('.')
            && segment
                .split('.')
                .all(|atom| !atom.is_empty() && atom.bytes().all(|b| b.is_ascii_digit()));
        // operator-coordinate: O/X/N/# alphabet
        let operator = segment
            .bytes()
            .all(|b| matches!(b, b'O' | b'X' | b'N' | b'M' | b'#'));
        ql || dotted || operator
    }

    fn parse(string: &str) -> Result<CoordinateString, String> {
        let body = string
            .strip_prefix('#')
            .ok_or("coordinate-string must open with #")?
            .strip_suffix('?')
            .ok_or("coordinate-string must close as a question")?;
        let (namespace, rest) = parse_namespace(body).ok_or("unknown namespace")?;
        let mut segments: Vec<&str> = rest
            .strip_prefix('-')
            .ok_or("namespace must be followed by at least one coordinate")?
            .split('-')
            .collect();

        let state_marker = segments
            .last()
            .filter(|s| STATE_MARKERS.contains(s))
            .map(|s| (*s).to_owned());
        if state_marker.is_some() {
            segments.pop();
        }
        let archetype = segments
            .last()
            .filter(|s| {
                s.len() >= 2 && s.starts_with('T') && s[1..].bytes().all(|b| b.is_ascii_digit())
            })
            .map(|s| (*s).to_owned());
        if archetype.is_some() {
            segments.pop();
        }

        if segments.is_empty() {
            return Err("at least one coordinate fragment required".to_owned());
        }
        for segment in &segments {
            if !is_coordinate(segment) {
                return Err(format!("illegal coordinate fragment `{segment}`"));
            }
        }

        Ok(CoordinateString {
            namespace,
            coordinates: segments.iter().map(|s| (*s).to_owned()).collect(),
            archetype,
            state_marker,
        })
    }

    fn render(parsed: &CoordinateString) -> String {
        let mut out = format!("#{}", parsed.namespace);
        for coordinate in &parsed.coordinates {
            out.push('-');
            out.push_str(coordinate);
        }
        if let Some(archetype) = &parsed.archetype {
            out.push('-');
            out.push_str(archetype);
        }
        if let Some(marker) = &parsed.state_marker {
            out.push('-');
            out.push_str(marker);
        }
        out.push('?');
        out
    }

    /// The binding corpus: the four M0'-SPEC examples plus the Verifier's
    /// own emission family plus the 01.T1.12-extended namespaces
    /// (nR chirality, ##/#R/R# principle triad, R5).
    const CORPUS: [&str; 14] = [
        "#R0-0/1/A-T7-pending?",
        "#L2-0/1/2-T9-unwitnessed?",
        "#M4-4.4.4.4-drift?",
        "#R3-O#X#N#-violated?",
        "#R5-0/1/A-T9-unwitnessed?",
        "#R0-0/1/S-T3-pending?",
        "#R1-0/1/R-T5-pending?",
        "#R2-0/1/A-T7-pending?",
        "#nR2-X#N#M#-incoherent?",
        "###-0/1-T9-unwitnessed?",
        "##R-7/8/9-T7-drift?",
        "#R#-5/0-T7-incoherent?",
        "#C5-5/0-violated?",
        "#M0-2/9/0-unwitnessed?",
    ];

    #[test]
    fn corpus_of_twelve_plus_distinct_forms_parses_round_trip() {
        let mut distinct = std::collections::BTreeSet::new();
        for member in CORPUS {
            let parsed = parse(member).unwrap_or_else(|err| panic!("`{member}` must parse: {err}"));
            assert_eq!(
                render(&parsed),
                member,
                "round-trip must preserve namespace, fragments, archetype, marker"
            );
            distinct.insert(member);
        }
        assert!(
            distinct.len() >= 12,
            "the binding corpus carries at least 12 distinct coordinate-string forms"
        );
    }

    #[test]
    fn namespace_families_are_distinguished_not_collapsed() {
        assert_eq!(parse("#R5-0/1-T9-pending?").unwrap().namespace, "R5");
        assert_eq!(parse("#R#-0/1-T9-pending?").unwrap().namespace, "R#");
        assert_eq!(parse("###-0/1?").unwrap().namespace, "##");
        assert_eq!(parse("##R-0/1?").unwrap().namespace, "#R");
        assert_eq!(parse("#nR3-0/1?").unwrap().namespace, "nR3");
        assert!(
            parse("#Q9-0/1-pending?").is_err(),
            "unknown namespace rejected"
        );
        assert!(parse("#R0-0/1-pending").is_err(), "questions end with ?");
    }

    // --- The Verifier's live emissions stay inside the grammar ------------

    const M0_VERIFIER_VIRTUE_COUNT: usize = 9;
    const M0_VERIFIER_COORDINATE_MAX: usize = 96;

    #[repr(C)]
    struct EmissionProbeState {
        committed_virtue_mask: u16,
        virtue_evidence: [f32; M0_VERIFIER_VIRTUE_COUNT],
        observed_core_relation_count: u16,
        syntax_layer_mask: u16,
        active_archetype: u8,
        active_tct_position: u8,
        slot_privacy_boundary_compliance: u8,
        route_step_count: u8,
        route_steps: [[u8; 4]; 16],
        engaged_coordinate_count: u8,
        engaged_coordinates: [[c_char; M0_VERIFIER_COORDINATE_MAX]; 8],
    }

    #[repr(C)]
    struct EmissionProbeQuery {
        law_family: u8,
        query_kind: [c_char; 32],
        symbolic_coordinate_string: [c_char; M0_VERIFIER_COORDINATE_MAX],
    }

    #[allow(clashing_extern_declarations)]
    extern "C" {
        #[link_name = "m0_verifier_emit_query"]
        fn m0_verifier_emit_query_probe(
            state: *const EmissionProbeState,
            out: *mut EmissionProbeQuery,
        ) -> i32;
    }

    #[test]
    fn live_verifier_emission_parses_through_the_canonical_grammar() {
        let mut state: EmissionProbeState = unsafe { std::mem::zeroed() };
        state.committed_virtue_mask = 0x01ff;
        state.virtue_evidence = [1.0; M0_VERIFIER_VIRTUE_COUNT];
        state.virtue_evidence[8] = 0.0; // unwitnessed Archetype-9 wholeness
        state.observed_core_relation_count = 65;
        state.syntax_layer_mask = 0x0f;
        state.slot_privacy_boundary_compliance = 1;

        let mut query: EmissionProbeQuery = unsafe { std::mem::zeroed() };
        let status = unsafe { m0_verifier_emit_query_probe(&state, &mut query) };
        assert_eq!(status, 0);

        let emitted = unsafe { CStr::from_ptr(query.symbolic_coordinate_string.as_ptr()) }
            .to_str()
            .expect("emission must be UTF-8");
        let parsed = parse(emitted).unwrap_or_else(|err| panic!("`{emitted}` must parse: {err}"));
        assert_eq!(render(&parsed), emitted, "emission round-trips");
        assert_eq!(parsed.state_marker.as_deref(), Some("unwitnessed"));
        assert_eq!(parsed.archetype.as_deref(), Some("T9"));
    }
}

/// Tranche 01.T1.12 / DR-(@#) — the band-turn law at the signature level:
/// the `(@#)` token TERMINATES the Beauty signature (2R, pravritti descent
/// ends at the Śakti-seed) and OPENS the Life signature (3R, nivritti
/// ascent begins there). One turning-point, two hands.
#[cfg(test)]
mod m0_rfactor_band_turn {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    /// Mirror of m0.h Virtue_Entry.
    #[repr(C)]
    struct VirtueEntry {
        r_factor: u8,
        divine_act: u8,
        cross_branch_refs: u16,
        name: *const c_char,
        symbol: *const c_char,
    }

    extern "C" {
        static VIRTUE_LUT: [VirtueEntry; 9];
    }

    fn symbol(index: usize) -> &'static str {
        let lut = unsafe { &VIRTUE_LUT };
        unsafe { CStr::from_ptr(lut[index].symbol) }
            .to_str()
            .expect("virtue symbol must be UTF-8")
    }

    #[test]
    fn band_turn_token_terminates_beauty_and_opens_life() {
        // Beauty 2R (VIRTUE_LUT[5]): pravritti descent ends at the seed.
        let beauty = symbol(5);
        assert!(
            beauty.ends_with("(@#))"),
            "Beauty signature must terminate at the (@#) Sakti-seed: {beauty}"
        );
        // Life 3R (VIRTUE_LUT[6]): nivritti ascent begins there.
        let life = symbol(6);
        assert!(
            life.contains("((@#)-"),
            "Life signature must open from the (@#) seed: {life}"
        );
        // The seed appears exactly once per signature — it is a pivot,
        // not a repeating fret.
        assert_eq!(beauty.matches("(@#)").count(), 1);
        assert_eq!(life.matches("(@#)").count(), 1);
    }

    #[test]
    fn beauty_and_life_traverse_the_same_spine_in_opposite_hands() {
        // 2R = (X#-N#-M#-#-(#)-(@#)) descends; 3R = ((@#)-(#)-#-M#-N#-X#)
        // ascends the identical base spine — per-fret complementarity
        // R2 + R3 = 5 is this reversal seen from the matrix side.
        let beauty = symbol(5);
        let life = symbol(6);
        for base in ["X#", "N#", "M#"] {
            assert!(beauty.contains(base), "Beauty spine carries {base}");
            assert!(life.contains(base), "Life spine carries {base}");
        }
        let beauty_x = beauty.find("X#").expect("X# in Beauty");
        let beauty_seed = beauty.find("(@#)").expect("seed in Beauty");
        let life_x = life.find("X#").expect("X# in Life");
        let life_seed = life.find("(@#)").expect("seed in Life");
        assert!(
            beauty_x < beauty_seed,
            "Beauty runs spine -> seed (descent ends at the turn)"
        );
        assert!(
            life_seed < life_x,
            "Life runs seed -> spine (ascent begins at the turn)"
        );
    }
}

/// Tranche 01.T1.13 — the Anuttara term-rewriting calculus as kernel
/// computational substrate. Shared FFI + corpus helpers: the X/N-system
/// formulations are fetched from the canonical coordinate-language registry
/// (anuttara_language.c, generated from the anuttara-deep dataset) and the
/// corpus identities are then checked numerically at multiple sample points
/// — polynomial degree ≤ 2, so three points pin each identity.
#[cfg(test)]
mod m0_calc_corpus {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int};

    pub const M0C_MAX_STEPS: usize = 128;
    pub const M0C_MAX_TERM_LEN: usize = 512;
    pub const M0C_MAX_RULE_NAME: usize = 48;
    pub const M0C_MAX_DIAGNOSTIC: usize = 256;

    #[repr(C)]
    pub struct M0CalcReductionStep {
        pub rule: c_int,
        pub rule_name: [c_char; M0C_MAX_RULE_NAME],
        pub before: [c_char; M0C_MAX_TERM_LEN],
        pub after: [c_char; M0C_MAX_TERM_LEN],
        pub depth: u8,
        pub match_pos: u8,
        pub is_interrogative: bool,
    }

    #[repr(C)]
    pub struct M0CalcTrace {
        pub steps: [M0CalcReductionStep; M0C_MAX_STEPS],
        pub step_count: u8,
        pub final_term: [c_char; M0C_MAX_TERM_LEN],
        pub is_normal_form: bool,
        pub is_palindrome: bool,
        pub has_interrogative: bool,
        pub diagnostic: [c_char; M0C_MAX_DIAGNOSTIC],
    }

    #[repr(C)]
    pub struct AnuttaraLanguageEntry {
        pub coordinate: *const c_char,
        pub symbol: *const c_char,
        pub name: *const c_char,
        pub kind: u8,
        pub packed: u16,
    }

    extern "C" {
        pub fn m0_calc_reduce(formulation: *const c_char, out: *mut M0CalcTrace) -> c_int;
        pub fn m0_calc_is_palindrome_normal(term: *const c_char) -> bool;
        pub fn m0_calc_is_query_object(term: *const c_char) -> bool;
        pub fn anuttara_language_lookup(coordinate: *const c_char) -> *const AnuttaraLanguageEntry;
    }

    pub fn registry_symbol(coordinate: &str) -> String {
        let c = CString::new(coordinate).unwrap();
        let entry = unsafe { anuttara_language_lookup(c.as_ptr()) };
        assert!(!entry.is_null(), "{coordinate} must be a registry member");
        unsafe { CStr::from_ptr((*entry).symbol) }
            .to_str()
            .expect("registry symbol must be UTF-8")
            .to_owned()
    }

    pub fn reduce(term: &str) -> (String, bool) {
        let c = CString::new(term).unwrap();
        let mut trace: Box<M0CalcTrace> = unsafe { Box::new(std::mem::zeroed()) };
        let status = unsafe { m0_calc_reduce(c.as_ptr(), trace.as_mut()) };
        assert_eq!(status, 0, "reduce({term}) must succeed");
        let final_term = unsafe { CStr::from_ptr(trace.final_term.as_ptr()) }
            .to_str()
            .expect("final term must be UTF-8")
            .to_owned();
        (final_term, trace.has_interrogative)
    }

    /// Evaluate an X-system formulation `((x)+(x)) s1 ((x)x(x)) s2 ((x)/(x))`
    /// at a sample x, reading the two signs from the formulation itself.
    pub fn eval_x_formulation(formulation: &str, x: f64) -> f64 {
        let rest = formulation
            .strip_prefix("((x)+(x))")
            .unwrap_or_else(|| panic!("X formulation opens with the additive dyad: {formulation}"));
        let s1 = rest.chars().next().expect("sign after additive dyad");
        let rest2 = rest[1..]
            .strip_prefix("((x)x(x))")
            .unwrap_or_else(|| panic!("X formulation carries the product dyad: {formulation}"));
        let s2 = rest2.chars().next().expect("sign after product dyad");
        assert!(
            rest2[1..].starts_with("((x)/(x))"),
            "quotient dyad closes: {formulation}"
        );
        let sign = |c: char| if c == '+' { 1.0 } else { -1.0 };
        2.0 * x + sign(s1) * (x * x) + sign(s2) * (x / x)
    }
}

/// `cargo test -p epi-lib m0_calc_x_logic_sums` — corpus law: ΣX1–4 = 8x
/// (the ± product/quotient dyads cancel pairwise), X5 = ΣX1–4 + (x) = 9(x)
/// (Law 5, explicate-eight plus implicate-one), X(1) = (0,4,2,2,9)
/// (the electroweak spectrum per DR-CALC-6).
#[cfg(test)]
mod m0_calc_x_logic_sums {
    use super::m0_calc_corpus::{eval_x_formulation, registry_symbol};

    const X_COORDS: [&str; 4] = [
        "M0-(4.0/1/2)-1",
        "M0-(4.0/1/2)-2",
        "M0-(4.0/1/2)-3",
        "M0-(4.0/1/2)-4",
    ];

    #[test]
    fn sum_of_x1_through_x4_is_8x_and_x5_is_9x() {
        let formulations: Vec<String> = X_COORDS.iter().map(|c| registry_symbol(c)).collect();
        for x in [1.0, 2.0, 3.5] {
            let sum: f64 = formulations.iter().map(|f| eval_x_formulation(f, x)).sum();
            assert!(
                (sum - 8.0 * x).abs() < 1e-9,
                "ΣX1–4({x}) = {sum} must be 8x"
            );
            // X5: the dataset's own closure — explicate eight plus the
            // implicate (x) equals ninefold wholeness.
            assert!((sum + x - 9.0 * x).abs() < 1e-9, "ΣX1–4 + x = 9x at {x}");
        }
        let x5 = registry_symbol("M0-(4.0/1/2)-5");
        assert!(
            x5.ends_with("= 9(x)"),
            "X5 states its own 9(x) identity: {x5}"
        );
        assert!(
            x5.contains("+/-"),
            "X5 retains the superposed ± branches (Law 4): {x5}"
        );
    }

    #[test]
    fn x_applied_to_one_yields_the_spectrum_0_4_2_2_9() {
        let spectrum: Vec<f64> = X_COORDS
            .iter()
            .map(|c| eval_x_formulation(&registry_symbol(c), 1.0))
            .collect();
        assert_eq!(spectrum, vec![0.0, 4.0, 2.0, 2.0]);
        // X5(1) = ΣX1–4(1) + 1 = 9 closes the spectrum (0,4,2,2,9).
        let x5_at_one: f64 = spectrum.iter().sum::<f64>() + 1.0;
        assert_eq!(x5_at_one, 9.0);
    }
}

/// `cargo test -p epi-lib m0_calc_n_logic` — corpus law: ΣN1–4 = 8n
/// (with i² = −1 the squared dyads cancel) and N5 = 8(n)±(n) ∈ {9n, 7n}.
#[cfg(test)]
mod m0_calc_n_logic {
    use super::m0_calc_corpus::registry_symbol;

    fn eval_n(coordinate: &str, n: f64) -> f64 {
        // The four N-system formulations, evaluated with i² = −1 exactly
        // as the registry states them.
        match registry_symbol(coordinate).as_str() {
            "(i²)*(n-1)²" => -((n - 1.0) * (n - 1.0)),
            "(n+1)²" => (n + 1.0) * (n + 1.0),
            "((n)x(n+1))+(n-1)" => n * (n + 1.0) + (n - 1.0),
            "(n-1)x((i²)x(n-1))+2" => (n - 1.0) * (-(n - 1.0)) + 2.0,
            other => panic!("unexpected N formulation at {coordinate}: {other}"),
        }
    }

    #[test]
    fn sum_of_n1_through_n4_is_8n_and_n5_branches_to_9n_or_7n() {
        for n in [2.0, 3.0, 5.0] {
            let sum: f64 = (1..=4)
                .map(|i| eval_n(&format!("M0-(4.0/1/2/3)-{i}"), n))
                .sum();
            assert!(
                (sum - 8.0 * n).abs() < 1e-9,
                "ΣN1–4({n}) = {sum} must be 8n"
            );
            assert_eq!(sum + n, 9.0 * n, "N5 upper branch 9n at {n}");
            assert_eq!(sum - n, 7.0 * n, "N5 lower branch 7n at {n}");
        }
        let n5 = registry_symbol("M0-(4.0/1/2/3)-5");
        assert_eq!(n5, "8(n)+/-(n)", "N5 states the 8n±n branch law");
    }
}

/// `cargo test -p epi-lib m0_calc_framing_asymmetry` — Law 2, the hinge of
/// the whole architecture: unframed void addition stays void (00+00 → 00);
/// framed void addition converts invariance into wholeness ((00+00) → 9).
#[cfg(test)]
mod m0_calc_framing_asymmetry {
    use super::m0_calc_corpus::reduce;

    #[test]
    fn unframed_stays_void_framed_becomes_nine() {
        let (unframed, _) = reduce("00+00");
        assert_eq!(unframed, "00", "unframed void addition stays void");
        let (framed, _) = reduce("(00+00)");
        assert_eq!(framed, "9", "framed void addition is ninefold wholeness");
    }

    #[test]
    fn o5_quadratic_expansion_lands_the_non_dual_binary() {
        // DR-CALC-4: x// is superposition-preserving multiplication; O5's
        // quadratic expansion resolves to the Non-Dual Binary.
        let (expanded, _) = reduce("((+/-0) x// (+/-0))");
        assert_eq!(expanded, "0/1");
    }
}

/// `cargo test -p epi-lib m0_calc_palindrome_nineness` — Law 5's computable
/// wholeness test: a term is whole (9) when its normal form reads
/// identically from either pole.
#[cfg(test)]
mod m0_calc_palindrome_nineness {
    use super::m0_calc_corpus::m0_calc_is_palindrome_normal;
    use std::ffi::CString;

    fn is_palindrome(term: &str) -> bool {
        let c = CString::new(term).unwrap();
        unsafe { m0_calc_is_palindrome_normal(c.as_ptr()) }
    }

    #[test]
    fn mirror_normal_forms_read_the_same_from_either_pole() {
        // Mirror-normal forms are TOKEN-symmetric, and Law-1 chirality is
        // lexical: any `R` immediately followed by `#` fuses into the
        // compound R# tao element, so R-then-# mirror shapes fragment
        // asymmetrically (asserted non-whole below). Whole forms keep the
        // pivot clear of the compound.
        assert!(is_palindrome("0##0"));
        assert!(is_palindrome("0R0"));
        // The C self-test corpus members.
        assert!(is_palindrome("00"));
        assert!(is_palindrome("9"));
        // Chirality is semantic: a one-sided mirror mark is NOT whole,
        // and the compound R# tao element breaks R-flanked symmetry.
        assert!(!is_palindrome("-0"));
        assert!(!is_palindrome("00+0"));
        assert!(!is_palindrome("R####R"));
        assert!(!is_palindrome("####R####"));
    }
}

/// `cargo test -p epi-lib m0_calc_query_objects` — Law 6, the constitution
/// of non-blocking: undefinedness returns a typed query-object, never an
/// error. The Mystery is the exception-system of the calculus.
#[cfg(test)]
mod m0_calc_query_objects {
    use super::m0_calc_corpus::{m0_calc_is_query_object, reduce};
    use std::ffi::CString;

    fn is_query(term: &str) -> bool {
        let c = CString::new(term).unwrap();
        unsafe { m0_calc_is_query_object(c.as_ptr()) }
    }

    #[test]
    fn undefinedness_returns_typed_query_objects_not_errors() {
        let (indeterminate, interrogative) = reduce("0/0");
        assert_eq!(indeterminate, "%", "0/0 is the indeterminate mark");
        assert!(interrogative, "the reduction records its ?-object");

        let (query, interrogative) = reduce("1/0");
        assert_eq!(query, "?/!", "1/0 is the query-exclamation pair");
        assert!(interrogative);

        let (applied_zero, interrogative) = reduce("X(0)");
        assert!(
            applied_zero.contains("?!"),
            "X(0) raises the Vimarsa-Prakasa query: {applied_zero}"
        );
        assert!(interrogative);
    }

    #[test]
    fn query_object_detector_matches_the_emission_family() {
        for query in ["%", "?/!", "?!", "!?", "?!/!?"] {
            assert!(is_query(query), "`{query}` is a typed query-object");
        }
        assert!(!is_query("00"), "the void is not a question");
        assert!(!is_query("9"), "wholeness is not a question");
    }
}

/// Tranche 03.T3.10 — the Asma mirror law at the C LUT level: every
/// populated `mirror_idx` resolves to a valid entry of `M2_ASMA_LUT[100]`,
/// `0xFF` is explicit absence (not an error), and the hidden 100th name
/// carries no domain mirror.
#[cfg(test)]
mod m2_asma_mirror_idx_round_trip {
    /// Mirror of m2.h Asma_Name_Desc (12 bytes, _Static_assert-pinned).
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct AsmaNameDescC {
        name_idx: u8,
        group: u8,
        index_in_group: u8,
        element_id: u8,
        digital_root: u8,
        mirror_idx: u8,
        abjad_value: u16,
        meaning_id: u16,
        _pad: [u8; 2],
    }

    extern "C" {
        static M2_ASMA_LUT: [AsmaNameDescC; 100];
    }

    #[test]
    fn populated_mirror_indices_resolve_and_absence_is_explicit() {
        let lut = unsafe { &M2_ASMA_LUT };
        assert_eq!(std::mem::size_of::<AsmaNameDescC>(), 12);

        let mut mirrored = 0usize;
        for (i, desc) in lut.iter().enumerate() {
            assert_eq!(desc.name_idx as usize, i, "LUT is index-addressed");
            if desc.mirror_idx != 0xFF {
                assert!(
                    desc.mirror_idx < 100,
                    "name {i}: mirror_idx {m} must resolve inside the LUT",
                    m = desc.mirror_idx
                );
                assert_ne!(
                    desc.mirror_idx as usize, i,
                    "name {i}: a domain mirror is another name, not itself"
                );
                mirrored += 1;
            }
            if i < 99 {
                assert!(desc.group <= 2, "name {i}: group is Jalal/Kamal/Jamal");
                assert!(desc.index_in_group <= 32, "name {i}: index_in_group bound");
            }
        }
        assert!(
            mirrored > 0,
            "the mirror corpus is populated, not vestigial"
        );

        // The hidden 100th name (Al-Ism al-A'zham, index 99) stands beyond
        // the three groups — 0xFF sentinels on group, index_in_group, and
        // mirror alike (portal-core maps the group sentinel to 3 at its own
        // layer). Absence is explicit, lawful, and total.
        assert_eq!(lut[99].group, 0xFF);
        assert_eq!(lut[99].index_in_group, 0xFF);
        assert_eq!(lut[99].mirror_idx, 0xFF);
    }

    /// The dataset's mirror law is DIRECTIONAL: a name may point at its
    /// domain counterpart without the counterpart pointing back (e.g. name
    /// 2 → 35 while 35 declares no mirror). What must hold: every declared
    /// pointer lands on a valid other name, and wherever BOTH ends declare,
    /// the pair is reciprocal.
    #[test]
    fn declared_mirror_pointers_land_and_double_declarations_are_reciprocal() {
        let lut = unsafe { &M2_ASMA_LUT };
        for (i, desc) in lut.iter().enumerate() {
            if desc.mirror_idx == 0xFF {
                continue;
            }
            let partner = &lut[desc.mirror_idx as usize];
            assert_eq!(
                partner.name_idx, desc.mirror_idx,
                "name {i}: pointer lands on a real entry"
            );
            if partner.mirror_idx != 0xFF {
                assert_eq!(
                    partner.mirror_idx as usize,
                    i,
                    "name {i} ↔ {m}: double-declared mirrors must be reciprocal",
                    m = desc.mirror_idx
                );
            }
        }
    }
}

/// Tranche 01.T1.14a — Law 3: the rewriting system runs MODULO the
/// equational theory. The registry's =-chains ARE the identity classes
/// (M0_IDENTITY_CHAINS is the same theory read as coordinate walks);
/// an unambiguous non-canonical member rewrites to its chain head before
/// any computational rule fires.
#[cfg(test)]
mod m0_calc_modulo_identity_chains {
    use super::m0_calc_corpus::reduce;
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int};

    extern "C" {
        fn m0_identity_class_find(term: *const c_char) -> c_int;
        fn m0_identity_class_canonical(class_id: c_int) -> *const c_char;
        fn m0_identity_class_count() -> usize;
    }

    fn class_of(term: &str) -> i32 {
        let c = CString::new(term).unwrap();
        unsafe { m0_identity_class_find(c.as_ptr()) }
    }

    #[test]
    fn registry_chains_compile_into_equivalence_classes() {
        assert!(
            unsafe { m0_identity_class_count() } >= 20,
            "the corpus carries dozens of =-chains"
        );
        // "## = @ = (0/1)-(00)-00" — the Truth chain (M0-2-9-1): the long
        // member resolves to a class whose canonical head is the glyph.
        let class_id = class_of("(0/1)-(00)-00");
        assert!(
            class_id >= 0,
            "the Truth chain tail is an unambiguous member"
        );
        let canonical = unsafe { std::ffi::CStr::from_ptr(m0_identity_class_canonical(class_id)) }
            .to_str()
            .unwrap();
        assert_eq!(canonical, "##", "the chain head is the canonical member");
        // `@` rides most virtue chains — ambiguous, so it NEVER rewrites.
        assert_eq!(class_of("@"), -1, "ambiguous members are excluded");
    }

    #[test]
    fn reduction_fires_over_chain_equivalent_forms_before_computation() {
        // Whole-term modulo rewrite: the Truth chain tail reduces to ## by
        // Law-3 identity resolution, not by arithmetic.
        let (reduced, _) = reduce("(0/1)-(00)-00");
        assert_eq!(reduced, "##");
        // The canonical head is stable — no loop.
        let (stable, _) = reduce("##");
        assert_eq!(stable, "##");
    }
}

/// Tranche 01.T1.14b — Law 7: the dash is five things BY POSITION.
/// Positional polysemy, hence parseable; the five readings classify
/// distinctly. `~` stays out of the grammar (DR-CALC-2: annotation only).
#[cfg(test)]
mod m0_calc_dash_pentavalence {
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int};

    extern "C" {
        fn m0_calc_dash_reading(term: *const c_char, pos: usize) -> c_int;
    }

    fn reading(term: &str, pos: usize) -> i32 {
        let c = CString::new(term).unwrap();
        unsafe { m0_calc_dash_reading(c.as_ptr(), pos) }
    }

    #[test]
    fn the_five_positional_readings_parse_distinctly() {
        const CHIRALITY: i32 = 0;
        const OPERATOR: i32 = 1;
        const RANGE: i32 = 2;
        const STRIKETHROUGH: i32 = 3;
        const SUBTRACTION: i32 = 4;

        // -0 : the mirror mark hugging the void (chirality)
        assert_eq!(reading("-0", 0), CHIRALITY);
        // O#-X#-N# : the base-spine connector (operator)
        assert_eq!(reading("O#-X#-N#", 2), OPERATOR);
        // (4.0/1-4.4/5) : the CF span between dotted coordinates (range)
        assert_eq!(reading("(4.0/1-4.4/5)", 6), RANGE);
        // 2-/2 : the dominance dash on a kinship ratio (strikethrough)
        assert_eq!(reading("2-/2", 1), STRIKETHROUGH);
        // 9-8 : plain numeric infix (subtraction)
        assert_eq!(reading("9-8", 1), SUBTRACTION);

        let readings = [
            reading("-0", 0),
            reading("O#-X#-N#", 2),
            reading("(4.0/1-4.4/5)", 6),
            reading("2-/2", 1),
            reading("9-8", 1),
        ];
        let distinct: std::collections::BTreeSet<i32> = readings.iter().copied().collect();
        assert_eq!(distinct.len(), 5, "five readings, five classifications");
    }

    #[test]
    fn non_dash_positions_are_unknown_and_trailing_kinship_dash_reads_strikethrough() {
        const STRIKETHROUGH: i32 = 3;
        const UNKNOWN: i32 = 5;
        assert_eq!(reading("00", 0), UNKNOWN);
        assert_eq!(reading("-0", 1), UNKNOWN);
        // 1/1- : Daughter's subdominant trailing dash (kinship denominator)
        assert_eq!(reading("1/1-", 3), STRIKETHROUGH);
    }
}

/// Tranche 01.T1.17 — R#/## tao-elements: the classic coin method
/// (Yin R# = 2, Yang ## = 3, 4-slot frame, value = yang-count + 5)
/// constructs the canonical nucleotide I-Ching values {6, 9, 8, 7} (M3-COIN-1).
#[cfg(test)]
mod m0_calc_tao_coin_method {
    use std::os::raw::c_int;

    extern "C" {
        fn m0_calc_nucleotide_from_coin(yin_count: c_int, yang_count: c_int) -> c_int;
        static NUCLEOTIDE_ICHING_VALUE: [u8; 4];
    }

    // (yin R#, yang ##) counts per nucleotide A/T/C/G — findings §III.6 table.
    const COIN: [(i32, i32); 4] = [(3, 1), (0, 4), (1, 3), (2, 2)]; // M3-COIN-1

    #[test]
    fn coin_method_reproduces_the_canonical_iching_values() {
        let expected = [6, 9, 8, 7]; // A Old Yin, T Old Yang, C Young Yin, G Young Yang (M3-COIN-1)
        for (n, (yin, yang)) in COIN.iter().enumerate() {
            let v = unsafe { m0_calc_nucleotide_from_coin(*yin, *yang) };
            assert_eq!(v, expected[n], "nucleotide {n}");
            assert_eq!(v, unsafe { NUCLEOTIDE_ICHING_VALUE[n] } as i32);
        }
    }

    #[test]
    fn coin_method_rejects_frames_that_are_not_four_slots() {
        for (yin, yang) in [(2, 1), (4, 1), (0, 0), (-1, 5), (5, -1)] {
            assert_eq!(
                unsafe { m0_calc_nucleotide_from_coin(yin, yang) },
                -1,
                "({yin},{yang}) is not a 4-slot frame"
            );
        }
    }
}

/// Tranche 01.T1.17 — Tao (5-/5) ≡ the codon charge-evaluation: the
/// kinship-grammar apex IS m3_compute_charges reading the R#/## binary
/// into the pp/nn/np/pn genetic charges. The kernel verifies the seam
/// structurally; the second test recomputes it independently in Rust.
#[cfg(test)]
mod m0_calc_tao_is_codon_eval {
    use std::os::raw::c_int;

    extern "C" {
        fn m0_calc_tao_is_codon_eval() -> bool;
        fn m0_calc_nucleotide_from_coin(yin_count: c_int, yang_count: c_int) -> c_int;
        fn m3_compute_charges(codon: u8, pp: *mut i8, nn: *mut i8, np: *mut i8, pn: *mut i8);
    }

    #[test]
    fn kernel_verifies_the_tao_codon_binding() {
        assert!(unsafe { m0_calc_tao_is_codon_eval() });
    }

    #[test]
    fn charges_are_the_sign_algebra_over_coin_values_for_every_codon() {
        const COIN: [(i32, i32); 4] = [(3, 1), (0, 4), (1, 3), (2, 2)]; // M3-COIN-1
        for codon in 0u8..64 {
            let value = |shift: u8| {
                let (yin, yang) = COIN[((codon >> shift) & 0x03) as usize];
                unsafe { m0_calc_nucleotide_from_coin(yin, yang) }
            };
            let (x, y, z) = (value(4), value(2), value(0));
            let (mut pp, mut nn, mut np, mut pn) = (0i8, 0i8, 0i8, 0i8);
            unsafe { m3_compute_charges(codon, &mut pp, &mut nn, &mut np, &mut pn) };
            assert_eq!(i32::from(pp), x + y + z, "pp for codon {codon}");
            assert_eq!(i32::from(nn), x - y - z, "nn for codon {codon}");
            assert_eq!(i32::from(np), x - y + z, "np for codon {codon}");
            assert_eq!(i32::from(pn), x + y - z, "pn for codon {codon}");
        }
    }
}
