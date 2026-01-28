use bevy_ecs::prelude::*;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_entities::bundles::*;
use ferrumc_entities::components::EntityMetadata;
use ferrumc_entities::markers::entity_types::*;
use ferrumc_entities::markers::{HasCollisions, HasGravity, HasWaterDrag};
use ferrumc_messages::{EntityType, SpawnEntityCommand, SpawnEntityEvent};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::spawn_entity::SpawnEntityPacket;
use tracing::{error, warn};

/// Macro for spawning ground entities (gravity + collisions + water drag)
macro_rules! spawn_ground_entity {
    ($commands:expr, $position:expr, $Bundle:ident, $Marker:ident) => {{
        let entity = $commands
            .spawn((
                $Bundle::new($position),
                $Marker,
                HasGravity,
                HasCollisions,
                HasWaterDrag,
            ))
            .id();
        $commands.queue(move |world: &mut World| {
            broadcast_entity_spawn(world, entity);
        });
    }};
}

/// Macro for spawning flying/swimming entities (collisions only)
macro_rules! spawn_flying_entity {
    ($commands:expr, $position:expr, $Bundle:ident, $Marker:ident) => {{
        let entity = $commands
            .spawn(($Bundle::new($position), $Marker, HasCollisions))
            .id();
        $commands.queue(move |world: &mut World| {
            broadcast_entity_spawn(world, entity);
        });
    }};
}

/// Macro for spawning entities with gravity but no water drag (lava/amphibian creatures)
macro_rules! spawn_gravity_entity {
    ($commands:expr, $position:expr, $Bundle:ident, $Marker:ident) => {{
        let entity = $commands
            .spawn(($Bundle::new($position), $Marker, HasGravity, HasCollisions))
            .id();
        $commands.queue(move |world: &mut World| {
            broadcast_entity_spawn(world, entity);
        });
    }};
}

/// Helper function to broadcast entity spawn packets to all connected players.
///
/// This function queries the entity's components and sends the spawn packet
/// to all players. It's generic and works for any entity type.
///
/// # Arguments
///
/// * `world` - The Bevy world
/// * `entity` - The entity to broadcast
fn broadcast_entity_spawn(world: &mut World, entity: Entity) {
    // Get entity components
    let metadata = match world.get::<EntityMetadata>(entity) {
        Some(m) => m,
        None => {
            error!("Failed to get entity metadata for {:?}", entity);
            return;
        }
    };
    let protocol_id = metadata.protocol_id();

    let identity = match world.get::<EntityIdentity>(entity) {
        Some(i) => i,
        None => {
            error!("Failed to get entity identity for {:?}", entity);
            return;
        }
    };

    let position = match world.get::<Position>(entity) {
        Some(p) => p,
        None => {
            error!("Failed to get entity position for {:?}", entity);
            return;
        }
    };

    let rotation = match world.get::<Rotation>(entity) {
        Some(r) => r,
        None => {
            error!("Failed to get entity rotation for {:?}", entity);
            return;
        }
    };

    // Create spawn packet
    let spawn_packet = SpawnEntityPacket::new(
        identity.entity_id,
        identity.uuid.as_u128(),
        protocol_id as i32,
        position,
        rotation,
    );

    // Broadcast to all connected players
    let mut writer_query = world.query::<&StreamWriter>();
    for writer in writer_query.iter(world) {
        if let Err(e) = writer.send_packet_ref(&spawn_packet) {
            error!("Failed to send spawn packet: {:?}", e);
        }
    }
}

/// System that processes spawn commands from messages
pub fn spawn_command_processor(
    mut spawn_commands: MessageReader<SpawnEntityCommand>,
    query: Query<(&Position, &Rotation)>,
    mut spawn_events: MessageWriter<SpawnEntityEvent>,
) {
    // Process all spawn command messages
    for command in spawn_commands.read() {
        // Get player position and rotation
        if let Ok((pos, rot)) = query.get(command.player_entity) {
            // Calculate spawn position 2 blocks in front of the player
            let spawn_pos = pos.offset_forward(rot, 2.0);

            spawn_events.write(SpawnEntityEvent {
                entity_type: command.entity_type,
                position: spawn_pos,
            });
        } else {
            warn!(
                "Failed to get position for entity {:?}",
                command.player_entity
            );
        }
    }
}

