# Embedded asset evidence

Legacy byte compatibility and official-runtime behavior are independent proofs.
A structural parse or successful compiler test does not substitute for rendering.

## Legacy byte compatibility

`legacy-digests.json` records 95 successful SceneSpec and lowered AuthoringSpec
fixture outputs, with four skipped unsupported/malformed inputs listed separately.
The oracle was captured before the builder patch in
[run 34693606141](https://github.com/George-RD/rive-rs-cli/actions/runs/34693606141).
Preparation commit `95101556b0a2979b0fb46c2af929c62473d7fc1b` still used the unchanged
compilation/build/encoding path from main `dfdf746e923a4935ab1d78c30425588d95bebcd5`.
Every successful hash matches after the implementation and callback-order fixes.
The public compiler contract checks those hashes on every test run.

Do not regenerate this baseline with the new implementation to hide a mismatch.

## Verified image and font byte controls

[verified-observation.json](verified-observation.json) identifies the tested head,
actual CI checkout, production source tree, test blob, browser, input/runtime
hashes and every retained PNG/Rive artifact digest. It records
[run 34703777838, attempt 2](https://github.com/George-RD/rive-rs-cli/actions/runs/34703777838/attempts/2)
on `8a762224ed0dc95b6a8c658bf9030a3a9b393e93`. The
[runtime artifact](https://github.com/George-RD/rive-rs-cli/actions/runs/34703777838/artifacts/10301696226)
contains the six original captures and compiled Rive files. Its ZIP digest was
checked after download; the whole-frame controls were independently recomputed
and the full frame visually inspected.

The test compiles the same SceneSpec with different supplied image bytes, keeping
all declarations, consumers and image dimensions unchanged. Exactly 57,344 pixels
change to the supplied replacement color; every non-image pixel remains identical.
Removing the image drawable supplies the pixel mask, not the byte-attribution proof.
Withholding font bytes removes exactly the same 2,865 glyph pixels as removing
text, while preserving the image. Repeated full captures and binaries match exactly.

The same full Rust run passed 1,198 tests with zero failures. Its single ignored
test is the existing explicit schema-regeneration utility. All 32 focused asset
contracts pass. Formatting, Clippy, Rust 1.88, browser/runtime evaluation, visual
regression, demo/site and Cairn gates also passed for this head. PR #275 records
verification of the final documentation-only commit separately.

```sh
RIVE_MEMORY_ASSET_EVIDENCE=target/embedded-asset-proof \
  cargo test --locked --test embedded_assets_runtime -- --nocapture
```

Use a working installed Chromium/Chrome, or set `RIVE_CHROME` explicitly. The
ordinary Rust CI job runs this test and retains its artifacts and environment
identity. The test host writes artifacts and launches a browser; compilation
receives only SceneSpec, options and caller-owned memory bytes, not a scene path.
Artifacts have finite retention. The committed tests, input provenance and digests
remain available after artifact expiry; a retained observation is not a fresh run.

## Historical observations and failed attempts

`observation.json` preserves the first real image/font run, 34693878309. Its
image-removal control proved image visibility but did not establish which supplied
image buffer was used. Its image-region count of 51,200 is not the later whole-frame
byte-attribution result. Use `verified-observation.json` for that stronger proof.

An intermediate test assumed y >= 200 was text-only, but the image extends to
row 223. The corrected test uses whole-frame controls rather than moving an
arbitrary boundary or relaxing its assertions. Run 34702984571 passed that stronger
runtime control but failed a separate callback-order contract, so it is not a
full-suite pass.

The first attempt of run 34703777838 timed out launching Chrome in an existing
console-runtime test before reaching the memory-asset test. Attempt 2 passed on
the unchanged head, without skipping tests, changing timeouts or weakening gates.
An earlier preparation attempt also had a Playwright browser launch failure;
neither failed launch is runtime evidence. Successful captures used installed Chrome.
