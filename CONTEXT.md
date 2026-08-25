# rive-cli

rive-cli is a code-first Rive authoring and verification project. Its domain separates authored intent, canonical scene description, and evidence about whether generated files are correct and faithful.

## Language

**SceneSpec**:
The canonical explicit description of a Rive scene used when precise runtime-facing control is required. It is also the expert escape hatch for concepts not yet represented at the higher authoring level.
_Avoid_: AI authoring format, high-level authoring model

**AuthoringSpec**:
The high-level description of visual, motion, and behavior intent intended for tools and agents. It uses authored concepts and stable identities rather than runtime indices or containment bookkeeping.
_Avoid_: SceneSpec, raw scene JSON

**Authored ID**:
A stable name chosen in AuthoringSpec to identify an authored concept across lowering, diagnostics, and edits.
_Avoid_: Runtime index, object index

**Runtime name**:
A deterministic name used to identify the runtime object or objects produced from authored intent.
_Avoid_: Authored ID

**Source map**:
The retained relationship between authored IDs and paths and the runtime names and SceneSpec locations they produce.
_Avoid_: Runtime index map

**Raw escape**:
An explicit SceneSpec fragment embedded in AuthoringSpec for a Rive concept outside the currently supported high-level subset.
_Avoid_: Default authoring path

**Motion**:
Time-varying authored intent expressed through named poses, tracks, timing, interpolation, and reusable easing concepts.
_Avoid_: Raw animation bookkeeping

**Behavior**:
Interactive and data-driven authored intent expressed through model properties, bindings, events, named states, and named transitions.
_Avoid_: Runtime state-machine indices

**Showcase**:
A checked-in representative AuthoringSpec document used to prove that a supported subset composes into a realistic static, animated, or interactive result.
_Avoid_: Toy example, isolated fixture

**Structural evidence**:
Evidence that a generated file or canonical scene is well-formed and accepted by the structural validation path.
_Avoid_: Runtime evidence, semantic evidence

**Runtime evidence**:
Evidence captured from the official Rive runtime showing that a generated file loads and behaves at representative frames or interactions.
_Avoid_: Structural evidence, semantic evidence

**Semantic evidence**:
Evidence that generated output matches the intended visual, motion, or interactive meaning, kept separate from structural and runtime correctness.
_Avoid_: Structural pass rate, runtime load result

**Incremental operation**:
An atomic edit addressed through stable authored identity that either validates completely or leaves the original authored document unchanged.
_Avoid_: Whole-document regeneration, runtime-index edit
