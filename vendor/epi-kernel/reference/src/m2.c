/**
 * m2.c — Parashakti: The Vibrational Architecture (Implementation)
 *
 * All .rodata LUT data + API implementation for M2.
 * FR Coverage: 2.2.0 – 2.2.12
 */

#include "m2.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


/* ===================================================================
 * .rodata: M2_ARCHETYPES — The Master 72-Byte Union
 *
 * MEF view is primary: flat indices 0-71 = (lens * 6 + position).
 * Same 72 bytes viewed as Tattvas, Decans, Shem through the union.
 * =================================================================== */

const M2_Vibrational_72_Space M2_ARCHETYPES = {
    .raw_vibration = {
         0,  1,  2,  3,  4,  5,     /*  Lens 0: Quaternal          */
         6,  7,  8,  9, 10, 11,     /*  Lens 1: Causal             */
        12, 13, 14, 15, 16, 17,     /*  Lens 2: Logical            */
        18, 19, 20, 21, 22, 23,     /*  Lens 3: Processual         */
        24, 25, 26, 27, 28, 29,     /*  Lens 4: Phenomenological   */
        30, 31, 32, 33, 34, 35,     /*  Lens 5: Para Vak           */
        36, 37, 38, 39, 40, 41,     /*  Lens 6: Archetypal-Num (') */
        42, 43, 44, 45, 46, 47,     /*  Lens 7: Phenomenal (')     */
        48, 49, 50, 51, 52, 53,     /*  Lens 8: Alchemical (')     */
        54, 55, 56, 57, 58, 59,     /*  Lens 9: Chronological (')  */
        60, 61, 62, 63, 64, 65,     /* Lens 10: Scientific (')     */
        66, 67, 68, 69, 70, 71      /* Lens 11: Divine Logos (')   */
    }
};


/* ===================================================================
 * .rodata: MEF LENS NAMES — Canonical 12-Lens Identity
 *
 * Source: QL Bimba Seed Mapping Planning - 2026-01-17.canvas
 * =================================================================== */

const char* const M2_MEF_LENS_NAMES[12] = {
    /* Base lenses (L0-L5) */
    "Quaternal",            /* L0 — QL as Psychoid Ground              */
    "Causal",               /* L1 — Aristotle, Iccha Shakti, Svatantrya */
    "Logical",              /* L2 — Tetralemmic / Catuskoti             */
    "Processual",           /* L3 — Concrescence, Rhea                 */
    "Phenomenological",     /* L4 — Science of Experience, Soteriology */
    "Para Vak",             /* L5 — Vak Ontology, Speech/Code          */
    /* Inverted lenses (L0'-L5') */
    "Archetypal-Numerical", /* L0' — Jung-Pauli Psychoid Numbers       */
    "Phenomenal",           /* L1' — Jungian Psychic Functions         */
    "Alchemical-Elemental", /* L2' — Transcendent Function             */
    "Chronological",        /* L3' — Hegel, Dialectic of Spirit, Aion  */
    "Scientific",           /* L4' — Knowledge Work, Research Paradigm */
    "Divine Logos"          /* L5' — Divine Personage, Epi-Logos       */
};


/* ===================================================================
 * .rodata: MEF_DESC[72] — Semantic Descriptor Table
 * =================================================================== */

const MEF_Condition_Desc M2_MEF_DESC[72] = {
    /* Lens 0: Quaternal (L0 base) */
    { 0, 0, 0, 0, 0x0000 }, { 0, 1, 0, 0, 0x0001 }, { 0, 2, 0, 0, 0x0002 },
    { 0, 3, 0, 0, 0x0003 }, { 0, 4, 0, 0, 0x0004 }, { 0, 5, 0, 0, 0x0005 },
    /* Lens 1: Causal (L1 base) */
    { 1, 0, 0, 1, 0x0010 }, { 1, 1, 0, 1, 0x0011 }, { 1, 2, 0, 1, 0x0012 },
    { 1, 3, 0, 1, 0x0013 }, { 1, 4, 0, 1, 0x0014 }, { 1, 5, 0, 1, 0x0015 },
    /* Lens 2: Logical (L2 base) */
    { 2, 0, 0, 2, 0x0020 }, { 2, 1, 0, 2, 0x0021 }, { 2, 2, 0, 2, 0x0022 },
    { 2, 3, 0, 2, 0x0023 }, { 2, 4, 0, 2, 0x0024 }, { 2, 5, 0, 2, 0x0025 },
    /* Lens 3: Processual (L3 base) */
    { 3, 0, 0, 3, 0x0030 }, { 3, 1, 0, 3, 0x0031 }, { 3, 2, 0, 3, 0x0032 },
    { 3, 3, 0, 3, 0x0033 }, { 3, 4, 0, 3, 0x0034 }, { 3, 5, 0, 3, 0x0035 },
    /* Lens 4: Phenomenological (L4 base) */
    { 4, 0, 0, 4, 0x0040 }, { 4, 1, 0, 4, 0x0041 }, { 4, 2, 0, 4, 0x0042 },
    { 4, 3, 0, 4, 0x0043 }, { 4, 4, 0, 4, 0x0044 }, { 4, 5, 0, 4, 0x0045 },
    /* Lens 5: Para Vak (L5 base) */
    { 5, 0, 0, 5, 0x0050 }, { 5, 1, 0, 5, 0x0051 }, { 5, 2, 0, 5, 0x0052 },
    { 5, 3, 0, 5, 0x0053 }, { 5, 4, 0, 5, 0x0054 }, { 5, 5, 0, 5, 0x0055 },
    /* Lens 6: Archetypal-Numerical (L0' inverted) */
    { 6, 0, 1, 0, 0x0060 }, { 6, 1, 1, 0, 0x0061 }, { 6, 2, 1, 0, 0x0062 },
    { 6, 3, 1, 0, 0x0063 }, { 6, 4, 1, 0, 0x0064 }, { 6, 5, 1, 0, 0x0065 },
    /* Lens 7: Phenomenal (L1' inverted) */
    { 7, 0, 1, 1, 0x0070 }, { 7, 1, 1, 1, 0x0071 }, { 7, 2, 1, 1, 0x0072 },
    { 7, 3, 1, 1, 0x0073 }, { 7, 4, 1, 1, 0x0074 }, { 7, 5, 1, 1, 0x0075 },
    /* Lens 8: Alchemical-Elemental (L2' inverted) */
    { 8, 0, 1, 2, 0x0080 }, { 8, 1, 1, 2, 0x0081 }, { 8, 2, 1, 2, 0x0082 },
    { 8, 3, 1, 2, 0x0083 }, { 8, 4, 1, 2, 0x0084 }, { 8, 5, 1, 2, 0x0085 },
    /* Lens 9: Chronological (L3' inverted) */
    { 9, 0, 1, 3, 0x0090 }, { 9, 1, 1, 3, 0x0091 }, { 9, 2, 1, 3, 0x0092 },
    { 9, 3, 1, 3, 0x0093 }, { 9, 4, 1, 3, 0x0094 }, { 9, 5, 1, 3, 0x0095 },
    /* Lens 10: Scientific (L4' inverted) */
    {10, 0, 1, 4, 0x00A0 }, {10, 1, 1, 4, 0x00A1 }, {10, 2, 1, 4, 0x00A2 },
    {10, 3, 1, 4, 0x00A3 }, {10, 4, 1, 4, 0x00A4 }, {10, 5, 1, 4, 0x00A5 },
    /* Lens 11: Divine Logos (L5' inverted) */
    {11, 0, 1, 5, 0x00B0 }, {11, 1, 1, 5, 0x00B1 }, {11, 2, 1, 5, 0x00B2 },
    {11, 3, 1, 5, 0x00B3 }, {11, 4, 1, 5, 0x00B4 }, {11, 5, 1, 5, 0x00B5 },
};


/* ===================================================================
 * .rodata: TATTVA_DESC[36] — 36 Tattva Descriptors
 *
 * 3 divisions: 5 Pure / 7 Pure-Impure / 24 Impure
 * =================================================================== */

const Tattva_Entry_Desc M2_TATTVA_DESC[36] = {
    /* Pure (Shuddha) — indices 0-4 */
    {  0, TATTVA_SHUDDHA, 0xFF, 0x00, 0x0100 },  /* Shiva              */
    {  1, TATTVA_SHUDDHA, 0xFF, 0x00, 0x0101 },  /* Shakti             */
    {  2, TATTVA_SHUDDHA, 0xFF, 0x00, 0x0102 },  /* Sadashiva          */
    {  3, TATTVA_SHUDDHA, 0xFF, 0x00, 0x0103 },  /* Ishvara            */
    {  4, TATTVA_SHUDDHA, 0xFF, 0x00, 0x0104 },  /* Shuddha Vidya      */
    /* Pure-Impure (Shuddhashuddha) — indices 5-11 */
    {  5, TATTVA_SHUDDHASHUDDHA, 0xFF, 0x01, 0x0110 },  /* Maya        */
    {  6, TATTVA_SHUDDHASHUDDHA, 0xFF, 0x02, 0x0111 },  /* Kanchuka: Kalaa    */
    {  7, TATTVA_SHUDDHASHUDDHA, 0xFF, 0x04, 0x0112 },  /* Kanchuka: Vidya    */
    {  8, TATTVA_SHUDDHASHUDDHA, 0xFF, 0x08, 0x0113 },  /* Kanchuka: Raga     */
    {  9, TATTVA_SHUDDHASHUDDHA, 0xFF, 0x10, 0x0114 },  /* Kanchuka: Kaala    */
    { 10, TATTVA_SHUDDHASHUDDHA, 0xFF, 0x20, 0x0115 },  /* Kanchuka: Niyati   */
    { 11, TATTVA_SHUDDHASHUDDHA, 0xFF, 0x00, 0x0116 },  /* Purusha            */
    /* Impure (Ashuddha) — indices 12-35 */
    { 12, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0120 },  /* Prakriti           */
    { 13, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0121 },  /* Buddhi             */
    { 14, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0122 },  /* Ahamkara           */
    { 15, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0123 },  /* Manas              */
    { 16, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0124 },  /* Jnanendriya: Ear   */
    { 17, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0125 },  /* Jnanendriya: Skin  */
    { 18, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0126 },  /* Jnanendriya: Eye   */
    { 19, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0127 },  /* Jnanendriya: Tongue*/
    { 20, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0128 },  /* Jnanendriya: Nose  */
    { 21, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0129 },  /* Karmendriya: Speech*/
    { 22, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x012A },  /* Karmendriya: Grasp */
    { 23, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x012B },  /* Karmendriya: Move  */
    { 24, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x012C },  /* Karmendriya: Excr  */
    { 25, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x012D },  /* Karmendriya: Repro */
    { 26, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x012E },  /* Tanmatra: Sound    */
    { 27, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x012F },  /* Tanmatra: Touch    */
    { 28, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0130 },  /* Tanmatra: Form     */
    { 29, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0131 },  /* Tanmatra: Taste    */
    { 30, TATTVA_ASHUDDHA, 0xFF, 0x00, 0x0132 },  /* Tanmatra: Smell    */
    /* Mahabhutas — the 5 elements (throughline anchor) */
    { 31, TATTVA_ASHUDDHA, ELEMENT_ID_AKASHA,  0x00, 0x0140 },  /* Akasha (Space) */
    { 32, TATTVA_ASHUDDHA, ELEMENT_ID_VAYU,    0x00, 0x0141 },  /* Vayu (Air)     */
    { 33, TATTVA_ASHUDDHA, ELEMENT_ID_AGNI,    0x00, 0x0142 },  /* Agni (Fire)    */
    { 34, TATTVA_ASHUDDHA, ELEMENT_ID_APAS,    0x00, 0x0143 },  /* Apas (Water)   */
    { 35, TATTVA_ASHUDDHA, ELEMENT_ID_PRITHVI, 0x00, 0x0144 },  /* Prithvi (Earth)*/
};


