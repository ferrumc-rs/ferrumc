use ferrumc_macros::{Component, Constructor};

#[derive(Component, Constructor, Debug, Clone)]
pub struct KeepAlive {
    pub last_received: std::time::Instant,
    pub last_sent: std::time::Instant,
}