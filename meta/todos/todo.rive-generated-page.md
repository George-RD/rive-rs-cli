---
node: rive-cli.delivery.site
status: done
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
- Resizing preserves inputs and play intent; reduced motion, bfcache policy and
  failed runtime loads retain usable navigation.
- Showcase and Verification Lab retain their own correctness routes. The previous
  landing remains the explicit text alternative.
- Inspect screenshots and exact-head browser evidence before merge.

## Evidence

All three sources compile and validate with byte-identical regeneration, both
locally and with the reconciled compiler on the repository runner. Eighteen
local authoring, playback, scheduling and lifecycle contracts pass. Resize
regressions failed before the fix at both 30 and 60 fps.

Run 34705406198 passed generation, contracts and complete Chromium acceptance for
head 26b03f8e11103f76bd9df114bd130fb6f5f12fba with main 478449c. Its actual PR
checkout is 0675a0174a396327bcbeb62a4ffff25d4883352e; Chrome is 152.0.7977.82.
Artifact 10301078104 records passed=true and zero normal-load console errors.
Desktop (1280x1040), tablet (780x1144), phone (390x844) and DPR-2 touch captures
were inspected. Native Rive clicks were exercised with the HTML controls
removed. Keyboard and pointer blending reached 0.75 while the paused frame
remained zero on every layout. Resizes across 320/619/620/979/980/1440 retained
selection and play intent; replay retained a native-selected shape. Rapid
transport toggles and dynamic reduced-motion policy passed. The bfcache test
exercises page-transition policy synthetically, not browser cache eligibility.

Blocked scene, manifest and vendored WASM loads all show usable fallback links
above the fold. No-JavaScript navigation also passes. The earlier observation
on 4248582 exposed the runtime CDN fallback; it was not a full acceptance pass.
The host now disables that fallback, and the failure-path regression passes.

The permanent read-only workflow records the tested commit, browser, screenshots
and observation on every run. Existing Rust, runtime, site, demo and Cairn checks
remain merge gates. The task metadata uses Cairn's supported done status;
temporary workbench files and its write-capable workflow are absent from the
final tree. This evidence does not claim physical-device or Safari verification.
