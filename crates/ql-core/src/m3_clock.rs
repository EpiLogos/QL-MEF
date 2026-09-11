//! M3's 360/720 clock. The numeric clock and the retained symbolic estimate
//! are deliberately different objects. Numeric parity cannot promote an
//! unpopulated Tarot/Ananda/archetype column into a recognized source fact.
use std::sync::OnceLock;

pub const M3_CLOCK_VERSION: &str = "ql.m3-clock/v1";
pub const M3_CLOCK_SOURCE: &str =
    include_str!("../../../vendor/epi-kernel/reference/src/m3_clock_lut.c");
pub const CLOCK_FIELD_COUNT: usize = 26;

/// The full retained LUT layout; positions are shared with QL_M3_ClockField.
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockField {
    Degree360,
    DoubledDegree,
    ZodiacSign,
    ZodiacDegree,
    Decan,
    DecanPosition,
    BackboneFlag,
    HexagramEstimate,
    LineEstimate,
    NondualEstimate,
    ClassEstimate,
    UpperPairEstimate,
    LowerPairEstimate,
    TarotPlaceholder,
    DecanPlanetRecord,
    DecanElementRecord,
    DecanChakraRecord,
    Tick12Record,
    StrandRecord,
    DrRingRecord,
    AnandaPlaceholder,
    ArchetypePlaceholder,
    ShadowDegree,
    PolarDegree,
    ChamberRecord,
    ChamberFaceRecord,
}

/// Unwrapped steps preserve identity-return information lost by modulo alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M3Clock {
    steps: u64,
}

impl M3Clock {
    pub const fn at_steps(steps: u64) -> Self {
        Self { steps }
    }
    pub const fn steps(self) -> u64 {
        self.steps
    }
    pub const fn degree720(self) -> u16 {
        (self.steps % 720) as u16
    }
    pub const fn degree360(self) -> u16 {
        (self.steps % 360) as u16
    }
    pub const fn layer(self) -> u8 {
        (self.degree720() / 360) as u8
    }
    /// The retained m0/hopf law: 12 ticks PER 360-degree layer, not per 720.
    pub const fn tick12(self) -> u8 {
        (self.degree360() / 30) as u8
    }
    pub const fn decan_phase(self) -> u8 {
        (self.degree360() / 10) as u8 + self.layer() * 36
    }
    pub const fn polar720(self) -> u16 {
        self.layer() as u16 * 360 + (self.degree360() + 180) % 360
    }
    /// Arithmetic sector estimate from m0_read_cosmic_clock; not King Wen id.
    pub const fn uniform_hexagram_estimate(self) -> u8 {
        (self.degree360() * 64 / 360) as u8
    }
    pub const fn completed_double_covers(self) -> u64 {
        self.steps / 720
    }
    /// Overflow refuses the operation rather than silently resetting lineage.
    pub fn advance(self, delta: u64) -> Option<Self> {
        self.steps.checked_add(delta).map(Self::at_steps)
    }
    pub fn recorded_projection(self) -> &'static RecordedClockProjection {
        &recorded_clock()[self.degree360() as usize]
    }
}

/// These are retained source *values*, not recognized symbolic assignments.
/// The original producer states "computed hexagram approximation (no Neo4j)".
/// A raw zero therefore remains observable without claiming card/archetype 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordedClockProjection {
    pub fields: [u16; CLOCK_FIELD_COUNT],
}
impl RecordedClockProjection {
    pub const fn field(self, field: ClockField) -> u16 {
        self.fields[field as usize]
    }
    pub const fn reconciled_symbolic_fields(self) -> u32 {
        0
    }
}

pub fn recorded_clock() -> &'static [RecordedClockProjection; 360] {
    static RECORDS: OnceLock<[RecordedClockProjection; 360]> = OnceLock::new();
    RECORDS.get_or_init(|| parse_recorded_clock(M3_CLOCK_SOURCE).expect("retained M3 clock schema"))
}

/// Independent reader, not a hand-transcribed LUT or the C generator's parser.
/// Exposed for source-refresh and mutation tests; malformed source fails closed.
pub fn parse_recorded_clock(source: &str) -> Result<[RecordedClockProjection; 360], String> {
    let bytes = source.as_bytes();
    let mut clean = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i..].starts_with(b"/*") {
            i += 2;
            while i + 1 < bytes.len() && !bytes[i..].starts_with(b"*/") {
                i += 1;
            }
            if i + 1 >= bytes.len() {
                return Err("unterminated clock source comment".into());
            }
            i += 2;
            clean.push(b' ');
        } else if bytes[i..].starts_with(b"//") {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
        } else {
            clean.push(bytes[i]);
            i += 1;
        }
    }
    let text = String::from_utf8(clean).map_err(|e| e.to_string())?;
    let (_, table) = text
        .split_once("CLOCK_DEGREE_LUT[360]")
        .ok_or("clock declaration absent")?;
    let (body, _) = table.split_once("};").ok_or("clock terminator absent")?;
    let (_, body) = body.split_once('{').ok_or("clock initializer absent")?;
    let mut rows = Vec::new();
    for entry in body.split('{').skip(1) {
        let (fields, _) = entry.split_once('}').ok_or("clock row terminator absent")?;
        let values = fields
            .split(',')
            .map(|token| {
                let token = token.trim().trim_end_matches(['U', 'u', 'F', 'f']);
                let number = token.parse::<f64>().map_err(|e| e.to_string())?;
                if !number.is_finite()
                    || number.fract() != 0.0
                    || !(0.0..=65535.0).contains(&number)
                {
                    return Err("non-integral or out-of-range clock field".into());
                }
                Ok(number as u16)
            })
            .collect::<Result<Vec<_>, String>>()?;
        let fields: [u16; CLOCK_FIELD_COUNT] =
            values.try_into().map_err(|_| "clock width changed")?;
        if fields[0] as usize != rows.len() {
            return Err("clock order, duplicate or missing degree".into());
        }
        rows.push(RecordedClockProjection { fields });
    }
    rows.try_into()
        .map_err(|_| "clock row count changed".into())
}
