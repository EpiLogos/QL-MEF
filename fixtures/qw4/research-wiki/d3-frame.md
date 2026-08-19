---
type: Research Comparison Frame
title: Transition anomaly field
wiki_profile: okf-wiki/v1
research_profile: research-canvas/wiki/v1
wiki:
  profile: okf-wiki/v1
  object: frame
  ref: research:frame:transition-anomaly
  revision: 5
  provenance:
    - source_ref: research:node:evidence
      source_revision: "8"
  inquiry_ref: research:node:question
  scope_refs:
    - research:space:canvas
  member_refs:
    - research:node:p2
    - research:node:p3
    - research:node:p2-prime
    - research:node:p3-prime
  space_refs:
    - research:space:question-local
  source_refs:
    - research:source:observations
  external_refs: []
  constellations:
    - anchor_ref: research:anchor:transition-whole
      ground_ref: research:ground:transition
      members:
        - ref: research:node:p2
          position: 2
          conjugate: false
        - ref: research:node:p3
          position: 3
          conjugate: false
        - ref: research:node:p2-prime
          position: 2
          conjugate: true
        - ref: research:node:p3-prime
          position: 3
          conjugate: true
  research_extension:
    structural_field:
      operator_ref: ql:structural:2.0.0:field:A:1:D3
      family: A
      pair_index: 1
      degree: D3
      coordinates:
        - {position: 2, face: direct}
        - {position: 3, face: direct}
        - {position: 2, face: conjugate}
        - {position: 3, face: conjugate}
    return:
      from_ref: research:node:p3
      through_anchor_ref: research:anchor:transition-whole
      target_ground_ref: research:ground:transition
      target_position: 0
      ground_kind: own
---
# Transition anomaly field

The Research Canvas treats the four vertices as one A1/D3 structural field, not four independent QL tags.
