# Native shape presentation reading

`ql shape presentation <exact-shape-ref> --json` exposes the existing QL
shape's addresses with a versioned numerical display convention. It is a
read-only projection of `ql-core`, with no workspace or persistence owner.

The source remains the [completed geometry programme](GEOMETRY-CONSTELLATION-WAYFINDER.md),
[canonical resolution](CANONICAL-CONSTELLATION-RESOLUTION.md), and
`QlShape` / `SixBySixField`. This contract does not add philosophical geometry.
The [constructive Wiki amendment](../L5-TECHNE-WIKI-CONSTELLATION-WAYFINDER.md)
keeps native membership, role interpretation, semantic relations and authored
presentation distinct.

The `ql.shape-presentation/v1` envelope retains `shape_ref`, the owner geometry
version, exact positional or row/column addresses, and `xyz` values. Only these
two embeddings are currently supplied:

| Native shape | Display convention |
| --- | --- |
| `ql:shape:1.0.0:constellation:sixfold` | Six direct positions on a unit ring; zero at the top, increasing positions clockwise in screen coordinates. |
| `ql:shape:1.0.0:6x6:direct-conjugate` | The existing 36 direct-row/conjugate-column addresses on a normalized grid from −1 to +1. |

Orientation, scale and spacing are presentation conventions. They do not assert
semantic distances, directions, relationships, sources or occupied members.
Matrix cells are addresses, not 36 invented constellation members. Unsupported
embeddings and noncanonical aliases fail explicitly.

The local constraint preserves unique exact addresses and their supplied
normalized slots. Core `ShapePresentation::conforms` checks finite coordinates,
duplicate/unknown addresses and slot deviation with a tolerance no greater than
0.001. Partial occupancy, including no occupied positions, is legitimate. A
host applies its own whole-object transform outside this normalized frame.

A consumer binds an actual native role/address to a supplied slot. It must
retain the reading schema, exact shape and embedding identity and the pinned
reading's digest. It must not apply this binding to unassigned members, create
source content for vacant positions, infer edges, or silently overwrite an
authored arrangement. Actual enforcement and explicit release into free layout
belong to the existing Expression presentation owner, not this read API.

Portable consumers may pin the exact CLI JSON output in their existing contract
registry and verify it against the native owner. They must retain source
provenance and distinguish that pin from a live owner read. Tests in
`shape_presentation.rs` and `shape_command.rs` exercise actual core geometry,
local constraint checks and CLI dispatch; they are not native application or
human-use evidence.
