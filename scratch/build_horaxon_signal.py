#!/usr/bin/env python3
import json
import math
from pathlib import Path


def l(value, unit):
    return {"kind": "literal", "value": value, "unit": unit}


def t(x, y, rotation=0, scale=1):
    return {
        "x": l(x, "px"),
        "y": l(y, "px"),
        "rotation": l(rotation, "degrees"),
        "scale_x": l(scale, "scalar"),
        "scale_y": l(scale, "scalar"),
    }


def r(id, w, h, x=0, y=0, fill="#151e27", stroke=None, radius=6, rotation=0):
    node = {
        "kind": "rectangle",
        "id": id,
        "width": l(w, "px"),
        "height": l(h, "px"),
        "corner_radius": l(radius, "px"),
        "fill": fill,
    }
    if stroke:
        node["stroke"] = {"paint": stroke, "width": l(1, "px")}
    if x or y or rotation:
        node["transform"] = t(x, y, rotation)
    return node


def e(id, w, h, x=0, y=0, fill="#f1ede5"):
    node = {
        "kind": "ellipse",
        "id": id,
        "width": l(w, "px"),
        "height": l(h, "px"),
        "fill": fill,
    }
    if x or y:
        node["transform"] = t(x, y)
    return node


INK = "#151e27"
RAISED = "#1b2631"
TEXT = "#f1ede5"
MUTED = "#8795a1"
RULE = "#52606b"
GOLD = "#c8a24b"


def card(id, w, h, children):
    return {
        "kind": "group",
        "id": id,
        "children": [
            r(f"{id}-surface", w, h, fill=INK, stroke=RULE, radius=10),
            *children,
        ],
    }


signal_nodes = [
    card(
        "signal-chat",
        96,
        62,
        [
            e("chat-avatar", 18, 18, -32, -12, GOLD),
            e("chat-unread", 7, 7, -24, -20, TEXT),
            r("chat-bubble", 52, 27, 15, 3, RAISED, radius=7),
            r("chat-line-a", 30, 4, 10, -2, TEXT, radius=2),
            r("chat-line-b", 22, 4, 6, 8, MUTED, radius=2),
            r("chat-tail", 9, 7, -5, 15, RAISED, radius=2, rotation=35),
        ],
    ),
    card(
        "signal-mail",
        100,
        64,
        [
            r("mail-envelope", 34, 24, -29, -4, RAISED, stroke=MUTED, radius=3),
            r("mail-flap-left", 19, 1.5, -36, -6, MUTED, radius=0, rotation=31),
            r("mail-flap-right", 19, 1.5, -22, -6, MUTED, radius=0, rotation=-31),
            r("mail-line-a", 37, 4, 20, -13, TEXT, radius=2),
            r("mail-line-b", 42, 4, 17, 1, MUTED, radius=2),
            r("mail-line-c", 28, 4, 10, 15, MUTED, radius=2),
        ],
    ),
    card(
        "signal-data",
        112,
        76,
        [
            r("data-header", 92, 7, 0, -25, RAISED, radius=2),
            r("data-col-a", 1, 44, -27, 7, RULE, radius=0),
            r("data-col-b", 1, 44, 12, 7, RULE, radius=0),
            r("data-row-a", 92, 1, 0, -7, RULE, radius=0),
            r("data-row-b", 92, 1, 0, 10, RULE, radius=0),
            r("data-cell-a", 20, 5, -38, 2, TEXT, radius=2),
            r("data-cell-b", 22, 5, -7, 19, MUTED, radius=2),
            r("data-cell-c", 20, 5, 35, 19, GOLD, radius=2),
        ],
    ),
    card(
        "signal-chart",
        94,
        74,
        [
            r("chart-axis", 68, 1, 0, 25, RULE, radius=0),
            r("chart-bar-a", 10, 20, -24, 14, MUTED, radius=2),
            r("chart-bar-b", 10, 34, -7, 7, MUTED, radius=2),
            r("chart-bar-c", 10, 49, 10, 0, GOLD, radius=2),
            r("chart-bar-d", 10, 29, 27, 10, TEXT, radius=2),
            e("chart-status", 7, 7, 31, -24, GOLD),
        ],
    ),
    card(
        "signal-doc",
        78,
        96,
        [
            r("doc-mark", 8, 18, -27, -31, GOLD, radius=1),
            r("doc-fold", 13, 13, 25, -36, RAISED, stroke=RULE, radius=2),
            r("doc-title", 29, 5, 6, -31, TEXT, radius=2),
            r("doc-line-a", 52, 4, -4, -11, MUTED, radius=2),
            r("doc-line-b", 46, 4, -7, 3, MUTED, radius=2),
            r("doc-line-c", 54, 4, -3, 17, MUTED, radius=2),
            r("doc-line-d", 31, 4, -14, 31, MUTED, radius=2),
        ],
    ),
    card(
        "signal-alert",
        88,
        60,
        [
            e("alert-ring", 29, 29, -25, 0, GOLD),
            e("alert-inner", 21, 21, -25, 0, INK),
            r("alert-mark", 3, 10, -25, -3, GOLD, radius=2),
            e("alert-dot", 4, 4, -25, 6, GOLD),
            r("alert-line-a", 34, 5, 17, -10, TEXT, radius=2),
            r("alert-line-b", 40, 4, 20, 4, MUTED, radius=2),
            r("alert-line-c", 27, 4, 13, 16, MUTED, radius=2),
        ],
    ),
    card(
        "signal-task",
        94,
        64,
        [
            r("task-header", 72, 7, 0, -21, RAISED, radius=2),
            r("task-cell-a", 16, 14, -26, 3, RAISED, stroke=RULE, radius=3),
            r("task-cell-b", 16, 14, 0, 3, RAISED, stroke=RULE, radius=3),
            r("task-cell-c", 16, 14, 26, 3, RAISED, stroke=RULE, radius=3),
            r("task-check-a", 7, 2, 22, 4, GOLD, radius=1, rotation=42),
            r("task-check-b", 11, 2, 29, 0, GOLD, radius=1, rotation=-48),
            r("task-line", 50, 4, -11, 21, MUTED, radius=2),
        ],
    ),
]

