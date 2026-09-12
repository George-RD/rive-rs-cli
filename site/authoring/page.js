const { value, scalar, transform, group, rectangle, ellipse, text, fixedTrack } = require('./primitives');

const colors = { paper: '#F4EFDF', ink: '#20231F', soft: '#55584F', rule: '#C8C0AD', red: '#BC3F28', blue: '#2F6383', pale: '#E8E0CB', white: '#FBF8EF', stage: '#242824', stageSoft: '#C0C8B9' };
const layouts = [
  { id: 'mobile', minWidth: 0, width: 390, height: 1310, margin: 22, heading: [22, 182, 50], description: [22, 316], stage: [22, 394, 346, 356], presets: [22, 810, 106], slider: [22, 902, 346], transport: [22, 986, 346], source: [22, 1078, 346, 152], footer: 1266 },
  { id: 'tablet', minWidth: 620, width: 780, height: 1144, margin: 32, heading: [32, 154, 60], description: [390, 174], stage: [32, 332, 716, 426], presets: [32, 806, 116], slider: [434, 808, 314], transport: [32, 902, 364], source: [32, 994, 716, 102], footer: 1112 },
  { id: 'desktop', minWidth: 980, width: 1200, height: 940, margin: 52, heading: [52, 156, 58], description: [52, 310], stage: [424, 154, 724, 544], presets: [52, 450, 100], slider: [52, 546, 320], transport: [52, 642, 320], source: [52, 746, 1096, 116], footer: 891 },
];

