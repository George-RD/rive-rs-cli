#!/usr/bin/env python3
import json
from pathlib import Path

from PIL import Image


RENDER_DIR = Path("scratch/horaxon-render")
COLORS = {
    "text": (241, 237, 229),
    "gold": (200, 162, 75),
    "muted": (208, 216, 222),
    "gate_core": (17, 27, 36),
}


def image(frame):
    return Image.open(RENDER_DIR / f"frame_{frame:05d}.png").convert("RGB")


def count_exact(img, box, rgb):
    return sum(1 for pixel in img.crop(box).getdata() if pixel == rgb)


checks = {}

# These regions correspond to the first three source cards after they have
# settled. The previous paint-order bug left these opaque rectangles with their
# identifying cue layers hidden underneath the surface.
chat = image(24)
checks["message_text_pixels"] = count_exact(chat, (95, 80, 213, 168), COLORS["text"])
checks["message_gold_pixels"] = count_exact(chat, (95, 80, 213, 168), COLORS["gold"])
assert checks["message_text_pixels"] >= 40, checks
assert checks["message_gold_pixels"] >= 40, checks

mail = image(52)
checks["email_text_pixels"] = count_exact(mail, (255, 80, 371, 166), COLORS["text"])
checks["email_gold_pixels"] = count_exact(mail, (255, 80, 371, 166), COLORS["gold"])
assert checks["email_text_pixels"] >= 30, checks
assert checks["email_gold_pixels"] >= 25, checks

data = image(78)
checks["spreadsheet_muted_pixels"] = count_exact(data, (115, 205, 245, 298), COLORS["muted"])
checks["spreadsheet_gold_pixels"] = count_exact(data, (115, 205, 245, 298), COLORS["gold"])
assert checks["spreadsheet_muted_pixels"] >= 60, checks
assert checks["spreadsheet_gold_pixels"] >= 50, checks

# The Horaxon gate must be a ring/dark centre with an H, not the solid gold disc
# seen in the phone screenshot from iteration 3.
connected = image(195)
checks["gate_center"] = connected.getpixel((240, 208))
checks["gate_core_above_h"] = connected.getpixel((240, 190))
assert checks["gate_center"] == COLORS["gold"], checks
assert checks["gate_core_above_h"] == COLORS["gate_core"], checks

# The output marker now ends around y=319. The HTML recommendation begins lower
# in the web composition; there must be no full-strength output gold continuing
# through the central gap where the label sits.
decision = image(310)
checks["gold_pixels_in_action_gap"] = count_exact(decision, (220, 322, 260, 340), COLORS["gold"])
assert checks["gold_pixels_in_action_gap"] == 0, checks

# The Rive track owns its own reset. The runtime loop boundary must therefore be
# visually identical rather than relying on JavaScript to hide a jump.
checks["loop_boundary_identical"] = list(image(0).getdata()) == list(image(480).getdata())
assert checks["loop_boundary_identical"], checks

(RENDER_DIR / "visual_checks.json").write_text(json.dumps(checks, indent=2) + "\n")
print(json.dumps(checks, indent=2))
