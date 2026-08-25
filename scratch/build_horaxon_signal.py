#!/usr/bin/env python3
import json
from pathlib import Path


def l(value, unit):
    return {"kind": "literal", "value": value, "unit": unit}


def t(x, y, rotation=0, scale=1):
    return {
        "x": l(x, "px"), "y": l(y, "px"),
        "rotation": l(rotation, "degrees"),
        "scale_x": l(scale, "scalar"), "scale_y": l(scale, "scalar"),
    }


def r(id, w, h, x=0, y=0, fill="#151e27", stroke=None, radius=6):
    node = {
        "kind": "rectangle", "id": id, "width": l(w, "px"), "height": l(h, "px"),
        "corner_radius": l(radius, "px"), "fill": fill,
    }
    if stroke:
        node["stroke"] = {"paint": stroke, "width": l(1, "px")}
    if x or y:
        node["transform"] = t(x, y)
    return node


def e(id, w, h, x=0, y=0, fill="#f1ede5"):
    node = {"kind": "ellipse", "id": id, "width": l(w, "px"), "height": l(h, "px"), "fill": fill}
    if x or y:
        node["transform"] = t(x, y)
    return node


INK = "#151e27"
RAISED = "#1b2631"
TEXT = "#f1ede5"
MUTED = "#8795a1"
RULE = "#52606b"
GOLD = "#c8a24b"
PAPER = "#e9e1d2"
PAPER_INK = "#15191d"
PAPER_DIM = "#5f625f"


def card(id, w, h, children):
    return {"kind": "group", "id": id, "children": [r(f"{id}-surface", w, h, fill=INK, stroke=RULE, radius=10), *children]}


nodes = [
    {"kind": "group", "id": "link-chat", "children": [r("link-chat-line", 118, 1.5, -58, -45, GOLD, radius=0)]},
    {"kind": "group", "id": "link-mail", "children": [r("link-mail-line", 118, 1.5, 58, -45, GOLD, radius=0)]},
    {"kind": "group", "id": "link-data", "children": [r("link-data-line", 112, 1.5, -59, 0, GOLD, radius=0)]},
    {"kind": "group", "id": "link-chart", "children": [r("link-chart-line", 112, 1.5, 59, 0, GOLD, radius=0)]},
    {"kind": "group", "id": "link-doc", "children": [r("link-doc-line", 116, 1.5, 0, 58, GOLD, radius=0)]},
    {"kind": "group", "id": "hub", "children": [e("hub-ring", 34, 34, fill=INK), e("hub-core", 12, 12, fill=GOLD)]},
    card("signal-chat", 98, 64, [
        e("chat-avatar", 16, 16, -32, -13, GOLD),
        r("chat-a", 36, 4, 12, -14, TEXT, radius=2),
        r("chat-b", 50, 4, 5, 0, MUTED, radius=2),
        r("chat-c", 30, 4, -5, 14, MUTED, radius=2),
    ]),
    card("signal-mail", 100, 66, [
        r("mail-icon", 30, 22, -29, -10, RAISED, MUTED, 3),
        r("mail-a", 38, 4, 20, -13, TEXT, radius=2),
        r("mail-b", 44, 4, 16, 1, MUTED, radius=2),
        r("mail-c", 30, 4, 9, 15, MUTED, radius=2),
    ]),
    card("signal-data", 118, 78, [
        r("data-head", 96, 5, 0, -25, MUTED, radius=2),
        r("data-rule-a", 96, 1, 0, -8, RULE, radius=0),
        r("data-rule-b", 96, 1, 0, 9, RULE, radius=0),
        r("data-cell-a", 22, 5, -34, 1, TEXT, radius=2),
        r("data-cell-b", 28, 5, 0, 18, MUTED, radius=2),
        r("data-cell-c", 18, 5, 35, 18, GOLD, radius=2),
    ]),
    card("signal-chart", 94, 76, [
        r("chart-axis", 68, 1, 0, 24, RULE, radius=0),
        r("chart-a", 10, 22, -24, 12, MUTED, radius=2),
        r("chart-b", 10, 36, -7, 5, MUTED, radius=2),
        r("chart-c", 10, 52, 10, -3, GOLD, radius=2),
        r("chart-d", 10, 29, 27, 8, TEXT, radius=2),
    ]),
    card("signal-doc", 82, 98, [
        r("doc-mark", 9, 20, -27, -31, GOLD, radius=1),
        r("doc-title", 30, 5, 8, -31, TEXT, radius=2),
        r("doc-a", 54, 4, -5, -12, MUTED, radius=2),
        r("doc-b", 48, 4, -8, 2, MUTED, radius=2),
        r("doc-c", 56, 4, -4, 16, MUTED, radius=2),
        r("doc-d", 34, 4, -15, 30, MUTED, radius=2),
    ]),
    {"kind": "group", "id": "action-card", "children": [
        r("action-surface", 330, 92, fill=PAPER, stroke=GOLD, radius=8),
        r("action-accent", 5, 58, -143, 0, GOLD, radius=2),
        e("action-dot", 16, 16, -116, -18, GOLD),
        r("action-title", 72, 5, -54, -20, PAPER_INK, radius=2),
        r("action-a", 208, 5, 24, 1, PAPER_DIM, radius=2),
        r("action-b", 154, 5, -3, 17, PAPER_DIM, radius=2),
    ]},
]