/* ===================================================================
 * .rodata: DECAN_DESC[72] — 72 Decan Face Descriptors
 *
 * [4 elements][3 signs][3 decans][2 faces] = 72
 * Element order: Fire(0), Earth(1), Air(2), Water(3)
 * =================================================================== */

const Decan_Face_Desc M2_DECAN_DESC[72] = {
    /* Fire — Aries(0), Leo(1), Sagittarius(2) */
    { ELEMENT_ID_AGNI, 0, 0, 0, PLANET_MARS,    0, 0x0200 },    /* Aries D1 Light   */
    { ELEMENT_ID_AGNI, 0, 0, 1, PLANET_MARS,    0, 0x0201 },    /* Aries D1 Shadow  */
    { ELEMENT_ID_AGNI, 0, 1, 0, PLANET_SUN,     0, 0x0202 },    /* Aries D2 Light   */
    { ELEMENT_ID_AGNI, 0, 1, 1, PLANET_SUN,     0, 0x0203 },    /* Aries D2 Shadow  */
    { ELEMENT_ID_AGNI, 0, 2, 0, PLANET_JUPITER, 0, 0x0204 },    /* Aries D3 Light   */
    { ELEMENT_ID_AGNI, 0, 2, 1, PLANET_JUPITER, 0, 0x0205 },    /* Aries D3 Shadow  */
    { ELEMENT_ID_AGNI, 1, 0, 0, PLANET_SUN,     0, 0x0206 },    /* Leo D1 Light     */
    { ELEMENT_ID_AGNI, 1, 0, 1, PLANET_SUN,     0, 0x0207 },    /* Leo D1 Shadow    */
    { ELEMENT_ID_AGNI, 1, 1, 0, PLANET_JUPITER, 0, 0x0208 },    /* Leo D2 Light     */
    { ELEMENT_ID_AGNI, 1, 1, 1, PLANET_JUPITER, 0, 0x0209 },    /* Leo D2 Shadow    */
    { ELEMENT_ID_AGNI, 1, 2, 0, PLANET_MARS,    0, 0x020A },    /* Leo D3 Light     */
    { ELEMENT_ID_AGNI, 1, 2, 1, PLANET_MARS,    0, 0x020B },    /* Leo D3 Shadow    */
    { ELEMENT_ID_AGNI, 2, 0, 0, PLANET_JUPITER, 0, 0x020C },    /* Sag D1 Light     */
    { ELEMENT_ID_AGNI, 2, 0, 1, PLANET_JUPITER, 0, 0x020D },    /* Sag D1 Shadow    */
    { ELEMENT_ID_AGNI, 2, 1, 0, PLANET_MARS,    0, 0x020E },    /* Sag D2 Light     */
    { ELEMENT_ID_AGNI, 2, 1, 1, PLANET_MARS,    0, 0x020F },    /* Sag D2 Shadow    */
    { ELEMENT_ID_AGNI, 2, 2, 0, PLANET_SUN,     0, 0x0210 },    /* Sag D3 Light     */
    { ELEMENT_ID_AGNI, 2, 2, 1, PLANET_SUN,     0, 0x0211 },    /* Sag D3 Shadow    */
    /* Earth — Taurus(0), Virgo(1), Capricorn(2) */
    { ELEMENT_ID_PRITHVI, 0, 0, 0, PLANET_VENUS,   0, 0x0220 }, /* Tau D1 Light     */
    { ELEMENT_ID_PRITHVI, 0, 0, 1, PLANET_VENUS,   0, 0x0221 }, /* Tau D1 Shadow    */
    { ELEMENT_ID_PRITHVI, 0, 1, 0, PLANET_MERCURY, 0, 0x0222 }, /* Tau D2 Light     */
    { ELEMENT_ID_PRITHVI, 0, 1, 1, PLANET_MERCURY, 0, 0x0223 }, /* Tau D2 Shadow    */
    { ELEMENT_ID_PRITHVI, 0, 2, 0, PLANET_SATURN,  0, 0x0224 }, /* Tau D3 Light     */
    { ELEMENT_ID_PRITHVI, 0, 2, 1, PLANET_SATURN,  0, 0x0225 }, /* Tau D3 Shadow    */
    { ELEMENT_ID_PRITHVI, 1, 0, 0, PLANET_MERCURY, 0, 0x0226 }, /* Vir D1 Light     */
    { ELEMENT_ID_PRITHVI, 1, 0, 1, PLANET_MERCURY, 0, 0x0227 }, /* Vir D1 Shadow    */
    { ELEMENT_ID_PRITHVI, 1, 1, 0, PLANET_SATURN,  0, 0x0228 }, /* Vir D2 Light     */
    { ELEMENT_ID_PRITHVI, 1, 1, 1, PLANET_SATURN,  0, 0x0229 }, /* Vir D2 Shadow    */
    { ELEMENT_ID_PRITHVI, 1, 2, 0, PLANET_VENUS,   0, 0x022A }, /* Vir D3 Light     */
    { ELEMENT_ID_PRITHVI, 1, 2, 1, PLANET_VENUS,   0, 0x022B }, /* Vir D3 Shadow    */
    { ELEMENT_ID_PRITHVI, 2, 0, 0, PLANET_SATURN,  0, 0x022C }, /* Cap D1 Light     */
    { ELEMENT_ID_PRITHVI, 2, 0, 1, PLANET_SATURN,  0, 0x022D }, /* Cap D1 Shadow    */
    { ELEMENT_ID_PRITHVI, 2, 1, 0, PLANET_VENUS,   0, 0x022E }, /* Cap D2 Light     */
    { ELEMENT_ID_PRITHVI, 2, 1, 1, PLANET_VENUS,   0, 0x022F }, /* Cap D2 Shadow    */
    { ELEMENT_ID_PRITHVI, 2, 2, 0, PLANET_MERCURY, 0, 0x0230 }, /* Cap D3 Light     */
    { ELEMENT_ID_PRITHVI, 2, 2, 1, PLANET_MERCURY, 0, 0x0231 }, /* Cap D3 Shadow    */
    /* Air — Gemini(0), Libra(1), Aquarius(2) */
    { ELEMENT_ID_VAYU, 0, 0, 0, PLANET_MERCURY, 0, 0x0240 },    /* Gem D1 Light     */
    { ELEMENT_ID_VAYU, 0, 0, 1, PLANET_MERCURY, 0, 0x0241 },    /* Gem D1 Shadow    */
    { ELEMENT_ID_VAYU, 0, 1, 0, PLANET_VENUS,   0, 0x0242 },    /* Gem D2 Light     */
    { ELEMENT_ID_VAYU, 0, 1, 1, PLANET_VENUS,   0, 0x0243 },    /* Gem D2 Shadow    */
    { ELEMENT_ID_VAYU, 0, 2, 0, PLANET_SATURN,  0, 0x0244 },    /* Gem D3 Light     */
    { ELEMENT_ID_VAYU, 0, 2, 1, PLANET_SATURN,  0, 0x0245 },    /* Gem D3 Shadow    */
    { ELEMENT_ID_VAYU, 1, 0, 0, PLANET_VENUS,   0, 0x0246 },    /* Lib D1 Light     */
    { ELEMENT_ID_VAYU, 1, 0, 1, PLANET_VENUS,   0, 0x0247 },    /* Lib D1 Shadow    */
    { ELEMENT_ID_VAYU, 1, 1, 0, PLANET_SATURN,  0, 0x0248 },    /* Lib D2 Light     */
    { ELEMENT_ID_VAYU, 1, 1, 1, PLANET_SATURN,  0, 0x0249 },    /* Lib D2 Shadow    */
    { ELEMENT_ID_VAYU, 1, 2, 0, PLANET_MERCURY, 0, 0x024A },    /* Lib D3 Light     */
    { ELEMENT_ID_VAYU, 1, 2, 1, PLANET_MERCURY, 0, 0x024B },    /* Lib D3 Shadow    */
    { ELEMENT_ID_VAYU, 2, 0, 0, PLANET_SATURN,  0, 0x024C },    /* Aqu D1 Light     */
    { ELEMENT_ID_VAYU, 2, 0, 1, PLANET_SATURN,  0, 0x024D },    /* Aqu D1 Shadow    */
    { ELEMENT_ID_VAYU, 2, 1, 0, PLANET_MERCURY, 0, 0x024E },    /* Aqu D2 Light     */
    { ELEMENT_ID_VAYU, 2, 1, 1, PLANET_MERCURY, 0, 0x024F },    /* Aqu D2 Shadow    */
    { ELEMENT_ID_VAYU, 2, 2, 0, PLANET_VENUS,   0, 0x0250 },    /* Aqu D3 Light     */
    { ELEMENT_ID_VAYU, 2, 2, 1, PLANET_VENUS,   0, 0x0251 },    /* Aqu D3 Shadow    */
    /* Water — Cancer(0), Scorpio(1), Pisces(2) */
    { ELEMENT_ID_APAS, 0, 0, 0, PLANET_MOON,    0, 0x0260 },    /* Can D1 Light     */
    { ELEMENT_ID_APAS, 0, 0, 1, PLANET_MOON,    0, 0x0261 },    /* Can D1 Shadow    */
    { ELEMENT_ID_APAS, 0, 1, 0, PLANET_MARS,    0, 0x0262 },    /* Can D2 Light     */
    { ELEMENT_ID_APAS, 0, 1, 1, PLANET_MARS,    0, 0x0263 },    /* Can D2 Shadow    */
    { ELEMENT_ID_APAS, 0, 2, 0, PLANET_JUPITER, 0, 0x0264 },    /* Can D3 Light     */
    { ELEMENT_ID_APAS, 0, 2, 1, PLANET_JUPITER, 0, 0x0265 },    /* Can D3 Shadow    */
    { ELEMENT_ID_APAS, 1, 0, 0, PLANET_MARS,    0, 0x0266 },    /* Sco D1 Light     */
    { ELEMENT_ID_APAS, 1, 0, 1, PLANET_MARS,    0, 0x0267 },    /* Sco D1 Shadow    */
    { ELEMENT_ID_APAS, 1, 1, 0, PLANET_JUPITER, 0, 0x0268 },    /* Sco D2 Light     */
    { ELEMENT_ID_APAS, 1, 1, 1, PLANET_JUPITER, 0, 0x0269 },    /* Sco D2 Shadow    */
    { ELEMENT_ID_APAS, 1, 2, 0, PLANET_MOON,    0, 0x026A },    /* Sco D3 Light     */
    { ELEMENT_ID_APAS, 1, 2, 1, PLANET_MOON,    0, 0x026B },    /* Sco D3 Shadow    */
    { ELEMENT_ID_APAS, 2, 0, 0, PLANET_JUPITER, 0, 0x026C },    /* Pis D1 Light     */
    { ELEMENT_ID_APAS, 2, 0, 1, PLANET_JUPITER, 0, 0x026D },    /* Pis D1 Shadow    */
    { ELEMENT_ID_APAS, 2, 1, 0, PLANET_MOON,    0, 0x026E },    /* Pis D2 Light     */
    { ELEMENT_ID_APAS, 2, 1, 1, PLANET_MOON,    0, 0x026F },    /* Pis D2 Shadow    */
    { ELEMENT_ID_APAS, 2, 2, 0, PLANET_MARS,    0, 0x0270 },    /* Pis D3 Light     */
    { ELEMENT_ID_APAS, 2, 2, 1, PLANET_MARS,    0, 0x0271 },    /* Pis D3 Shadow    */
};