SIGNALS = [
    "signal-chat",
    "signal-mail",
    "signal-data",
    "signal-chart",
    "signal-doc",
    "signal-alert",
    "signal-task",
]

SPAWN = {
    "signal-chat": (-70, 116, -8),
    "signal-mail": (550, 105, 8),
    "signal-data": (-85, 308, 5),
    "signal-chart": (555, 300, -7),
    "signal-doc": (240, -80, 8),
    "signal-alert": (-65, 210, -10),
    "signal-task": (545, 210, 8),
}

PILE = {
    "signal-chat": (165, 132, -6),
    "signal-mail": (277, 124, 6),
    "signal-data": (194, 239, 3),
    "signal-chart": (290, 239, -5),
    "signal-doc": (241, 166, 8),
    "signal-alert": (171, 204, -8),
    "signal-task": (306, 193, 5),
}

OVERLOAD = {
    "signal-chat": (198, 163, -11, 1.04),
    "signal-mail": (278, 157, 9, 1.04),
    "signal-data": (216, 229, -5, 1.02),
    "signal-chart": (286, 232, 8, 1.02),
    "signal-doc": (243, 187, -9, 1.04),
    "signal-alert": (190, 214, 11, 1.03),
    "signal-task": (300, 201, -10, 1.03),
}

INTAKE = {
    "signal-chat": (70, 70, 0, 0.55),
    "signal-mail": (165, 70, 0, 0.54),
    "signal-data": (70, 166, 0, 0.46),
    "signal-chart": (165, 166, 0, 0.52),
    "signal-doc": (70, 270, 0, 0.43),
    "signal-alert": (165, 260, 0, 0.56),
    "signal-task": (118, 354, 0, 0.54),
}

GATE = (292, 205)

ROUTE_STARTS = {
    "signal-chat": (105, 70),
    "signal-mail": (198, 70),
    "signal-data": (107, 166),
    "signal-chart": (198, 166),
    "signal-doc": (101, 270),
    "signal-alert": (198, 260),
    "signal-task": (157, 354),
}
ROUTE_END = (263, 205)


def route_geometry(start, end):
    dx = end[0] - start[0]
    dy = end[1] - start[1]
    return (
        math.hypot(dx, dy),
        (start[0] + end[0]) / 2,
        (start[1] + end[1]) / 2,
        math.degrees(math.atan2(dy, dx)),
    )


ROUTES = {}
route_nodes = []
for signal_id in SIGNALS:
    route_id = f"route-{signal_id.removeprefix('signal-')}"
    length, x, y, rotation = route_geometry(ROUTE_STARTS[signal_id], ROUTE_END)
    ROUTES[route_id] = (x, y, rotation)
    route_nodes.append(
        {
            "kind": "group",
            "id": route_id,
            "children": [
                r(f"{route_id}-line", length, 1.5, fill=GOLD, radius=0),
            ],
        }
    )

gate_node = {
    "kind": "group",
    "id": "horaxon-gate",
    "children": [
        e("gate-outer", 66, 66, fill=GOLD),
        e("gate-inner", 56, 56, fill=INK),
        r("gate-h-left", 4, 25, -9, 0, GOLD, radius=2),
        r("gate-h-right", 4, 25, 9, 0, GOLD, radius=2),
        r("gate-h-mid", 18, 4, 0, 0, GOLD, radius=2),
    ],
}

output_node = {
    "kind": "group",
    "id": "output-route",
    "children": [
        r("output-line", 116, 2, 58, 0, GOLD, radius=0),
        e("output-ring", 22, 22, 118, 0, GOLD),
        e("output-core", 12, 12, 118, 0, INK),
        e("output-point", 5, 5, 118, 0, TEXT),
    ],
}

# First siblings paint above later siblings in the Rive runtime. Keep the gate and
# source surfaces above route geometry, with the output rail behind both.
nodes = [gate_node, *signal_nodes, *route_nodes, output_node]


