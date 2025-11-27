pub mod generated;

// Re-export for easy access
pub use generated::blocks::{get_block_by_id, get_block_by_name};
pub use generated::items::{get_item_by_id, get_item_by_name};
pub use generated::mappings::{get_block_id_from_item_id, get_item_id_from_block_id};
