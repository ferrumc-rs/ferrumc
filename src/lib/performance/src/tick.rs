use std::collections::VecDeque;

#[derive(Debug)]
pub struct TickData {
    pub start_ns: u128,
    pub duration_ns: u128,
    pub entity_count: u32,
    pub ran_count: usize,
}

pub(crate) struct TickHistory {
    buffer: VecDeque<TickData>,
    capacity: usize,
}

impl TickHistory {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub(crate) fn record(&mut self, data: TickData) {
        if self.buffer.len().eq(&self.capacity) {
            self.buffer.pop_front();
        }
        self.buffer.push_back(data);
    }

    /// Iterate newest → oldest
    pub(crate) fn iter_rev(&self) -> impl Iterator<Item = &TickData> {
        self.buffer.iter().rev()
    }
}
