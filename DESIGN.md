<!--
THESIS: Make a generated animation feel like a package with a clear contents map: input, output, and proof. Refuse the usual neon tool landing page.
OWN-WORLD: Warm paper, ink-black type, vermilion marks, blue test stamps, and ruled labels. Panels feel like opened packaging, not nested SaaS cards.
STORY: An AI/tool builder sees the review gap, watches JSON become a `.riv`, checks real frames, then opens the lab or GitHub repo.
FIRST VIEWPORT: A narrow header leads to a split hero: plain problem copy on the left; a live Rive canvas inside a labeled parcel on the right. The primary action is “Open the lab”.
FORM: Ekiben wrapper kiosk, assigned direction seed a9c89410, expressed as a technical shipping label and contents map rather than a literal food box.
-->

# Design system

## Mode

Persuade. The page must explain the product and earn one next action in seconds.

## Visual world

A workshop shipping label: warm paper ground, ink-black type, vermilion action marks, blue verification stamps, thin ruled lines, and small inventory labels. The page should feel made, measured, and ready to send.

## Color roles

- `--paper`: page background, warm but not cream-white.
- `--paper-deep`: quiet section background.
- `--ink`: main text and rules.
- `--ink-soft`: secondary text, tinted toward ink.
- `--vermilion`: primary action and active mark.
- `--blue`: verification proof and links.
- `--green`: passing status only.

Use color in large fields and meaningful marks. Never use gradients or gray text on colored fields.

## Type

Use `DM Sans` for clear human text and `IBM Plex Mono` for code, labels, and measured output. Display type is bold, compact, and never decorative. Body copy stays short, direct, and near a 65ch measure.

## Composition

Use a centered 1160px column. The hero is a two-part parcel: copy and live proof. Sections alternate between quiet paper and ink panels. Rules and labels organize content. Avoid a grid of identical feature cards; use one proof strip, one workflow map, and one command panel.

## Components

- Wordmark: plain `rive-cli` with a vermilion slash mark.
- Button: rectangular label with a small corner cut, not a pill.
- Proof parcel: border, label row, live canvas, measured status.
- Workflow map: three numbered steps connected by one ruled line.
- Code slip: dark ink panel with a blue verification stamp.
- Lab link: blue outlined label with explicit “Verification Lab” text.

## Motion and behavior

The hero canvas is the authored motion. Other motion is short and purposeful: parcel labels reveal on scroll, buttons lift 2px, and no animation is required for comprehension. Respect `prefers-reduced-motion`. Focus rings use vermilion with a visible offset.

## Responsive rules

At 860px, stack hero content and proof. At 640px, keep labels legible, let code scroll horizontally, and stack buttons. Lab comparison canvases remain side by side only above 560px; below that they stack with explicit labels.
