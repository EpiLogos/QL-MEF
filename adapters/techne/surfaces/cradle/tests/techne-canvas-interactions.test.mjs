import test from 'node:test';
import assert from 'node:assert/strict';
import {
  hitTestNode,
  hitTestTopmost,
  lassoSelect,
  moveSelection,
  moveFrame,
  nudgeSelection,
  resetSelectionArrangement,
  alignSelected,
  distributeSelected,
  snapPosition,
  boundingBoxOf,
  viewBounds,
  semanticZoomLevel,
  labelsForZoom,
} from '../src/techne/canvas/interactions.ts';

const nodes = [
  { ref: 'whole', x: 0, y: 0, radius: 0.018 },
  { ref: 'a', x: 0.36, y: 0, radius: 0.009 },
  { ref: 'b', x: 0, y: 0.36, radius: 0.009 },
  { ref: 'c', x: -0.36, y: 0, radius: 0.009 },
];

test('hit testing: centre distance against the hit radius, topmost wins', () => {
  assert.equal(hitTestNode(nodes[1], { x: 0.365, y: 0 }), true);
  assert.equal(hitTestNode(nodes[1], { x: 0.4, y: 0 }), false);
  assert.equal(hitTestTopmost(nodes, { x: 0.36, y: 0 })?.ref, 'a');
  assert.equal(hitTestTopmost(nodes, { x: 10, y: 10 }), null);
});

test('lasso selects by centre inside the polygon; open bands select nothing', () => {
  const box = [
    { x: -0.5, y: -0.1 },
    { x: 0.5, y: -0.1 },
    { x: 0.5, y: 0.1 },
    { x: -0.5, y: 0.1 },
  ];
  assert.deepEqual(lassoSelect(nodes, box), ['whole', 'a', 'c'], 'the horizontal band catches whole, a, c — not b below');
  assert.deepEqual(lassoSelect(nodes, box.slice(0, 2)), [], 'an open polygon selects nothing');
});

test('moveSelection moves exactly the selection from their current positions', () => {
  const moved = moveSelection(nodes, ['a', 'c'], { x: 0.1, y: -0.05 });
  assert.equal(moved.a.x, 0.46);
  assert.equal(moved.a.y, -0.05);
  assert.equal(moved.c.x, -0.26);
  assert.equal(moved.b, undefined, 'unselected refs are untouched');
});

test('moveFrame translates a frame’s members (presentation grouping)', () => {
  const moved = moveFrame(nodes, ['a', 'c'], { x: 0, y: 0.2 });
  assert.equal(moved.a.y, 0.2);
  assert.equal(moved.c.y, 0.2);
});

test('nudge is moveSelection with a step — keyboard and pointer cannot drift', () => {
  const nudged = nudgeSelection(nodes, ['b'], 'up', 0.02);
  assert.equal(nudged.b.y, 0.34);
});

test('resetSelectionArrangement clears exactly the selection’s overrides', () => {
  const overrides = { a: { x: 1, y: 1 }, b: { x: 2, y: 2 } };
  const reset = resetSelectionArrangement(overrides, ['a', 'whole']);
  assert.deepEqual(reset.reset, ['a']);
  assert.deepEqual(reset.overrides, { b: { x: 2, y: 2 } });
});

test('align puts the selection on the selection’s own bounding box; fewer than two do nothing', () => {
  const selection = ['a', 'c'];
  assert.deepEqual(alignSelected(nodes, ['a'], 'left'), {});
  const left = alignSelected(nodes, selection, 'left');
  assert.equal(left.a.x, -0.36);
  assert.equal(left.c.x, -0.36);
  const hcenter = alignSelected(nodes, selection, 'hcenter');
  assert.equal(hcenter.a.x, 0);
  assert.equal(hcenter.c.x, 0);
  assert.equal(hcenter.a.y, 0, 'align does not disturb the other axis');
});

test('distribute spaces three or more evenly; fewer than three do nothing', () => {
  assert.deepEqual(distributeSelected(nodes, ['a', 'b'], 'h'), {});
  const line = [
    { ref: 'l1', x: 0, y: 0, radius: 0.009 },
    { ref: 'l2', x: 0.4, y: 0.3, radius: 0.009 },
    { ref: 'l3', x: 1, y: -0.2, radius: 0.009 },
  ];
  const distributed = distributeSelected(line, ['l1', 'l2', 'l3'], 'h');
  assert.equal(distributed.l2.x, 0.5);
  assert.equal(distributed.l1.x, 0);
  assert.equal(distributed.l3.x, 1);
  assert.equal(distributed.l2.y, 0.3, 'only the distributed axis moves');
});

test('snap attracts to the nearest centre within threshold and names the guide', () => {
  const others = [
    { ref: 'a', x: 0.36, y: 0, radius: 0.009 },
    { ref: 'b', x: 0, y: 0.36, radius: 0.009 },
  ];
  const snapped = snapPosition({ x: 0.355, y: 0.02 }, others, 'moving', 0.01);
  assert.equal(snapped.position.x, 0.36);
  assert.deepEqual(snapped.guides, [{ axis: 'x', at: 0.36 }]);
  const free = snapPosition({ x: 0.3, y: 0.3 }, others, 'moving', 0.01);
  assert.deepEqual(free, { position: { x: 0.3, y: 0.3 }, guides: [] }, 'nothing within threshold → no snap, no guide');
  const origin = snapPosition({ x: 0.008, y: 0.02 }, others, 'moving', 0.01, { snapToOrigin: true });
  assert.equal(origin.position.x, 0, 'the whole’s origin axis attracts when enabled');
});

test('bounding box and saved-view bounds describe the arrangement extent', () => {
  const box = boundingBoxOf(nodes);
  assert.deepEqual(box, { x: -0.36, y: 0, width: 0.72, height: 0.36 });
  const bounds = viewBounds(nodes);
  assert.equal(bounds.x, -0.36 - 0.018);
  assert.equal(bounds.width, 0.72 + 0.036);
});

test('semantic zoom thresholds are deterministic and the label policy has one home', () => {
  assert.equal(semanticZoomLevel(0.5), 'constellation');
  assert.equal(semanticZoomLevel(0.7), 'named');
  assert.equal(semanticZoomLevel(1), 'named');
  assert.equal(semanticZoomLevel(1.6), 'detailed');
  assert.deepEqual(labelsForZoom('constellation'), { nodeLabels: false, relationLabels: false });
  assert.deepEqual(labelsForZoom('named'), { nodeLabels: true, relationLabels: false });
  assert.deepEqual(labelsForZoom('detailed'), { nodeLabels: true, relationLabels: true });
});