const Decan_Face_Desc M2_QUINTESSENCE_DECAN = {
    .element = ELEMENT_ID_AKASHA,
    .sign    = 0xFF,
    .decan   = 0xFF,
    .face    = 0xFF,
    .ruling_planet = 0xFF,
    ._pad    = 0,
    .meaning_id = 0xFFFF
};


/* ===================================================================
 * .rodata: PLANET_LUT[10] — All 10 Planetary Bodies
 *
 * Canonical mod-10 ordering (2026-03-16, 00-canonical-invariants §2):
 *   index 0=Sun, 1=Moon, 2=Mercury, 3=Venus, 4=Mars,
 *         5=Jupiter, 6=Saturn, 7=Uranus, 8=Neptune, 9=Pluto
 * Earth is NOT in this array — see EarthBodyState.
 * Cousto frequencies: Hans Cousto cosmic octave derivation.
 * =================================================================== */

const Planet_Operator M2_PLANET_LUT[10] = {
    /* [0] Sun: stable root/parent, 126 Hz, DR=9 — not chakra-mapped */
    { PLANET_SUN,     0, 0, ELEM_SIG_PACK(ELEMENT_ID_AGNI, CHAKRA_EARTH, ELEM_PHASE_FUSED),
      PLANET_FREQ_SUN, PLANET_KEPLERIAN_SUN, 9, 0, 0x0300 },
    /* [1] Moon: U(1) phase factor, 210 Hz, DR=3 */
    { PLANET_MOON,    3, 0, ELEM_SIG_PACK(ELEMENT_ID_APAS, CHAKRA_SVADHISTHANA, ELEM_PHASE_ASCENT),
      PLANET_FREQ_MOON, PLANET_KEPLERIAN_MOON, 3, 1, 0x0301 },
    /* [2] Mercury: gauge boson SU(2)<->SU(3), 141 Hz, DR=6 */
    { PLANET_MERCURY, 1, 0, ELEM_SIG_PACK(ELEMENT_ID_VAYU, CHAKRA_VISHUDDHA, ELEM_PHASE_DESCENT),
      PLANET_FREQ_MERCURY, PLANET_KEPLERIAN_MERCURY, 6, 2, 0x0302 },
    /* [3] Venus: SU(3) lambda_3, aesthetic, 221 Hz, DR=5 */
    { PLANET_VENUS,   2, 0, ELEM_SIG_PACK(ELEMENT_ID_APAS, CHAKRA_ANAHATA, ELEM_PHASE_DESCENT),
      PLANET_FREQ_VENUS, PLANET_KEPLERIAN_VENUS, 5, 3, 0x0303 },
    /* [4] Mars: catalytic operator, 145 Hz, DR=1 */
    { PLANET_MARS,    4, 0, ELEM_SIG_PACK(ELEMENT_ID_AGNI, CHAKRA_MANIPURA, ELEM_PHASE_DESCENT),
      PLANET_FREQ_MARS, PLANET_KEPLERIAN_MARS, 1, 4, 0x0304 },
    /* [5] Jupiter: prime-41 expander, 184 Hz, DR=4 */
    { PLANET_JUPITER, 4, 41, ELEM_SIG_PACK(ELEMENT_ID_AGNI, CHAKRA_MANIPURA, ELEM_PHASE_ASCENT),
      PLANET_FREQ_JUPITER, PLANET_KEPLERIAN_JUPITER, 4, 5, 0x0305 },
    /* [6] Saturn: prime-43 constraint, 148 Hz, DR=4 */
    { PLANET_SATURN,  4, 43, ELEM_SIG_PACK(ELEMENT_ID_PRITHVI, CHAKRA_MULADHARA, ELEM_PHASE_DESCENT),
      PLANET_FREQ_SATURN, PLANET_KEPLERIAN_SATURN, 4, 6, 0x0306 },
    /* [7] Uranus: transpersonal awakener, 207 Hz, DR=9 */
    { PLANET_URANUS,  5, 0, ELEM_SIG_PACK(ELEMENT_ID_AKASHA, CHAKRA_AJNA, ELEM_PHASE_BEYOND),
      PLANET_FREQ_URANUS, PLANET_KEPLERIAN_URANUS, 9, 7, MEANING_ID_PREEMPTED },
    /* [8] Neptune: transpersonal dissolver, 211 Hz, DR=4 (Lemniscate) */
    { PLANET_NEPTUNE, 5, 47, ELEM_SIG_PACK(ELEMENT_ID_APAS, CHAKRA_SAHASRARA, ELEM_PHASE_BEYOND),
      PLANET_FREQ_NEPTUNE, PLANET_KEPLERIAN_NEPTUNE, 4, 8, MEANING_ID_PREEMPTED },
    /* [9] Pluto: transpersonal transformer, 140 Hz, DR=5 (Mobius) */
    { PLANET_PLUTO,   5, 53, ELEM_SIG_PACK(ELEMENT_ID_PRITHVI, CHAKRA_MULADHARA, ELEM_PHASE_BEYOND),
      PLANET_FREQ_PLUTO, PLANET_KEPLERIAN_PLUTO, 5, 9, MEANING_ID_PREEMPTED },
};


/* ===================================================================
 * .rodata: CHAKRA_LUT[8] — 8 Chakra Descriptors
 * =================================================================== */

const Chakra_Descriptor M2_CHAKRA_LUT[8] = {
    { CHAKRA_EARTH,        0xFF,             0xFF, 0, 0x0380 },  /* Ground potential  */
    { CHAKRA_MULADHARA,    ELEMENT_ID_PRITHVI, 35, 0, 0x0381 },  /* Prithvi           */
    { CHAKRA_SVADHISTHANA, ELEMENT_ID_APAS,    34, 0, 0x0382 },  /* Apas              */
    { CHAKRA_MANIPURA,     ELEMENT_ID_AGNI,    33, 0, 0x0383 },  /* Agni              */
    { CHAKRA_ANAHATA,      ELEMENT_ID_VAYU,    32, 0, 0x0384 },  /* Vayu              */
    { CHAKRA_VISHUDDHA,    ELEMENT_ID_AKASHA,  31, 0, 0x0385 },  /* Akasha            */
    { CHAKRA_AJNA,         0xFF,             0xFF, 0, 0x0386 },  /* Beyond elements   */
    { CHAKRA_SAHASRARA,    0xFF,             0xFF, 0, 0x0387 },  /* Beyond elements   */
};


/* ===================================================================
 * .rodata: SHEM_DESC[72] — 72 Angelic Name Entries
 *
 * 8 choirs x 9 names. Each carries element, decan_link, planet_link.
 * Element assignment cycles through the 5 elements.
 * =================================================================== */

const Shem_Name_Desc M2_SHEM_DESC[72] = {
    /* Choir 0: Seraphim (positions 0-8) */
    {  0, 0, 0, ELEMENT_ID_AGNI,    0, PLANET_MARS,     0x0400 },
    {  1, 0, 1, ELEMENT_ID_PRITHVI,  1, PLANET_SUN,      0x0401 },
    {  2, 0, 2, ELEMENT_ID_VAYU,     2, PLANET_JUPITER,  0x0402 },
    {  3, 0, 3, ELEMENT_ID_APAS,     3, PLANET_SUN,      0x0403 },
    {  4, 0, 4, ELEMENT_ID_AKASHA,   4, PLANET_JUPITER,  0x0404 },
    {  5, 0, 5, ELEMENT_ID_AGNI,     5, PLANET_MARS,     0x0405 },
    {  6, 0, 6, ELEMENT_ID_PRITHVI,  6, PLANET_JUPITER,  0x0406 },
    {  7, 0, 7, ELEMENT_ID_VAYU,     7, PLANET_MARS,     0x0407 },
    {  8, 0, 8, ELEMENT_ID_APAS,     8, PLANET_SUN,      0x0408 },
    /* Choir 1: Cherubim (positions 0-8) */
    {  9, 1, 0, ELEMENT_ID_AKASHA,   9, PLANET_VENUS,    0x0410 },
    { 10, 1, 1, ELEMENT_ID_AGNI,    10, PLANET_MERCURY,  0x0411 },
    { 11, 1, 2, ELEMENT_ID_PRITHVI, 11, PLANET_SATURN,   0x0412 },
    { 12, 1, 3, ELEMENT_ID_VAYU,    12, PLANET_MERCURY,  0x0413 },
    { 13, 1, 4, ELEMENT_ID_APAS,    13, PLANET_SATURN,   0x0414 },
    { 14, 1, 5, ELEMENT_ID_AKASHA,  14, PLANET_VENUS,    0x0415 },
    { 15, 1, 6, ELEMENT_ID_AGNI,    15, PLANET_SATURN,   0x0416 },
    { 16, 1, 7, ELEMENT_ID_PRITHVI, 16, PLANET_VENUS,    0x0417 },
    { 17, 1, 8, ELEMENT_ID_VAYU,    17, PLANET_MERCURY,  0x0418 },
    /* Choir 2: Thrones (positions 0-8) */
    { 18, 2, 0, ELEMENT_ID_APAS,    18, PLANET_VENUS,    0x0420 },
    { 19, 2, 1, ELEMENT_ID_AKASHA,  19, PLANET_MERCURY,  0x0421 },
    { 20, 2, 2, ELEMENT_ID_AGNI,    20, PLANET_SATURN,   0x0422 },
    { 21, 2, 3, ELEMENT_ID_PRITHVI, 21, PLANET_VENUS,    0x0423 },
    { 22, 2, 4, ELEMENT_ID_VAYU,    22, PLANET_SATURN,   0x0424 },
    { 23, 2, 5, ELEMENT_ID_APAS,    23, PLANET_MERCURY,  0x0425 },
    { 24, 2, 6, ELEMENT_ID_AKASHA,  24, PLANET_SATURN,   0x0426 },
    { 25, 2, 7, ELEMENT_ID_AGNI,    25, PLANET_VENUS,    0x0427 },
    { 26, 2, 8, ELEMENT_ID_PRITHVI, 26, PLANET_MERCURY,  0x0428 },
    /* Choir 3: Dominions (positions 0-8) */
    { 27, 3, 0, ELEMENT_ID_VAYU,    27, PLANET_MOON,     0x0430 },
    { 28, 3, 1, ELEMENT_ID_APAS,    28, PLANET_MARS,     0x0431 },
    { 29, 3, 2, ELEMENT_ID_AKASHA,  29, PLANET_JUPITER,  0x0432 },
    { 30, 3, 3, ELEMENT_ID_AGNI,    30, PLANET_MARS,     0x0433 },
    { 31, 3, 4, ELEMENT_ID_PRITHVI, 31, PLANET_JUPITER,  0x0434 },
    { 32, 3, 5, ELEMENT_ID_VAYU,    32, PLANET_MOON,     0x0435 },
    { 33, 3, 6, ELEMENT_ID_APAS,    33, PLANET_JUPITER,  0x0436 },
    { 34, 3, 7, ELEMENT_ID_AKASHA,  34, PLANET_MOON,     0x0437 },
    { 35, 3, 8, ELEMENT_ID_AGNI,    35, PLANET_MARS,     0x0438 },
    /* Choir 4: Virtues (positions 0-8) */
    { 36, 4, 0, ELEMENT_ID_PRITHVI, 36, PLANET_MOON,     0x0440 },
    { 37, 4, 1, ELEMENT_ID_VAYU,    37, PLANET_VENUS,    0x0441 },
    { 38, 4, 2, ELEMENT_ID_APAS,    38, PLANET_SATURN,   0x0442 },
    { 39, 4, 3, ELEMENT_ID_AKASHA,  39, PLANET_VENUS,    0x0443 },
    { 40, 4, 4, ELEMENT_ID_AGNI,    40, PLANET_SATURN,   0x0444 },
    { 41, 4, 5, ELEMENT_ID_PRITHVI, 41, PLANET_MOON,     0x0445 },
    { 42, 4, 6, ELEMENT_ID_VAYU,    42, PLANET_SATURN,   0x0446 },
    { 43, 4, 7, ELEMENT_ID_APAS,    43, PLANET_VENUS,    0x0447 },
    { 44, 4, 8, ELEMENT_ID_AKASHA,  44, PLANET_MOON,     0x0448 },
    /* Choir 5: Powers (positions 0-8) */
    { 45, 5, 0, ELEMENT_ID_AGNI,    45, PLANET_SATURN,   0x0450 },
    { 46, 5, 1, ELEMENT_ID_PRITHVI, 46, PLANET_MERCURY,  0x0451 },
    { 47, 5, 2, ELEMENT_ID_VAYU,    47, PLANET_VENUS,    0x0452 },
    { 48, 5, 3, ELEMENT_ID_APAS,    48, PLANET_MERCURY,  0x0453 },
    { 49, 5, 4, ELEMENT_ID_AKASHA,  49, PLANET_VENUS,    0x0454 },
    { 50, 5, 5, ELEMENT_ID_AGNI,    50, PLANET_SATURN,   0x0455 },
    { 51, 5, 6, ELEMENT_ID_PRITHVI, 51, PLANET_VENUS,    0x0456 },
    { 52, 5, 7, ELEMENT_ID_VAYU,    52, PLANET_MERCURY,  0x0457 },
    { 53, 5, 8, ELEMENT_ID_APAS,    53, PLANET_SATURN,   0x0458 },
    /* Choir 6: Principalities (positions 0-8) */
    { 54, 6, 0, ELEMENT_ID_AKASHA,  54, PLANET_JUPITER,  0x0460 },
    { 55, 6, 1, ELEMENT_ID_AGNI,    55, PLANET_MOON,     0x0461 },
    { 56, 6, 2, ELEMENT_ID_PRITHVI, 56, PLANET_MARS,     0x0462 },
    { 57, 6, 3, ELEMENT_ID_VAYU,    57, PLANET_JUPITER,  0x0463 },
    { 58, 6, 4, ELEMENT_ID_APAS,    58, PLANET_MOON,     0x0464 },
    { 59, 6, 5, ELEMENT_ID_AKASHA,  59, PLANET_MARS,     0x0465 },
    { 60, 6, 6, ELEMENT_ID_AGNI,    60, PLANET_MOON,     0x0466 },
    { 61, 6, 7, ELEMENT_ID_PRITHVI, 61, PLANET_JUPITER,  0x0467 },
    { 62, 6, 8, ELEMENT_ID_VAYU,    62, PLANET_MARS,     0x0468 },
    /* Choir 7: Archangels (positions 0-8) */
    { 63, 7, 0, ELEMENT_ID_APAS,    63, PLANET_MARS,     0x0470 },
    { 64, 7, 1, ELEMENT_ID_AKASHA,  64, PLANET_JUPITER,  0x0471 },
    { 65, 7, 2, ELEMENT_ID_AGNI,    65, PLANET_MOON,     0x0472 },
    { 66, 7, 3, ELEMENT_ID_PRITHVI, 66, PLANET_MARS,     0x0473 },
    { 67, 7, 4, ELEMENT_ID_VAYU,    67, PLANET_JUPITER,  0x0474 },
    { 68, 7, 5, ELEMENT_ID_APAS,    68, PLANET_MOON,     0x0475 },
    { 69, 7, 6, ELEMENT_ID_AKASHA,  69, PLANET_MARS,     0x0476 },
    { 70, 7, 7, ELEMENT_ID_AGNI,    70, PLANET_JUPITER,  0x0477 },
    { 71, 7, 8, ELEMENT_ID_PRITHVI, 71, PLANET_MOON,     0x0478 },
};


