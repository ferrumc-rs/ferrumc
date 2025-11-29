pub mod components; // Position, Health, Inventory, NetworkConnection (holds Box\<dyn PacketSender\>).
pub mod messages; // PlayerJoin, BlockBreak, Chat, EntityMove.
pub mod resources; // ServerState (holds Box\<dyn ChunkSource\>), PlayerList.
