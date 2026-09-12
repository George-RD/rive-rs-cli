const assert = require('node:assert/strict');
const test = require('node:test');
const { createPage, layouts } = require('../../site/authoring/page');

test('the whole page is authored in Rive with native preset listeners and embedded text', () => {
  for (const layout of layouts) {
    const { scene, controls } = createPage(layout);
    const nodes = [];
    const visit = (items) => items.forEach(node => { nodes.push(node); visit(node.children || []); });
    visit(scene.visual.nodes);
    assert.ok(nodes.some(node => node.kind === 'text' && node.text === 'rive-cli'));
    assert.ok(nodes.some(node => node.kind === 'text' && node.text === 'Verification Lab'));
    assert.ok(nodes.some(node => node.id === 'tile-35'));
    const chart = scene.behavior.statecharts[0];
    assert.equal(chart.states[0].blend.input, 'morph');
    assert.equal(chart.states[0].blend.stops.length, 3);
    for (const control of controls) {
      assert.ok(control.width > 0 && control.height >= 54, control.id);
      assert.ok(control.x >= 0 && control.x + control.width <= layout.width, control.id);
      assert.ok(control.y >= 0 && control.y + control.height <= layout.height, control.id);
    }
    for (const id of ['orbit', 'weave', 'stack']) {
      assert.ok(chart.listeners.some(listener => listener.target === `${id}-surface` && listener.listener_type === 'click'));
    }
  }
});

const { chooseLayout, pointerValue, describeBlend } = require('../../site/page');
const { runtimeName } = require('../../site/authoring/build');

test('responsive breakpoints and slider coordinates share the authored geometry', () => {
  for (const [width, expected] of [[320, 'mobile'], [619, 'mobile'], [620, 'tablet'], [979, 'tablet'], [980, 'desktop'], [1600, 'desktop']]) {
    assert.equal(chooseLayout(layouts, width).id, expected);
  }
  for (const layout of layouts) {
    const slider = createPage(layout).controls.find(control => control.id === 'morph');
    const bounds = { left: 13, width: layout.width * 0.8 };
    assert.equal(pointerValue(bounds.left + slider.rail.x * 0.8, bounds, slider.rail, layout.width), 0);
    assert.ok(Math.abs(pointerValue(bounds.left + (slider.rail.x + slider.rail.width / 2) * 0.8, bounds, slider.rail, layout.width) - 1) < 1e-9);
    assert.equal(pointerValue(10000, bounds, slider.rail, layout.width), 2);
  }
  assert.equal(describeBlend(0), 'Orbit');
  assert.equal(describeBlend(1.25), 'Weave to Stack, 25 percent');
});

test('runtime identifiers are taken from an exact source-map binding', () => {
  const result = { source_map: { entries: [{ authored_id: 'interface', authored_path: '$.behavior.statecharts[0]', runtime_names: ['runtime-chart'] }] } };
  assert.equal(runtimeName(result, 'interface', '$.behavior.statecharts[0]'), 'runtime-chart');
  assert.throws(() => runtimeName(result, 'interface', '$.wrong'), /missing runtime binding/);
});

test('authored action targets do not overlap and stay usable at the narrowest supported viewport', () => {
  const minima = { mobile: 320, tablet: 620, desktop: 980 };
  for (const layout of layouts) {
    const { controls } = createPage(layout);
    for (const control of controls) assert.ok(control.height * minima[layout.id] / layout.width >= 44);
    for (let i = 0; i < controls.length; i += 1) for (let j = i + 1; j < controls.length; j += 1) {
      const a = controls[i], b = controls[j];
      assert.ok(a.x + a.width <= b.x || b.x + b.width <= a.x || a.y + a.height <= b.y || b.y + b.height <= a.y, `${layout.id}: ${a.id} overlaps ${b.id}`);
    }
  }
});
