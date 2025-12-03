# **FerrumC Core**

**Layer:** 1 (The Primitives)  
**Dependencies:** serde, uuid, bitcode (optional), deepsize (optional)

## **Purpose**

This crate is the **Absolute Source of Truth** for the server's data structures. It defines the "Language" of Minecraft (Blocks, Items, Coordinates, Chunks) in pure Rust.  
**Crucially, this crate has ZERO dependencies on the Game Engine (Bevy) or the Network Stack (Tokio).**  
This allows every other crate in the system (net, storage, plugins) to speak the same language without creating circular dependencies.

## **What DOES NOT belong here?**

* **Game Logic:** "How high does a player jump?" belongs in ferrumc-plugins.  
* **ECS Components:** \#\[derive(Component)\] belongs in ferrumc-ecs.  
* **I/O Implementation:** "How do we save a chunk to LMDB?" belongs in ferrumc-storage.  
* **Network Implementation:** "How do we write bytes to a socket?" belongs in ferrumc-net.

## **What DOES belong here?**

1. **Pure Data Structs:** BlockPos, Chunk, ItemID.  
2. **Math Primitives:** AABB, Rotation.  
3. **Interfaces (Traits):** The definitions of PacketSender and ChunkSource.

## **Module Overview**

### **math/**

Fundamental geometric types.

* **BlockPos**: Integer coordinates (x, y, z).  
* **ChunkPos**: Chunk coordinates (x, z).  
* **AABB**: Axis-Aligned Bounding Box for collision detection.  
* **Rotation**: Pitch/Yaw handling.

### **world/**

Terrain data structures.

* **Chunk / Section**: The actual storage format for blocks (Paletted Container).  
* **BlockStateId**: A u32 wrapper representing a specific block state (e.g., stone vs grass\_block\[snowy=true\]).  
* **BlockData**: Static properties (Hardness, Blast Resistance) loaded from the registry.

### **items/**

Item data structures.

* **ItemID**: A u32 wrapper for Item Protocol IDs.  
* **InventorySlot**: Represents a slot in a container (Count, ItemID, NBT).  
* **ItemData**: Static properties (Stack Size, Food Value, Tool Tier).

### **entities/**

Entity definitions.

* **EntityType**: Enum of all entity kinds.  
* **EntityData**: Static properties (Hitbox Size, Fire Immunity).

### **net/ (Traits)**

Abstract interfaces for dependency injection.

* **PacketSender**: Allows game logic to send packets without knowing about Tokio channels.

## **The Dependency Injection Pattern**

To prevent circular dependencies (e.g., WorldGen needs to save chunks, but Storage depends on Core), we define **Traits** in this crate.

1. **Define Trait in Core:**  
```rust
   pub trait ChunkSource: Send \+ Sync {  
       fn get\_chunk(\&self, x: i32, z: i32) \-\> Option\<Arc\<Chunk\>\>;  
   }
```
2. **Implement in Infrastructure:** ferrumc-storage implements ChunkSource for LmdbDatabase.  
3. **Use in Logic:** ferrumc-plugins accepts Box\<dyn ChunkSource\>.

This allows High-Level Logic to call Low-Level Infrastructure without depending on it directly.