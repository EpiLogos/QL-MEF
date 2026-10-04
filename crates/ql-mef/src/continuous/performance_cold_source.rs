//! Exact source reconstruction for a privately selected cold Act.
//! Retained bundles are compared with the native constructor, never installed
//! as configuration, receiving authority, or a live body checkpoint.
use super::*;
use crate::continuous::performance_receiving::NativePerformanceReceivingSource;
use crate::continuous::performance_receiving::exact_value as same_retained;

impl PerformanceOwner {
    /// Pure preparation. Only the private FieldHost child, while C holds the
    /// complete selected Act, uses this owner for prospective score replay.
    pub(in crate::continuous) fn prepare_cold_act_source(
        current: &CoupledBasis,
        instance: &str,
        source: &NativePerformanceReceivingSource,
        expected: &Value,
    ) -> Result<Self, String> {
        if expected["schema"] != "ql.retained-performance-source-assets/v1" {
            return Err("cold Act has no original native source bundle".into());
        }
        let config: PerformanceConfig =
            serde_json::from_value(expected["configuration"].clone()).map_err(|e| e.to_string())?;
        let mut owner = Self::prepare(current, instance, config)?;
        let admitted_at = decimal(
            &expected["current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        let admitted = source.admit_retained(
            &mut owner,
            current,
            admitted_at,
            &expected["current_receiving"],
        )?;
        owner.source_assets["receiving_source_inputs"] = admitted.fresh().source_inputs().clone();
        owner.source_assets["receiving_definition"] = admitted.fresh().definition().snapshot()?;
        owner.source_assets["current_receiving"] = admitted.retained_snapshot().clone();
        if let Some(retained) = expected.get("acoustic_receiving") {
            let birth = decimal(&retained["packet"]["history_origin_sample"])?;
            let origin = decimal(&retained["packet"]["origin_sample"])?;
            let acoustic = owner.prepare_acoustic_receiving_segment_for_replay(
                current,
                source,
                birth,
                origin,
                crate::continuous::performance_receiving::ReceivingImplementation::of_retained(
                    &retained["current_receiving"],
                )?,
            )?;
            acoustic.validate_current(&owner, current, source, birth)?;
            owner.source_assets["acoustic_receiving"] = acoustic.snapshot();
        } else if source.acoustic_configuration().is_some() {
            return Err("cold Act acoustic configuration lost its original source segment".into());
        }
        // These are the original observed consumer-role facts in the complete
        // selected source. They stay historical during pure score compilation.
        // The fresh worker must return the SAME complete role set before this
        // reconstructed owner can be published or used for restoration.
        let historical_roles = expected
            .get("consumer_roles")
            .filter(|v| v.is_object())
            .ok_or("cold Act original consumer roles absent")?;
        owner.source_assets["consumer_roles"] = historical_roles.clone();
        if !same_retained(&owner.source_assets, expected) {
            return Err(
                "cold Act full original source/body/context/occasion no longer replays".into(),
            );
        }
        Ok(owner)
    }

    /// Activate only AFTER every original C page has compiled. Every actual
    /// parsed preparation/catalog reply survives a failure, including a native
    /// commit followed by an invalid role/source observation.
    pub(in crate::continuous) fn activate_cold_act_source(
        &mut self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
    ) -> Result<Vec<Value>, NativeStoppedExchangeFailure> {
        self.validate_current(current)?;
        let original_assets = self.source_assets.clone();
        let original_at = decimal(
            &original_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        let original = source.prepare_retained(
            self,
            current,
            original_at,
            &original_assets["current_receiving"],
        )?;
        if !same_retained(
            original.retained_snapshot(),
            &original_assets["current_receiving"],
        ) {
            return Err("cold source changed after complete original score compilation".into());
        }
        // A new actual resident starts at zero; the saved body's numerical
        // state is restored later through the native saved-cursor readmission.
        // This fresh admission is separate from the original retained source.
        let fresh = source.prepare_current(self, current, 0)?;
        fresh.validate_current(source, self, current, 0)?;
        let packet = self.packet()?;
        let admission = fresh.admission().snapshot()?;
        let prepare = json!({"schema":CONTROL,"operation":"prepare",
            "session_ref":self.config.session_ref,"packet":packet,
            "current_source_packet":packet,"actual_native_basis":self.binding.native_basis(),
            "m1_pratibimba":self.config.source_face==1,
            "physical_pratibimba":self.config.physical_face==1,
            "body_source":{"kind":"sourceForm","recipe_ref":self.config.recipe.provenance.reference,
                "validated_m3_generation":self.config.controls.expected_m3_generation.to_string()},
            "receiving_admission":admission,"current_receiving_admission":admission});
        let first = self.stopped_exchange_with_receipts(session, prepare)?;
        if first["accepted"] != true {
            return Err(NativeStoppedExchangeFailure {
                reason: "cold native body/source preparation refused".into(),
                native_receipts: vec![first],
            });
        }
        let mut catalog = self
            .raw("catalog")
            .map_err(|reason| NativeStoppedExchangeFailure {
                reason: session.performance_invalidate(&reason),
                native_receipts: vec![first.clone()],
            })?;
        catalog["cells"] = json!(self.cells);
        catalog["transpose"] = json!(self.config.transpose);
        let second = self
            .stopped_exchange_with_receipts(session, catalog)
            .map_err(|mut failure| {
                failure.native_receipts.insert(0, first.clone());
                failure
            })?;
        let checked = (|| -> Result<(), String> {
            if second["accepted"] != true
                || second["reading"]["samples_elapsed"] != "0"
                || second["reading"]["physical"]["samples_elapsed"] != "0"
                || !matches!(
                    second["reading"]["device"]["state"].as_str(),
                    Some("closed" | "prepared")
                )
                || second["reading"]["consumer_roles"] != original_assets["consumer_roles"]
                || !same_retained(&self.source_assets, &original_assets)
            {
                return Err(
                    "cold worker did not admit the same original source/body/consumer roles".into(),
                );
            }
            let repeated = source.prepare_retained(
                self,
                current,
                original_at,
                &original_assets["current_receiving"],
            )?;
            if !same_retained(repeated.retained_snapshot(), original.retained_snapshot()) {
                return Err("complete historical source changed during native activation".into());
            }
            self.validate_current(current)
        })();
        match checked {
            Ok(()) => Ok(vec![first, second]),
            Err(reason) => Err(NativeStoppedExchangeFailure {
                reason: session.performance_invalidate(&reason),
                native_receipts: vec![first, second],
            }),
        }
    }
}
