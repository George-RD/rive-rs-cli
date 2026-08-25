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


def r(id, w, h, x=0, y=0, fill="#1e2d39", stroke=None, radius=6, rotation=0, stroke_width=1.4):
    node = {
        "kind": "rectangle",
        "id": id,
        "width": l(w, "px"),
        "height": l(h, "px"),
        "corner_radius": l(radius, "px"),
        "fill": fill,
    }
    if stroke:
        node["stroke"] = {"paint": stroke, "width": l(stroke_width, "px")}
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


CARD = "#1e2d39"
CARD_RAISED = "#304657"
TEXT = "#f1ede5"
MUTED = "#d0d8de"
EDGE = "#aab5bf"
RULE = "#8795a1"
GOLD = "#c8a24b"


def card(id, w, h, children, radius=11):
    # The current Rive runtime paints the first sibling on top. The cue lists are
    # authored background-to-foreground for readability, so reverse them and put
    # the opaque surface last. Iteration 3 accidentally put the surface first and
    # hid the very iconography intended to identify the source cards.
    return {
        "kind": "group",
        "id": id,
        "children": [
            *reversed(children),
            r(f"{id}-surface", w, h, fill=CARD, stroke=EDGE, radius=radius, stroke_width=1.8),
        ],
    }


# The cues are deliberately large and simple. At phone size the category should
# read before any small interface detail does.
signal_nodes = [
    card(
        "signal-chat",
        110,
        76,
        [
            r("chat-bubble", 74, 42, 3, 1, CARD_RAISED, stroke=RULE, radius=11, stroke_width=1.6),
            r("chat-tail", 13, 9, -24, 20, CARD_RAISED, radius=2, rotation=36),
            e("chat-dot-a", 7, 7, -17, 0, TEXT),
            e("chat-dot-b", 7, 7, 3, 0, TEXT),
            e("chat-dot-c", 7, 7, 23, 0, TEXT),
            e("chat-unread", 11, 11, 39, -25, GOLD),
        ],
    ),
    card(
        "signal-mail",
        110,
        72,
        [
            r("mail-envelope", 68, 42, 0, 1, CARD_RAISED, stroke=EDGE, radius=4, stroke_width=1.8),
            r("mail-flap-left", 39, 2.3, -15, -6, TEXT, radius=0, rotation=31),
            r("mail-flap-right", 39, 2.3, 15, -6, TEXT, radius=0, rotation=-31),
            r("mail-bottom-left", 30, 2.2, -19, 12, MUTED, radius=0, rotation=-27),
            r("mail-bottom-right", 30, 2.2, 19, 12, MUTED, radius=0, rotation=27),
            e("mail-unread", 10, 10, 39, -23, GOLD),
        ],
    ),
    card(
        "signal-data",
        122,
        84,
        [
            r("data-sheet", 94, 58, 0, 4, CARD_RAISED, stroke=EDGE, radius=3, stroke_width=1.6),
            r("data-header", 94, 8, 0, -21, RULE, radius=2),
            r("data-col-a", 1.8, 48, -24, 5, MUTED, radius=0),
            r("data-col-b", 1.8, 48, 15, 5, MUTED, radius=0),
            r("data-row-a", 90, 1.8, 0, -5, MUTED, radius=0),
            r("data-row-b", 90, 1.8, 0, 11, MUTED, radius=0),
            r("data-highlight", 28, 10, 30, 20, GOLD, radius=2),
        ],
    ),
    card(
        "signal-chart",
        108,
        82,
        [
            r("chart-axis", 76, 2.2, 0, 25, RULE, radius=0),
            r("chart-bar-a", 13, 24, -27, 12, MUTED, radius=2),
            r("chart-bar-b", 13, 39, -8, 5, MUTED, radius=2),
            r("chart-bar-c", 13, 57, 11, -4, GOLD, radius=2),
            r("chart-bar-d", 13, 33, 30, 8, TEXT, radius=2),
        ],
    ),
    card(
        "signal-doc",
        86,
        106,
        [
            r("doc-page", 58, 78, 0, 2, CARD_RAISED, stroke=EDGE, radius=3, stroke_width=1.8),
            r("doc-fold", 17, 17, 20, -29, CARD, stroke=EDGE, radius=1, rotation=45, stroke_width=1.6),
            r("doc-title", 31, 6, -7, -23, TEXT, radius=2),
            r("doc-line-a", 39, 5, -4, -6, MUTED, radius=2),
            r("doc-line-b", 43, 5, -2, 9, MUTED, radius=2),
            r("doc-line-c", 31, 5, -8, 24, MUTED, radius=2),
            r("doc-mark", 6, 18, -23, -25, GOLD, radius=1),
        ],
    ),
    card(
        "signal-alert",
        92,
        68,
        [
            r("alert-diamond", 37, 37, -20, 0, CARD_RAISED, stroke=GOLD, radius=5, rotation=45, stroke_width=2),
            r("alert-mark", 4, 16, -20, -4, GOLD, radius=2),
            e("alert-dot", 5, 5, -20, 9, GOLD),
            r("alert-line-a", 32, 6, 22, -10, TEXT, radius=2),
            r("alert-line-b", 35, 5, 23, 6, MUTED, radius=2),
        ],
    ),
    card(
        "signal-task",
        104,
        72,
        [
            r("task-calendar", 72, 48, 0, 2, CARD_RAISED, stroke=EDGE, radius=5, stroke_width=1.8),
            r("task-header", 72, 9, 0, -17, RULE, radius=4),
            r("task-cell-a", 15, 13, -22, 5, CARD, stroke=MUTED, radius=2),
            r("task-cell-b", 15, 13, 0, 5, CARD, stroke=MUTED, radius=2),
            r("task-cell-c", 15, 13, 22, 5, CARD, stroke=MUTED, radius=2),
            r("task-check-a", 8, 2.5, 18, 6, GOLD, radius=1, rotation=42),
            r("task-check-b", 12, 2.5, 25, 1, GOLD, radius=1, rotation=-48),
            e("task-ring-a", 5, 5, -23, -26, GOLD),
            e("task-ring-b", 5, 5, 23, -26, GOLD),
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
    "signal-chat": (-75, 115, -7),
    "signal-mail": (555, 110, 8),
    "signal-data": (-90, 315, 5),
    "signal-chart": (560, 300, -7),
    "signal-doc": (240, -90, 8),
    "signal-alert": (-70, 215, -9),
    "signal-task": (555, 210, 8),
}

PILE = {
    "signal-chat": (154, 124, -5),
    "signal-mail": (313, 120, 5),
    "signal-data": (179, 251, 3),
    "signal-chart": (307, 250, -4),
    "signal-doc": (241, 168, 7),
    "signal-alert": (171, 210, -7),
    "signal-task": (309, 199, 5),
}

OVERLOAD = {
    "signal-chat": (196, 163, -10, 1.03),
    "signal-mail": (282, 158, 8, 1.03),
    "signal-data": (214, 232, -5, 1.01),
    "signal-chart": (286, 234, 7, 1.01),
    "signal-doc": (243, 188, -8, 1.02),
    "signal-alert": (188, 215, 10, 1.02),
    "signal-task": (302, 202, -9, 1.02),
}

# Connected arrangement deliberately surrounds Horaxon instead of creating a
# left-to-right flowchart. It should read as one system gathering many sources.
CONNECTED = {
    "signal-chat": (94, 72, -2, 0.78),
    "signal-mail": (240, 66, 0, 0.76),
    "signal-data": (76, 193, 0, 0.66),
    "signal-chart": (404, 184, 0, 0.72),
    "signal-doc": (78, 316, 0, 0.60),
    "signal-alert": (239, 341, 0, 0.76),
    "signal-task": (401, 306, 0, 0.70),
}

GATE = (240, 208)


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
    start = CONNECTED[signal_id][:2]
    route_id = f"route-{signal_id.removeprefix('signal-')}"
    length, x, y, rotation = route_geometry(start, GATE)
    ROUTES[route_id] = (x, y, rotation)
    route_nodes.append(
        {
            "kind": "group",
            "id": route_id,
            "children": [r(f"{route_id}-line", length, 1.8, fill=GOLD, radius=0)],
        }
    )


def token(id):
    # Foreground core first because first siblings paint on top.
    return {
        "kind": "group",
        "id": id,
        "children": [
            e(f"{id}-core", 4, 4, fill=TEXT),
            e(f"{id}-outer", 10, 10, fill=GOLD),
        ],
    }


TOKEN_IDS = {signal_id: f"token-{signal_id.removeprefix('signal-')}" for signal_id in SIGNALS}
token_nodes = [token(token_id) for token_id in TOKEN_IDS.values()]

# Same paint-order rule as the cards: H foreground first, ring background last.
gate_node = {
    "kind": "group",
    "id": "horaxon-gate",
    "children": [
        r("gate-h-mid", 21, 5, 0, 0, GOLD, radius=2),
        r("gate-h-right", 5, 28, 10, 0, GOLD, radius=2),
        r("gate-h-left", 5, 28, -10, 0, GOLD, radius=2),
        e("gate-core", 48, 48, fill="#111b24"),
        e("gate-inner", 62, 62, fill=CARD),
        e("gate-outer", 74, 74, fill=GOLD),
    ],
}

# Shorten the output route so its marker ends clearly above the HTML action copy.
OUTPUT_LENGTH = 90
OUTPUT_MARKER_X = 94

output_node = {
    "kind": "group",
    "id": "output-route",
    "children": [
        r("output-check-b", 13, 3, 98, -4, GOLD, radius=1, rotation=-48),
        r("output-check-a", 8, 3, 90, 1, GOLD, radius=1, rotation=42),
        r("output-marker", 24, 24, OUTPUT_MARKER_X, 0, CARD_RAISED, stroke=GOLD, radius=5, rotation=45, stroke_width=2),
        r("output-line", OUTPUT_LENGTH, 2.5, OUTPUT_LENGTH / 2, 0, GOLD, radius=0),
    ],
}

output_token_node = token("output-token")

# First top-level siblings are retained above later infrastructure in the current
# runtime. Tokens and gate therefore stay readable while routes sit behind them.
nodes = [*token_nodes, output_token_node, gate_node, *signal_nodes, *route_nodes, output_node]


def target(id, x, y, rotation=0, opacity=1, scale=1):
    return {
        "target": id,
        "transform": t(x, y, rotation, scale),
        "opacity": l(opacity, "scalar"),
    }


def signal_targets(layout, opacity=1):
    targets = []
    for signal_id in SIGNALS:
        values = layout[signal_id]
        if len(values) == 3:
            x, y, rotation = values
            scale = 1
        else:
            x, y, rotation, scale = values
        targets.append(target(signal_id, x, y, rotation, opacity, scale))
    return targets


def token_position(signal_id, progress):
    sx, sy = CONNECTED[signal_id][:2]
    return (
        sx + (GATE[0] - sx) * progress,
        sy + (GATE[1] - sy) * progress,
    )


def infrastructure(
    route_opacity=0,
    gate_opacity=0,
    gate_scale=0.84,
    token_opacity=0,
    token_progress=0,
    token_scale=1,
    output_opacity=0,
    output_scale=0.05,
    output_token_opacity=0,
    output_token_progress=0,
):
    targets = [
        target(route_id, x, y, rotation, route_opacity, 1)
        for route_id, (x, y, rotation) in ROUTES.items()
    ]
    targets.append(target("horaxon-gate", GATE[0], GATE[1], 0, gate_opacity, gate_scale))
    for signal_id, token_id in TOKEN_IDS.items():
        x, y = token_position(signal_id, token_progress)
        targets.append(target(token_id, x, y, 0, token_opacity, token_scale))
    # Output route travels down from the gate. Keep the endpoint above the HTML
    # recommendation while the separate token makes selection direction legible.
    targets.append(target("output-route", GATE[0], GATE[1], 90, output_opacity, output_scale))
    output_y = GATE[1] + OUTPUT_MARKER_X * output_token_progress
    targets.append(target("output-token", GATE[0], output_y, 0, output_token_opacity, 1))
    return targets


def arrival_pose(id, visible_count):
    targets = []
    for index, signal_id in enumerate(SIGNALS):
        if index < visible_count:
            x, y, rotation = PILE[signal_id]
            targets.append(target(signal_id, x, y, rotation, 1, 1))
        else:
            x, y, rotation = SPAWN[signal_id]
            targets.append(target(signal_id, x, y, rotation, 0, 0.92))
    targets.extend(infrastructure())
    return {"id": id, "targets": targets}


def overload_pose():
    targets = signal_targets(OVERLOAD)
    targets.extend(infrastructure())
    return {"id": "overload", "targets": targets}


def connected_pose(id, token_opacity=0, token_progress=0, token_scale=1, gate_scale=1):
    targets = signal_targets(CONNECTED, 0.96)
    targets.extend(
        infrastructure(
            route_opacity=0.78,
            gate_opacity=1,
            gate_scale=gate_scale,
            token_opacity=token_opacity,
            token_progress=token_progress,
            token_scale=token_scale,
        )
    )
    return {"id": id, "targets": targets}


def absorbed_pose():
    targets = signal_targets(CONNECTED, 0.58)
    targets.extend(
        infrastructure(
            route_opacity=0.62,
            gate_opacity=1,
            gate_scale=1.10,
            token_opacity=0,
            token_progress=1,
            token_scale=0.45,
        )
    )
    return {"id": "absorbed", "targets": targets}


def output_pose(id, route_scale, output_token_progress, decision=False):
    source_opacity = 0.14 if decision else 0.30
    route_opacity = 0.12 if decision else 0.26
    targets = signal_targets(CONNECTED, source_opacity)
    targets.extend(
        infrastructure(
            route_opacity=route_opacity,
            gate_opacity=1,
            gate_scale=1.0,
            output_opacity=1,
            output_scale=route_scale,
            output_token_opacity=0 if decision else 1,
            output_token_progress=output_token_progress,
        )
    )
    return {"id": id, "targets": targets}


def dissolve_pose():
    targets = signal_targets(CONNECTED, 0)
    targets.extend(
        infrastructure(
            route_opacity=0,
            gate_opacity=0.18,
            gate_scale=0.92,
            output_opacity=0.18,
            output_scale=1,
            output_token_opacity=0,
            output_token_progress=1,
        )
    )
    return {"id": "dissolve", "targets": targets}


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
    connected_pose("connected"),
    connected_pose("flow-start", token_opacity=1, token_progress=0),
    connected_pose("flow-mid", token_opacity=1, token_progress=0.56),
    connected_pose("flow-arrive", token_opacity=1, token_progress=1, token_scale=0.62, gate_scale=1.05),
    absorbed_pose(),
    output_pose("output-start", 0.28, 0.12),
    output_pose("output-flow", 0.70, 0.58),
    output_pose("decision", 1, 1, decision=True),
    dissolve_pose(),
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
    (195, "connected", "settle"),
    (210, "flow-start", "settle"),
    (230, "flow-mid", "settle"),
    (248, "flow-arrive", "settle"),
    (260, "absorbed", "settle"),
    (285, "output-start", "settle"),
    (300, "output-flow", "settle"),
    (310, "decision", "settle"),
    (410, "decision", "settle"),
    (445, "dissolve", "settle"),
    (480, "empty", None),
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
                "duration_frames": l(480, "scalar"),
                "loop_type": "loop",
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
