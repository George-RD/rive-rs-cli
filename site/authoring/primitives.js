const value = (number, unit = 'px') => ({ kind: 'literal', value: Number(number.toFixed(5)), unit });
const scalar = number => value(number, 'scalar');
const transform = (x = 0, y = 0, rotation = 0, scale = 1) => ({ x: value(x), y: value(y), rotation: value(rotation, 'degrees'), scale_x: scalar(scale), scale_y: scalar(scale) });
const group = (id, children, x = 0, y = 0) => ({ kind: 'group', id, stacking: 'back_to_front', transform: transform(x, y), children });
function rectangle(id, x, y, width, height, fill, radius = 0, stroke = null) {
  const node = { kind: 'rectangle', id, width: value(width), height: value(height), fill, transform: transform(x + width / 2, y + height / 2) };
  if (radius) node.corner_radius = value(radius);
  if (stroke) node.stroke = { paint: stroke, width: value(1) };
  return node;
}
const ellipse = (id, x, y, size, fill) => ({ kind: 'ellipse', id, width: value(size), height: value(size), fill, transform: transform(x, y) });
function text(id, content, x, y, size, fill, width = 600) {
  return { kind: 'text', id, text: content, font: 'inter', font_size: value(size), fill, width: value(width), origin_x: scalar(0), origin_y: scalar(0), transform: transform(x, y) };
}
function fixedTrack(id, targets) {
  return {
    pose: { id: `${id}-pose`, targets },
    track: { id, fps: 60, duration_frames: scalar(1), loop_type: 'oneshot', keyframes: [0, 1].map(frame => ({ frame: scalar(frame), pose: `${id}-pose` })) },
  };
}
module.exports = { value, scalar, transform, group, rectangle, ellipse, text, fixedTrack };
