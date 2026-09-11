# M3 native/source observation parity

K7 / #131. Companion to `M3-SOURCE-PARITY.md` and the existing native engine suite.

`scripts/m3-observation-parity.py` adds the third side of the parity check. It
compares output from the native engine probe against the independent locked
source projection, rather than allowing agreement between C and Rust to stand
for agreement with Bimba. The finite observation contract checks **623 named
coordinate bindings, 128 DNA/RNA outputs and 1,441 successive clock frames**
(two complete 720-degree covers, including the final boundary).

The source-based expectations use nucleotide symbols, codon/pair sequences,
explicit hexagram trigram relations, Tarot source parent/rank properties, source
clock degree/backbone properties and all validated clock edges. In particular,
hexagram coordinates are not manufactured by swapping suffixes in a copied map.

Run after producing the native probe output from the exact checkout:

```sh
bash scripts/test-m3-engine.sh
git rev-parse HEAD > target/m3-native/commit.txt
python3 scripts/m3-observation-parity.py \
  --source-root /path/to/pinned-epi-source \
  --input target/m3-native/cc.jsonl \
  --producer-revision-file target/m3-native/commit.txt \
  --expected-revision "$(git rev-parse HEAD)"
```

The output receipt includes the exact input digest and producer revision. The
producer file must agree with the explicitly expected revision; copying an old
observation without that check cannot count as a new-head receipt. Those metadata
checks do not authenticate an arbitrary caller-provided log: the workflow must
actually execute the native probe at the declared revision before invoking the
reader. The observer does not create a producer receipt itself.

The input may include the engine probe's other finite-domain records. Their bytes
remain covered by the input digest, but this reader explicitly does **not**
re-certify their computation. The existing retained-C/native-C and C/Rust suites
remain responsible for matrices, rotations, quaternion operations, poses and
other numeric tables. This reader does not promote symbolic clock placeholders,
resolve the historical source discrepancies or claim full #131 acceptance.

Fourteen additional validator/mutation tests check missing and duplicate records,
incorrect IDs despite correct coordinate text, the shared upper/lower transpose,
RNA substitution, source clock edges, backbone/degree distinction, cover boundaries,
malformed JSON and producer-revision mismatch. Their positive fixtures are
**synthetic expectations**, not execution evidence; actual evidence comes from
checking the produced native stream.

### Reproduced native regression

The native output from PR #151 head
`c34f736b58644f8e95525f58ae24486f0e232df1`, Actions run `34550805775`, artifact
`10180747239`, was checked with this reader. It reports exactly **56 incorrect
hexagram coordinate bindings**: both C and Rust had attached the native address to
upper/lower rather than the source's lower/upper relation. Address 1 illustrates
the difference: the observed `#3-1-1-2` is not the required `#3-1-2-1`. This is a
new native binding regression to fix, not permission to rewrite historical source
relations. The 128 transcription outputs and 1,441 clock frames in that same
executed stream match the independently checked expectations. No claim is made
that a later native revision is fixed until its output is actually checked.
