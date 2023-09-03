#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}