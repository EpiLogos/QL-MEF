//! Explicit reception of actual M3 command receipts through the historical
//! PatternPacket/Sprite law. This is not the historical contemplation close.
use crate::m3_state::{M3Receipt, M3Request};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Policy {
    #[serde(rename = "historical-personal-frame-sprite-v1")]
    HistoricalPersonalFrameSprite,
}
// Field order and omission exactly preserve historical VakAddress serde bytes.
#[derive(Serialize)]
struct Vak<'a> {
    cpf: &'a str,
    ct: &'a [String],
    cp: &'a str,
    cf: &'a str,
    cfp: &'a str,
    cs: Cs<'a>,
}
#[derive(Serialize)]
struct Cs<'a> {
    code: &'a str,
    direction: &'a str,
}
fn normal(q: [f32; 4]) -> [f32; 4] {
    let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if n < f32::EPSILON {
        [1., 0., 0., 0.]
    } else {
        [q[0] / n, q[1] / n, q[2] / n, q[3] / n]
    }
}
fn multiply(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        a[0] * b[0] - a[1] * b[1] - a[2] * b[2] - a[3] * b[3],
        a[0] * b[1] + a[1] * b[0] + a[2] * b[3] - a[3] * b[2],
        a[0] * b[2] - a[1] * b[3] + a[2] * b[0] + a[3] * b[1],
        a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0],
    ]
}
fn perturb(q: [f32; 4], bytes: &[u8], delta: f32) -> [f32; 4] {
    let mut h = blake3::Hasher::new();
    h.update(bytes);
    h.update(&delta.to_le_bytes());
    let hash = h.finalize();
    let b = hash.as_bytes();
    let raw = [
        (b[0] ^ 0x57) as f32 / 127.5 - 1.,
        b[11].wrapping_add(0x57) as f32 / 127.5 - 1.,
        b[23].wrapping_sub(0x57) as f32 / 127.5 - 1.,
    ];
    let mag = (raw[0] * raw[0] + raw[1] * raw[1] + raw[2] * raw[2]).sqrt();
    let axis = if mag < f32::EPSILON {
        [1., 0., 0.]
    } else {
        [raw[0] / mag, raw[1] / mag, raw[2] / mag]
    };
    let half = (0.0300 + delta.abs().min(8.) * 0.035) / 2.;
    let s = half.sin();
    normal(multiply(
        normal(q),
        normal([half.cos(), axis[0] * s, axis[1] * s, axis[2] * s]),
    ))
}

/// Receipts come directly from M3State::apply in this process. This function
/// accepts neither a supplied activity quaternion nor a claimed close bundle.
pub fn receive(
    request: &M3Request,
    receipts: &[M3Receipt],
    policy: Policy,
    start_generation: u64,
) -> Result<Value, String> {
    let coordinate = request
        .bases
        .iter()
        .find(|b| b.role == "coordinate")
        .ok_or("Selected M3 activity requires its native coordinate basis")?;
    let identity = request
        .bases
        .iter()
        .find(|b| b.role == "identity-source")
        .ok_or("Selected M3 activity requires its saved identity source basis")?;
    let mut q = [1f32, 0., 0., 0.];
    let mut codons = Vec::new();
    let mut packets = Vec::new();
    let mut previous = None;
    for receipt in receipts
        .iter()
        .filter(|r| r.status == "applied" && r.before_generation >= start_generation)
    {
        if receipt.subject_ref != request.subject_ref
            || receipt.event_ref != request.stamp.identity.event_ref
        {
            return Err("M3 activity receipt belongs to another subject or event".into());
        }
        let timestamp = receipt.command.occurrence_unix_ms;
        if previous.is_some_and(|p| timestamp < p) {
            return Err("M3 activity occurrence order moved backwards".into());
        }
        let delta = previous.map_or(0., |p| {
            (((timestamp - p) as f64 / 1_800_000.) as f32).clamp(0., 8.)
        });
        let sequence = receipt.after["transcription"]["sequence"]
            .as_str()
            .ok_or("Native M3 receipt lacks transcription")?;
        codons.push(sequence.to_owned());
        let vak = Vak {
            cpf: "(4.0/1-4.4/5)",
            ct: &codons,
            cp: &coordinate.reference,
            cf: "(4.0/1-4.4/5)",
            cfp: "4.4",
            cs: Cs {
                code: "M4",
                direction: "Day",
            },
        };
        let bytes = serde_json::to_vec(&vak).map_err(|e| e.to_string())?;
        q = perturb(q, &bytes, delta);
        let digest = format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(receipt).map_err(|e| e.to_string())?)
        );
        packets.push(json!({"packet_ref":format!("personal:m3-activity:{digest}"),"receipt_revision":format!("sha256:{digest}"),
            "vak_address":vak,"kairos_delta":delta,"occurrence_unix_ms":timestamp,
            "before_generation":receipt.before_generation,"after_generation":receipt.after_generation,
            "q_activity_after":{"w":q[0],"x":q[1],"y":q[2],"z":q[3]}}));
        previous = Some(timestamp);
    }
    let available = !packets.is_empty();
    Ok(
        json!({"schema":"ql.nara-m3-activity/v1","policy":policy,"start_generation":start_generation,"status":if available{"available"}else{"unavailable"},
        "subject_ref":request.subject_ref,"event_ref":request.stamp.identity.event_ref,"coordinate_ref":coordinate.reference,
        "identity_source_ref":identity.reference,"identity_revision":identity.revision,
        "q_activity":available.then_some(json!({"w":q[0],"x":q[1],"y":q[2],"z":q[3]})),"packets":packets,"turn_count":packets.len(),
        "emission":"one packet per successful native M3 command; CT retains ordered successful transcriptions",
        "frame_selection":"explicit historical personal-session frame; not inferred from activity content",
        "arithmetic":"historical f32 BLAKE3 VakAddress bytes + delta LE; Sprite 0.030+0.035*min(abs(delta),8); normalized Hamilton right fold",
        "source":{"repository":"EpiLogos/Epi-Logos-C-Experiments","revision":"c7872e96a12e8253de6818876c2a039fd8082e46",
            "packet_law":"Body/S/S0/portal-core/src/nara/mod.rs:333-379","perturbation":"Body/S/S0/portal-core/src/vama_shakti.rs:412-441",
            "frame_policy":"Body/S/S0/epi-cli/src/gate/nara.rs:863-955"},
        "standing":"explicit symbolic policy over native M3 interactions; no emotional inference or historical contemplation closure",
        "legacy_contemplation_close":false,"identity_mutated":false,"private":true,"public_export":false}),
    )
}
