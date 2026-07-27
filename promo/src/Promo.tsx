import React from "react";
import {
  AbsoluteFill,
  Img,
  Sequence,
  interpolate,
  staticFile,
  useCurrentFrame,
} from "remotion";
import { TRANSCRIPT } from "./transcript";

export const FPS = 30;

const INK = "#e8eefc";
const DIM = "#93a3c4";
const CYAN = "#22d3ee";
const LIME = "#b7f34a";
const MONO = "SF Mono, Menlo, Consolas, monospace";
const SANS = "Inter, -apple-system, Segoe UI, sans-serif";

const BACKDROP: React.CSSProperties = {
  background:
    "radial-gradient(1400px 800px at 78% -10%, #182242 0%, transparent 62%), radial-gradient(1000px 640px at 6% 10%, #14243c 0%, transparent 58%), #070a12",
  color: INK,
  fontFamily: SANS,
};

type Scene = { id: string; title: string; note: string; frames: number; step: number };

const SCENES: Scene[] = [
  { id: "wordmark", title: "Wordmark", note: "embedded font", frames: 151, step: 1 },
  { id: "liquid_loader", title: "Liquid Loader", note: "path morphing", frames: 121, step: 1 },
  { id: "textured_scene", title: "Textured Scene", note: "embedded image", frames: 121, step: 2 },
  { id: "control_panel", title: "Control Panel", note: "pointer + blend state", frames: 121, step: 1 },
  { id: "orbital_loader", title: "Orbital Loader", note: "trim paths", frames: 121, step: 1 },
  { id: "pulse_button", title: "Pulse Button", note: "state machine", frames: 91, step: 1 },
  { id: "radial_dashboard", title: "Radial Dashboard", note: "gauge", frames: 121, step: 1 },
  { id: "audio_equaliser", title: "Audio Equaliser", note: "phase offsets", frames: 121, step: 1 },
  { id: "day_night_toggle", title: "Day / Night Toggle", note: "state machine", frames: 91, step: 1 },
  { id: "rocket_launch", title: "Rocket Launch", note: "eased motion", frames: 121, step: 1 },
];

const PROBLEM = 4 * FPS;
const LOOP = 9 * FPS;
const COVERAGE = 6 * FPS;
const PER_SCENE = 3 * FPS;
const CLOSE = 4 * FPS;

export const TOTAL_FRAMES =
  PROBLEM + LOOP + COVERAGE + SCENES.length * PER_SCENE + CLOSE;

const fadeIn = (frame: number, over = 12) =>
  interpolate(frame, [0, over], [0, 1], { extrapolateRight: "clamp" });

const Terminal: React.FC<{ lines: string[]; visible: number }> = ({ lines, visible }) => (
  <pre
    style={{
      fontFamily: MONO,
      fontSize: 26,
      lineHeight: 1.5,
      background: "#060a14",
      border: "1px solid #23304f",
      borderRadius: 18,
      padding: "34px 40px",
      margin: 0,
      color: "#cfe0ff",
      whiteSpace: "pre",
    }}
  >
    {lines.slice(0, visible).map((line, index) => (
      <div key={index} style={{ color: line.startsWith("$") ? LIME : "#cfe0ff" }}>
        {line || " "}
      </div>
    ))}
  </pre>
);

const Problem: React.FC = () => {
  const frame = useCurrentFrame();
  return (
    <AbsoluteFill
      style={{ ...BACKDROP, justifyContent: "center", alignItems: "center", padding: 140 }}
    >
      <div style={{ opacity: fadeIn(frame), textAlign: "center", maxWidth: 1400 }}>
        <p
          style={{
            fontFamily: MONO,
            letterSpacing: "0.3em",
            textTransform: "uppercase",
            color: CYAN,
            fontSize: 24,
            margin: 0,
          }}
        >
          the problem
        </p>
        <h1 style={{ fontSize: 96, lineHeight: 1.05, letterSpacing: "-0.03em", margin: "28px 0 0" }}>
          An agent cannot see
          <br />
          what it just made.
        </h1>
        <p style={{ fontSize: 34, color: DIM, marginTop: 34 }}>
          It can write a scene. It cannot tell whether the scene is blank.
        </p>
      </div>
    </AbsoluteFill>
  );
};

const Loop: React.FC = () => {
  const frame = useCurrentFrame();
  const lines = TRANSCRIPT.slice(0, 8);
  const visible = Math.min(lines.length, Math.floor(frame / 8) + 1);
  return (
    <AbsoluteFill style={{ ...BACKDROP, padding: 120, justifyContent: "center" }}>
      <p
        style={{
          fontFamily: MONO,
          letterSpacing: "0.3em",
          textTransform: "uppercase",
          color: CYAN,
          fontSize: 24,
          margin: "0 0 22px",
        }}
      >
        the loop
      </p>
      <h2 style={{ fontSize: 62, letterSpacing: "-0.02em", margin: "0 0 40px" }}>
        generate &rarr; validate &rarr; render &rarr; measure
      </h2>
      <div style={{ opacity: fadeIn(frame, 8) }}>
        <Terminal lines={lines} visible={visible} />
      </div>
    </AbsoluteFill>
  );
};

