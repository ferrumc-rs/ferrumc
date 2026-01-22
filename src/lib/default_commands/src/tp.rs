#![expect(clippy::type_complexity)]
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{MessageWriter, Query};
use ferrumc_commands::arg::entities::EntityArgument;
use ferrumc_commands::arg::position::CommandPosition;
use ferrumc_commands::Sender;
use ferrumc_commands::Sender::Player;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::command;
use ferrumc_messages::teleport_player::TeleportPlayer;

#[command("tp pos")]
fn tp_command(
    #[sender] sender: Sender,
    #[arg] pos: CommandPosition,
    args: (Query<(&Rotation, &Position)>, MessageWriter<TeleportPlayer>),
) {
    let (mut query, mut tp_player_msg) = args;
    let Player(entity) = sender else {
        sender.send_message("This command can only be used by players.".into(), false);
        return;
    };

    let Ok((rot, position)) = query.get_mut(entity) else {
        sender.send_message("Could not find your player entity.".into(), false);
        return;
    };
    let resolved_pos = pos.resolve(position);

    tp_player_msg.write(TeleportPlayer {
        entity,
        x: resolved_pos.x,
        y: resolved_pos.y,
        z: resolved_pos.z,
        vel_x: 0.0,
        vel_y: 0.0,
        vel_z: 0.0,
        yaw: rot.yaw,
        pitch: rot.pitch,
    });

    sender.send_message(format!("Teleported to ({}).", resolved_pos).into(), false);
}

#[command("tp entity")]
fn tp_to_command(
    #[sender] sender: Sender,
    #[arg] target: EntityArgument,
    args: (
        Query<(&Rotation, &Position)>,
        MessageWriter<TeleportPlayer>,
        Query<(Entity, Option<&EntityIdentity>, Option<&PlayerIdentity>)>,
    ),
) {
    let (query, mut tp_player_msg, resolve_q) = args;

    let resolved_targets = target.resolve(resolve_q.iter());

    if resolved_targets.len() != 1 {
        sender.send_message(
            "You must specify exactly one target to teleport to.".into(),
            false,
        );
        return;
    } else if resolved_targets.is_empty() {
        sender.send_message("No valid target found to teleport to.".into(), false);
        return;
    } else if matches!(sender, Sender::Server) {
        sender.send_message("This command can only be used by players.".into(), false);
        return;
    }

    let target_entity = resolved_targets.first().expect("Checked above; qed");

    let sender_e = match sender {
        Player(e) => e,
        _ => {
            sender.send_message("This command can only be used by players.".into(), false);
            return;
        }
    };

    let Ok([(sender_rot, _), (_, target_pos)]) = query.get_many([sender_e, *target_entity]) else {
        sender.send_message("Could not find player entities.".into(), false);
        return;
    };

    tp_player_msg.write(TeleportPlayer {
        entity: sender_e,
        x: target_pos.x,
        y: target_pos.y,
        z: target_pos.z,
        vel_x: 0.0,
        vel_y: 0.0,
        vel_z: 0.0,
        yaw: sender_rot.yaw,
        pitch: sender_rot.pitch,
    });

    sender.send_message(
        format!("Teleported to the entity at {}.", target_pos).into(),
        false,
    );
}
