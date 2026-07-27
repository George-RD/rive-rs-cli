# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Primary users are AI and tool builders who want agents to create interactive Rive animations from code. They work in code-first workflows and need outputs they can inspect, repeat, and ship.

## Product Purpose

rive-cli turns JSON scene specifications into Rive `.riv` files, then validates and renders those files through the Rive runtime. It exists to give code-driven animation work a real feedback loop instead of asking builders to trust generated output.

Success means a builder can describe a scene, make a binary, check that it loads, and see measurable proof that the result is close to the intended Rive file or scene.

## Positioning

The product's distinct mechanism is a code-first Rive compiler paired with runtime and pixel checks: the same workflow makes the asset and shows whether it works.

## Operating Context

Builders use a terminal, JSON scene files, generated `.riv` binaries, headless Chromium, and CI. They may be working without the Rive editor or without a person watching every frame. The public site must help them decide quickly whether this workflow solves their review and shipping problem.

## Capabilities and Constraints

- `generate` compiles JSON scene specifications into `.riv` files.
- `validate`, `inspect`, and `decompile` inspect generated binaries.
- `render` drives the vendored Rive runtime and captures frames.
- `compare` compares an official file with a reproduction using type deltas and per-frame pixel differences.
- `schema`, `types`, and `describe` expose the authoring model to tools and agents.
- The site is a static GitHub Pages site with a browser-based verification lab route.
- Claims must use committed files and measured results. Do not invent customers, pricing, adoption, or performance benchmarks.

## Brand Commitments

Use the name `rive-cli`. Voice is direct, plain, and useful. Avoid hype and vague AI claims. Explain the workflow in fifth-grade English where possible.

## Evidence on Hand

- Committed Rive parity files and results in `parity/`.
- Existing browser validation in `tests/playwright/site-validation.js`.
- Existing site animations and vendored Rive runtime in `site/`.
- Public discussions about AI-generated code repeatedly describe the hard part as reviewing and validating large, fast outputs. One example is the Reddit discussion at https://www.reddit.com/r/OnlyAICoding/comments/1rvse3k/how_do_you_all_actually_validate_your_vibe_coded/ . This is directional research, not a product metric.
- No customer quotes, adoption numbers, or independent performance studies are supplied.

## Product Principles

1. Show the work, not just the claim.
2. Make every generated file checkable.
3. Keep the path from JSON to runtime short.
4. Use plain words for technical power.
5. Give agents a signal they can act on.

## Accessibility & Inclusion

The site should use semantic HTML, visible keyboard focus, readable contrast, reduced-motion support, and text alternatives for every animation.
