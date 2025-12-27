import json
import logging
from datetime import datetime
from pathlib import Path
from collections import Counter
from typing import Dict, Any

ASSETS_DIR = Path('assets')
BLOCKS_PATH = ASSETS_DIR / 'extracted' / 'blocks.json'
MAPPING_PATH = ASSETS_DIR / 'data' / 'item_to_block_mapping.json'
LOG_PATH = Path('mapping_gen.log')

def load_json(path: Path) -> Dict[str, Any]:
    """Helper to load JSON safely."""
    try:
        return json.loads(path.read_text(encoding='utf-8'))
    except (FileNotFoundError, json.JSONDecodeError):
        return {}

def save_json(path: Path, data: Dict[str, Any]) -> None:
    """Helper to save JSON with consistent formatting."""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=4, sort_keys=True), encoding='utf-8')

def generate_mapping() -> None:
    log_buffer = [f"=== Generation Log: {datetime.now()} ==="]

    if not BLOCKS_PATH.exists():
        print(f"Error: Source file not found at {BLOCKS_PATH}")
        return

    print(f"Reading {BLOCKS_PATH.name}...")
    source_data = load_json(BLOCKS_PATH)
    
    old_mapping = load_json(MAPPING_PATH)
    if not old_mapping and MAPPING_PATH.exists():
        print("Warning: Existing mapping file is corrupt or empty. Treating as full rebuild.")

    new_mapping = {}
    stats = Counter()
    blocks = source_data.get("blocks", [])

    print(f"Processing {len(blocks)} blocks...")

    for block in blocks:
        block_name = block.get("name", "unknown")
        item_id = block.get("item_id")
        default_state = block.get("default_state_id")

        if item_id in (None, -1):
            stats["skipped_no_item"] += 1
            continue

        key_id = str(item_id)
        val_id = str(default_state)

        # Detect changes vs new entries
        if key_id not in old_mapping:
            stats["added"] += 1
            log_buffer.append(f"[NEW] Item {key_id} ({block_name}) -> State {val_id}")
        elif old_mapping[key_id] != val_id:
            stats["updated"] += 1
            log_buffer.append(f"[UPDATE] Item {key_id} ({block_name}): {old_mapping[key_id]} -> {val_id}")
        
        new_mapping[key_id] = val_id
        stats["processed"] += 1

    print(f"Writing output to {MAPPING_PATH}...")
    save_json(MAPPING_PATH, new_mapping)

    summary = (
        f"\n=== Summary ===\n"
        f"Total Blocks Scanned: {len(blocks)}\n"
        f"Valid Mappings:       {stats['processed']}\n"
        f"Newly Added:          {stats['added']}\n"
        f"Updated:              {stats['updated']}\n"
        f"Skipped (No Item ID): {stats['skipped_no_item']}\n"
    )
    log_buffer.append(summary)

    print(f"Saving log to {LOG_PATH}...")
    LOG_PATH.write_text('\n'.join(log_buffer), encoding='utf-8')
    
    print(summary)
    print("Done.")

if __name__ == "__main__":
    generate_mapping()