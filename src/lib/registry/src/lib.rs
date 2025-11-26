<<<<<<< HEAD
pub mod generated;

// Re-export for easy access
pub use generated::blocks::{get_block_by_id, get_block_by_name};
pub use generated::items::{get_item_by_id, get_item_by_name};
pub use generated::mappings::{get_block_id_from_item_id, get_item_id_from_block_id};
=======
use ferrumc_core::items::item_id::ItemID;
use ferrumc_world::block_state_id::BlockStateId;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use simd_json::OwnedValue;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use tokio::runtime::Builder;
use tokio::task;
use tracing::info;
use walkdir::WalkDir;

// --- MODULES ---
pub mod blocks;
pub mod items;

// Re-export logic so other crates can use `ferrumc_registry::lookup_...`
pub use blocks::*;
pub use items::*;

// --- 1. Deserialization Structs (Matches JSON layout) ---

#[derive(Deserialize, Debug)]
struct BlockStateEntry {
    name: String,
}

#[derive(Deserialize, Debug)]
struct ItemEntry {
    protocol_id: i32,
}

#[derive(Deserialize, Debug)]
struct BlockEntry {
    name: String,
    hardness: Option<f32>,
}

#[derive(Deserialize, Debug)]
struct RegistryRoot {
    #[serde(rename = "minecraft:item")]
    item: ItemRegistryFile,
    // We ignore `minecraft:block` here because we load blocks.json separately
}

#[derive(Deserialize, Debug)]
struct ItemRegistryFile {
    entries: HashMap<String, ItemEntry>,
}

// --- 2. Runtime Registry Structs (The "Database") ---

#[derive(Default, Debug)]
pub struct ItemRegistry {
    pub name_to_id: HashMap<String, i32>,
    pub id_to_name: HashMap<i32, String>,
}

#[derive(Default, Debug)]
pub struct BlockstateRegistry {
    pub id_to_name: HashMap<String, String>,
}

#[derive(Default, Debug)]
pub struct BlockPropertiesRegistry {
    pub name_to_hardness: HashMap<String, f32>,
}

