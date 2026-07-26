# Rive Encoding Notes

This document tracks format behavior validated during `rive-cli` development and bug fixing.

## Header

- Fingerprint: ASCII `RIVE` (4 bytes)
- Version: major varuint (`7`), minor varuint (`0`)
- File id: varuint
- Followed by ToC bytes

## Table of Contents (ToC)

- Property keys are written as a varuint sequence, terminated by `0`
- Backing types are packed in 2-bit fields in little-endian `u32` words
- **One `u32` holds only 4 properties.** The reader loads a fresh `u32` once it has consumed 4 codes (bit positions 0, 2, 4, 6) and never reads the upper 24 bits: see `rive-runtime/include/rive/runtime_header.hpp:87-104`, where `currentBit` starts at 8 and resets on reload. A file therefore carries `ceil(key_count / 4)` words.
- The official format page states "property count / 4 bytes", i.e. 16 codes per word. That contradicts the runtime implementation and is wrong for any file with more than 4 ToC keys. Verified against `demo/riv/reference/official_test.riv` (8 ToC keys): decoding 4-per-word puts Backboard (`23`) as the first object, decoding 16-per-word leaves the header 4 bytes short and yields two phantom `type=0` objects.
- Backing bits:
  - `0` = uint/bool
  - `1` = string/bytes (both are varuint length + payload, and share field id 1)
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

- Every property key the file writes is declared in the ToC. The runtime resolves a key by trying the object's own deserializer, then its built-in registry, then the file ToC (`rive-runtime/src/file.cpp:147-168`). A key that is in none of the three aborts the object mid-stream and cascades into `Problem loading file; may be corrupt!`. Declaring a key the runtime already knows natively is redundant but harmless, because the native lookup wins before the ToC is consulted; declaring too few is not recoverable. Emitting all of them is what lets an older runtime skip object types it does not have.
- Artboard property order must be width (`7`) -> height (`8`) -> name (`4`)
- Artboard must not emit parentId
- LinearAnimation emits defaults selectively:
  - always: name/fps/duration
  - optional when non-default: speed/loop/workStart/workEnd
  - quantize (`376`) is emitted only when non-zero, and omitted at its default of 0

## State Machine Requirements

- `StateMachineLayer` import requires sentinel states to exist: AnyState (`62`), EntryState (`63`), ExitState (`64`)
- If user spec omits AnyState, builder injects it
- Transitions must be emitted immediately after their source state
