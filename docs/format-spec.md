# Rive Encoding Notes

This document tracks format behavior validated during `rive-cli` development and bug fixing.

## Header

- Fingerprint: ASCII `RIVE` (4 bytes)
- Version: major varuint (`7`), minor varuint (`0`)
- File id: varuint
- Followed by ToC bytes

## Table of Contents (ToC)

- Property keys are written as varuint sequence, terminated by `0`
- Backing types are packed in 2-bit fields in little-endian `u32` words
- One `u32` holds 16 properties (`16 * 2 = 32 bits`)
- Backing bits:
  - `0` = uint/bool
  - `1` = string
  - `2` = float
  - `3` = color

## Objects

- Each object starts with object `type_key` varuint
- Then repeated `property_key` + property value entries
- Object terminator is property key `0`

## Backing Type Rules

- Bool properties are encoded as single byte, not LEB128 varuint
- Float properties are encoded as `f32` little-endian
- Color values are encoded as `u32` little-endian

## Emission Rules Required for Runtime Compatibility

- Do not include baseline properties in ToC: name (`4`), parentId (`5`), width (`7`), height (`8`)
- Artboard property order must be width (`7`) -> height (`8`) -> name (`4`)
- Artboard must not emit parentId
- LinearAnimation emits defaults selectively:
  - always: name/fps/duration
  - optional when non-default: speed/loop/workStart/workEnd
  - never emit quantize (`376`)

## State Machine Requirements

- `StateMachineLayer` import requires sentinel states to exist: AnyState (`62`), EntryState (`63`), ExitState (`64`)
- If user spec omits AnyState, builder injects it
- Transitions must be emitted immediately after their source state
