use crate::component::ComponentStorage;
use crate::entity::EntityManager;
use crate::helpers::entity_builder::EntityBuilder;
use crate::query::Query;

/// <p style="color:#4CAF50;font-size:1.2em;font-weight:bold;">The World struct</p>
///
/// The `World` struct is the central point of the ECS (Entity Component System).
/// It manages entities and components, and provides an interface for querying and manipulating the game state.
///
/// <p style="color:#607D8B;font-size:1.1em;font-weight:bold;">Additional Examples</p>
///
/// <p style="color:#795548;">Creating and using a complete game world:</p>
///
/// ```rust
/// // Create a new world
/// let mut world = World::new();
///
/// // Create some entities
/// let player = world.create_entity()
///     .with(Position { x: 0.0, y: 0.0 })
///     .with(Velocity { x: 0.0, y: 0.0 })
///     .with(Player { name: "Hero".to_string() })
///     .build();
///
/// let enemy = world.create_entity()
///     .with(Position { x: 10.0, y: 10.0 })
///     .with(Enemy { hp: 100 })
///     .build();
///
/// // Run a game loop
/// loop {
///     // Update player position
///     let mut query = world.query::<(&mut Position, &Velocity)>();
///     for (_, (mut pos, vel)) in query.iter().await {
///         pos.x += vel.x;
///         pos.y += vel.y;
///     }
///
///     // Check for collisions
///     let mut collision_query = world.query::<(&Position, Option<&Player>, Option<&Enemy>)>();
///     for (entity, (pos, player, enemy)) in collision_query.iter().await {
///         // Handle collisions...
///     }
///
///     // Break the loop when game is over
///     // break;
/// }
/// ```
///
/// This example demonstrates creating a world, adding entities with various components,
/// and running a simple game loop with position updates and collision detection.
pub struct World {
    entity_manager: EntityManager,
    component_storage: ComponentStorage,
}

impl World {
    /// <p style="color:#2196F3;">Creates a new World instance</p>
    ///
    /// # Example
    ///
    /// ```rust
    /// let world = World::new();
    /// ```
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_storage: ComponentStorage::new(),
        }
    }

    /// <p style="color:#FFC107;">Creates a new entity and returns an EntityBuilder</p>
    ///
    /// Use this method to create and configure new entities in the world.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut world = World::new();
    /// let entity = world.create_entity()
    ///     .with(Position { x: 0.0, y: 0.0 })
    ///     .with(Velocity { x: 1.0, y: 1.0 })
    ///     .build();
    /// ```
    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.entity_manager.create_entity();
        EntityBuilder::new(entity, &self.component_storage)
    }

    /// <p style="color:#E91E63;">Creates a new query for components</p>
    ///
    /// Use this method to query entities with specific components.
    ///
    /// # Example
    ///
    /// ```rust
    /// let world = World::new();
    ///
    /// // Query for all entities with Position and Velocity components
    /// let query = world.query::<(&Position, &Velocity)>();
    ///
    /// // Iterate over the query results
    /// for (entity_id, (position, velocity)) in query.iter().await {
    ///     println!("Entity {} at {:?} moving with velocity {:?}", entity_id, position, velocity);
    /// }
    ///
    /// // Query with mutable components
    /// let mut update_query = world.query::<(&mut Position, &Velocity)>();
    /// for (_, (mut position, velocity)) in update_query.iter().await {
    ///     position.x += velocity.x;
    ///     position.y += velocity.y;
    /// }
    /// ```
    pub fn query<Q>(&self) -> Query<Q>
    where
        Q: crate::query::QueryItem,
    {
        Query::<Q>::new(&self.entity_manager, &self.component_storage)
    }

    /// <p style="color:#9C27B0;">Returns a reference to the ComponentStorage</p>
    ///
    /// This method provides direct access to the component storage.
    /// <p style="color:#FF5722;"><strong>Note:</strong> Use with caution, as it bypasses the usual query system.</p>
    ///
    /// # Example
    ///
    /// ```rust
    /// let world = World::new();
    /// let component_storage = world.get_component_storage();
    ///
    /// // Directly insert a component for an entity
    /// component_storage.insert(entity_id, Position { x: 10.0, y: 20.0 });
    /// ```
    pub fn get_component_storage(&self) -> &ComponentStorage {
        &self.component_storage
    }
}


#[allow(dead_code)]
struct WorldDoc;