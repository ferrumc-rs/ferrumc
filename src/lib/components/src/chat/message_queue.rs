//! System message queuing.

use std::sync::LazyLock;

use bevy_ecs::entity::Entity;
use crossbeam_queue::SegQueue;
use ferrumc_text::TextComponent;

#[doc(hidden)]
#[derive(Clone)]
pub struct QueueEntry {
    pub message: TextComponent,
    /// None if broadcasting
    pub receiver: Option<Entity>,
    pub overlay: bool,
}

#[doc(hidden)]
pub static QUEUE: LazyLock<crossbeam_queue::SegQueue<QueueEntry>> = LazyLock::new(SegQueue::new);

/// Queues a `message` to be sent to the given `receiver`.
pub fn queue(message: TextComponent, actionbar: bool, receiver: Entity) {
    QUEUE.push(QueueEntry {
        message,
        receiver: Some(receiver),
        overlay: actionbar,
    });
}

/// Queues a `message` to be broadcasted to the entire server.
pub fn broadcast(message: TextComponent, actionbar: bool) {
    QUEUE.push(QueueEntry {
        message,
        receiver: None,
        overlay: actionbar,
    });
}
