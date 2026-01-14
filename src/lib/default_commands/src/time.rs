use bevy_ecs::prelude::{Commands, Entity, Query, Res, ResMut, With};
use ferrumc_commands::arg::primitive::int::Integer;
use ferrumc_commands::arg::primitive::string::SingleWord;
use ferrumc_commands::Sender;
use ferrumc_core::time::{LastSentTimeUpdate, WorldTime};
use ferrumc_macros::command;
use ferrumc_text::TextComponent;

#[command("time set")]
fn time_set(
    #[sender] sender: Sender,
    #[arg] time: SingleWord,
    args: (
        ResMut<WorldTime>,
        Query<Entity, With<LastSentTimeUpdate>>,
        Commands,
    ),
) {
    let (mut world_time, query, mut cmds) = args;

    match time.parse::<u16>() {
        Ok(time) => world_time.set_time(time),
        Err(_) => match time.as_str() {
            "day" => world_time.set_time_to_start(WorldTime::DAY),
            "dawn" => world_time.set_time_to_start(WorldTime::DAWN),
            "night" => world_time.set_time_to_start(WorldTime::NIGHT),
            "dusk" => world_time.set_time_to_start(WorldTime::DUSK),

            "noon" | "midday" => world_time.set_time_to_middle(WorldTime::DAY),
            "midnight" => world_time.set_time_to_middle(WorldTime::NIGHT),

            time => {
                sender.send_message(TextComponent::from(format!("Unknown time '{time}'")), false);

                return;
            }
        },
    }

    sender.send_message(
        TextComponent::from(format!(
            "Set the world time to {} ticks",
            world_time.current_time()
        )),
        false,
    );

    for eid in query.iter() {
        cmds.entity(eid).remove::<LastSentTimeUpdate>();
    }
}

type TimeInteger = Integer<0, 24000>;

#[command("time add")]
fn time_add(
    #[sender] sender: Sender,
    #[arg] time: TimeInteger,
    args: (
        ResMut<WorldTime>,
        Query<Entity, With<LastSentTimeUpdate>>,
        Commands,
    ),
) {
    let (mut world_time, query, mut cmds) = args;

    let new_time = world_time.current_time() + *time as u16;
    world_time.set_time(new_time);

    sender.send_message(
        TextComponent::from(format!("Advanced the world time by {} ticks", *time)),
        false,
    );

    for eid in query.iter() {
        cmds.entity(eid).remove::<LastSentTimeUpdate>();
    }
}

#[command("time get")]
fn time_get(#[sender] sender: Sender, world_time: Res<WorldTime>) {
    sender.send_message(
        TextComponent::from(format!(
            "The current world time is: {}",
            world_time.current_time()
        )),
        false,
    );
}
