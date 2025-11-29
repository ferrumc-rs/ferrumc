# **FerrumC Net**

**Layer:** 4 (Infrastructure)  
**Dependencies:** ferrumc-core, ferrumc-protocol, ferrumc-ecs, tokio

## **Purpose**

This crate is the **Asynchronous I/O Engine** of the server. It manages the lifecycle of every TCP connection, from the initial handshake to the final disconnect.  
It acts as the translation layer between the "Real World" (Bytes on a socket) and the "Game World" (Events in the ECS).

## **What DOES NOT belong here?**

* **Packet Definitions:** Structs like KeepAlivePacket belong in **ferrumc-protocol**.  
* **Game Logic:** Handling "What happens when a player joins" belongs in **ferrumc-plugins**.  
* **Entity Data:** Storing Position or Health belongs in **ferrumc-ecs**.

## **What DOES belong here?**

1. **Tokio Runtime:** The main TCP listener loop and per-client connection tasks.  
2. **Pipeline Logic:** Compression (zlib) and Encryption (AES/CFB8) streams.  
3. **Packet Routing:** Deserializing raw bytes into ferrumc-protocol structs.  
4. **Event Firing:** Injecting those structs into the ECS as Events (e.g. EventWriter\<PlayerAction\>).  
5. **Packet Sending:** Implementing the PacketSender trait to allow other systems to send data.

## **Architecture**

### **The Connection Lifecycle**

1. **Accept:** The main listener accepts a new TcpStream.  
2. **Handshake:** A temporary task handles the initial Handshake/Login/Status negotiation.  
3. **Spawn:** Once authenticated, the connection is moved to the main Game Loop.  
4. **Loop:** A dedicated Tokio task (handle\_connection) enters a select loop:  
   * **Read:** Reads bytes \-\> Decrypts \-\> Decompresses \-\> Deserializes \-\> **Fires ECS Event**.  
   * **Write:** Receives Packet struct from Channel \-\> Serializes \-\> Compresses \-\> Encrypts \-\> **Writes to Socket**.

### **The "PacketSender" Trait**

To prevent circular dependencies, this crate implements the PacketSender trait defined in ferrumc-core.
```rust
// In ferrumc-net  
pub struct NetPacketSender {  
    tx: mpsc::UnboundedSender\<Vec\<u8\>\>,  
}

impl PacketSender for NetPacketSender {  
    fn send(\&self, packet: \&dyn NetEncode) \-\> Result\<()\> {  
        // 1\. Encode packet to bytes  
        // 2\. Send bytes to the connection task via channel  
    }  
}
```
This allows the **Game Logic** (Layer 5\) to send packets using the trait, without needing to depend on ferrumc-net or tokio directly.