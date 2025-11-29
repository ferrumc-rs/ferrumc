# **FerrumC ECS**

**Layer:** 2 (Engine State)  
**Dependencies:** ferrumc-core, bevy\_ecs

## **Purpose**

This crate is the **Schema of the Game Engine**. It defines exactly how the game state is represented in memory using Bevy's Entity Component System.  
To prevent circular dependencies, this crate serves as the **Single Source of Truth** for:

1. **Components:** Data attached to entities (e.g., Health, Position).  
2. **Messages:** Events fired between systems (e.g., BlockBreak, PlayerJoin).  
3. **Resources:** Global singleton state (e.g., ServerState, PlayerList).

## **What DOES NOT belong here?**

* **Pure Data Structs:** Basic structs like HealthData or BlockPos belong in **ferrumc-core**. This crate only contains the *wrappers* that allow Bevy to use them.  
* **Game Logic:** Systems, queries, and mutation logic belong in **ferrumc-plugins**.  
* **Network Logic:** Packet definitions belong in **ferrumc-protocol**.  
* **Implementation Details:** Concrete implementations of ChunkStorage or NetworkConnection do not belong here (use Traits/Boxed objects).

## **What DOES belong here?**

1. **Component Wrappers:** Newtypes around Core data (e.g., pub struct Health(pub HealthData)).  
2. **Bundles:** Pre-defined groups of components (e.g., PlayerBundle) to ensure consistency when spawning entities.  
3. **Events/Messages:** Structs derived with \#\[derive(Event)\].  
4. **Global Resources:** Structs derived with \#\[derive(Resource)\].

## **Usage Guide**

### **1\. Defining a Component (The Wrapper Pattern)**

We do not define fields directly on Components. Instead, we wrap "Pure Data" types from ferrumc-core. This allows the Network crate to serialize the data without depending on Bevy.
```rust 
use bevy\_ecs::prelude::Component;  
use ferrumc\_core::player::health::HealthData;  
use std::ops::{Deref, DerefMut};

\#\[derive(Component, Debug, Clone, Default)\]  
pub struct Health(pub HealthData);

// Implement Deref to allow direct access to inner fields  
impl Deref for Health {  
    type Target \= HealthData;  
    fn deref(\&self) \-\> \&Self::Target { \&self.0 }  
}

impl DerefMut for Health {  
    fn deref\_mut(\&mut self) \-\> \&mut Self::Target { \&mut self.0 }  
}
```

### **2\. Defining a Message (Event)**

Messages are used to communicate between decoupled systems (e.g., Mining \-\> Replication).
```rust
use bevy\_ecs::prelude::{Entity, Event};  
use ferrumc\_core::math::BlockPos;

\#\[derive(Event, Debug)\]  
pub struct BlockBreak {  
    pub player: Entity,  
    pub location: BlockPos,  
}
```

### **3\. Using Resources with Traits**

If a Resource needs to perform complex I/O (like database access), do **not** import the implementation here. Use the Traits defined in ferrumc-core.
```rust
use bevy\_ecs::prelude::Resource;  
use ferrumc\_core::traits::ChunkSource;

\#\[derive(Resource)\]  
pub struct ServerState {  
    // Logic layers can use this without knowing it is LMDB  
    pub chunk\_storage: Box\<dyn ChunkSource\>,  
}  
```