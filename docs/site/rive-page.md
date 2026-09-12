# Rive-generated Pages interface

The landing page is itself an original-work example. The compiler draws its
headings, navigation labels, buttons, slider, sculpture and status labels into
three `.riv` files. It is not a screenshot, a CSS reconstruction or an official
Rive CLI export. The lab remains the parity route; the showcase remains the
original/production-work catalogue.

## Surface brief

Keep the existing warm paper, dark ink and vermilion workshop palette. The
interactive object, rather than promotional copy, occupies most of the page.
Use the existing licensed Inter subset inside the binary so the interface does
not wait for a remote font. At 320px, action targets remain at least 44px high.
Tablet and desktop are separately authored compositions, not a scaled-down
wide page. The browser chooses the composition at 620px and 980px.

## Boundaries

`site/authoring/page.js` composes ordinary typed AuthoringSpec nodes, motion poses
and statecharts. `primitives.js` only abbreviates the public JSON syntax. The
public `rive-cli authoring compile` command performs all lowering and binary
encoding. Nothing here reimplements the compiler or modifies its API.

The `interface` state machine blends 36 vector elements between Orbit, Weave and
Stack. Native Rive click listeners select the three poses. A parallel motion
region supplies the slow ambient movement. The host assigns the numeric blend
input for dragging and keyboard control, and controls playback through the
existing shared `RivePlayback` seam.

The host does not draw shapes or text. Transparent HTML links, buttons and a
keyboard-operable slider match the authored controls. They supply accessible
names, focus, browser navigation, downloads and pointer capture. Focus outlines,
loading/error messages and the explicit text version are browser-rendered. This
is therefore a Rive-rendered page with a semantic browser host, not a standalone
Rive navigation engine. Layout selection, scrolling and reduced-motion policy
belong to that host too.

The previous HTML page remains at `text.html`; its existing proof and lifecycle
checks still run. The new root page has its own generation and browser contracts.
No-JavaScript or failed WASM/file loads retain ordinary navigation links.

## Rebuild and verify

```sh
cargo build --locked
node site/authoring/build.js
node site/authoring/build.js --check
node --test tests/playwright/rive-page-contract.js
node tests/playwright/rive-page-validation.js
node site/serve.js
```

Set `RIVE_CLI` to an already built compiler or `RIVE_CHROME` to an installed Chrome
executable when necessary. No AI provider, login or official CLI is used.

The generator commits both AuthoringSpec and `.riv` outputs. The manifest carries
source/binary SHA-256 values and runtime names resolved from the compiler source
map. `--check` compiles and validates fresh binaries in a temporary directory and
fails on any source, binary or manifest drift. The Pages workflow only stages
these verified artifacts; it does not compile on a visitor's device.

Browser evidence records its exact source commit, Chrome version, responsive
captures and pixel/interaction observations under `target/rive-page-evidence`.
The separate workflow publishes these as an artifact. Compilation alone is not
runtime evidence. Local browser navigation is blocked in the authoring session;
actual browser acceptance runs on the repository runner.
