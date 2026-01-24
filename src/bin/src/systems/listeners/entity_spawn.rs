use bevy_ecs::prelude::*;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_entities::bundles::{
    AllayBundle, ArmadilloBundle, AxolotlBundle, BatBundle, BeeBundle, CamelBundle, CatBundle,
    ChickenBundle, CodBundle, CowBundle, DolphinBundle, DonkeyBundle, FoxBundle, FrogBundle,
    GlowSquidBundle, GoatBundle, HorseBundle, LlamaBundle, MooshroomBundle, MuleBundle,
    OcelotBundle, PandaBundle, ParrotBundle, PigBundle, PolarBearBundle, PufferfishBundle,
    RabbitBundle, SalmonBundle, SheepBundle, SnifferBundle, SquidBundle, StriderBundle,
    TadpoleBundle, TropicalFishBundle, TurtleBundle, WolfBundle,
};
use ferrumc_entities::components::EntityMetadata;
use ferrumc_entities::markers::entity_types::{
    Allay, Armadillo, Axolotl, Bat, Bee, Camel, Cat, Chicken, Cod, Cow, Dolphin, Donkey, Fox, Frog,
    GlowSquid, Goat, Horse, Llama, Mooshroom, Mule, Ocelot, Panda, Parrot, Pig, PolarBear,
    Pufferfish, Rabbit, Salmon, Sheep, Sniffer, Squid, Strider, Tadpole, TropicalFish, Turtle, Wolf,
};
use ferrumc_entities::markers::{HasCollisions, HasGravity, HasWaterDrag};
use ferrumc_messages::{EntityType, SpawnEntityCommand, SpawnEntityEvent};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::spawn_entity::SpawnEntityPacket;
use tracing::{error, warn};

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
        match event.entity_type {
            EntityType::Pig => {
                // Spawn the pig entity
                let pig_entity = commands
                    .spawn((
                        PigBundle::new(event.position),
                        Pig,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                // Queue a deferred system to broadcast spawn packets after entity is fully spawned
                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, pig_entity);
                });
            }
            EntityType::Cow => {
                let cow_entity = commands
                    .spawn((
                        CowBundle::new(event.position),
                        Cow,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();
                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, cow_entity);
                });
            }
            EntityType::Allay => {
                // Spawn the allay entity
                let allay_entity = commands
                    .spawn((
                        AllayBundle::new(event.position),
                        Allay,
                        HasCollisions, // Only collisions for now cause it's a flying entity
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, allay_entity);
                });
            }
            EntityType::Armadillo => {
                // Spawn the armadillo entity
                let armadillo_entity = commands
                    .spawn((
                        ArmadilloBundle::new(event.position),
                        Armadillo,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, armadillo_entity);
                });
            }
            EntityType::Axolotl => {
                // Spawn the axolotl entity
                let axolotl_entity = commands
                    .spawn((
                        AxolotlBundle::new(event.position),
                        Axolotl,
                        HasGravity,
                        HasCollisions,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, axolotl_entity);
                });
            }
            EntityType::Bat => {
                // Spawn the bat entity
                let bat_entity = commands
                    .spawn((BatBundle::new(event.position), Bat, HasCollisions))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, bat_entity);
                });
            }
            EntityType::Bee => {
                // Spawn the bee entity (flying insect)
                let bee_entity = commands
                    .spawn((BeeBundle::new(event.position), Bee, HasCollisions))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, bee_entity);
                });
            }
            EntityType::Camel => {
                // Spawn the camel entity
                let camel_entity = commands
                    .spawn((
                        CamelBundle::new(event.position),
                        Camel,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, camel_entity);
                });
            }
            EntityType::Cat => {
                // Spawn the cat entity
                let cat_entity = commands
                    .spawn((
                        CatBundle::new(event.position),
                        Cat,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, cat_entity);
                });
            }
            EntityType::Chicken => {
                // Spawn the chicken entity
                let chicken_entity = commands
                    .spawn((
                        ChickenBundle::new(event.position),
                        Chicken,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, chicken_entity);
                });
            }
            EntityType::Cod => {
                // Spawn the cod entity (fish - no water drag)
                let cod_entity = commands
                    .spawn((CodBundle::new(event.position), Cod, HasCollisions))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, cod_entity);
                });
            }
            EntityType::Dolphin => {
                // Spawn the dolphin entity (water creature - no water drag)
                let dolphin_entity = commands
                    .spawn((DolphinBundle::new(event.position), Dolphin, HasCollisions))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, dolphin_entity);
                });
            }
            EntityType::Donkey => {
                // Spawn the donkey entity
                let donkey_entity = commands
                    .spawn((
                        DonkeyBundle::new(event.position),
                        Donkey,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, donkey_entity);
                });
            }
            EntityType::Fox => {
                // Spawn the fox entity
                let fox_entity = commands
                    .spawn((
                        FoxBundle::new(event.position),
                        Fox,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, fox_entity);
                });
            }
            EntityType::Frog => {
                let frog_entity = commands
                    .spawn((
                        FrogBundle::new(event.position),
                        Frog,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, frog_entity);
                });
            }
            EntityType::Goat => {
                let goat_entity = commands
                    .spawn((
                        GoatBundle::new(event.position),
                        Goat,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, goat_entity);
                });
            }
            EntityType::Horse => {
                let horse_entity = commands
                    .spawn((
                        HorseBundle::new(event.position),
                        Horse,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, horse_entity);
                });
            }
            EntityType::Llama => {
                let llama_entity = commands
                    .spawn((
                        LlamaBundle::new(event.position),
                        Llama,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, llama_entity);
                });
            }
            EntityType::Mooshroom => {
                let mooshroom_entity = commands
                    .spawn((
                        MooshroomBundle::new(event.position),
                        Mooshroom,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, mooshroom_entity);
                });
            }
            EntityType::Ocelot => {
                let ocelot_entity = commands
                    .spawn((
                        OcelotBundle::new(event.position),
                        Ocelot,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, ocelot_entity);
                });
            }
            EntityType::Panda => {
                let panda_entity = commands
                    .spawn((
                        PandaBundle::new(event.position),
                        Panda,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, panda_entity);
                });
            }
            EntityType::Parrot => {
                // Parrot is a flying entity
                let parrot_entity = commands
                    .spawn((ParrotBundle::new(event.position), Parrot, HasCollisions))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, parrot_entity);
                });
            }
            EntityType::PolarBear => {
                let polar_bear_entity = commands
                    .spawn((
                        PolarBearBundle::new(event.position),
                        PolarBear,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, polar_bear_entity);
                });
            }
            EntityType::Pufferfish => {
                // Fish - no water drag, no gravity
                let pufferfish_entity = commands
                    .spawn((PufferfishBundle::new(event.position), Pufferfish, HasCollisions))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, pufferfish_entity);
                });
            }
            EntityType::Rabbit => {
                let rabbit_entity = commands
                    .spawn((
                        RabbitBundle::new(event.position),
                        Rabbit,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, rabbit_entity);
                });
            }
            EntityType::Salmon => {
                // Fish - no water drag, no gravity
                let salmon_entity = commands
                    .spawn((SalmonBundle::new(event.position), Salmon, HasCollisions))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, salmon_entity);
                });
            }
            EntityType::Sheep => {
                let sheep_entity = commands
                    .spawn((
                        SheepBundle::new(event.position),
                        Sheep,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, sheep_entity);
                });
            }
            EntityType::Sniffer => {
                let sniffer_entity = commands
                    .spawn((
                        SnifferBundle::new(event.position),
                        Sniffer,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, sniffer_entity);
                });
            }
            EntityType::Squid => {
                // Water creature - no water drag
                let squid_entity = commands
                    .spawn((SquidBundle::new(event.position), Squid, HasCollisions))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, squid_entity);
                });
            }
            EntityType::Strider => {
                // Lava creature - no water drag
                let strider_entity = commands
                    .spawn((
                        StriderBundle::new(event.position),
                        Strider,
                        HasGravity,
                        HasCollisions,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, strider_entity);
                });
            }
            EntityType::Tadpole => {
                // Water creature - no water drag
                let tadpole_entity = commands
                    .spawn((TadpoleBundle::new(event.position), Tadpole, HasCollisions))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, tadpole_entity);
                });
            }
            EntityType::TropicalFish => {
                // Fish - no water drag
                let tropical_fish_entity = commands
                    .spawn((
                        TropicalFishBundle::new(event.position),
                        TropicalFish,
                        HasCollisions,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, tropical_fish_entity);
                });
            }
            EntityType::Turtle => {
                let turtle_entity = commands
                    .spawn((
                        TurtleBundle::new(event.position),
                        Turtle,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, turtle_entity);
                });
            }
            EntityType::Wolf => {
                let wolf_entity = commands
                    .spawn((
                        WolfBundle::new(event.position),
                        Wolf,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, wolf_entity);
                });
            }
            EntityType::GlowSquid => {
                // Water creature - no water drag
                let glow_squid_entity = commands
                    .spawn((GlowSquidBundle::new(event.position), GlowSquid, HasCollisions))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, glow_squid_entity);
                });
            }
            EntityType::Mule => {
                let mule_entity = commands
                    .spawn((
                        MuleBundle::new(event.position),
                        Mule,
                        HasGravity,
                        HasCollisions,
                        HasWaterDrag,
                    ))
                    .id();

                commands.queue(move |world: &mut World| {
                    broadcast_entity_spawn(world, mule_entity);
                });
            }
        }
    }
}
