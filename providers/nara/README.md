# Nara natal provider

`natal.py` is a supervised JSON stdin/stdout worker using the existing qualified
`providers/sky/kerykeion_snapshot.py` and **Kerykeion 5.7.1 / pyswisseph 2.10.3.2**.
The normal `ql nara calculate` route prepares **uv-managed Python 3.13** with the
same pinned `../sky/requirements.txt` used by the dated-sky provider. It discovers
uv through `QL_NARA_UV`, PATH, then `~/.local/bin/uv`, including when the desktop
has a minimal PATH. No workspace path or previously activated virtual environment
is required. `QL_NARA_PYTHON` remains an explicit diagnostic interpreter override.

The binary carries the provider sources, native table references and requirements
in its existing digest-qualified provider cache (`QL_NARA_PROVIDER_CACHE` overrides
that cache). Runtime preparation uses uv's managed interpreter and package cache;
only missing dependencies may require a download. The astronomical calculation
uses no network or geocoding account. `UV_OFFLINE=1` can enforce cached preparation.
No second symbolic, quaternion, identity or chakral solver is introduced here.

Each resource generation prepares a relocatable environment in a unique staging
directory and publishes it atomically after uv completes the pinned install.
The ready marker retains Python policy and exact requirements; subsequent calls
reuse that environment. Concurrent preparation cannot expose a partial install.
The environment is cache material, not a second configuration or profile store.
The native command launches Python directly after preparation, so its existing
process guard kills and reaps the actual provider. Preparation and calculation
share one 45-second deadline; a cold download exceeding it fails explicitly and
may be retried. `ql nara inspect` requires neither Python nor uv.

Ordinary invocation, from any directory:

```sh
ql nara calculate profile.json --json
```

For direct provider diagnostics in an already qualified environment:

```sh
/path/to/managed/python providers/nara/natal.py < birth-request.json
/path/to/managed/python providers/nara/test_natal.py
```

The caller must supply coordinates and an IANA timezone from its actual entered,
imported or geocoded birthplace route, retaining that route in `place.source_ref`.
The provider validates ranges and civil time; it does not claim to verify that a
place label or timezone geographically matches the supplied coordinates.

## Request (`ql.nara-natal-request/v1`)

All keys shown are required; unknown keys are rejected. `person_ref` and
`source_revision` are nonempty strings supplied by the native profile owner.
The revision is opaque to this worker. Never derive it from a receipt timestamp.

```json
{
  "schema": "ql.nara-natal-request/v1",
  "person_ref": "nara:controlled-one",
  "source_revision": "profile-input-revision-1",
  "backend_policy": "allow-moshier",
  "birth": {
    "date": "1990-06-15",
    "time": "12:30:00",
    "precision": "exact",
    "uncertainty_minutes": null,
    "fold": null,
    "place": {
      "label": "London",
      "latitude_degrees": 51.5074,
      "longitude_degrees": -0.1278,
      "timezone": "Europe/London",
      "source_ref": "manual:birthplace-coordinates"
    }
  }
}
```

- `date`: `YYYY-MM-DD`, `YYYY-MM`, `YYYY` or `null`, within 1800–2399.
  Partial dates remain source; no day or month is imputed into a calculation.
- `precision`: `exact`, `approximate` or `unknown`.
- `time`: local `HH:MM` or `HH:MM:SS`; must be `null` for unknown precision.
- `uncertainty_minutes`: integer 1–1440 for approximate times; otherwise `null`.
  This is a declared symmetric elapsed-time interval around the entered instant,
  not a calculated statistical confidence interval.
- `fold`: `null` normally. An ambiguous local time requires `0` for the earlier
  instant or `1` for the later one; fold on an unambiguous time is rejected.
  Nonexistent civil times are rejected and must be corrected explicitly.
- `backend_policy`: `allow-moshier` or `require-swiss-files`. The actual backend,
  file checksums and calculation flags are carried in the native sky snapshot.
- `place` may be `null` while material is incomplete; the result is unavailable
  with reason `missing-birthplace` (unless the date is also incomplete).
- Geographic coordinates accept finite numbers, latitude −90…90 and longitude
  −180…180. They must not be boolean values. The native chart is withheld outside
  ±66° to prevent Kerykeion's silent polar relocation; real geocentric planetary
  positions remain available. The conservative chart limit is explicit.

