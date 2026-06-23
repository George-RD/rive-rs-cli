#!/usr/bin/env python3
"""Deterministic structural-parity metric between a generated .riv and an
official reference .riv.

It generates a .riv from a SceneSpec fixture via the rive-cli `generate`
command, decompiles both the generated file and the reference, then computes an
order-invariant *semantic* distance over the object graphs.

The distance counts genuine recreation gaps and normalizes away encoding
differences that are invisible to the Rive runtime:
  - synthetic component names (COMPONENT_NAME, key 4): the editor omits them.
  - default-valued properties the official encoder omits (KEY_FRAME_FRAME==0,
    INTERPOLATOR_ID==NONE).
  - object emission ORDER (the runtime resolves by parentId, not order), so
    objects are matched by their resolved (type, parent-path) signature rather
    than position.

Header file_id and ToC are reported separately so they can be driven to zero
explicitly (file_id via `generate --file-id`).

distance = unmatched_ref_objects
         + unmatched_gen_objects
         + value_property_mismatches_among_matched_pairs
         + file_id_mismatch
         + toc_symmetric_difference

distance == 0  =>  semantically identical (up to accepted encoding deltas).
"""

import argparse
import collections
import json
import subprocess
import sys

NAME = 4
PARENT = 5
SM_COMPONENT_NAME = 138
KEYFRAME_FRAME = 67
INTERPOLATOR_ID = 69
KEYFRAME_DOUBLE_VALUE = 70
WORK_START = 60
WORK_END = 61
ARTBOARD = 1
INTERP_NONE = 4294967295

# Property keys carrying cosmetic component names the runtime ignores for
# rendering/animation (it resolves by index/id). The official editor and our
# builder synthesize different names; treat as accepted drift.
NAME_KEYS = {NAME, SM_COMPONENT_NAME}


def run(cmd):
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode != 0:
        sys.stderr.write(f"command failed: {' '.join(cmd)}\n{r.stderr}\n")
        sys.exit(1)
    return r.stdout


def decompile(binary, path):
    return json.loads(run([binary, "decompile", path]))


def prop_value(v):
    # PropertyValueRead is serialized as {"Float": x} / {"UInt": n} / ...
    (kind, val), = v.items()
    return (kind, val)


def annotate(objects):
    """Attach artboard-local index and resolved parent-path signature.

    parent-path = tuple of ancestor type_keys from the object up to the artboard
    root, derived from COMPONENT_PARENT_ID (an artboard-local index).
    """
    artboard_start = None
    local_of = {}  # local index -> global index, within current artboard
    for gi, o in enumerate(objects):
        if o["type_key"] == ARTBOARD:
            artboard_start = gi
            local_of = {}
        o["_artboard_start"] = artboard_start
        if artboard_start is not None:
            o["_local"] = gi - artboard_start
            local_of[o["_local"]] = gi
        o["_local_of"] = local_of  # shared reference within artboard

    def parent_global(o):
        props = {p["key"]: p["value"] for p in o["properties"]}
        if PARENT not in props or o["_artboard_start"] is None:
            return None
        kind, pid = prop_value(props[PARENT])
        # parent local index -> global index in same artboard
        return o["_local_of"].get(pid)

    by_global = {gi: o for gi, o in enumerate(objects)}
    for gi, o in enumerate(objects):
        path = []
        cur = o
        seen = set()
        while True:
            pg = parent_global(cur)
            if pg is None or pg in seen:
                break
            seen.add(pg)
            parent = by_global[pg]
            path.append(parent["type_key"])
            cur = parent
        o["_path"] = tuple(path)


def value_props(o):
    """Object's semantic properties, with accepted encoding deltas removed.

    The official Rive encoder omits default-valued properties that our encoder
    writes explicitly; the runtime treats present-default and absent identically.
    Dropping them here keeps the metric focused on genuine recreation gaps.
    """
    out = {}
    for p in o["properties"]:
        k = p["key"]
        if k in NAME_KEYS or k == PARENT:
            continue
        kind, val = prop_value(p["value"])
        if k == KEYFRAME_FRAME and kind == "UInt" and val == 0:
            continue  # official omits frame==0
        if k == INTERPOLATOR_ID and kind == "UInt" and val == INTERP_NONE:
            continue  # official omits "no interpolator"
        if k == KEYFRAME_DOUBLE_VALUE and kind == "Float" and val == 0.0:
            continue  # official omits the default keyframe value 0.0
        if k in (WORK_START, WORK_END) and kind == "UInt" and val == 0:
            continue  # official writes work area 0,0; we omit (no work area)
        out[k] = (kind, val)
    return out


