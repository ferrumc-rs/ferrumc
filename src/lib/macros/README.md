# **FerrumC Macros**

Layer: 1 (Primitives)  
Dependencies: syn, quote, proc-macro2

## **Purpose**

This crate provides **Procedural Macros** to reduce boilerplate code across the server. It automates the implementation of complex traits for Networking and File I/O.  
Instead of manually writing byte-parsing logic for every single packet (which is error-prone and tedious), we derive it automatically from the struct definitions.

## **Macros Overview**

### **1\. Network Encoding (NetEncode / NetDecode)**

These macros implement the binary protocol used by the Minecraft TCP connection.

* **NetEncode**: Writes the struct fields sequentially to a std::io::Write or tokio::io::AsyncWrite.  
* **NetDecode**: Reads bytes from a stream and constructs the struct.

**Features:**

* Automatically handles VarInt, String (prefixed), and Vec\<T\> (prefixed).  
* Supports enums (writes the discriminant if needed, or matches variants).

**Usage:**
```rust
use ferrumc\_macros::{NetEncode, NetDecode};

\#\[derive(NetEncode, NetDecode)\]  
pub struct ExamplePacket {  
    pub count: i32,  
    pub message: String,  
}
```
### **2\. NBT Serialization (NBTSerialize / NBTDeserialize)**

These macros implement the **Named Binary Tag** format used for Chunk data, Registry data, and complex Item components.  
**Attributes:**

* \#\[nbt(rename \= "Name")\]: Changes the tag name written to NBT.  
* \#\[nbt(rename \= "")\]: Marks the struct as a Root Compound (empty name).  
* \#\[nbt(skip)\]: Ignores a field.  
* \#\[nbt(flatten)\]: Flattens a nested struct into the parent compound.

**Usage:**  
```rust
use ferrumc\_macros::NBTSerialize;

\#\[derive(NBTSerialize)\]  
\#\[nbt(rename \= "")\] // Root Tag  
pub struct LevelData {  
    \#\[nbt(rename \= "DataVersion")\]  
    pub version: i32,  
}
```

### **3\. Packet Metadata (\#\[packet\])**

This attribute macro links a struct to a specific **Packet ID** and **Connection State**.  
It generates the impl Packet block required by the networking stack to route data correctly.  
**Arguments:**

* id: The protocol ID (u8). Use constants from ferrumc\_protocol::ids.  
* state: The connection state ("handshake", "status", "login", "play", "configuration").

**Usage:**
```rust
use ferrumc\_macros::packet;  
use ferrumc\_protocol::ids;

\#\[derive(NetEncode)\]  
\#\[packet(id \= ids::PLAY\_CLIENTBOUND\_KEEP\_ALIVE, state \= "play")\]  
pub struct KeepAlivePacket {  
    pub id: i64,  
}
```
