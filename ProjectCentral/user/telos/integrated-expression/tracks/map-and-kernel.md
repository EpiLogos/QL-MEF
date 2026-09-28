# Track: QL reads the live map; graph↔C disagreements recorded and ruled

Why: QL's registry and its frozen C reference were stale against the live map,
and the retained C tables disagree with the map in named places. Nothing
downstream is sound until those are reconciled at their owners.

Tickets:
- #255 (ruling: the map is the authority);
- #257 (reference moved to C-Experiments `c7872e96`);
- #260 (registry built from the map);
- #262 (map/m3-5-clock-inscription). Green at 73382e0, but one map write behind: re-run `scripts/refresh-from-bimba-map.sh` before landing.
- C-Experiments #40 (C=8/G=7 and the D0 importer keys).

Work: record each disagreement in #254 §5 in the ledger with both values, then
apply the ruling at its owner with a test. This follows the coin precedent,
`docs/pole`, #111–#112. Neither side is picked silently.

Depends: nothing. Parallel-with: infrastructure.

Done-when: every §5 row is applied or explicitly held, and the C tables that
`m2_engine` reads are regenerated from map literals.