const Coverage: React.FC = () => {
  const frame = useCurrentFrame();
  const grid = TRANSCRIPT.filter((line) => /^[.+# ]{20,}$/.test(line)).slice(0, 32);
  const visible = Math.min(grid.length, Math.floor(frame / 3) + 1);
  return (
    <AbsoluteFill style={{ ...BACKDROP, padding: 100, flexDirection: "row", alignItems: "center", gap: 80 }}>
      <div style={{ flex: "0 0 640px" }}>
        <p
          style={{
            fontFamily: MONO,
            letterSpacing: "0.3em",
            textTransform: "uppercase",
            color: CYAN,
            fontSize: 24,
            margin: "0 0 22px",
          }}
        >
          the fix
        </p>
        <h2 style={{ fontSize: 58, letterSpacing: "-0.02em", margin: "0 0 26px" }}>
          Give it eyes it can read.
        </h2>
        <p style={{ fontSize: 30, color: DIM, lineHeight: 1.5 }}>
          Every render reports distinct colours, a dominant-colour ratio, a content bounding box, and this
          coverage map. An agent can act on all four.
        </p>
      </div>
      <pre
        style={{
          fontFamily: MONO,
          fontSize: 22,
          lineHeight: 1.05,
          color: LIME,
          background: "#060a14",
          border: "1px solid #23304f",
          borderRadius: 18,
          padding: 30,
          margin: 0,
          opacity: fadeIn(frame, 8),
        }}
      >
        {grid.slice(0, visible).join("\n")}
      </pre>
    </AbsoluteFill>
  );
};

const SceneCard: React.FC<{ scene: Scene }> = ({ scene }) => {
  const frame = useCurrentFrame();
  const index = Math.min(scene.frames - 1, frame * scene.step);
  const file = `seq/${scene.id}/frame_${String(index).padStart(5, "0")}.png`;
  return (
    <AbsoluteFill
      style={{ ...BACKDROP, flexDirection: "row", alignItems: "center", padding: "0 140px", gap: 100 }}
    >
      <div
        style={{
          width: 720,
          height: 720,
          borderRadius: 30,
          border: "1px solid #23304f",
          overflow: "hidden",
          opacity: fadeIn(frame, 8),
        }}
      >
        <Img src={staticFile(file)} style={{ width: "100%", height: "100%" }} />
      </div>
      <div style={{ opacity: fadeIn(frame, 12) }}>
        <p
          style={{
            fontFamily: MONO,
            letterSpacing: "0.26em",
            textTransform: "uppercase",
            color: LIME,
            fontSize: 24,
            margin: 0,
          }}
        >
          {scene.note}
        </p>
        <h2 style={{ fontSize: 78, letterSpacing: "-0.02em", margin: "18px 0 0" }}>{scene.title}</h2>
        <p style={{ fontFamily: MONO, fontSize: 26, color: DIM, marginTop: 22 }}>
          showcase/{scene.id}.json &rarr; {scene.id}.riv
        </p>
      </div>
    </AbsoluteFill>
  );
};

const Close: React.FC = () => {
  const frame = useCurrentFrame();
  return (
    <AbsoluteFill style={{ ...BACKDROP, justifyContent: "center", alignItems: "center" }}>
      <div style={{ opacity: fadeIn(frame), textAlign: "center" }}>
        <h1 style={{ fontSize: 92, letterSpacing: "-0.03em", margin: 0 }}>
          Rive animations, compiled from JSON.
        </h1>
        <p style={{ fontSize: 34, color: DIM, marginTop: 30 }}>
          Every frame in this video was rendered by the tool itself.
        </p>
        <p style={{ fontFamily: MONO, fontSize: 30, color: CYAN, marginTop: 44 }}>
          github.com/George-RD/rive-rs-cli
        </p>
      </div>
    </AbsoluteFill>
  );
};

export const Promo: React.FC = () => {
  let cursor = 0;
  const blocks: React.ReactNode[] = [];

  blocks.push(
    <Sequence key="problem" from={cursor} durationInFrames={PROBLEM}>
      <Problem />
    </Sequence>
  );
  cursor += PROBLEM;

  blocks.push(
    <Sequence key="loop" from={cursor} durationInFrames={LOOP}>
      <Loop />
    </Sequence>
  );
  cursor += LOOP;

  blocks.push(
    <Sequence key="coverage" from={cursor} durationInFrames={COVERAGE}>
      <Coverage />
    </Sequence>
  );
  cursor += COVERAGE;

  for (const scene of SCENES) {
    blocks.push(
      <Sequence key={scene.id} from={cursor} durationInFrames={PER_SCENE}>
        <SceneCard scene={scene} />
      </Sequence>
    );
    cursor += PER_SCENE;
  }

  blocks.push(
    <Sequence key="close" from={cursor} durationInFrames={CLOSE}>
      <Close />
    </Sequence>
  );

  return <AbsoluteFill style={BACKDROP}>{blocks}</AbsoluteFill>;
};
