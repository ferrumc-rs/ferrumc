use bevy_ecs::message::MessageReader;
use ferrumc_messages::BlockBrokenEvent;

pub fn handle(
    mut events: MessageReader<BlockBrokenEvent>,
) {

}