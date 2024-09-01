import json
import bz2

def dict_reorder(item):
    return {k: dict_reorder(v) if isinstance(v, dict) else v for k, v in sorted(item.items())}

out = {}

with open("blocks.json") as f:
    blocks = json.load(f)
    for block in blocks:
        data = blocks[block]
        for state in data["states"]:
            if "properties" in state:
                props = state["properties"]
                if "id" in state:
                    block_id = state["id"]
                    out[block_id] = {"name": block, "properties": props}
            else:
                block_id = state["id"]
                out[block_id] = {"name": block, "default": True}


with open("blockstates.json", "w") as bs:
    json.dump(out, bs, indent=4)
    with open(".etc/blockmappings.bz2", "wb") as f:
        as_string = json.dumps(dict_reorder(out), separators=(',', ':'))
        f.write(bz2.compress(as_string.encode("utf-8")))