/* ===================================================================
 * .rodata: MAQAM RATIOS — 10 Maqam Family Harmonic Ratios
 * =================================================================== */

const Harmonic_Ratio M2_MAQAM_RATIOS[10] = {
    {  0,  0 },  /* Independent (hybrid forms) */
    { 27, 22 },  /* Rast — neutral third       */
    { 12, 11 },  /* Bayati — neutral second     */
    { 24, 23 },  /* Sikah — quarter-tone tonic  */
    { 75, 64 },  /* Hijaz — augmented second    */
    {  6,  5 },  /* Nahawand — minor third      */
    {  5,  4 },  /* Ajam — major third          */
    { 16, 15 },  /* Kurd — minor second         */
    { 13, 12 },  /* Saba — lowered second       */
    { 45, 32 },  /* Nawa Athar — raised fourth  */
};


/* ===================================================================
 * .rodata: MAQAM_DESC[72] — Musical Maqam Descriptors
 *
 * 9 families generating 72 modes total.
 * Interval patterns in quarter-tone units (24-TET).
 * =================================================================== */

const Maqam_Musical_Desc M2_MAQAM_DESC[72] = {
    /* Family 0: Independent (9 modes) */
    { 0, 0, { 4, 3, 3, 4, 3, 3, 4 }, PLANET_SUN,     0x0500 },
    { 0, 1, { 3, 4, 3, 3, 4, 3, 4 }, PLANET_SUN,     0x0501 },
    { 0, 2, { 4, 3, 4, 3, 3, 4, 3 }, PLANET_SUN,     0x0502 },
    { 0, 3, { 3, 3, 4, 4, 3, 3, 4 }, PLANET_SUN,     0x0503 },
    { 0, 4, { 4, 4, 3, 3, 4, 3, 3 }, PLANET_SUN,     0x0504 },
    { 0, 5, { 3, 4, 4, 3, 3, 4, 3 }, PLANET_SUN,     0x0505 },
    { 0, 6, { 4, 3, 3, 4, 4, 3, 3 }, PLANET_SUN,     0x0506 },
    { 0, 7, { 3, 3, 4, 3, 4, 4, 3 }, PLANET_SUN,     0x0507 },
    { 0, 8, { 3, 4, 3, 4, 3, 4, 3 }, PLANET_SUN,     0x0508 },
    /* Family 1: Rast (8 modes) */
    { 1, 0, { 4, 3, 3, 4, 4, 3, 3 }, PLANET_VENUS,   0x0510 },
    { 1, 1, { 3, 3, 4, 4, 3, 3, 4 }, PLANET_VENUS,   0x0511 },
    { 1, 2, { 3, 4, 4, 3, 3, 4, 3 }, PLANET_VENUS,   0x0512 },
    { 1, 3, { 4, 4, 3, 3, 4, 3, 3 }, PLANET_VENUS,   0x0513 },
    { 1, 4, { 4, 3, 3, 4, 3, 3, 4 }, PLANET_VENUS,   0x0514 },
    { 1, 5, { 3, 3, 4, 3, 3, 4, 4 }, PLANET_VENUS,   0x0515 },
    { 1, 6, { 3, 4, 3, 3, 4, 4, 3 }, PLANET_VENUS,   0x0516 },
    { 1, 7, { 4, 3, 3, 4, 4, 3, 3 }, PLANET_VENUS,   0x0517 },
    /* Family 2: Bayati (8 modes) */
    { 2, 0, { 3, 3, 4, 4, 3, 4, 3 }, PLANET_MOON,    0x0520 },
    { 2, 1, { 3, 4, 4, 3, 4, 3, 3 }, PLANET_MOON,    0x0521 },
    { 2, 2, { 4, 4, 3, 4, 3, 3, 3 }, PLANET_MOON,    0x0522 },
    { 2, 3, { 4, 3, 4, 3, 3, 3, 4 }, PLANET_MOON,    0x0523 },
    { 2, 4, { 3, 4, 3, 3, 3, 4, 4 }, PLANET_MOON,    0x0524 },
    { 2, 5, { 4, 3, 3, 3, 4, 4, 3 }, PLANET_MOON,    0x0525 },
    { 2, 6, { 3, 3, 3, 4, 4, 3, 4 }, PLANET_MOON,    0x0526 },
    { 2, 7, { 3, 3, 4, 4, 3, 4, 3 }, PLANET_MOON,    0x0527 },
    /* Family 3: Sikah (5 modes) */
    { 3, 0, { 3, 4, 3, 4, 3, 4, 3 }, PLANET_MERCURY, 0x0530 },
    { 3, 1, { 4, 3, 4, 3, 4, 3, 3 }, PLANET_MERCURY, 0x0531 },
    { 3, 2, { 3, 4, 3, 4, 3, 3, 4 }, PLANET_MERCURY, 0x0532 },
    { 3, 3, { 4, 3, 4, 3, 3, 4, 3 }, PLANET_MERCURY, 0x0533 },
    { 3, 4, { 3, 4, 3, 3, 4, 3, 4 }, PLANET_MERCURY, 0x0534 },
    /* Family 4: Hijaz (7 modes) */
    { 4, 0, { 2, 6, 2, 4, 2, 6, 2 }, PLANET_SATURN,  0x0540 },
    { 4, 1, { 6, 2, 4, 2, 6, 2, 2 }, PLANET_SATURN,  0x0541 },
    { 4, 2, { 2, 4, 2, 6, 2, 2, 6 }, PLANET_SATURN,  0x0542 },
    { 4, 3, { 4, 2, 6, 2, 2, 6, 2 }, PLANET_SATURN,  0x0543 },
    { 4, 4, { 2, 6, 2, 2, 6, 2, 4 }, PLANET_SATURN,  0x0544 },
    { 4, 5, { 6, 2, 2, 6, 2, 4, 2 }, PLANET_SATURN,  0x0545 },
    { 4, 6, { 2, 2, 6, 2, 4, 2, 6 }, PLANET_SATURN,  0x0546 },
    /* Family 5: Nahawand (8 modes) */
    { 5, 0, { 4, 2, 4, 4, 2, 4, 4 }, PLANET_JUPITER, 0x0550 },
    { 5, 1, { 2, 4, 4, 2, 4, 4, 4 }, PLANET_JUPITER, 0x0551 },
    { 5, 2, { 4, 4, 2, 4, 4, 4, 2 }, PLANET_JUPITER, 0x0552 },
    { 5, 3, { 4, 2, 4, 4, 4, 2, 4 }, PLANET_JUPITER, 0x0553 },
    { 5, 4, { 2, 4, 4, 4, 2, 4, 4 }, PLANET_JUPITER, 0x0554 },
    { 5, 5, { 4, 4, 4, 2, 4, 4, 2 }, PLANET_JUPITER, 0x0555 },
    { 5, 6, { 4, 4, 2, 4, 4, 2, 4 }, PLANET_JUPITER, 0x0556 },
    { 5, 7, { 4, 2, 4, 4, 2, 4, 4 }, PLANET_JUPITER, 0x0557 },
    /* Family 6: Ajam (6 modes) */
    { 6, 0, { 4, 4, 2, 4, 4, 4, 2 }, PLANET_VENUS,   0x0560 },
    { 6, 1, { 4, 2, 4, 4, 4, 2, 4 }, PLANET_VENUS,   0x0561 },
    { 6, 2, { 2, 4, 4, 4, 2, 4, 4 }, PLANET_VENUS,   0x0562 },
    { 6, 3, { 4, 4, 4, 2, 4, 4, 2 }, PLANET_VENUS,   0x0563 },
    { 6, 4, { 4, 4, 2, 4, 4, 2, 4 }, PLANET_VENUS,   0x0564 },
    { 6, 5, { 4, 2, 4, 4, 2, 4, 4 }, PLANET_VENUS,   0x0565 },
    /* Family 7: Kurd (7 modes) */
    { 7, 0, { 2, 4, 4, 4, 2, 4, 4 }, PLANET_MARS,    0x0570 },
    { 7, 1, { 4, 4, 4, 2, 4, 4, 2 }, PLANET_MARS,    0x0571 },
    { 7, 2, { 4, 4, 2, 4, 4, 2, 4 }, PLANET_MARS,    0x0572 },
    { 7, 3, { 4, 2, 4, 4, 2, 4, 4 }, PLANET_MARS,    0x0573 },
    { 7, 4, { 2, 4, 4, 2, 4, 4, 4 }, PLANET_MARS,    0x0574 },
    { 7, 5, { 4, 4, 2, 4, 4, 4, 2 }, PLANET_MARS,    0x0575 },
    { 7, 6, { 4, 2, 4, 4, 4, 2, 4 }, PLANET_MARS,    0x0576 },
    /* Family 8: Saba (9 modes) */
    { 8, 0, { 3, 3, 2, 6, 2, 4, 4 }, PLANET_NEPTUNE, 0x0580 },
    { 8, 1, { 3, 2, 6, 2, 4, 4, 3 }, PLANET_NEPTUNE, 0x0581 },
    { 8, 2, { 2, 6, 2, 4, 4, 3, 3 }, PLANET_NEPTUNE, 0x0582 },
    { 8, 3, { 6, 2, 4, 4, 3, 3, 2 }, PLANET_NEPTUNE, 0x0583 },
    { 8, 4, { 2, 4, 4, 3, 3, 2, 6 }, PLANET_NEPTUNE, 0x0584 },
    { 8, 5, { 4, 4, 3, 3, 2, 6, 2 }, PLANET_NEPTUNE, 0x0585 },
    { 8, 6, { 4, 3, 3, 2, 6, 2, 4 }, PLANET_NEPTUNE, 0x0586 },
    { 8, 7, { 3, 3, 2, 6, 2, 4, 4 }, PLANET_NEPTUNE, 0x0587 },
    { 8, 8, { 3, 2, 6, 2, 4, 4, 3 }, PLANET_NEPTUNE, 0x0588 },
    /* Family 9: Nawa Athar (5 modes) */
    { 9, 0, { 2, 6, 2, 4, 2, 6, 2 }, PLANET_PLUTO,   0x0590 },
    { 9, 1, { 6, 2, 4, 2, 6, 2, 2 }, PLANET_PLUTO,   0x0591 },
    { 9, 2, { 2, 4, 2, 6, 2, 2, 6 }, PLANET_PLUTO,   0x0592 },
    { 9, 3, { 4, 2, 6, 2, 2, 6, 2 }, PLANET_PLUTO,   0x0593 },
    { 9, 4, { 2, 6, 2, 2, 6, 2, 4 }, PLANET_PLUTO,   0x0594 },
};