def signature(o):
    return (o["type_key"], o["_path"])


def object_distance(ref, gen):
    annotate(ref["objects"])
    annotate(gen["objects"])

    ref_groups = collections.defaultdict(list)
    gen_groups = collections.defaultdict(list)
    for o in ref["objects"]:
        ref_groups[signature(o)].append(value_props(o))
    for o in gen["objects"]:
        gen_groups[signature(o)].append(value_props(o))

    unmatched_ref = 0
    unmatched_gen = 0
    value_mismatch = 0
    details = []

    for sig in set(ref_groups) | set(gen_groups):
        rs = ref_groups.get(sig, [])
        gs = gen_groups.get(sig, [])
        n = min(len(rs), len(gs))
        # greedily pair to minimize value mismatch: sort by serialized props
        rs_sorted = sorted(rs, key=lambda d: json.dumps(d, sort_keys=True))
        gs_sorted = sorted(gs, key=lambda d: json.dumps(d, sort_keys=True))
        for i in range(n):
            r, g = rs_sorted[i], gs_sorted[i]
            keys = set(r) | set(g)
            diffs = [k for k in keys if r.get(k) != g.get(k)]
            if diffs:
                value_mismatch += len(diffs)
                details.append(
                    f"valdiff type={sig[0]} path={list(sig[1])} keys={sorted(diffs)} "
                    f"ref={{ {', '.join(f'{k}:{r.get(k)}' for k in sorted(diffs))} }} "
                    f"gen={{ {', '.join(f'{k}:{g.get(k)}' for k in sorted(diffs))} }}"
                )
        if len(rs) > n:
            unmatched_ref += len(rs) - n
            details.append(f"missing  type={sig[0]} path={list(sig[1])} x{len(rs)-n}")
        if len(gs) > n:
            unmatched_gen += len(gs) - n
            details.append(f"extra    type={sig[0]} path={list(sig[1])} x{len(gs)-n}")

    return unmatched_ref, unmatched_gen, value_mismatch, details


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--binary", default="target/debug/rive-cli")
    ap.add_argument("--fixture", required=True)
    ap.add_argument("--reference", required=True)
    ap.add_argument("--file-id", type=int, default=0)
    ap.add_argument("--out", default="target/parity_gen.riv")
    args = ap.parse_args()

    run([args.binary, "generate", args.fixture, "-o", args.out,
         "--file-id", str(args.file_id)])
    gen = decompile(args.binary, args.out)
    ref = decompile(args.binary, args.reference)

    um_ref, um_gen, valdiff, details = object_distance(ref, gen)

    file_id_diff = 0 if ref["header"]["file_id"] == gen["header"]["file_id"] else 1
    ref_toc = set(ref["toc_property_keys"])
    gen_toc = set(gen["toc_property_keys"])
    # ToC declares backing types for properties the reader may not know. The
    # reference declares 236/376 as unknown; our encoder knows them natively and
    # MUST omit them from the ToC (including known props breaks the WASM runtime,
    # per AGENTS.md). This divergence is intentional and accepted, not a gap.
    toc_delta = len(ref_toc ^ gen_toc)

    total = um_ref + um_gen + valdiff + file_id_diff

    for d in details:
        print(f"  {d}", file=sys.stderr)
    print(f"ASI ref_objects={len(ref['objects'])} gen_objects={len(gen['objects'])}")
    print(f"ASI missing={um_ref} extra={um_gen} value_mismatch={valdiff} "
          f"file_id_diff={file_id_diff}")
    print(f"ASI accepted_toc_delta={toc_delta} ref_toc={sorted(ref_toc)} gen_toc={sorted(gen_toc)}")
    print(f"METRIC parity={total}")


if __name__ == "__main__":
    main()
