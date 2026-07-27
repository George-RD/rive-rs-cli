# Promo

A Remotion composition that shows what `rive-cli` does and that its output is verified.

**Rive never runs inside Remotion.** Every animated frame in the video is a PNG that `rive-cli render`
produced — the same deterministic renderer the test suite uses — so the video is made of exactly the frames
CI verifies, not a re-interpretation of them. Terminal text in the video is copied verbatim from
`site/verify.txt`, which is itself captured from a real run.

## Building it

```bash
cargo build --release          # from the repository root
bash promo/render-sequences.sh # ~2 min, writes ~140 MB of PNGs
cd promo && npm install        # first time only
npm run render                 # writes promo/out/promo.mp4
```

`npm run studio` opens the Remotion editor against the same composition.

## What is and is not committed

| Path | Committed | Why |
|---|---|---|
| `src/`, `package.json`, `render-sequences.sh` | yes | the composition and how to reproduce its inputs |
| `public/seq/` | no | ~140 MB of regenerable PNG frames |
| `out/` | no | the rendered video |

Because the frames are regenerable and byte-identical for a given `.riv`, re-running the two commands above
reproduces the same video.

## Structure

| Scene | Seconds | Content |
|---|---:|---|
| Problem | 0–4 | an agent cannot see its own output |
| Loop | 4–13 | `new` → `generate` → `validate` → `render --preview`, real terminal text |
| Coverage | 13–19 | the ASCII coverage grid that gives the agent eyes |
| Gallery | 19–49 | the ten showcases, three seconds each |
| Close | 49–53 | the claim and the repository |