/// System that listens for `SpawnEntityEvent` and spawns the entity,
/// then broadcasts the spawn packet.
pub fn handle_spawn_entity(mut events: MessageReader<SpawnEntityEvent>, mut commands: Commands) {
    for event in events.read() {
        let pos = event.position;
        match event.entity_type {
            // Ground entities (gravity + collisions + water drag)
            EntityType::Pig => spawn_ground_entity!(commands, pos, PigBundle, Pig),
            EntityType::Cow => spawn_ground_entity!(commands, pos, CowBundle, Cow),
            EntityType::Armadillo => {
                spawn_ground_entity!(commands, pos, ArmadilloBundle, Armadillo)
            }
            EntityType::Camel => spawn_ground_entity!(commands, pos, CamelBundle, Camel),
            EntityType::Cat => spawn_ground_entity!(commands, pos, CatBundle, Cat),
            EntityType::Chicken => spawn_ground_entity!(commands, pos, ChickenBundle, Chicken),
            EntityType::Donkey => spawn_ground_entity!(commands, pos, DonkeyBundle, Donkey),
            EntityType::Fox => spawn_ground_entity!(commands, pos, FoxBundle, Fox),
            EntityType::Frog => spawn_ground_entity!(commands, pos, FrogBundle, Frog),
            EntityType::Goat => spawn_ground_entity!(commands, pos, GoatBundle, Goat),
            EntityType::Horse => spawn_ground_entity!(commands, pos, HorseBundle, Horse),
            EntityType::Llama => spawn_ground_entity!(commands, pos, LlamaBundle, Llama),
            EntityType::Mooshroom => {
                spawn_ground_entity!(commands, pos, MooshroomBundle, Mooshroom)
            }
            EntityType::Mule => spawn_ground_entity!(commands, pos, MuleBundle, Mule),
            EntityType::Ocelot => spawn_ground_entity!(commands, pos, OcelotBundle, Ocelot),
            EntityType::Panda => spawn_ground_entity!(commands, pos, PandaBundle, Panda),
            EntityType::PolarBear => {
                spawn_ground_entity!(commands, pos, PolarBearBundle, PolarBear)
            }
            EntityType::Rabbit => spawn_ground_entity!(commands, pos, RabbitBundle, Rabbit),
            EntityType::Sheep => spawn_ground_entity!(commands, pos, SheepBundle, Sheep),
            EntityType::SkeletonHorse => {
                spawn_ground_entity!(commands, pos, SkeletonHorseBundle, SkeletonHorse)
            }
            EntityType::Sniffer => spawn_ground_entity!(commands, pos, SnifferBundle, Sniffer),
            EntityType::SnowGolem => {
                spawn_ground_entity!(commands, pos, SnowGolemBundle, SnowGolem)
            }
            EntityType::Turtle => spawn_ground_entity!(commands, pos, TurtleBundle, Turtle),
            EntityType::Villager => spawn_ground_entity!(commands, pos, VillagerBundle, Villager),
            EntityType::WanderingTrader => {
                spawn_ground_entity!(commands, pos, WanderingTraderBundle, WanderingTrader)
            }
            EntityType::Wolf => spawn_ground_entity!(commands, pos, WolfBundle, Wolf),
            EntityType::ZombieHorse => {
                spawn_ground_entity!(commands, pos, ZombieHorseBundle, ZombieHorse)
            }

            // Flying entities (collisions only)
            EntityType::Allay => spawn_flying_entity!(commands, pos, AllayBundle, Allay),
            EntityType::Bat => spawn_flying_entity!(commands, pos, BatBundle, Bat),
            EntityType::Bee => spawn_flying_entity!(commands, pos, BeeBundle, Bee),
            EntityType::Parrot => spawn_flying_entity!(commands, pos, ParrotBundle, Parrot),

            // Water creatures (collisions only, no gravity/water drag)
            EntityType::Cod => spawn_flying_entity!(commands, pos, CodBundle, Cod),
            EntityType::Dolphin => spawn_flying_entity!(commands, pos, DolphinBundle, Dolphin),
            EntityType::GlowSquid => {
                spawn_flying_entity!(commands, pos, GlowSquidBundle, GlowSquid)
            }
            EntityType::Pufferfish => {
                spawn_flying_entity!(commands, pos, PufferfishBundle, Pufferfish)
            }
            EntityType::Salmon => spawn_flying_entity!(commands, pos, SalmonBundle, Salmon),
            EntityType::Squid => spawn_flying_entity!(commands, pos, SquidBundle, Squid),
            EntityType::Tadpole => spawn_flying_entity!(commands, pos, TadpoleBundle, Tadpole),
            EntityType::TropicalFish => {
                spawn_flying_entity!(commands, pos, TropicalFishBundle, TropicalFish)
            }

            // Special: gravity but no water drag (amphibians, lava creatures)
            EntityType::Axolotl => spawn_gravity_entity!(commands, pos, AxolotlBundle, Axolotl),
            EntityType::Strider => spawn_gravity_entity!(commands, pos, StriderBundle, Strider),
        }
    }
}
