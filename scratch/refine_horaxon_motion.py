#!/usr/bin/env python3
import copy
import json
from pathlib import Path


AUTHORING_PATH = Path("scratch/horaxon-signal.authoring.json")
OUTPUT_LINE_LENGTH = 54
OUTPUT_MARKER_X = 58


def literal(value):
    return {"kind": "literal", "value": value, "unit": "scalar"}


def cubic(id, x1, y1, x2, y2):
    return {
        "kind": "cubic",
        "id": id,
        "x1": literal(x1),
        "y1": literal(y1),
        "x2": literal(x2),
        "y2": literal(y2),
    }


def keyframe(frame, pose, easing=None):
    item = {"frame": literal(frame), "pose": pose}
    if easing:
        item["easing"] = easing
    return item


def node_by_id(nodes, id):
    for node in nodes:
        if node.get("id") == id:
            return node
    raise RuntimeError(f"missing visual node: {id}")


def target_by_id(pose, id):
    for target in pose["targets"]:
        if target.get("target") == id:
            return target
    raise RuntimeError(f"pose {pose['id']} is missing target: {id}")


def set_px(expression, value):
    expression["value"] = value
    expression["unit"] = "px"


document = json.loads(AUTHORING_PATH.read_text())
motion = document["motion"]

# Iteration 6 pulls the action payoff upward. The output route remains part of
# the Rive geometry, while the larger accessible HTML conclusion sits just below
# it in the website. Keeping this geometry change in the generated proof lets the
# official-runtime screenshots verify the real endpoint rather than CSS alone.
output_route = node_by_id(document["visual"]["nodes"], "output-route")
children = {child["id"]: child for child in output_route["children"]}
set_px(children["output-line"]["width"], OUTPUT_LINE_LENGTH)
set_px(children["output-line"]["transform"]["x"], OUTPUT_LINE_LENGTH / 2)
set_px(children["output-marker"]["transform"]["x"], OUTPUT_MARKER_X)
set_px(children["output-check-a"]["transform"]["x"], OUTPUT_MARKER_X - 4)
set_px(children["output-check-b"]["transform"]["x"], OUTPUT_MARKER_X + 4)

# Strong ease-out is useful when an object is meant to settle. It was a poor fit
# for travel split across several intermediate poses because each segment slowed
# almost to zero before the next segment restarted. Keep settle for arrivals,
# use one ease-in-out segment for continuous travel, and one uninterrupted fade
# for the loop reset.
motion["easings"] = [
    easing
    for easing in motion.get("easings", [])
    if easing.get("id") not in {"travel", "fade"}
]
motion["easings"].extend(
    [
        cubic("travel", 0.42, 0.0, 0.58, 1.0),
        cubic("fade", 0.40, 0.0, 0.60, 1.0),
    ]
)

poses = {pose["id"]: pose for pose in motion["poses"]}
decision = poses["decision"]
gate_y = target_by_id(decision, "horaxon-gate")["transform"]["y"]["value"]
endpoint_y = gate_y + OUTPUT_MARKER_X

# The existing decision pose has the final route geometry. Clone it with the
# outbound token still visible so Horaxon -> action remains one continuous travel
# segment; then hide the token in decision without changing its settled position.
output_arrive = copy.deepcopy(decision)
output_arrive["id"] = "output-arrive"
arrive_token = target_by_id(output_arrive, "output-token")
arrive_token["opacity"] = literal(1)
set_px(arrive_token["transform"]["y"], endpoint_y)
set_px(target_by_id(decision, "output-token")["transform"]["y"], endpoint_y)

remove = {"flow-mid", "output-start", "output-flow", "dissolve", "output-arrive"}
filtered = [pose for pose in motion["poses"] if pose["id"] not in remove]
insert_at = next(i for i, pose in enumerate(filtered) if pose["id"] == "decision")
filtered.insert(insert_at, output_arrive)
motion["poses"] = filtered

# One moving segment in, one moving segment out. Duplicate decision poses create
# a deliberate reading hold without adding a motion waypoint. The final fade is
# one segment directly to the exact empty loop boundary.
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
    (210, "flow-start", "travel"),
    (248, "flow-arrive", "settle"),
    (260, "absorbed", "travel"),
    (310, "output-arrive", "settle"),
    (322, "decision", "settle"),
    (420, "decision", "fade"),
    (480, "empty", None),
]

track = motion["tracks"][0]
track["duration_frames"] = literal(480)
track["loop_type"] = "loop"
track["keyframes"] = [keyframe(*frame) for frame in frames]

AUTHORING_PATH.write_text(json.dumps(document, indent=2) + "\n")
print(AUTHORING_PATH)
