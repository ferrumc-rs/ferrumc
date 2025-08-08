//! System message queuing.

use std::sync::LazyLock;

use bevy_ecs::entity::Entity;
use crossbeam_queue::SegQueue;
use ferrumc_text::TextComponent;

#[doc(hidden)]
pub struct QueueEntry {
    pub message: TextComponent,
    pub receiver: Entity,
    pub overlay: bool,
}

#[doc(hidden)]
pub static QUEUE: LazyLock<crossbeam_queue::SegQueue<QueueEntry>> = LazyLock::new(SegQueue::new);

/// Queues a `message` to the given `receiver`.
pub fn queue(message: TextComponent, actionbar: bool, receiver: Entity) {
    QUEUE.push(QueueEntry {
        message,
        receiver,
        overlay: actionbar,
    });
}