#[derive(Default, Debug)]
pub struct ItemBlockMapRegistry {
    pub item_id_str_to_blockstate_id_str: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct ExtractedBlocksRoot {
    blocks: Vec<BlockEntry>,
}

/// The single, global registry struct.
#[derive(Default, Debug)]
pub struct GameRegistry {
    pub items: ItemRegistry,
    pub blockstates: BlockstateRegistry,
    pub block_properties: BlockPropertiesRegistry,
    pub item_block_map: ItemBlockMapRegistry,
    /// Catch-all for other JSON files
    pub generic_data: HashMap<String, OwnedValue>,
}

static REGISTRY: OnceCell<GameRegistry> = OnceCell::new();

// --- Loading Logic ---

/// Helper to find the assets directory by walking up the tree.
/// This makes it work for both `cargo run` (root) and `cargo test` (nested crate).
fn get_assets_dir() -> PathBuf {
    let mut current = std::env::current_dir().expect("Failed to get current dir");

    // Safety check loop (max 5 levels up)
    for _ in 0..5 {
        let candidate = current.join("assets");
        if candidate.exists() && candidate.is_dir() {
            return candidate;
        }
        if !current.pop() {
            break;
        }
    }

    panic!("Could not find 'assets' directory! Are you running this from within the project?");
}

fn load_json<T>(path: PathBuf) -> T
where
    T: for<'de> Deserialize<'de>,
{
    let mut buf = fs::read(&path)
        .unwrap_or_else(|e| panic!("Failed to read asset file {}: {}", path.display(), e));
    simd_json::from_slice(&mut buf)
        .unwrap_or_else(|e| panic!("Failed to parse asset file {}: {}", path.display(), e))
}

fn load_generic_json(dir_path: PathBuf) -> HashMap<String, OwnedValue> {
    let mut map = HashMap::new();

    for entry in WalkDir::new(&dir_path).into_iter().filter_map(|e| e.ok()) {
        let file_path = entry.path();
        if file_path.is_file() && file_path.extension().is_some_and(|s| s == "json") {
            let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

            let mut buf = fs::read(file_path).unwrap();
            let parsed_json = simd_json::to_owned_value(&mut buf).unwrap();

            map.insert(file_name, parsed_json);
        }
    }
    map
}

/// Initializes the game registry.
/// This parses all registries in parallel at startup.
pub fn init() {
    if REGISTRY.get().is_some() {
        return;
    }

    info!("Loading and parsing all asset files in parallel...");
    let start = Instant::now();

    // --- Resolve paths dynamically ---
    let assets_dir = get_assets_dir();
    let data_dir = assets_dir.join("data");
    let extracted_dir = assets_dir.join("extracted");

    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime for registry loading");

    let registry = runtime.block_on(async {
        // Capture paths for threads
        let p1 = data_dir.clone();
        let p2 = data_dir.clone();
        let p3 = data_dir.clone();
        let p4 = extracted_dir.clone();
        let p5_data = data_dir.clone();
        let p5_extracted = extracted_dir.clone();

        let reg_handle =
            task::spawn_blocking(move || load_json::<RegistryRoot>(p1.join("registries.json")));

        let bs_handle = task::spawn_blocking(move || {
            load_json::<HashMap<String, BlockStateEntry>>(p2.join("blockstates.json"))
        });

        let i2b_handle = task::spawn_blocking(move || {
            load_json::<HashMap<String, String>>(p3.join("item_to_block_mapping.json"))
        });

        let blocks_handle = task::spawn_blocking(move || {
            // 1. Parse the Root Object
            let root = load_json::<ExtractedBlocksRoot>(p4.join("blocks.json"));

            // 2. Return just the inner list of blocks
            root.blocks
        });

        let generic_data_handle = task::spawn_blocking(move || {
            let mut data_map = load_generic_json(p5_data);
            let extracted_map = load_generic_json(p5_extracted);
            data_map.extend(extracted_map);
            data_map.remove("registries.json");
            data_map.remove("blockstates.json");
            data_map.remove("item_to_block_mapping.json");
            data_map.remove("blocks.json");
            data_map
        });

        let registry_file = reg_handle.await.unwrap();
        let blockstates_file = bs_handle.await.unwrap();
        let item_to_block_file = i2b_handle.await.unwrap();
        let blocks_file = blocks_handle.await.unwrap();
        let generic_data = generic_data_handle.await.unwrap();

        let mut game_registry = GameRegistry::default();

        for (name, entry) in registry_file.item.entries {
            game_registry
                .items
                .name_to_id
                .insert(name.clone(), entry.protocol_id);
            game_registry
                .items
                .id_to_name
                .insert(entry.protocol_id, name);
        }

        for (id_str, entry) in blockstates_file {
            game_registry
                .blockstates
                .id_to_name
                .insert(id_str, entry.name);
        }

        for block_entry in blocks_file {
            game_registry
                .block_properties
                .name_to_hardness
                .insert(block_entry.name, block_entry.hardness.unwrap_or(0.0));
        }

        game_registry
            .item_block_map
            .item_id_str_to_blockstate_id_str = item_to_block_file;
        game_registry.generic_data = generic_data;

        game_registry
    });

    if REGISTRY.set(registry).is_err() {
        info!("Registry was already initialized.");
    }

    info!("All asset files loaded in {:?}", start.elapsed());
}

/// A private helper to safely get the initialized registry.
/// It is `pub(crate)` so `items.rs` and `blocks.rs` can use it.
pub(crate) fn get_registry() -> &'static GameRegistry {
    REGISTRY.get().expect(
        "Registry has not been initialized. Call ferrumc_registry::init() at server startup.",
    )
}

/// Helper to convert a BlockStateId (from the world) into an ItemID (for inventory).
/// e.g. BlockStateId(Stone) -> ItemID(Stone)
pub fn get_item_from_block_state(block_state: BlockStateId) -> Option<ItemID> {
    // 1. Get the Protocol ID string (e.g. "1")
    let state_id_str = block_state.0.to_string();

    // 2. Lookup the Block Name (e.g. "minecraft:stone")
    let block_name = lookup_blockstate_name(&state_id_str)?;

    // 3. Lookup the Item Protocol ID (e.g. 1)
    let item_id = lookup_item_protocol_id(block_name)?;

    // 4. Return the ItemID
    Some(ItemID::new(item_id))
}
>>>>>>> origin/master