/* ===================================================================
 * .rodata: MAQAM_SPIRITUAL[8][3] — Spiritual Maqam Stations
 * =================================================================== */

const Maqam_Spiritual_Desc M2_MAQAM_SPIRITUAL[8][3] = {
    /* La Maqam (meta-station, fused ground) */
    {{ 0, 0, 0x0600 }, { 0, 1, 0x0601 }, { 0, 2, 0x0602 }},
    /* Tawba (Repentance) */
    {{ 1, 0, 0x0610 }, { 1, 1, 0x0611 }, { 1, 2, 0x0612 }},
    /* Sabr (Patience) */
    {{ 2, 0, 0x0620 }, { 2, 1, 0x0621 }, { 2, 2, 0x0622 }},
    /* Shukr (Gratitude) */
    {{ 3, 0, 0x0630 }, { 3, 1, 0x0631 }, { 3, 2, 0x0632 }},
    /* Khawf (Awe/Fear) */
    {{ 4, 0, 0x0640 }, { 4, 1, 0x0641 }, { 4, 2, 0x0642 }},
    /* Raja (Hope) */
    {{ 5, 0, 0x0650 }, { 5, 1, 0x0651 }, { 5, 2, 0x0652 }},
    /* Tawakkul (Trust/Surrender) */
    {{ 6, 0, 0x0660 }, { 6, 1, 0x0661 }, { 6, 2, 0x0662 }},
    /* Rida (Contentment/Satisfaction) */
    {{ 7, 0, 0x0670 }, { 7, 1, 0x0671 }, { 7, 2, 0x0672 }},
};


/* ===================================================================
 * .rodata: ASMA_LUT[100] — 99 Divine Names + Al-Ism al-A'zham
 *
 * 3 groups of 33 (Jalal/Kamal/Jamal) + 1 hidden name.
 * Each carries abjad_value, digital_root, mirror_idx.
 * =================================================================== */

