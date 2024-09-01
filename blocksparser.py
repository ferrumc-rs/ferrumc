import json

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
with open("newblocks.json", "w") as f:
    json.dump(dict_reorder(out), f, separators=(',', ':'))
