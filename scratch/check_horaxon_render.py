#!/usr/bin/env python3
import json
from pathlib import Path

from PIL import Image


RENDER_DIR = Path("scratch/horaxon-render")
COLORS = {
    "background": (15, 23, 32),
    "text": (241, 237, 229),
    "gold": (200, 162, 75),
    "muted": (208, 216, 222),
    "gate_core": (17, 27, 36),
}


def image(frame):
    return Image.open(RENDER_DIR / f"frame_{frame:05d}.png").convert("RGB")


def count_exact(img, box, rgb):
    return sum(1 for pixel in img.crop(box).getdata() if pixel == rgb)


def background_energy(img, box):
    background = COLORS["background"]
    return sum(
        sum(abs(channel - background[index]) for index, channel in enumerate(pixel))
        for pixel in img.crop(box).getdata()
    )


def neutral_centroid_y(img, box):
    left, top, right, bottom = box
    points = []
    for y in range(top, bottom):
        for x in range(left, right):
            red, green, blue = img.getpixel((x, y))
            # The moving token has a parchment core. Restrict to roughly neutral,
            # brighter pixels so gold routes and the dark field do not dominate.
            if min(red, green, blue) >= 55 and max(red, green, blue) - min(red, green, blue) <= 38:
                weight = red + green + blue
                points.append((y, weight))
    if not points:
        raise AssertionError(f"no neutral token pixels found in {box}")
    total = sum(weight for _, weight in points)
    return sum(y * weight for y, weight in points) / total


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

# The Horaxon gate must be a ring/dark centre with an H, not a solid gold disc.
connected = image(195)
checks["gate_center"] = connected.getpixel((240, 208))
checks["gate_core_above_h"] = connected.getpixel((240, 190))
assert checks["gate_center"] == COLORS["gold"], checks
assert checks["gate_core_above_h"] == COLORS["gate_core"], checks

# Inbound travel has no intermediate motion keyframe. Sample the vertical email
# token between start and arrival and require it to keep moving toward H.
inbound_box = (235, 100, 246, 196)
checks["inbound_token_y"] = [
    neutral_centroid_y(image(frame), inbound_box) for frame in (225, 229, 233)
]
assert checks["inbound_token_y"][0] < checks["inbound_token_y"][1] < checks["inbound_token_y"][2], checks

# Horaxon -> action is one travel segment. Iteration 6 shortens that route so the
# payoff sits in the lower-middle rather than near the bottom edge. Samples that
# used to straddle separate eased waypoints must still move monotonically.
outbound_box = (234, 220, 247, 278)
checks["outbound_token_y"] = [
    neutral_centroid_y(image(frame), outbound_box) for frame in (285, 295, 305)
]
assert checks["outbound_token_y"][0] < checks["outbound_token_y"][1] < checks["outbound_token_y"][2], checks

# The lifted output marker must finish before the HTML conclusion begins. Keep a
# clean central gap below the Rive endpoint for the larger payoff text.
decision = image(322)
checks["gold_pixels_below_lifted_endpoint"] = count_exact(decision, (220, 285, 260, 302), COLORS["gold"])
assert checks["gold_pixels_below_lifted_endpoint"] == 0, checks

# Reset is one uninterrupted fade from the held decision directly to the empty
# loop boundary. Its visual energy must decrease through the fade rather than
# pausing at an intermediate dissolve pose.
fade_box = (185, 165, 295, 292)
checks["fade_energy"] = [background_energy(image(frame), fade_box) for frame in (420, 450, 480)]
assert checks["fade_energy"][0] > checks["fade_energy"][1] > checks["fade_energy"][2], checks
assert checks["fade_energy"][2] == 0, checks

# The Rive track owns its own reset. The runtime loop boundary must therefore be
# visually identical rather than relying on JavaScript to hide a jump.
checks["loop_boundary_identical"] = list(image(0).getdata()) == list(image(480).getdata())
assert checks["loop_boundary_identical"], checks

(RENDER_DIR / "visual_checks.json").write_text(json.dumps(checks, indent=2) + "\n")
print(json.dumps(checks, indent=2))