function createPage(layout) {
  const { width, height, margin } = layout;
  const mobile = layout.id === 'mobile';
  const nodes = [], controls = [], poses = [], tracks = [], listeners = [];
  const add = (...items) => nodes.push(...items);
  const label = (id, content, x, y, size = 16, fill = colors.ink, width = 600) => text(id, content, x, y, size, fill, width);
  const control = (id, name, x, y, w, action, options = {}) => {
    const entry = { id, label: name, x, y, width: w, height: 56, action, ...options };
    controls.push(entry);
    return entry;
  };
  const line = (id, x, y, w, fill = colors.rule) => rectangle(id, x, y, w, 1, fill);
  const button = (entry, dark = false) => {
    add(rectangle(`${entry.id}-surface`, entry.x, entry.y, entry.width, entry.height, dark ? colors.ink : colors.paper, 4, dark ? colors.ink : colors.rule));
    add(label(`${entry.id}-label`, entry.label, entry.x + 16, entry.y + 18, 16, dark ? colors.paper : colors.ink, entry.width - 26));
  };

  add(rectangle('page-background', 0, 0, width, height, colors.paper));
  add(rectangle('brand-slash', margin, 37, 6, 24, colors.red));
  add(label('brand', 'rive-cli', margin + 18, 30, 27, colors.ink, 180));
  const nav = mobile
    ? [['showcase', 'Showcase', 22, 82, 140, 'showcase.html'], ['lab', 'Verification Lab', 181, 82, 187, 'lab.html'], ['github', 'GitHub', 280, 20, 88, 'https://github.com/George-RD/rive-rs-cli']]
    : [['showcase', 'Showcase', width - 465, 24, 130, 'showcase.html'], ['lab', 'Verification Lab', width - 323, 24, 196, 'lab.html'], ['github', 'GitHub', width - 115, 24, 96, 'https://github.com/George-RD/rive-rs-cli']];
  for (const [id, title, x, y, w, href] of nav) {
    control(id, title, x, y, w, 'link', { href });
    add(label(`${id}-nav`, title, x + 8, y + 18, mobile ? 16 : 15, id === 'lab' ? colors.blue : colors.ink, w - 8));
  }
  add(line('header-rule', margin, mobile ? 148 : 100, width - margin * 2));
  const [hx, hy, hs] = layout.heading;
  add(label('heading-one', 'Made with', hx, hy, hs, colors.ink, 390));
  add(label('heading-two', 'rive-cli.', hx, hy + hs * 1.08, hs, colors.red, 390));
  const [dx, dy] = layout.description;
  add(label('intro-one', 'A whole interface, generated', dx, dy, 18, colors.soft, 350));
  add(label('intro-two', 'with our own Rive compiler.', dx, dy + 28, 18, colors.soft, 350));

  const [sx, sy, sw, sh] = layout.stage;
  add(rectangle('stage-background', sx, sy, sw, sh, colors.stage, 8));
  add(label('stage-label', 'SHAPES / TYPE / INTERACTION', sx + 22, sy + 22, mobile ? 10 : 12, colors.stageSoft, sw - 44));
  add(ellipse('live-dot', sx + sw - 26, sy + 28, 6, '#ACD89A'));
  const sculptureScale = Math.min((sw - 42) / 430, (sh - 92) / 430);
  const tiles = [];
  const palette = ['#DF492F', '#E85839', '#F47250', '#F89472', '#FFB396', '#F89472', '#F47250', '#CF3E28', '#B73522'];
  for (let index = 0; index < 36; index += 1) {
    tiles.push(group(`tile-${index}`, [
      rectangle(`tile-face-${index}`, -54, -14, 108, 28, palette[index % palette.length], 5),
      rectangle(`tile-edge-${index}`, -46, -10, 91, 2, index % 3 === 0 ? '#FFBCA0' : '#F89B7B', 1),
    ]));
  }
  const sculpture = group('sculpture', [group('breathing-shape', tiles)], sx + sw / 2, sy + sh / 2 + 8);
  sculpture.transform.scale_x = scalar(sculptureScale);
  sculpture.transform.scale_y = scalar(sculptureScale);
  add(sculpture);
  for (const [index, name] of ['ORBIT', 'WEAVE', 'STACK'].entries()) {
    add(group(`state-label-${index}`, [label(`state-name-${index}`, name, sx + 22, sy + sh - 43, 15, colors.paper, 160)]));
  }
  add(label('stage-note', 'Drag the slider', sx + sw - (mobile ? 168 : 192), sy + sh - 41, mobile ? 11 : 13, colors.stageSoft, 180));

  const [px, py, pw] = layout.presets;
  add(label('shape-label', 'Shape', px, py - 29, 15, colors.soft, 120));
  ['Orbit', 'Weave', 'Stack'].forEach((name, index) => {
    const entry = control(name.toLowerCase(), name, px + index * (pw + 10), py, pw, 'preset', { value: index });
    button(entry);
    add(group(`selected-${index}`, [rectangle(`selected-face-${index}`, entry.x, py, pw, 56, colors.ink, 4), label(`selected-text-${index}`, name, entry.x + 16, py + 18, 16, colors.paper, pw - 20)]));
    listeners.push({ id: `${entry.id}-click`, target: `${entry.id}-surface`, listener_type: 'click', actions: [{ kind: 'number_change', input: 'morph', value: scalar(index) }] });
  });

  const [lx, ly, lw] = layout.slider;
  add(label('blend-label', 'Blend the shapes', lx, ly, 16, colors.soft, lw));
  const rail = { x: lx + 16, y: ly + 46, width: lw - 32 };
  add(rectangle('blend-rail', rail.x, rail.y, rail.width, 3, colors.rule, 1));
  [0, 1, 2].forEach(index => add(ellipse(`blend-stop-${index}`, rail.x + rail.width * index / 2, rail.y + 1.5, 7, colors.ink)));
  add(group('blend-thumb', [ellipse('blend-thumb-disc', 0, 0, 30, colors.red), ellipse('blend-thumb-core', 0, 0, 7, colors.paper)], rail.x, rail.y + 1.5));
  control('morph', 'Blend between Orbit, Weave and Stack', lx, ly + 21, lw, 'range', { rail, min: 0, max: 2, step: 0.01 });
  const [tx, ty, tw] = layout.transport;
  const transportWidth = (tw - 10) / 2;
  control('playback', 'Pause motion', tx, ty, transportWidth, 'playback');
  add(rectangle('playback-surface', tx, ty, transportWidth, 56, colors.paper, 4, colors.rule));
  add(group('pause-label', [label('pause-text', 'Pause', tx + 16, ty + 18, 16, colors.ink, transportWidth - 20)]));
  add(group('play-label', [label('play-text', 'Play', tx + 16, ty + 18, 16, colors.ink, transportWidth - 20)]));
  button(control('replay', 'Replay', tx + transportWidth + 10, ty, transportWidth, 'replay'));

  const [cx, cy, cw, ch] = layout.source;
  add(line('source-rule', cx, cy, cw));
  add(label('source-kicker', 'THIS PAGE IS THE EXAMPLE', cx, cy + 16, 12, colors.blue, cw));
  const code = mobile ? ['authoring compile', 'mobile.v0.json -o mobile.riv'] : ['rive-cli authoring compile', `${layout.id}.v0.json -o ${layout.id}.riv`];
  add(label('command-one', code[0], cx, cy + 42, mobile ? 14 : 17, colors.ink, cw));
  add(label('command-two', code[1], cx, cy + 67, mobile ? 14 : 17, colors.ink, cw));
  const sourceWidth = mobile ? (cw - 10) / 2 : layout.id === 'tablet' ? 158 : 194;
  const sourceX = mobile ? cx : cx + cw - 2 * sourceWidth - 12;
  const sourceY = mobile ? cy + 95 : cy + 35;
  button(control('source', 'View source', sourceX, sourceY, sourceWidth, 'source'));
  button(control('download', '.riv file', sourceX + sourceWidth + 10, sourceY, sourceWidth, 'download'), true);
  add(label('footer-one', mobile ? 'Rive draws the page. The browser hosts it.' : 'Rive draws the page. The browser hosts it. No editor export.', margin, layout.footer, mobile ? 11 : 12, colors.soft, width - margin * 2));
  add(label('footer-two', 'Independent project. Not the official Rive CLI.', margin, layout.footer + 20, mobile ? 10 : 11, colors.soft, width - margin * 2));

  for (let mode = 0; mode < 3; mode += 1) {
    const targets = [];
    for (let index = 0; index < 36; index += 1) {
      const angle = index / 36 * Math.PI * 2;
      let x, y, rotation;
      if (mode === 0) {
        x = Math.cos(angle) * 126; y = Math.sin(angle) * 126; rotation = index * 10 + 32;
      } else if (mode === 1) {
        x = (index - 17.5) * 9.7; y = Math.sin(index / 35 * Math.PI * 2) * 104; rotation = Math.cos(index / 35 * Math.PI * 2) * 42 + 72;
      } else {
        x = ((index % 6) - 2.5) * 52; y = (Math.floor(index / 6) - 2.5) * 45; rotation = -30;
      }
      targets.push({ target: `sculpture/breathing-shape/tile-${index}`, transform: transform(x, y, rotation, mode === 2 ? 0.64 : 1) });
    }
    for (let selected = 0; selected < 3; selected += 1) {
      targets.push({ target: `selected-${selected}`, opacity: scalar(selected === mode ? 1 : 0) });
      targets.push({ target: `state-label-${selected}`, opacity: scalar(selected === mode ? 1 : 0) });
    }
    targets.push({ target: 'blend-thumb', transform: transform(rail.x + rail.width * mode / 2, rail.y + 1.5) });
    const fixed = fixedTrack(`form-${mode}`, targets);
    poses.push(fixed.pose); tracks.push(fixed.track);
  }
  for (const [id, paused] of [['playing', false], ['paused', true]]) {
    const fixed = fixedTrack(id, [{ target: 'play-label', opacity: scalar(paused ? 1 : 0) }, { target: 'pause-label', opacity: scalar(paused ? 0 : 1) }]);
    poses.push(fixed.pose); tracks.push(fixed.track);
  }
  poses.push({ id: 'breath-left', targets: [{ target: 'sculpture/breathing-shape', transform: transform(0, -5, -7) }] }, { id: 'breath-right', targets: [{ target: 'sculpture/breathing-shape', transform: transform(0, 5, 7) }] });
  tracks.push({ id: 'breathe', fps: 60, duration_frames: scalar(240), loop_type: 'pingpong', keyframes: [{ frame: scalar(0), pose: 'breath-left', easing: 'smooth' }, { frame: scalar(240), pose: 'breath-right', easing: 'smooth' }] });
  const scene = {
    authoring_format_version: 0,
    artboard: { id: 'page', width: { value: width, unit: 'px' }, height: { value: height, unit: 'px' } },
    font_assets: { inter: '../../assets/fonts/Inter-Bold-Subset.ttf' },
    visual: { stacking: 'back_to_front', nodes },
    motion: { easings: [{ id: 'smooth', kind: 'cubic', x1: scalar(0.42), y1: scalar(0), x2: scalar(0.58), y2: scalar(1) }], poses, tracks },
    behavior: { statecharts: [{ id: 'interface', inputs: [{ kind: 'number', id: 'morph', value: scalar(0) }, { kind: 'bool', id: 'paused', value: false }], listeners, initial: 'shape', states: [{ id: 'shape', blend: { input: 'morph', stops: [0, 1, 2].map(mode => ({ motion: `form-${mode}`, value: scalar(mode) })) } }], transitions: [], regions: [
      { id: 'ambient', initial: 'moving', states: [{ id: 'moving', motion: 'breathe' }], transitions: [] },
      { id: 'transport', initial: 'playing-state', states: [{ id: 'playing-state', motion: 'playing' }, { id: 'paused-state', motion: 'paused' }], transitions: [{ id: 'pause', from: 'playing-state', to: 'paused-state', when: { input: 'paused', equals: true } }, { id: 'play', from: 'paused-state', to: 'playing-state', when: { input: 'paused', equals: false } }] },
    ] }] },
  };
  return { scene, controls };
}

module.exports = { createPage, layouts };
