#!/usr/bin/env python3
import json
from pathlib import Path

REG_PATH = Path("../assets/data/registries.json")  # items with protocol_id
BS_PATH = Path("../assets/data/blockstates.json")  # { "<block_id>": { "name": "...", "default": bool, ... } }
OUT_PATH = Path("../assets/data/item_to_block_mapping.json")  # { "<item_pid>": "<block_id>" }


def main():
    root = json.loads(REG_PATH.read_text(encoding="utf-8"))
    blockstates = json.loads(BS_PATH.read_text(encoding="utf-8"))

    # Build name -> chosen block id
    # Rule: default:true wins and can't be overridden; otherwise use first seen.
    name_to_blockid = {}
    first_seen = {}

    for sid, data in blockstates.items():
        if not isinstance(data, dict):
            continue
        name = data.get("name")
        if not isinstance(name, str):
            continue
        sid = str(sid)
        is_def = bool(data.get("default", False))

        if name not in first_seen:
            first_seen[name] = sid
        if is_def:
            name_to_blockid[name] = sid
        elif name not in name_to_blockid:
            name_to_blockid[name] = sid

    # Build item_pid -> block_id via exact name match
    items = root.get("minecraft:item", {}).get("entries", {})
    mapping = {}
    for item_name, item_data in items.items():
        if isinstance(item_data, dict) and "protocol_id" in item_data:
            block_id = name_to_blockid.get(item_name)
            if block_id is not None:
                mapping[str(item_data["protocol_id"])] = block_id

    # OPTIONAL sanity guard (remove if you don't like the assert)
    if mapping.get("35") != "14":
        raise RuntimeError(f"Sanity-check failed: item 35 should map to 14, got {mapping.get('35')}")

    OUT_PATH.write_text(json.dumps(mapping, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