const Asma_Name_Desc M2_ASMA_LUT[100] = {
    /* Jalal (Majesty) — indices 0-32 */
    {  0, 0,  0, ELEMENT_ID_AGNI,    1, 0xFF, 66,  0x0700, {0,0} },  /* Ar-Rahman        */
    {  1, 0,  1, ELEMENT_ID_AGNI,    9, 0xFF, 298, 0x0701, {0,0} },  /* Ar-Rahim         */
    {  2, 0,  2, ELEMENT_ID_AGNI,    8,   35, 62,  0x0702, {0,0} },  /* Al-Malik          */
    {  3, 0,  3, ELEMENT_ID_AGNI,    2,   36, 170, 0x0703, {0,0} },  /* Al-Quddus         */
    {  4, 0,  4, ELEMENT_ID_AGNI,    2,   37, 170, 0x0704, {0,0} },  /* As-Salam          */
    {  5, 0,  5, ELEMENT_ID_AGNI,    3, 0xFF, 48,  0x0705, {0,0} },  /* Al-Mu'min         */
    {  6, 0,  6, ELEMENT_ID_AGNI,    4, 0xFF, 526, 0x0706, {0,0} },  /* Al-Muhaymin       */
    {  7, 0,  7, ELEMENT_ID_AGNI,    1, 0xFF, 19,  0x0707, {0,0} },  /* Al-Aziz           */
    {  8, 0,  8, ELEMENT_ID_AGNI,    3, 0xFF, 165, 0x0708, {0,0} },  /* Al-Jabbar         */
    {  9, 0,  9, ELEMENT_ID_AGNI,    2, 0xFF, 1020,0x0709, {0,0} },  /* Al-Mutakabbir     */
    { 10, 0, 10, ELEMENT_ID_AGNI,    4, 0xFF, 931, 0x070A, {0,0} },  /* Al-Khaliq         */
    { 11, 0, 11, ELEMENT_ID_AGNI,    9, 0xFF, 207, 0x070B, {0,0} },  /* Al-Bari           */
    { 12, 0, 12, ELEMENT_ID_AGNI,    5, 0xFF, 284, 0x070C, {0,0} },  /* Al-Musawwir       */
    { 13, 0, 13, ELEMENT_ID_AGNI,    8, 0xFF, 1116,0x070D, {0,0} },  /* Al-Ghaffar        */
    { 14, 0, 14, ELEMENT_ID_AGNI,    7, 0xFF, 952, 0x070E, {0,0} },  /* Al-Qahhar         */
    { 15, 0, 15, ELEMENT_ID_AGNI,    3, 0xFF, 30,  0x070F, {0,0} },  /* Al-Wahhab         */
    { 16, 0, 16, ELEMENT_ID_AGNI,    2, 0xFF, 308, 0x0710, {0,0} },  /* Ar-Razzaq         */
    { 17, 0, 17, ELEMENT_ID_AGNI,    7, 0xFF, 862, 0x0711, {0,0} },  /* Al-Fattah         */
    { 18, 0, 18, ELEMENT_ID_AGNI,    2, 0xFF, 218, 0x0712, {0,0} },  /* Al-Alim           */
    { 19, 0, 19, ELEMENT_ID_AGNI,    1, 0xFF, 100, 0x0713, {0,0} },  /* Al-Qabid          */
    { 20, 0, 20, ELEMENT_ID_AGNI,    8, 0xFF, 800, 0x0714, {0,0} },  /* Al-Basit          */
    { 21, 0, 21, ELEMENT_ID_AGNI,    1,   22, 100, 0x0715, {0,0} },  /* Al-Khafid         */
    { 22, 0, 22, ELEMENT_ID_AGNI,    5,   21, 500, 0x0716, {0,0} },  /* Ar-Rafi           */
    { 23, 0, 23, ELEMENT_ID_AGNI,    2, 0xFF, 200, 0x0717, {0,0} },  /* Al-Mu'izz         */
    { 24, 0, 24, ELEMENT_ID_AGNI,    1, 0xFF, 802, 0x0718, {0,0} },  /* Al-Mudhill        */
    { 25, 0, 25, ELEMENT_ID_AGNI,    3, 0xFF, 210, 0x0719, {0,0} },  /* As-Sami           */
    { 26, 0, 26, ELEMENT_ID_AGNI,    4, 0xFF, 112, 0x071A, {0,0} },  /* Al-Basir          */
    { 27, 0, 27, ELEMENT_ID_AGNI,    7, 0xFF, 115, 0x071B, {0,0} },  /* Al-Hakam          */
    { 28, 0, 28, ELEMENT_ID_AGNI,    3, 0xFF, 210, 0x071C, {0,0} },  /* Al-Adl            */
    { 29, 0, 29, ELEMENT_ID_AGNI,    3, 0xFF, 273, 0x071D, {0,0} },  /* Al-Latif          */
    { 30, 0, 30, ELEMENT_ID_AGNI,    6, 0xFF, 1032,0x071E, {0,0} },  /* Al-Khabir         */
    { 31, 0, 31, ELEMENT_ID_AGNI,    8, 0xFF, 305, 0x071F, {0,0} },  /* Al-Halim          */
    { 32, 0, 32, ELEMENT_ID_AGNI,    1, 0xFF, 1066,0x0720, {0,0} },  /* Al-Azim           */
    /* Kamal (Perfection) — indices 33-65 */
    { 33, 1,  0, ELEMENT_ID_VAYU,    4, 0xFF, 526, 0x0730, {0,0} },  /* Al-Ghafur         */
    { 34, 1,  1, ELEMENT_ID_VAYU,    4, 0xFF, 526, 0x0731, {0,0} },  /* Ash-Shakur        */
    { 35, 1,  2, ELEMENT_ID_VAYU,    8, 0xFF, 305, 0x0732, {0,0} },  /* Al-Aliyy          */
    { 36, 1,  3, ELEMENT_ID_VAYU,    4, 0xFF, 256, 0x0733, {0,0} },  /* Al-Kabir          */
    { 37, 1,  4, ELEMENT_ID_VAYU,    5, 0xFF, 338, 0x0734, {0,0} },  /* Al-Hafiz          */
    { 38, 1,  5, ELEMENT_ID_VAYU,    5, 0xFF, 140, 0x0735, {0,0} },  /* Al-Muqit          */
    { 39, 1,  6, ELEMENT_ID_VAYU,    5, 0xFF, 392, 0x0736, {0,0} },  /* Al-Hasib          */
    { 40, 1,  7, ELEMENT_ID_VAYU,    2, 0xFF, 236, 0x0737, {0,0} },  /* Al-Jalil          */
    { 41, 1,  8, ELEMENT_ID_VAYU,    2, 0xFF, 236, 0x0738, {0,0} },  /* Al-Karim          */
    { 42, 1,  9, ELEMENT_ID_VAYU,    7, 0xFF, 340, 0x0739, {0,0} },  /* Ar-Raqib          */
    { 43, 1, 10, ELEMENT_ID_VAYU,    7, 0xFF, 340, 0x073A, {0,0} },  /* Al-Mujib          */
    { 44, 1, 11, ELEMENT_ID_VAYU,    7, 0xFF, 232, 0x073B, {0,0} },  /* Al-Wasi           */
    { 45, 1, 12, ELEMENT_ID_VAYU,    4, 0xFF, 256, 0x073C, {0,0} },  /* Al-Hakim          */
    { 46, 1, 13, ELEMENT_ID_VAYU,    4, 0xFF, 139, 0x073D, {0,0} },  /* Al-Wadud          */
    { 47, 1, 14, ELEMENT_ID_VAYU,    2, 0xFF, 416, 0x073E, {0,0} },  /* Al-Majid          */
    { 48, 1, 15, ELEMENT_ID_VAYU,    7, 0xFF, 403, 0x073F, {0,0} },  /* Al-Ba'ith         */
    { 49, 1, 16, ELEMENT_ID_VAYU,    8, 0xFF, 341, 0x0740, {0,0} },  /* Ash-Shahid        */
    { 50, 1, 17, ELEMENT_ID_VAYU,    5, 0xFF, 500, 0x0741, {0,0} },  /* Al-Haqq           */
    { 51, 1, 18, ELEMENT_ID_VAYU,    1, 0xFF, 514, 0x0742, {0,0} },  /* Al-Wakil          */
    { 52, 1, 19, ELEMENT_ID_VAYU,    5, 0xFF, 500, 0x0743, {0,0} },  /* Al-Qawiyy         */
    { 53, 1, 20, ELEMENT_ID_VAYU,    5, 0xFF, 311, 0x0744, {0,0} },  /* Al-Matin          */
    { 54, 1, 21, ELEMENT_ID_VAYU,    4, 0xFF, 1168,0x0745, {0,0} },  /* Al-Waliyy         */
    { 55, 1, 22, ELEMENT_ID_VAYU,    7, 0xFF, 232, 0x0746, {0,0} },  /* Al-Hamid          */
    { 56, 1, 23, ELEMENT_ID_VAYU,    3, 0xFF, 453, 0x0747, {0,0} },  /* Al-Muhsi          */
    { 57, 1, 24, ELEMENT_ID_VAYU,    7, 0xFF, 214, 0x0748, {0,0} },  /* Al-Mubdi          */
    { 58, 1, 25, ELEMENT_ID_VAYU,    7, 0xFF, 214, 0x0749, {0,0} },  /* Al-Mu'id          */
    { 59, 1, 26, ELEMENT_ID_VAYU,    8, 0xFF, 305, 0x074A, {0,0} },  /* Al-Muhyi          */
    { 60, 1, 27, ELEMENT_ID_VAYU,    6, 0xFF, 312, 0x074B, {0,0} },  /* Al-Mumit          */
    { 61, 1, 28, ELEMENT_ID_VAYU,    9, 0xFF, 18,  0x074C, {0,0} },  /* Al-Hayy           */
    { 62, 1, 29, ELEMENT_ID_VAYU,    3, 0xFF, 228, 0x074D, {0,0} },  /* Al-Qayyum         */
    { 63, 1, 30, ELEMENT_ID_VAYU,    4, 0xFF, 256, 0x074E, {0,0} },  /* Al-Wajid          */
    { 64, 1, 31, ELEMENT_ID_VAYU,    4, 0xFF, 256, 0x074F, {0,0} },  /* Al-Majid          */
    { 65, 1, 32, ELEMENT_ID_VAYU,    2, 0xFF, 101, 0x0750, {0,0} },  /* Al-Wahid          */
    /* Jamal (Beauty) — indices 66-98 */
    { 66, 2,  0, ELEMENT_ID_APAS,    4, 0xFF, 139, 0x0760, {0,0} },  /* As-Samad          */
    { 67, 2,  1, ELEMENT_ID_APAS,    4, 0xFF, 256, 0x0761, {0,0} },  /* Al-Qadir          */
    { 68, 2,  2, ELEMENT_ID_APAS,    5, 0xFF, 500, 0x0762, {0,0} },  /* Al-Muqtadir       */
    { 69, 2,  3, ELEMENT_ID_APAS,    8, 0xFF, 305, 0x0763, {0,0} },  /* Al-Muqaddim       */
    { 70, 2,  4, ELEMENT_ID_APAS,    5, 0xFF, 842, 0x0764, {0,0} },  /* Al-Mu'akhkhir     */
    { 71, 2,  5, ELEMENT_ID_APAS,    7, 0xFF, 232, 0x0765, {0,0} },  /* Al-Awwal          */
    { 72, 2,  6, ELEMENT_ID_APAS,    4, 0xFF, 802, 0x0766, {0,0} },  /* Al-Akhir          */
    { 73, 2,  7, ELEMENT_ID_APAS,    5, 0xFF, 1106,0x0767, {0,0} },  /* Az-Zahir          */
    { 74, 2,  8, ELEMENT_ID_APAS,    5, 0xFF, 311, 0x0768, {0,0} },  /* Al-Batin          */
    { 75, 2,  9, ELEMENT_ID_APAS,    3, 0xFF, 210, 0x0769, {0,0} },  /* Al-Wali           */
    { 76, 2, 10, ELEMENT_ID_APAS,    1, 0xFF, 550, 0x076A, {0,0} },  /* Al-Muta'ali       */
    { 77, 2, 11, ELEMENT_ID_APAS,    1, 0xFF, 208, 0x076B, {0,0} },  /* Al-Barr           */
    { 78, 2, 12, ELEMENT_ID_APAS,    4, 0xFF, 256, 0x076C, {0,0} },  /* At-Tawwab         */
    { 79, 2, 13, ELEMENT_ID_APAS,    4, 0xFF, 256, 0x076D, {0,0} },  /* Al-Muntaqim       */
    { 80, 2, 14, ELEMENT_ID_APAS,    2, 0xFF, 236, 0x076E, {0,0} },  /* Al-Afuww          */
    { 81, 2, 15, ELEMENT_ID_APAS,    5, 0xFF, 167, 0x076F, {0,0} },  /* Ar-Ra'uf          */
    { 82, 2, 16, ELEMENT_ID_APAS,    4, 0xFF, 301, 0x0770, {0,0} },  /* Malik al-Mulk     */
    { 83, 2, 17, ELEMENT_ID_APAS,    3, 0xFF, 1100,0x0771, {0,0} },  /* Dhul-Jalal wal-Ikram */
    { 84, 2, 18, ELEMENT_ID_APAS,    5, 0xFF, 302, 0x0772, {0,0} },  /* Al-Muqsit         */
    { 85, 2, 19, ELEMENT_ID_APAS,    3, 0xFF, 471, 0x0773, {0,0} },  /* Al-Jami           */
    { 86, 2, 20, ELEMENT_ID_APAS,    1, 0xFF, 1060,0x0774, {0,0} },  /* Al-Ghaniyy        */
    { 87, 2, 21, ELEMENT_ID_APAS,    1, 0xFF, 1060,0x0775, {0,0} },  /* Al-Mughni         */
    { 88, 2, 22, ELEMENT_ID_APAS,    9, 0xFF, 270, 0x0776, {0,0} },  /* Al-Mani           */
    { 89, 2, 23, ELEMENT_ID_APAS,    4, 0xFF, 1060,0x0777, {0,0} },  /* Ad-Darr           */
    { 90, 2, 24, ELEMENT_ID_APAS,    3, 0xFF, 300, 0x0778, {0,0} },  /* An-Nafi           */
    { 91, 2, 25, ELEMENT_ID_APAS,    3, 0xFF, 300, 0x0779, {0,0} },  /* An-Nur            */
    { 92, 2, 26, ELEMENT_ID_APAS,    1, 0xFF, 208, 0x077A, {0,0} },  /* Al-Hadi           */
    { 93, 2, 27, ELEMENT_ID_APAS,    5, 0xFF, 500, 0x077B, {0,0} },  /* Al-Badi           */
    { 94, 2, 28, ELEMENT_ID_APAS,    1, 0xFF, 100, 0x077C, {0,0} },  /* Al-Baqi           */
    { 95, 2, 29, ELEMENT_ID_APAS,    8, 0xFF, 350, 0x077D, {0,0} },  /* Al-Warith         */
    { 96, 2, 30, ELEMENT_ID_APAS,    3, 0xFF, 246, 0x077E, {0,0} },  /* Ar-Rashid         */
    { 97, 2, 31, ELEMENT_ID_APAS,    2, 0xFF, 200, 0x077F, {0,0} },  /* As-Sabur          */
    { 98, 2, 32, ELEMENT_ID_APAS,    7, 0xFF, 232, 0x0780, {0,0} },  /* Reserved          */
    /* Al-Ism al-A'zham — the Hidden Name (index 99) */
    { 99, 0xFF, 0xFF, ELEMENT_ID_AKASHA, 9, 0xFF, 0xFFFF, 0x07FF, {0,0} },
};


/* ===================================================================
 * .rodata: MANTRA_LUT[100] — 100 Mantra Entries (50+50)
 *
 * Frequencies 144-432 Hz with elemental signatures.
 * First 50 = Matrika (Bimba), Last 50 = Malini (Pratibimba).
 * =================================================================== */

