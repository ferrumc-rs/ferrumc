# **FerrumC Protocol**

**Layer:** 3 (Interface)  
**Dependencies:** ferrumc-core, ferrumc-nbt, ferrumc-macros

## **Purpose**

This crate is the **Dictionary of the Network**. It defines the structure of every packet sent between the client and server.  
It is a **Pure Data** crate. It does not know about TCP, Tokio, ECS Entities, or database storage. It simply defines structs and how to turn them into bytes.

## **What DOES NOT belong here?**

* **Socket Logic:** Reading/Writing to a TcpStream belongs in ferrumc-net.  
* **Game Logic:** Handling "What happens when I click a block" belongs in ferrumc-modules.  
* **ECS Components:** Player, Position, or Health structs belong in ferrumc-ecs.

## **What DOES belong here?**

1. **Packet Structs:** pub struct ChunkDataPacket { ... }  
2. **Encoding Traits:** The definition of NetEncode and NetDecode.  
3. **Packet IDs:** The generated constants ids::PLAY\_CLIENTBOUND\_KEEP\_ALIVE.  
4. **Network Types:** Wrappers specific to the wire format (e.g., VarInt, NetAngle, RawNbt).

## **Usage Guide**

### **1\. Defining a Packet**

Packets are just structs marked with the \#\[packet\] macro. They must implement NetEncode.  
```rust
use ferrumc\_protocol::ids;  
use ferrumc\_macros::{packet, NetEncode};

\#\[derive(NetEncode)\]  
\#\[packet(id \= ids::PLAY\_CLIENTBOUND\_KEEP\_ALIVE, state \= "play")\]  
pub struct KeepAlivePacket {  
    pub id: i64,  
}
```

### **2\. Using Core Types**

Use types from ferrumc-core where possible, but wrap them if they need special encoding logic.

* **Good:** pub position: BlockPos (If BlockPos impls NetEncode)  
* **Good:** pub item: InventorySlot (If InventorySlot impls NetEncode)  
* **Bad:** pub entity: Entity (Bevy Entities are local only; use EntityID or VarInt).

### **3\. Serialization Logic**

If a packet requires complex serialization (like NBT), do not put that logic in the packet struct itself if possible. Use a wrapper type in src/types/.

* **Example:** RawNbt(Vec\<u8\>) is a wrapper defined here that handles writing the bytes, so the Packet struct remains clean.

## **Dependency Flow**

* **ferrumc-net** uses this crate to deserialize bytes off the wire.  
* **ferrumc-modules** uses this crate to construct packets to send to clients.