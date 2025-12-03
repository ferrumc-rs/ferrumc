# **FerrumC Modules**

**Layer:** 5 (Game Logic)  
**Dependencies:** ferrumc-core, ferrumc-ecs, ferrumc-registry, ferrumc-protocol

## **Purpose**

These crates contain the **Actual Gameplay Logic** of the server. It is the "Brain" of FerrumC.  
Everything here runs as a **Bevy System**. This is where you implement physics, mining, crafting, combat, and chat handling.

## **What DOES NOT belong here?**

* **TCP / Sockets:** You do not read bytes here. You listen for Event\<Packet\>.  
* **Database / Files:** You do not read files here. You use the ChunkSource trait.  
* **Component Definitions:** You do not define struct Health. You import it from ferrumc-ecs.  
* **Direct Infrastructure:** You must **NOT** depend on ferrumc-net or ferrumc-storage. This ensures your game logic can be unit-tested without spinning up a real server or database.

## **What DOES belong here?**

1. **Bevy Plugins:** pub struct MiningPlugin;  
2. **Systems:** fn tick\_physics(query: Query\<\&mut Position\>)  
3. **Event Handlers:** fn on\_block\_break(mut events: EventReader\<BlockBreak\>)

## **Module Overview**

The logic is organized by **Feature**, not by technical type.

### **movement/**

* Validates player movement packets.  
* Applies gravity and physics.  
* Handles collision detection against the world.

### **mining/**

* Handles the digging state machine (Start \-\> Cancel \-\> Finish).  
* Calculates break times based on tools and blocks (ferrumc-registry).  
* Fires BlockBreakEvent upon success.

### **interactions/**

* Item usage (Eating, Drinking).  
* Inventory management (Clicking slots, Hotbar swapping).  
* Entity interaction (Attacking, Trading).

### **replication/ (The "View" Layer)**

* **The Reflector:** This module is responsible for syncing the ECS state back to the clients.  
* Listens for Changed\<Component\> or specific Events.  
* Calculates visibility (Who needs to see this?).  
* Sends packets using the PacketSender trait from ferrumc-core.

### **world\_gen/**

* Handles terrain generation requests.  
* Uses the ChunkSource trait to read neighbors safely.

## **Developer Guide**

### **How to add a new feature (e.g. "Thirst")**

1. **Define Data (Layer 1 & 2):**  
   * Add ThirstData to ferrumc-core.  
   * Add Thirst component to ferrumc-ecs.  
2. **Create Logic (Layer 5):**  
   * Create src/lib/modules/src/survival/thirst.rs.  
   * Write a system: fn tick\_thirst(mut query: Query\<\&mut Thirst\>).  
3. **Create Replication (Layer 5):**  
   * In replication/, write fn sync\_thirst(query: Query\<(\&Thirst, \&NetworkConnection), Changed\<Thirst\>\>).  
   * Use conn.send\_packet(...) to notify the client.  
4. **Register:**  
   * Add the systems to the SurvivalPlugin.