const Mantra_Entry_Desc M2_MANTRA_LUT[100] = {
    /* Matrika (Bimba) — 16 vowels + 34 consonants */
    /* Vowels (0-15) */
    {  0, 0, ELEMENT_ID_AKASHA, 0, 432, 0x0800 },  /* A          */
    {  1, 0, ELEMENT_ID_AKASHA, 0, 428, 0x0801 },  /* Aa         */
    {  2, 0, ELEMENT_ID_AKASHA, 0, 424, 0x0802 },  /* I          */
    {  3, 0, ELEMENT_ID_AKASHA, 0, 420, 0x0803 },  /* Ii         */
    {  4, 0, ELEMENT_ID_AKASHA, 0, 416, 0x0804 },  /* U          */
    {  5, 0, ELEMENT_ID_AKASHA, 0, 412, 0x0805 },  /* Uu         */
    {  6, 0, ELEMENT_ID_AKASHA, 0, 408, 0x0806 },  /* Ri         */
    {  7, 0, ELEMENT_ID_AKASHA, 0, 404, 0x0807 },  /* Rii        */
    {  8, 0, ELEMENT_ID_AKASHA, 0, 400, 0x0808 },  /* Lri        */
    {  9, 0, ELEMENT_ID_AKASHA, 0, 396, 0x0809 },  /* Lrii       */
    { 10, 0, ELEMENT_ID_AKASHA, 0, 392, 0x080A },  /* E          */
    { 11, 0, ELEMENT_ID_AKASHA, 0, 388, 0x080B },  /* Ai         */
    { 12, 0, ELEMENT_ID_AKASHA, 0, 384, 0x080C },  /* O          */
    { 13, 0, ELEMENT_ID_AKASHA, 0, 380, 0x080D },  /* Au         */
    { 14, 0, ELEMENT_ID_AKASHA, 0, 376, 0x080E },  /* Am (anusvara) */
    { 15, 0, ELEMENT_ID_AKASHA, 0, 372, 0x080F },  /* Ah (visarga)  */
    /* Guttural consonants (16-20) — Akasha */
    { 16, 1, ELEMENT_ID_AKASHA, 0, 368, 0x0810 },
    { 17, 1, ELEMENT_ID_AKASHA, 0, 364, 0x0811 },
    { 18, 1, ELEMENT_ID_AKASHA, 0, 360, 0x0812 },
    { 19, 1, ELEMENT_ID_AKASHA, 0, 356, 0x0813 },
    { 20, 1, ELEMENT_ID_AKASHA, 0, 352, 0x0814 },
    /* Palatal consonants (21-25) — Vayu */
    { 21, 2, ELEMENT_ID_VAYU, 0, 348, 0x0815 },
    { 22, 2, ELEMENT_ID_VAYU, 0, 344, 0x0816 },
    { 23, 2, ELEMENT_ID_VAYU, 0, 340, 0x0817 },
    { 24, 2, ELEMENT_ID_VAYU, 0, 336, 0x0818 },
    { 25, 2, ELEMENT_ID_VAYU, 0, 332, 0x0819 },
    /* Retroflex consonants (26-30) — Agni */
    { 26, 3, ELEMENT_ID_AGNI, 0, 328, 0x081A },
    { 27, 3, ELEMENT_ID_AGNI, 0, 324, 0x081B },
    { 28, 3, ELEMENT_ID_AGNI, 0, 320, 0x081C },
    { 29, 3, ELEMENT_ID_AGNI, 0, 316, 0x081D },
    { 30, 3, ELEMENT_ID_AGNI, 0, 312, 0x081E },
    /* Dental consonants (31-35) — Apas */
    { 31, 4, ELEMENT_ID_APAS, 0, 308, 0x081F },
    { 32, 4, ELEMENT_ID_APAS, 0, 304, 0x0820 },
    { 33, 4, ELEMENT_ID_APAS, 0, 300, 0x0821 },
    { 34, 4, ELEMENT_ID_APAS, 0, 296, 0x0822 },
    { 35, 4, ELEMENT_ID_APAS, 0, 292, 0x0823 },
    /* Labial consonants (36-40) — Prithvi */
    { 36, 5, ELEMENT_ID_PRITHVI, 0, 288, 0x0824 },
    { 37, 5, ELEMENT_ID_PRITHVI, 0, 284, 0x0825 },
    { 38, 5, ELEMENT_ID_PRITHVI, 0, 280, 0x0826 },
    { 39, 5, ELEMENT_ID_PRITHVI, 0, 276, 0x0827 },
    { 40, 5, ELEMENT_ID_PRITHVI, 0, 272, 0x0828 },
    /* Semivowels (41-44) — mixed */
    { 41, 6, ELEMENT_ID_VAYU,    0, 268, 0x0829 },
    { 42, 6, ELEMENT_ID_AGNI,    0, 264, 0x082A },
    { 43, 6, ELEMENT_ID_APAS,    0, 260, 0x082B },
    { 44, 6, ELEMENT_ID_PRITHVI, 0, 256, 0x082C },
    /* Sibilants + Ha (45-49) */
    { 45, 7, ELEMENT_ID_AKASHA,  0, 252, 0x082D },
    { 46, 7, ELEMENT_ID_AGNI,    0, 248, 0x082E },
    { 47, 7, ELEMENT_ID_VAYU,    0, 244, 0x082F },
    { 48, 7, ELEMENT_ID_AKASHA,  0, 240, 0x0830 },
    { 49, 7, ELEMENT_ID_AKASHA,  0, 236, 0x0831 },
    /* Malini (Pratibimba) — 50 consciousness-mantras */
    { 50, 0, ELEMENT_ID_AKASHA,  1, 232, 0x0840 },
    { 51, 0, ELEMENT_ID_AKASHA,  1, 228, 0x0841 },
    { 52, 0, ELEMENT_ID_AKASHA,  1, 224, 0x0842 },
    { 53, 1, ELEMENT_ID_AKASHA,  1, 220, 0x0843 },
    { 54, 1, ELEMENT_ID_VAYU,    1, 216, 0x0844 },
    { 55, 2, ELEMENT_ID_VAYU,    1, 212, 0x0845 },
    { 56, 2, ELEMENT_ID_AGNI,    1, 208, 0x0846 },
    { 57, 3, ELEMENT_ID_AGNI,    1, 204, 0x0847 },
    { 58, 3, ELEMENT_ID_APAS,    1, 200, 0x0848 },
    { 59, 4, ELEMENT_ID_APAS,    1, 196, 0x0849 },
    { 60, 4, ELEMENT_ID_PRITHVI, 1, 192, 0x084A },
    { 61, 5, ELEMENT_ID_PRITHVI, 1, 188, 0x084B },
    { 62, 5, ELEMENT_ID_VAYU,    1, 184, 0x084C },
    { 63, 6, ELEMENT_ID_AGNI,    1, 180, 0x084D },
    { 64, 6, ELEMENT_ID_APAS,    1, 176, 0x084E },
    { 65, 7, ELEMENT_ID_PRITHVI, 1, 172, 0x084F },
    { 66, 7, ELEMENT_ID_AKASHA,  1, 168, 0x0850 },
    { 67, 0, ELEMENT_ID_AKASHA,  1, 164, 0x0851 },
    { 68, 1, ELEMENT_ID_VAYU,    1, 160, 0x0852 },
    { 69, 2, ELEMENT_ID_AGNI,    1, 158, 0x0853 },
    { 70, 3, ELEMENT_ID_APAS,    1, 156, 0x0854 },
    { 71, 4, ELEMENT_ID_PRITHVI, 1, 154, 0x0855 },
    { 72, 5, ELEMENT_ID_AKASHA,  1, 152, 0x0856 },
    { 73, 6, ELEMENT_ID_VAYU,    1, 150, 0x0857 },
    { 74, 7, ELEMENT_ID_AGNI,    1, 148, 0x0858 },
    { 75, 0, ELEMENT_ID_APAS,    1, 147, 0x0859 },
    { 76, 1, ELEMENT_ID_PRITHVI, 1, 146, 0x085A },
    { 77, 2, ELEMENT_ID_AKASHA,  1, 145, 0x085B },
    { 78, 3, ELEMENT_ID_VAYU,    1, 144, 0x085C },
    { 79, 4, ELEMENT_ID_AGNI,    1, 144, 0x085D },
    { 80, 5, ELEMENT_ID_APAS,    1, 144, 0x085E },
    { 81, 6, ELEMENT_ID_PRITHVI, 1, 144, 0x085F },
    { 82, 7, ELEMENT_ID_AKASHA,  1, 144, 0x0860 },
    { 83, 0, ELEMENT_ID_VAYU,    1, 144, 0x0861 },
    { 84, 1, ELEMENT_ID_AGNI,    1, 144, 0x0862 },
    { 85, 2, ELEMENT_ID_APAS,    1, 144, 0x0863 },
    { 86, 3, ELEMENT_ID_PRITHVI, 1, 144, 0x0864 },
    { 87, 4, ELEMENT_ID_AKASHA,  1, 144, 0x0865 },
    { 88, 5, ELEMENT_ID_VAYU,    1, 144, 0x0866 },
    { 89, 6, ELEMENT_ID_AGNI,    1, 144, 0x0867 },
    { 90, 7, ELEMENT_ID_APAS,    1, 144, 0x0868 },
    { 91, 0, ELEMENT_ID_PRITHVI, 1, 144, 0x0869 },
    { 92, 1, ELEMENT_ID_AKASHA,  1, 144, 0x086A },
    { 93, 2, ELEMENT_ID_VAYU,    1, 144, 0x086B },
    { 94, 3, ELEMENT_ID_AGNI,    1, 144, 0x086C },
    { 95, 4, ELEMENT_ID_APAS,    1, 144, 0x086D },
    { 96, 5, ELEMENT_ID_PRITHVI, 1, 144, 0x086E },
    { 97, 6, ELEMENT_ID_AKASHA,  1, 144, 0x086F },
    { 98, 7, ELEMENT_ID_VAYU,    1, 144, 0x0870 },
    { 99, 0, ELEMENT_ID_AGNI,    1, 144, 0x0871 },
};


/* ===================================================================
 * .rodata: ROUTING MASKS — Real 36:64 Derived from Group Structure
 *
 * 3 groups of 33 (Jalal/Kamal/Jamal). First 12 of each = internal.
 * Internal (36): indices 0-11, 33-44, 66-77
 * Projective (64): everything else including hidden name (99)
 * =================================================================== */

const Routing_Mask_128 ASMA_36_INTERNAL_MASK = {
    /* low_64: bits 0-11 (Jalal internal) + bits 33-44 (Kamal internal) */
    (0x0000000000000FFFull |    /* bits 0-11:  Jalal first 12  */
     0x00001FFE00000000ull),    /* bits 33-44: Kamal first 12  */
    /* high_64: bits 66-77 (= high bits 2-13) for Jamal internal */
    0x0000000000003FFCull       /* bits 2-13 of high_64 = indices 66-77 */
};

const Routing_Mask_128 ASMA_64_PROJECTIVE_MASK = {
    /* low_64: complement of internal bits in low word + hidden name bits */
    ~(0x0000000000000FFFull | 0x00001FFE00000000ull),  /* ~internal_low */
    /* high_64: complement of internal bits in high word, masked to 100 bits */
    (~0x0000000000003FFCull & 0x0000000FFFFFFFFFull)   /* ~internal_high, within indices 64-99 */
};


/* ===================================================================
 * .rodata: ELEMENT THROUGHLINE TABLE
 * =================================================================== */

const Element_Throughline M2_ELEMENTS[5] = {
    { 31, 4, 0, CHAKRA_VISHUDDHA    },  /* Akasha  → Quintessence → Vishuddha  */
    { 32, 2, 1, CHAKRA_ANAHATA      },  /* Vayu    → Air          → Anahata    */
    { 33, 0, 2, CHAKRA_MANIPURA     },  /* Agni    → Fire         → Manipura   */
    { 34, 3, 3, CHAKRA_SVADHISTHANA },  /* Apas    → Water        → Svadhisthana */
    { 35, 1, 4, CHAKRA_MULADHARA    },  /* Prithvi → Earth        → Muladhara  */
};


/* ===================================================================
 * .rodata: DET — Discrete Epistemic Transform
 *
 * 72 uint64_t masks. States 0-63 get unique bit.
 * States 64-71 fold back (epogdoon tax: 72-64=8 folded).
 * popcount(OR of all 72) == 64.
 * =================================================================== */

