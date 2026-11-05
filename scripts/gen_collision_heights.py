#!/usr/bin/env python3
"""Regenerate the vendored per-state collision-top heights for ferrumc-registry.

The registry needs, per block state, the top height (max Y) of the block's
collision box so the simulation can decide falling-block landings precisely (a
support whose collision top is below a full cube breaks a faller; a full cube
settles it). The upstream PrismarineJS `minecraft-data` snapshot ships the full
collision shapes in `blockCollisionShapes.json`; this script reduces them to a
single top height per state, aligned to the block-state encoding used by
`blocks.json`, and writes the compact `block_collision_heights.json` the build
script consumes.

Provenance: both inputs are the SAME pinned snapshot recorded in
`fixtures/protocol/1_21_8/manifest.toml`:

    repo   = https://github.com/PrismarineJS/minecraft-data
    commit = a8cf733fe44f3069f87f63e7ec3d74b521840ded
    path   = data/pc/1.21.8   (Minecraft 1.21.8, dataVersion 4440)

`blocks.json` is vendored (fixtures + registry/data); `blockCollisionShapes.json`
is fetched at that commit and passed in rather than vendored (it is ~1.7 MiB and
only its derived heights are needed).

Usage:

    # fetch the collision shapes at the pinned commit, then derive:
    curl -sSL -o /tmp/bcs.json \\
      https://raw.githubusercontent.com/PrismarineJS/minecraft-data/a8cf733fe44f3069f87f63e7ec3d74b521840ded/data/pc/1.21.8/blockCollisionShapes.json
    python3 scripts/gen_collision_heights.py /tmp/bcs.json

Output format (`block_collision_heights.json`): a JSON object mapping a bare
block name to either a single height (uniform across every state) or an ordered
per-state list (states whose height varies, e.g. slabs, snow layers, dripstone),
indexed by `state_id - minStateId`. A block absent from the collision snapshot is
omitted; the build script then falls back to the coarse `boundingBox` flag.
"""

import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BLOCKS = os.path.join(REPO_ROOT, "fixtures", "protocol", "1_21_8", "blocks.json")
OUT = os.path.join(
    REPO_ROOT, "fixtures", "protocol", "1_21_8", "block_collision_heights.json"
)


def max_top(shape_boxes):
    """The highest collision-box top (max y1), or 0.0 for an empty shape."""
    return round(max((box[4] for box in shape_boxes), default=0.0), 5)


def main(argv):
    if len(argv) != 2:
        sys.exit(f"usage: {argv[0]} <blockCollisionShapes.json>")
    with open(argv[1], encoding="utf-8") as f:
        collision = json.load(f)
    with open(BLOCKS, encoding="utf-8") as f:
        blocks = json.load(f)

    block_index = collision["blocks"]
    shapes = collision["shapes"]

    out = {}
    missing = []
    for block in blocks:
        name = block["name"]
        state_count = block["maxStateId"] - block["minStateId"] + 1
        entry = block_index.get(name)
        if entry is None:
            missing.append(name)
            continue
        if isinstance(entry, int):
            out[name] = max_top(shapes[str(entry)])
            continue
        heights = [max_top(shapes[str(i)]) for i in entry]
        if len(heights) != state_count:
            # Length disagreement: emit a single conservative max so the build
            # script never mis-indexes a state.
            out[name] = max(heights)
        elif all(h == heights[0] for h in heights):
            out[name] = heights[0]
        else:
            out[name] = heights

    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(out, f, separators=(",", ":"), sort_keys=True)

    per_state = sum(1 for v in out.values() if isinstance(v, list))
    print(f"wrote {OUT} ({os.path.getsize(OUT)} bytes)")
    print(f"blocks: {len(out)}  per-state: {per_state}  missing: {len(missing)}")
    if missing:
        print(f"missing from collision snapshot (build.rs falls back): {missing}")


if __name__ == "__main__":
    main(sys.argv)
