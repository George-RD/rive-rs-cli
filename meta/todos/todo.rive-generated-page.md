---
node: rive-cli.delivery.site
status: in-progress
created: 2026-09-12
---

# Rive-generated Pages landing page

User-requested public-proof surface, implemented in PR #276. It uses the public
AuthoringSpec compiler without changing its API. The merge incorporates main
478449c11fc4ec623d8764f17178f71eb502fd09, including #257 and #258. The updated
compiler roadmap is preserved; this site work does not reorder that frontier.

## Acceptance

- The visible landing interface is generated with the public AuthoringSpec CLI,
  including typography, navigation labels, controls and motion.
- Rebuilding all three responsive compositions is byte-for-byte deterministic.
- Native Rive preset listeners and host numeric blending drive visible pixels.
- Pointer, touch and keyboard controls work while motion is paused.
- Resizing preserves inputs and play intent; reduced motion, bfcache and failed
  runtime loads retain usable navigation.
- Showcase and Verification Lab retain their own correctness routes. The previous
  landing remains the explicit text alternative.
- Inspect screenshots and exact-head browser evidence before merge.

## Evidence

All three sources compile and validate locally with byte-identical regeneration.
Eighteen authoring, playback, scheduling and lifecycle contracts pass. Resize
regressions failed before the fix at both 30 and 60 fps.

Chromium 152 observation on 4248582f83fab678ffd10f70efa502ad7c528ead verified
rendered desktop/tablet/phone output, native Rive listeners without the HTML
controls, drag and keyboard blending, frozen reduced-motion pixels, resize and
replay retention, rapid transport toggles and touch at DPR 2. Screenshots were
inspected. Its final failed-WASM case exposed the runtime's CDN fallback; the
host now disables that fallback. This earlier run is not a full acceptance pass.

The permanent read-only Rive-generated page workflow records the exact tested
commit, browser version, screenshots and final observation. Final acceptance on
the reconciled head remains the merge gate. Temporary workbench files and its
write-capable workflow are absent from the final tree.