const uint64_t M2_TO_M3_CYMATIC_PROJECTION[72] = {
    /* States 0-7 */
    0x0000000000000001ULL, 0x0000000000000002ULL,
    0x0000000000000004ULL, 0x0000000000000008ULL,
    0x0000000000000010ULL, 0x0000000000000020ULL,
    0x0000000000000040ULL, 0x0000000000000080ULL,
    /* States 8-15 */
    0x0000000000000100ULL, 0x0000000000000200ULL,
    0x0000000000000400ULL, 0x0000000000000800ULL,
    0x0000000000001000ULL, 0x0000000000002000ULL,
    0x0000000000004000ULL, 0x0000000000008000ULL,
    /* States 16-23 */
    0x0000000000010000ULL, 0x0000000000020000ULL,
    0x0000000000040000ULL, 0x0000000000080000ULL,
    0x0000000000100000ULL, 0x0000000000200000ULL,
    0x0000000000400000ULL, 0x0000000000800000ULL,
    /* States 24-31 */
    0x0000000001000000ULL, 0x0000000002000000ULL,
    0x0000000004000000ULL, 0x0000000008000000ULL,
    0x0000000010000000ULL, 0x0000000020000000ULL,
    0x0000000040000000ULL, 0x0000000080000000ULL,
    /* States 32-39 */
    0x0000000100000000ULL, 0x0000000200000000ULL,
    0x0000000400000000ULL, 0x0000000800000000ULL,
    0x0000001000000000ULL, 0x0000002000000000ULL,
    0x0000004000000000ULL, 0x0000008000000000ULL,
    /* States 40-47 */
    0x0000010000000000ULL, 0x0000020000000000ULL,
    0x0000040000000000ULL, 0x0000080000000000ULL,
    0x0000100000000000ULL, 0x0000200000000000ULL,
    0x0000400000000000ULL, 0x0000800000000000ULL,
    /* States 48-55 */
    0x0001000000000000ULL, 0x0002000000000000ULL,
    0x0004000000000000ULL, 0x0008000000000000ULL,
    0x0010000000000000ULL, 0x0020000000000000ULL,
    0x0040000000000000ULL, 0x0080000000000000ULL,
    /* States 56-63 */
    0x0100000000000000ULL, 0x0200000000000000ULL,
    0x0400000000000000ULL, 0x0800000000000000ULL,
    0x1000000000000000ULL, 0x2000000000000000ULL,
    0x4000000000000000ULL, 0x8000000000000000ULL,
    /* States 64-71: epogdoon fold-back (8 folded states) */
    0x0000000000000001ULL,  /* 64 → bit 0  */
    0x0000000000000100ULL,  /* 65 → bit 8  */
    0x0000000000010000ULL,  /* 66 → bit 16 */
    0x0000000001000000ULL,  /* 67 → bit 24 */
    0x0000000100000000ULL,  /* 68 → bit 32 */
    0x0000010000000000ULL,  /* 69 → bit 40 */
    0x0001000000000000ULL,  /* 70 → bit 48 */
    0x0100000000000000ULL,  /* 71 → bit 56 */
};


/* ===================================================================
 * .rodata: CAUSAL RESONANCE MASKS — 36 x uint64_t
 *
 * Each base MEF condition (0-35) has a 36-bit mask of resonances.
 * Bit N set = condition K resonates with condition N.
 * Derived from CAUSAL_RESONANCE relation (294 internal edges).
 * Initial derivation: each condition resonates with same-position
 * across all 6 lenses (6 positions each share cross-lens resonance).
 * =================================================================== */

const uint64_t M2_CAUSAL_RESONANCE_MASKS[36] = {
    /* Pos 0 across all lenses: conditions 0,6,12,18,24,30 */
    0x000000041041041ULL,  /* = bits 0,6,12,18,24,30 */
    0x000000082082082ULL,  /* 1: bits 1,7,13,19,25,31 */
    0x000000104104104ULL,  /* 2: bits 2,8,14,20,26,32 */
    0x000000208208208ULL,  /* 3: bits 3,9,15,21,27,33 */
    0x000000410410410ULL,  /* 4: bits 4,10,16,22,28,34 */
    0x000000820820820ULL,  /* 5: bits 5,11,17,23,29,35 */
    /* Lens 1 positions */
    0x000000041041041ULL, 0x000000082082082ULL,
    0x000000104104104ULL, 0x000000208208208ULL,
    0x000000410410410ULL, 0x000000820820820ULL,
    /* Lens 2 positions */
    0x000000041041041ULL, 0x000000082082082ULL,
    0x000000104104104ULL, 0x000000208208208ULL,
    0x000000410410410ULL, 0x000000820820820ULL,
    /* Lens 3 positions */
    0x000000041041041ULL, 0x000000082082082ULL,
    0x000000104104104ULL, 0x000000208208208ULL,
    0x000000410410410ULL, 0x000000820820820ULL,
    /* Lens 4 positions */
    0x000000041041041ULL, 0x000000082082082ULL,
    0x000000104104104ULL, 0x000000208208208ULL,
    0x000000410410410ULL, 0x000000820820820ULL,
    /* Lens 5 positions */
    0x000000041041041ULL, 0x000000082082082ULL,
    0x000000104104104ULL, 0x000000208208208ULL,
    0x000000410410410ULL, 0x000000820820820ULL,
};


/* ===================================================================
 * _Static_assert block — Compile-time invariant enforcement
 * =================================================================== */

_Static_assert(sizeof(M2_TO_M3_CYMATIC_PROJECTION) == 72 * sizeof(uint64_t),
    "M2_TO_M3_CYMATIC_PROJECTION must have exactly 72 entries");


/* ===================================================================
 * API: m2_init — Allocate and HC-link M2_Root
 * =================================================================== */

M2_Root* m2_init(Coordinate_Arena* arena, Holographic_Coordinate* hc) {
    if (!arena || !hc) return NULL;
    if (hc->ql_position != 2) return NULL;

    M2_Root* root = (M2_Root*)malloc(sizeof(M2_Root));
    if (!root) return NULL;

    memset(root, 0, sizeof(M2_Root));
    HC_LINK(hc, root);
    root->active_cf = cf_get(CF_TRIKA);
    root->active_elem = ELEM_SIG_PACK(ELEMENT_ID_AKASHA, CHAKRA_EARTH, ELEM_PHASE_FUSED);
    root->active_decan = 0;
    root->active_tattva = 0;

    return root;
}


/* ===================================================================
 * API: m2_teardown — Release M2_Root heap state
 * =================================================================== */

void m2_teardown(M2_Root* root) {
    if (!root) return;
    if (root->hc) {
        HC_UNLINK(root->hc);
    }
    free(root);
}


/* ===================================================================
 * API: m2_verify — Boot-time .rodata integrity check
 * =================================================================== */

bool m2_verify(void) {
    /* Planet LUT integrity — canonical mod-10 order */
    if (M2_PLANET_LUT[0].id != PLANET_SUN)     return false;
    if (M2_PLANET_LUT[1].id != PLANET_MOON)    return false;
    if (M2_PLANET_LUT[7].id != PLANET_URANUS)  return false;
    if (M2_PLANET_LUT[9].id != PLANET_PLUTO)   return false;

    /* All Cousto frequencies must be non-zero */
    for (int i = 0; i < 10; i++) {
        if (M2_PLANET_LUT[i].cousto_freq == 0) return false;
    }

    /* Sun is personal; Uranus is transpersonal */
    if (!PLANET_IS_PERSONAL(M2_PLANET_LUT[0].id))      return false;
    if (!PLANET_IS_TRANSPERSONAL(M2_PLANET_LUT[7].id)) return false;

    /* 72-Invariant via union size (also compile-time, but belt-and-braces) */
    if (sizeof(M2_Vibrational_72_Space) != 72) return false;

    /* DET popcount check: OR of all 72 must cover all 64 bits */
    uint64_t det_union = 0;
    for (int i = 0; i < 72; i++) {
        det_union |= M2_TO_M3_CYMATIC_PROJECTION[i];
    }
    if (det_union != 0xFFFFFFFFFFFFFFFFULL) return false;

    /* MEF lens names populated */
    for (int i = 0; i < 12; i++) {
        if (M2_MEF_LENS_NAMES[i] == NULL) return false;
    }

    return true;
}


/* ===================================================================
 * API: m2_cli_dispatch — CLI entry point
 * =================================================================== */

static void m2_print_info(const M2_Root* root) {
    printf("M2 (Parashakti) — The Vibrational Architecture\n");
    printf("  Context Frame: CF_TRIKA (0/1/2)\n");
    printf("  72-Invariant:  sizeof(M2_Vibrational_72_Space) = %zu bytes\n",
           sizeof(M2_Vibrational_72_Space));
    printf("  Active Element: %u (Chakra: %u, Phase: %u)\n",
           ELEM_SIG_GET_ELEMENT(root->active_elem),
           ELEM_SIG_GET_CHAKRA(root->active_elem),
           ELEM_SIG_GET_PHASE(root->active_elem));
    printf("  MEF: 12 lenses x 6 positions = 72 conditions\n");
    printf("  Tattvas: 36 (5/7/24 division)\n");
    printf("  Decans: 72 (4x3x3x2)\n");
    printf("  Planets: 10 (Sun/Earth identity + 8 operators)\n");
    printf("  Chakras: 8 (Earth ground + 7 centers)\n");
    printf("  Shem: 72 (8 choirs x 9 names)\n");
    printf("  Asma: 100 (99 + hidden)\n");
    printf("  Mantras: 100 (50 + 50)\n");
    printf("  DET: 72 -> 64 (epogdoon 9:8)\n");
}

static void m2_print_mef(void) {
    printf("MEF Meta-Logikon — 12 Lenses x 6 Positions:\n\n");
    for (int lens = 0; lens < 12; lens++) {
        printf("  [%2d] %-24s (%s):", lens, M2_MEF_LENS_NAMES[lens],
               lens < 6 ? "base" : "inv ");
        for (int pos = 0; pos < 6; pos++) {
            printf(" %2d", M2_ARCHETYPES.mef_lenses[lens][pos]);
        }
        printf("  -> L%d%s\n", lens % 6, lens >= 6 ? "'" : "");
    }
}

static void m2_print_planets(void) {
    printf("Planetary Harmonics — 10 Bodies:\n\n");
    printf("  %-10s %-5s %-6s %-5s %-7s %-4s %-4s\n",
           "Planet", "Id", "Freq", "DR", "Kepler", "Grp", "Prm");
    printf("  %-10s %-5s %-6s %-5s %-7s %-4s %-4s\n",
           "------", "--", "----", "--", "------", "---", "---");
    const char* names[] = {
        "Sun", "Moon", "Mercury", "Venus", "Mars",
        "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto"
    };
    for (int i = 0; i < 10; i++) {
        const Planet_Operator* p = &M2_PLANET_LUT[i];
        printf("  %-10s %-5d %-6u %-5u %-7u %-4u %-4u%s\n",
               names[i], p->id, p->cousto_freq, p->digital_root,
               p->keplerian_vel, p->group_type, p->prime,
               PLANET_IS_TRANSPERSONAL(p->id) ? " [transpersonal]" :
               PLANET_IS_PERSONAL(p->id) ? " [personal]" : "");
    }
}

static void m2_print_tattvas(void) {
    printf("36 Tattvas (5 Pure / 7 Mixed / 24 Impure):\n\n");
    const char* div_names[] = { "Pure", "Mixed", "Impure" };
    for (int i = 0; i < 36; i++) {
        const Tattva_Entry_Desc* t = &M2_TATTVA_DESC[i];
        printf("  [%2d] %-8s elem=%s\n", i, div_names[t->division],
               t->element_id == 0xFF ? "---" : "yes");
    }
}

static void m2_print_chakras(void) {
    printf("8 Chakras:\n\n");
    const char* names[] = {
        "Earth", "Muladhara", "Svadhisthana", "Manipura",
        "Anahata", "Vishuddha", "Ajna", "Sahasrara"
    };
    for (int i = 0; i < 8; i++) {
        const Chakra_Descriptor* c = &M2_CHAKRA_LUT[i];
        printf("  [%d] %-14s elem=%d tattva=%d\n",
               i, names[i], c->element_id, c->tattva_idx);
    }
}

int m2_cli_dispatch(int argc, char** argv, M2_Root* root) {
    if (argc < 2) {
        m2_print_info(root);
        return 0;
    }

    const char* cmd = argv[1];
    if (strcmp(cmd, "info") == 0)        { m2_print_info(root);    return 0; }
    if (strcmp(cmd, "mef") == 0)         { m2_print_mef();         return 0; }
    if (strcmp(cmd, "planet") == 0)      { m2_print_planets();     return 0; }
    if (strcmp(cmd, "tattva") == 0)      { m2_print_tattvas();     return 0; }
    if (strcmp(cmd, "chakra") == 0)      { m2_print_chakras();     return 0; }

    fprintf(stderr, "m2: unknown subcommand '%s'\n", cmd);
    fprintf(stderr, "Usage: m2 [info|mef|tattva|planet|chakra]\n");
    return 1;
}
