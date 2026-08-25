#!/usr/bin/env python3
import copy
import json
from pathlib import Path


AUTHORING_PATH = Path("scratch/horaxon-signal.authoring.json")


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


document = json.loads(AUTHORING_PATH.read_text())
motion = document["motion"]

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

# The existing decision pose already has the final route geometry and endpoint.
# Clone it with the outbound token still visible so the entire Horaxon -> action
# journey can be one continuous segment; the following decision pose only fades
# the token away after it has arrived.
output_arrive = copy.deepcopy(poses["decision"])
output_arrive["id"] = "output-arrive"
for target in output_arrive["targets"]:
    if target["target"] == "output-token":
        target["opacity"] = literal(1)
        break
else:
    raise RuntimeError("decision pose is missing output-token")

remove = {"flow-mid", "output-start", "output-flow", "dissolve", "output-arrive"}
filtered = [pose for pose in motion["poses"] if pose["id"] not in remove]
insert_at = next(i for i, pose in enumerate(filtered) if pose["id"] == "decision")
filtered.insert(insert_at, output_arrive)
motion["poses"] = filtered

# One moving segment in, one moving segment out. Duplicate decision poses create
# a deliberate reading hold without adding a motion waypoint. The final fade is
# also a single segment directly to the exact empty loop boundary.
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