def target(id, x, y, rotation=0, opacity=1, scale=1):
    return {
        "target": id,
        "transform": t(x, y, rotation, scale),
        "opacity": l(opacity, "scalar"),
    }


def infrastructure(
    route_opacity=0,
    gate_opacity=0,
    gate_scale=0.84,
    output_opacity=0,
    output_scale=0.05,
):
    targets = [
        target(route_id, x, y, rotation, route_opacity, 1)
        for route_id, (x, y, rotation) in ROUTES.items()
    ]
    targets.append(target("horaxon-gate", GATE[0], GATE[1], 0, gate_opacity, gate_scale))
    targets.append(target("output-route", GATE[0], GATE[1], 0, output_opacity, output_scale))
    return targets


def arrival_pose(id, visible_count):
    targets = []
    for index, signal_id in enumerate(SIGNALS):
        if index < visible_count:
            x, y, rotation = PILE[signal_id]
            targets.append(target(signal_id, x, y, rotation, 0.98, 1))
        else:
            x, y, rotation = SPAWN[signal_id]
            targets.append(target(signal_id, x, y, rotation, 0, 0.92))
    targets.extend(infrastructure())
    return {"id": id, "targets": targets}


def overload_pose():
    targets = [
        target(signal_id, *OVERLOAD[signal_id][:3], 1, OVERLOAD[signal_id][3])
        for signal_id in SIGNALS
    ]
    targets.extend(infrastructure())
    return {"id": "overload", "targets": targets}


def intake_pose():
    targets = [
        target(signal_id, *INTAKE[signal_id][:3], 0.78, INTAKE[signal_id][3])
        for signal_id in SIGNALS
    ]
    targets.extend(infrastructure(route_opacity=0.76, gate_opacity=1, gate_scale=1))
    return {"id": "horaxon-intake", "targets": targets}


def distill_pose():
    targets = []
    for signal_id in SIGNALS:
        x, y, _, scale = INTAKE[signal_id]
        tx = x + (GATE[0] - x) * 0.72
        ty = y + (GATE[1] - y) * 0.72
        targets.append(target(signal_id, tx, ty, 0, 0.30, scale * 0.48))
    targets.extend(
        infrastructure(
            route_opacity=0.92,
            gate_opacity=1,
            gate_scale=1.08,
            output_opacity=0.28,
            output_scale=0.32,
        )
    )
    return {"id": "distill", "targets": targets}


def decision_pose():
    targets = [
        target(signal_id, GATE[0] - 10, GATE[1], 0, 0, 0.12)
        for signal_id in SIGNALS
    ]
    targets.extend(
        infrastructure(
            route_opacity=0.20,
            gate_opacity=1,
            gate_scale=1,
            output_opacity=1,
            output_scale=1,
        )
    )
    return {"id": "decision", "targets": targets}


poses = [
    arrival_pose("empty", 0),
    arrival_pose("message-arrives", 1),
    arrival_pose("mail-arrives", 2),
    arrival_pose("data-arrives", 3),
    arrival_pose("report-arrives", 4),
    arrival_pose("document-arrives", 5),
    arrival_pose("alert-arrives", 6),
    arrival_pose("task-arrives", 7),
    overload_pose(),
    intake_pose(),
    distill_pose(),
    decision_pose(),
]

frames = [
    (0, "empty", "settle"),
    (24, "message-arrives", "settle"),
    (52, "mail-arrives", "settle"),
    (78, "data-arrives", "settle"),
    (100, "report-arrives", "settle"),
    (118, "document-arrives", "settle"),
    (133, "alert-arrives", "settle"),
    (145, "task-arrives", "settle"),
    (162, "overload", "settle"),
    (205, "horaxon-intake", "settle"),
    (228, "distill", "settle"),
    (258, "decision", None),
]

keyframes = []
for frame, pose_id, easing in frames:
    keyframe = {"frame": l(frame, "scalar"), "pose": pose_id}
    if easing:
        keyframe["easing"] = easing
    keyframes.append(keyframe)


document = {
    "authoring_format_version": 0,
    "artboard": {
        "id": "horaxon-signal-stage",
        "width": {"value": 480, "unit": "px"},
        "height": {"value": 420, "unit": "px"},
    },
    "visual": {"nodes": nodes},
    "motion": {
        "easings": [
            {
                "kind": "cubic",
                "id": "settle",
                "x1": l(0.23, "scalar"),
                "y1": l(1, "scalar"),
                "x2": l(0.32, "scalar"),
                "y2": l(1, "scalar"),
            }
        ],
        "poses": poses,
        "tracks": [
            {
                "id": "signal-to-action",
                "fps": 60,
                "duration_frames": l(258, "scalar"),
                "loop_type": "oneshot",
                "keyframes": keyframes,
            }
        ],
    },
    "behavior": {},
}

out = Path("scratch/horaxon-signal.authoring.json")
out.parent.mkdir(parents=True, exist_ok=True)
out.write_text(json.dumps(document, indent=2) + "\n")
print(out)
