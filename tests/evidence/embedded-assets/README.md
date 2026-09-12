# Embedded asset evidence

There are two independent proofs here. Neither a structural parse nor a green
compiler test substitutes for the runtime observation.

## Legacy byte compatibility

`legacy-digests.json` records 95 successful SceneSpec and lowered AuthoringSpec
fixture outputs, with four skipped unsupported/malformed inputs listed separately.
The oracle was captured before the builder patch in
[run 34693606141](https://github.com/George-RD/rive-rs-cli/actions/runs/34693606141).
Preparation commit `95101556b0a2979b0fb46c2af929c62473d7fc1b` still used the
unchanged compilation/build/encoding path from main
`dfdf746e923a4935ab1d78c30425588d95bebcd5`.
The same manifest matched after the implementation was applied. The public
compiler contract also checks those hashes on every test run.

Do not regenerate this baseline with the new implementation to hide a mismatch.

## Official-runtime observation

`observation.json` identifies the exact source tree, browser, runtime/input digests,
frame hashes and measured controls from
[run 34693878309](https://github.com/George-RD/rive-rs-cli/actions/runs/34693878309).
The [runtime artifact](https://github.com/George-RD/rive-rs-cli/actions/runs/34693878309/artifacts/10297638415)
contains the original PNGs and compiled Rive files. Artifacts have finite retention;
the test is the reproducible proof, not an external binary dependency.

The full scene shows the Aurora image and text drawn using supplied Inter font
bytes. Removing the image sprite changes 51,200 pixels in the image region.
Removing text changes 2,865 pixels in the text region. Withholding the embedded
font bytes produces exactly the same pixels as removing text, while retaining the
image. Repeated full captures are identical. The full PNG was visually inspected
as well as compared numerically.

```sh
RIVE_MEMORY_ASSET_EVIDENCE=target/embedded-asset-proof \
  cargo test --locked --test embedded_assets_runtime -- --nocapture
```

Use a working installed Chromium/Chrome, or set `RIVE_CHROME` explicitly. The first
preparation attempt pointed at a Playwright download that exited before exposing
DevTools; it produced no runtime proof. The successful observation used the
runner-installed Google Chrome without changing the compiler or assertions.

The test harness writes artifacts and launches a browser. The compiler itself
receives only SceneSpec, options and caller-owned memory assets. Source lookup does
not read those artifacts or a scene directory. Final exact-head CI, MSRV, Cairn
and review results are recorded in PR #275 rather than inferred from this snapshot.
