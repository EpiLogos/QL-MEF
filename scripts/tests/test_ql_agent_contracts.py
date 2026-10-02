"""Real QD0 contract checks using installed QL owner label receipts.
Controlled proposal payloads test the contract, not classifier performance.
No provider is installed, invoked or mocked.
"""
import copy
import json
import subprocess
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
import ql_agent_contracts as c

BASE = ROOT / "fixtures/agent-decision/v1"

class ContractChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.snapshot = json.loads((BASE / "native-label-receipts.json").read_text())
        cls.original_event = json.loads((BASE / "event-v1.json").read_text())
        cls.original_frame = json.loads((BASE / "decision-frame-v1.json").read_text())
        cls.original_determination = json.loads((BASE / "unavailable-determination-v1.json").read_text())

    def setUp(self):
        self.event=copy.deepcopy(self.original_event)
        self.frame=copy.deepcopy(self.original_frame)
        self.result=copy.deepcopy(self.original_determination)

    def rejects(self, reason, operation):
        with self.assertRaisesRegex(c.ContractError, reason):
            operation()

    def proposal(self):
        # Supplied adversarial/protocol input, never a claimed inference receipt.
        self.result["provider"]={"provider_ref":"input:controlled-proposal",
            "model_ref":"input:controlled-proposal", "model_revision":"contract-test-v1",
            "runtime_revision":"contract-test-v1"}
        self.result["learned"]=[{"head_id":"semantic-lens",
            "label_ids":[self.frame["unresolved"][0]["labels"][0]["id"]], "spans":[]}]
        self.result["status"]="unresolved"
        return self.result["learned"][0]

    def test_retained_native_labels_and_descriptions_are_retained(self):
        expected=[(x["code"],x["name"]) for x in self.snapshot["lenses"]["data"]["lenses"]]
        actual=[(x["id"],x["description"]) for x in self.frame["unresolved"][0]["labels"]]
        self.assertEqual(expected,actual)
        self.assertEqual(len(expected),12)
        self.assertEqual(len(self.snapshot["context_frames"]["data"]["frames"]),7)
        c.validate_frame(self.event,self.frame)

    def test_provider_unavailable_keeps_original_basis_and_unresolved_head(self):
        c.validate_determination(self.event,self.frame,self.result)
        self.assertEqual(self.result["status"],"unavailable")
        self.assertEqual(self.result["learned"],[])

    def test_prime_pi_native_bindings_do_not_change_semantic_basis(self):
        pi=copy.deepcopy(self.event)
        pi["bindings"]={"body_ref":"body:pi:contract-specimen",
            "session_ref":"session:qd0:pi","native_event_refs":["pi:event:different-native-id"]}
        self.assertEqual(c.semantic_basis(pi),c.semantic_basis(self.event))
        c.validate_frame(pi,self.frame)

    def test_event_without_optional_bindings_retains_valid_basis(self):
        unbound=copy.deepcopy(self.event)
        del unbound["bindings"]
        self.assertEqual(c.semantic_basis(unbound),c.semantic_basis(self.event))
        c.validate_determination(unbound,self.frame,self.result)

    def test_source_revision_changes_semantic_basis(self):
        self.event["source_basis"][0]["revision"]="sha256:"+"a"*64
        self.rejects("stale event",lambda:c.validate_frame(self.event,self.frame))

    def test_text_changes_semantic_basis(self):
        self.event["material"]["text"]+=" Later evidence."
        self.rejects("stale event",lambda:c.validate_frame(self.event,self.frame))

    def test_event_generation_fences_late_answer(self):
        self.event["generation"]+=1
        self.rejects("stale event",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_schema_rejects_provider_ontology_as_event_field(self):
        self.event["noul"]=0.8
        self.rejects("schema",lambda:c.validate(self.event))

    def test_duplicate_source_ref_is_refused(self):
        self.event["source_basis"].append(copy.deepcopy(self.event["source_basis"][0]))
        self.rejects("duplicate source",lambda:c.validate(self.event))

    def test_observed_coordinate_cannot_be_repredicted(self):
        fact={"field":"lens","value":self.frame["unresolved"][0]["labels"][0]["id"],
              "origin":"observed","basis_refs":[self.event["material"]["ref"]]}
        self.event["observed"]=[fact]
        self.frame["determined"]["observed"]=[fact]
        self.frame["event_basis_digest"]=c.semantic_basis(self.event)
        self.rejects("predict determined",lambda:c.validate_frame(self.event,self.frame))

    def test_observed_fact_cannot_disappear(self):
        self.event["observed"]=[{"field":"native-tool-state","value":"ready",
            "origin":"observed","basis_refs":[self.event["material"]["ref"]]}]
        self.frame["event_basis_digest"]=c.semantic_basis(self.event)
        self.rejects("observed facts lost",lambda:c.validate_frame(self.event,self.frame))

    def test_derived_fact_requires_native_rule_ref(self):
        self.frame["determined"]["derived"]=[{"field":"face","value":"day",
            "origin":"derived","basis_refs":[self.event["material"]["ref"]]}]
        self.rejects("schema",lambda:c.validate(self.frame))

    def test_impossible_cardinality_is_refused(self):
        self.frame["unresolved"][0]["cardinality"]["max"]=99
        self.rejects("impossible cardinality",lambda:c.validate_frame(self.event,self.frame))

    def test_constraint_cannot_name_absent_head(self):
        self.frame["constraints"]=[{"kind":"exclusion","id":"test:constraint",
            "rule_ref":"ql:rule:controlled-input","forbidden_together":[
                {"head_id":"absent","label_ids":["unknown"],"match":"any"}]}]
        self.rejects("absent head",lambda:c.validate_frame(self.event,self.frame))

    def test_illegal_learned_label_is_refused(self):
        self.proposal()["label_ids"]=["impossible-lens"]
        self.rejects("illegal learned label",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_missing_model_identity_is_refused(self):
        self.proposal()
        del self.result["provider"]
        self.rejects("lacks provider",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_provider_probability_is_not_source_evidence(self):
        proposal=self.proposal()
        proposal["confidence"]=0.99
        proposal["spans"]=[{"material_ref":self.event["material"]["ref"],
            "revision":self.event["material"]["revision"],"start":0,"end":1,"text":"invented"}]
        self.rejects("fabricated evidence",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_actual_unicode_codepoint_span_is_admitted(self):
        self.event["material"]["text"] = "Concern: 🌀 L2′ — evidence?"
        self.event["material"]["revision"] = c.digest(self.event["material"]["text"])
        self.event["source_basis"][0]["revision"] = self.event["material"]["revision"]
        self.frame["source_basis"] = copy.deepcopy(self.event["source_basis"])
        self.frame["event_basis_digest"] = c.semantic_basis(self.event)
        self.result["source_basis"] = copy.deepcopy(self.frame["source_basis"])
        self.result["event_basis_digest"] = c.semantic_basis(self.event)
        self.result["frame_digest"] = c.digest(self.frame)
        proposal=self.proposal()
        text=self.event["material"]["text"][:14]
        proposal["spans"]=[{"material_ref":self.event["material"]["ref"],
            "revision":self.event["material"]["revision"],"start":0,"end":len(text),"text":text}]
        c.validate_determination(self.event,self.frame,self.result)

    def test_stale_frame_cannot_validate_result(self):
        self.frame["unresolved"][0]["labels"][0]["description"]+=" new description"
        self.rejects("stale decision frame",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_post_validation_derivation_retains_original_frame_and_accepted_basis(self):
        proposal=self.proposal()
        self.result["status"]="determined"
        self.result["unresolved"]=[]
        self.result["validated"]=[{"field":"lens","value":proposal["label_ids"],"origin":"learned",
            "proposal_head":"semantic-lens","kernel_rule_refs":["ql:rule:controlled-input"],
            "basis_digest":c.digest(self.frame)}]
        self.result["derived"].append({"field":"lens-face","value":"day","origin":"derived",
            "basis_refs":[self.frame["frame_ref"]+"#semantic-lens"],"rule_ref":"ql:mef:lens:1.0.0"})
        c.validate_determination(self.event,self.frame,self.result)
        self.result["derived"][-1]["basis_refs"]=["unrelated:basis"]
        self.rejects("accepted semantic basis",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_post_validation_cannot_replace_fixed_state(self):
        self.frame["determined"]["derived"]=[{"field":"fixed","value":1,"origin":"derived",
            "basis_refs":[self.event["event_ref"]],"rule_ref":"ql:rule:controlled-input"}]
        self.result["derived"]=copy.deepcopy(self.frame["determined"]["derived"])
        self.result["frame_digest"]=c.digest(self.frame)
        self.result["derived"][0]["value"]=2
        self.rejects("overridden derivation",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_no_silent_repair_of_learned_proposal(self):
        self.proposal()
        self.result["unresolved"]=[]
        self.result["validated"]=[{"field":"lens","value":["impossible-lens"],
            "origin":"learned","proposal_head":"semantic-lens",
            "kernel_rule_refs":["ql:rule:controlled-input"],"basis_digest":c.digest(self.frame)}]
        self.rejects("silent repair",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_deterministic_frame_cannot_carry_provider_attribution(self):
        self.frame["unresolved"]=[]
        self.result["frame_digest"]=c.digest(self.frame)
        self.result["unresolved"]=[]
        self.result["status"]="determined"
        c.validate_determination(self.event,self.frame,self.result)
        self.result["provider"]={"provider_ref":"input:controlled-proposal",
            "model_ref":"input:controlled-proposal","model_revision":"v1","runtime_revision":"v1"}
        self.rejects("provider on deterministic",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_unknown_constraint_result_is_refused(self):
        self.result["constraint_application"]=[{"constraint_id":"test:missing",
            "result":"silently-repaired","reason":"invalid wire status"}]
        self.rejects("schema",lambda:c.validate(self.result))

    def test_no_unresolved_heads_cannot_be_reported_provider_unavailable(self):
        self.frame["unresolved"]=[]
        self.result["frame_digest"]=c.digest(self.frame)
        self.result["unresolved"]=[]
        self.rejects("deterministic-only result",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_standalone_event_cannot_mislabel_derivation_as_observed(self):
        self.event["observed"]=[{"field":"face","value":"day","origin":"derived",
            "basis_refs":[self.event["material"]["ref"]],"rule_ref":"input:controlled-rule"}]
        self.rejects("schema",lambda:c.validate(self.event))

    def test_derivation_cannot_be_readmitted_as_observation(self):
        fact={"field":"face","value":"day","origin":"derived",
            "basis_refs":[self.event["material"]["ref"]],"rule_ref":"input:controlled-rule"}
        self.frame["determined"]["derived"]=[fact]
        self.result["derived"]=[fact]
        self.result["frame_digest"]=c.digest(self.frame)
        self.result["validated"]=[{"field":"face","value":"day","origin":"observed",
            "kernel_rule_refs":["input:controlled-rule"],"basis_digest":c.digest(self.frame)}]
        self.rejects("observation differs",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_unavailable_cannot_carry_learned_output(self):
        self.proposal()
        self.result["status"]="unavailable"
        self.rejects("unavailable status",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_nonoperative_status_cannot_carry_learned_admission(self):
        proposal=self.proposal()
        self.result["unresolved"]=[]
        self.result["validated"]=[{"field":"lens","value":proposal["label_ids"],"origin":"learned",
            "proposal_head":"semantic-lens","kernel_rule_refs":["input:controlled-rule"],
            "basis_digest":c.digest(self.frame)}]
        self.rejects("non-operative status",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_exact_impossible_proposal_is_retained_as_refused(self):
        proposal=self.proposal()
        proposal["label_ids"]=["impossible-lens"]
        self.result["status"]="refused"
        self.result["unresolved"]=[]
        self.result["refused_candidates"]=[{"head_id":"semantic-lens","label_ids":proposal["label_ids"],
            "origin":"learned","reason":"outside owner candidate field","rule_refs":["input:controlled-rule"]}]
        c.validate_determination(self.event,self.frame,self.result)
        self.assertEqual(self.result["learned"][0]["label_ids"],["impossible-lens"])
        self.assertEqual(self.result["validated"],[])

    def test_refused_proposal_still_requires_provider_identity(self):
        proposal=self.proposal()
        self.result["unresolved"]=[]
        self.result["status"]="refused"
        self.result["refused_candidates"]=[{"head_id":"semantic-lens","label_ids":proposal["label_ids"],
            "origin":"learned","reason":"insufficient semantic evidence","rule_refs":["input:controlled-rule"]}]
        self.result["learned"]=[]
        del self.result["provider"]
        self.rejects("lacks provider",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_zero_label_abstention_is_retained_with_zero_minimum(self):
        self.proposal()["label_ids"]=[]
        c.validate_determination(self.event,self.frame,self.result)
        self.assertEqual(self.result["unresolved"][0]["head_id"],"semantic-lens")

    def test_zero_label_answer_cannot_bypass_positive_minimum(self):
        self.frame["unresolved"][0]["cardinality"]["min"]=1
        self.result["frame_digest"]=c.digest(self.frame)
        self.proposal()["label_ids"]=[]
        self.rejects("cardinality violation",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_zero_label_abstention_cannot_be_promoted_to_determined(self):
        self.proposal()["label_ids"]=[]
        self.result["unresolved"]=[]
        self.result["status"]="determined"
        self.result["validated"]=[{"field":"lens","value":[],"origin":"learned",
            "proposal_head":"semantic-lens","kernel_rule_refs":["input:controlled-rule"],
            "basis_digest":c.digest(self.frame)}]
        self.rejects("remains abstention",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_constraint_receipt_requires_declared_constraint(self):
        self.result["constraint_application"]=[{"constraint_id":"never-declared",
            "result":"passed","reason":"controlled probe"}]
        self.rejects("undeclared constraint",lambda:c.validate_determination(self.event,self.frame,self.result))

    def test_all_four_constraint_kinds_have_explicit_wire_grammar(self):
        label=self.frame["unresolved"][0]["labels"][0]["id"]
        selector={"head_id":"semantic-lens","label_ids":[label],"match":"exact"}
        self.frame["constraints"]=[
            {"kind":"implication","id":"c1","rule_ref":"input:controlled-rule",
             "antecedent":[selector],"consequent":[selector]},
            {"kind":"exclusion","id":"c2","rule_ref":"input:controlled-rule",
             "forbidden_together":[selector]},
            {"kind":"legal-combination","id":"c3","rule_ref":"input:controlled-rule",
             "allowed_tuples":[[selector]]},
            {"kind":"candidate-restriction","id":"c4","rule_ref":"input:controlled-rule",
             "allowed":{**selector,"match":"all"}}]
        c.validate_frame(self.event,self.frame)

    def test_old_ambiguous_constraint_wire_shape_is_rejected(self):
        self.frame["constraints"]=[{"kind":"implication","id":"c1",
            "rule_ref":"input:controlled-rule","selections":[]}]
        self.rejects("schema",lambda:c.validate_frame(self.event,self.frame))

    def harmonic(self):
        return {"schema":"ql.harmonic-event/v1","event_ref":self.result["event_ref"],
            "source_basis":self.result["source_basis"],"kernel_basis":self.result["kernel_basis"],
            "determination_digest":c.digest(self.result),"formal":[],"harmonic":[]}

    def test_harmonic_rendering_requires_derived_origin_and_rule(self):
        packet=self.harmonic()
        packet["harmonic"]=[{"field":"pitch-class","value":"controlled-input",
            "origin":"observed","basis_refs":[self.event["material"]["ref"]]}]
        self.rejects("schema",lambda:c.validate(packet))

    def test_harmonic_cannot_add_unaccepted_formal_state(self):
        packet=self.harmonic()
        packet["formal"]=[{"field":"lens","value":"L0","origin":"observed",
            "basis_refs":[self.event["material"]["ref"]]}]
        self.rejects("formal state changed",lambda:c.validate_harmonic(self.result,packet))

    def test_harmonic_determination_digest_fences_late_readout(self):
        packet=self.harmonic()
        self.result["unresolved"][0]["reason"]+="; later evidence arrived"
        self.rejects("stale harmonic",lambda:c.validate_harmonic(self.result,packet))

    def test_published_schema_equals_source_grammar(self):
        self.assertEqual(json.loads(c.SCHEMA_PATH.read_text()), c.SCHEMA)

    def test_public_validator_accepts_complete_fixture_chain(self):
        command = [sys.executable, str(ROOT / "scripts/ql_agent_contracts.py"),
            "--check-schema", "--event", str(BASE / "event-v1.json"),
            "--frame", str(BASE / "decision-frame-v1.json"),
            "--determination", str(BASE / "unavailable-determination-v1.json")]
        result = subprocess.run(command, capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        receipt = json.loads(result.stdout)
        self.assertEqual(receipt["documents_checked"], ["event", "frame", "determination"])
        self.assertTrue(receipt["schema_checked"])

    def test_public_validator_refuses_incomplete_chain(self):
        result = subprocess.run([sys.executable, str(ROOT / "scripts/ql_agent_contracts.py"),
            "--determination", str(BASE / "unavailable-determination-v1.json")],
            capture_output=True, text=True, timeout=10)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("--determination requires --event and --frame", result.stderr)

    def test_public_validator_refuses_document_of_wrong_contract_kind(self):
        result = subprocess.run([sys.executable, str(ROOT / "scripts/ql_agent_contracts.py"),
            "--event", str(BASE / "decision-frame-v1.json")],
            capture_output=True, text=True, timeout=10)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("expected event schema", result.stderr)

    def test_public_validator_refuses_non_json_material(self):
        result = subprocess.run([sys.executable, str(ROOT / "scripts/ql_agent_contracts.py"),
            "--event", str(BASE / "semantic-specimen.txt")],
            capture_output=True, text=True, timeout=10)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Expecting value", result.stderr)

    def test_public_validator_requires_an_actual_check(self):
        result = subprocess.run([sys.executable, str(ROOT / "scripts/ql_agent_contracts.py")],
            capture_output=True, text=True, timeout=10)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("select --check-schema", result.stderr)


if __name__=="__main__":
    unittest.main(verbosity=2)