## Result (`ql.nara-natal/v1`)

The result always contains:

| Key | Meaning |
| --- | --- |
| `request` | Exact source request, preserved without mutation. |
| `request_ref` | `sha256:` of canonical JSON source request, independent of receipt time. |
| `status` | `available`, `partial` or `unavailable`. |
| `reason` | `null` on complete calculation; otherwise a stable reason code. |
| `epoch_utc` | Resolved ISO UTC instant, or `null` when unavailable. |
| `time_resolution` | Local ISO datetime, timezone, offset seconds, fold, precision, declared uncertainty and UTC interval; or `null`. |
| `sky` | Original `ql.sky-snapshot/v1`, unchanged; or `null`. |
| `chart` | Real Kerykeion SVG and calculation details below; or `null`. |
| `provider` | Actual Kerykeion/pyswisseph versions, this adapter's SHA256 and disabled-network declaration. |
| `standing` | `calculated-natal-source; no-personality-or-centre-inference`. |

`available` carries both `sky` and `chart`. `unavailable` with
`incomplete-birth-date`, `missing-birthplace` or `unknown-birth-time` carries neither: in particular,
there is no fabricated noon chart, ascendant or house distribution. `partial`
with `chart-latitude-outside-unadjusted-provider-range` carries real `sky` but no
chart. A provider or source validation error is a separate error response.

`chart` contains `media_type: "image/svg+xml"`, `svg`, `sha256`, `generator`,
`precision`, `conditional`, `uncertainty_minutes`, `uncertainty_policy`,
`display_time_basis`, `zodiac`, `perspective`, `houses_system`,
`houses_system_name`, `house_policy`, `renderer_sha256`, `bodies`, `houses`,
`angles` and `aspects`. The ten `bodies` use the existing native planet ordering;
the twelve `houses`, four `angles` (Ascendant, Medium Coeli, Descendant, Imum
Coeli) and aspects are Kerykeion's actual serialized models. Their numeric
longitude is `abs_pos`. Before rendering, every planet and the Julian epoch
must agree with the retained native sky provider.

The chart is generated through `AstrologicalSubjectFactory`,
`ChartDataFactory.create_natal_chart_data` and `ChartDrawer.generate_svg_string`,
never through a replacement wheel renderer. Its explicit convention is tropical,
apparent geocentric, Whole Sign houses. These chart choices do not establish
canonical Nara identity weighting, elemental/quaternion composition or centre
relationships. Those belong to the existing native composition owner.

The SVG shows UTC to avoid conflicting timezone-resolution heuristics in the
chart library. The caller should show the entered civil time, IANA timezone,
precision and fold from `time_resolution` alongside it. Approximate charts are
labelled **Approximate** in the real SVG and have `conditional: true`; neither
the chart nor its ascendant/houses should be presented as exact. Uncertainty
intervals are retained, not silently averaged into a new identity.

A saved profile should retain this entire result and the source request. The
source-only `request_ref` supports stale-result detection and correction.
Reopening can display the saved SVG without calculating again; recomputation of
the same source produces the same chart SHA256. Sky snapshots have separate
receipt times and references, and must not be folded into profile input identity.
Changing the source revision, date, time, place or precision changes the request
reference; correction must invoke the worker again before replacing natal output.

## Process and verification

At most 64 KiB of UTF-8 JSON is accepted; duplicate keys and nonfinite values are
rejected. Success, including honestly unavailable/partial data, exits `0`.
Invalid source or qualified sky failure exits `2` with
`{"schema":"ql.nara-natal-error/v1","error":"..."}`. Unexpected operational
failure exits `3` with the same error schema and an exception class. No partial
chart is emitted on a process error. The host controls executable selection,
process timeout, profile persistence and result-size limits. A 4 MiB SVG ceiling
is enforced; JSON-escaped output may be larger.

Tests use the real installed provider. They verify numerical sky/chart agreement,
actual SVG parsing, time correction/recomputation and source-stable reopening,
approximate/unknown/partial data, polar location preservation, DST gaps/folds,
invalid-source rejection and subprocess success/error behavior. These tests do
not claim installed UI, native engine consumption or a completed personal
Expression journey; those are joined acceptance obligations of the host owner.