def target(id, x, y, rotation=0, opacity=1, scale=1):
    return {"target": id, "transform": t(x, y, rotation, scale), "opacity": l(opacity, "scalar")}


links = ["link-chat", "link-mail", "link-data", "link-chart", "link-doc"]


def pose(id, link_opacity, hub, signals, action):
    targets = [target(name, 240, 205, opacity=link_opacity) for name in links]
    targets.append(target("hub", *hub))
    targets.extend(target(*args) for args in signals)
    targets.append(target("action-card", *action))
    return {"id": id, "targets": targets}


poses = [
    pose("scattered", 0, (240, 205, 0, 0, 0.7), [
        ("signal-chat", 92, 82, -8, 0.95, 1),
        ("signal-mail", 365, 92, 7, 0.95, 1),
        ("signal-data", 112, 305, 4, 0.95, 1),
        ("signal-chart", 366, 300, -7, 0.95, 1),
        ("signal-doc", 245, 110, 10, 0.95, 1),
    ], (240, 322, 0, 0, 0.94)),
    pose("overload", 0, (240, 205, 0, 0.15, 0.8), [
        ("signal-chat", 190, 170, -13, 1, 1),
        ("signal-mail", 280, 165, 11, 1, 1),
        ("signal-data", 220, 225, -5, 1, 1),
        ("signal-chart", 295, 235, 9, 1, 1),
        ("signal-doc", 245, 190, -9, 1, 1),
    ], (240, 322, 0, 0, 0.94)),
    pose("connected", 0.82, (240, 205, 0, 1, 1), [
        ("signal-chat", 98, 91, 0, 0.82, 0.78),
        ("signal-mail", 382, 92, 0, 0.82, 0.78),
        ("signal-data", 90, 211, 0, 0.82, 0.72),
        ("signal-chart", 390, 211, 0, 0.82, 0.76),
        ("signal-doc", 240, 326, 0, 0.82, 0.70),
    ], (240, 325, 0, 0, 0.95)),
    pose("decision", 0.28, (240, 185, 0, 1, 0.9), [
        ("signal-chat", 73, 70, 0, 0.28, 0.62),
        ("signal-mail", 407, 70, 0, 0.28, 0.62),
        ("signal-data", 67, 197, 0, 0.28, 0.58),
        ("signal-chart", 413, 197, 0, 0.28, 0.60),
        ("signal-doc", 240, 350, 0, 0.20, 0.55),
    ], (240, 257, 0, 1, 1)),
]


document = {
    "authoring_format_version": 0,
    "artboard": {"id": "horaxon-signal-stage", "width": {"value": 480, "unit": "px"}, "height": {"value": 420, "unit": "px"}},
    "visual": {"nodes": nodes},
    "motion": {
        "easings": [{"kind": "cubic", "id": "settle", "x1": l(0.23, "scalar"), "y1": l(1, "scalar"), "x2": l(0.32, "scalar"), "y2": l(1, "scalar")}],
        "poses": poses,
        "tracks": [{
            "id": "signal-to-action", "fps": 60, "duration_frames": l(180, "scalar"), "loop_type": "oneshot",
            "keyframes": [
                {"frame": l(0, "scalar"), "pose": "scattered", "easing": "settle"},
                {"frame": l(55, "scalar"), "pose": "overload", "easing": "settle"},
                {"frame": l(118, "scalar"), "pose": "connected", "easing": "settle"},
                {"frame": l(180, "scalar"), "pose": "decision"},
            ],
        }],
    },
    "behavior": {},
}

out = Path("scratch/horaxon-signal.authoring.json")
out.parent.mkdir(parents=True, exist_ok=True)
out.write_text(json.dumps(document, indent=2) + "\n")
print(out)
