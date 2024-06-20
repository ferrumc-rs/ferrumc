use dashmap::DashMap;
use lariv::Lariv;
use lazy_static::lazy_static;
use rand::random;
use std::sync::atomic;
use std::sync::atomic::AtomicU32;
lazy_static!(
    pub static ref CONNECTIONS: ConnectionList = ConnectionList {
        connections: DashMap::new(),
        connection_count: AtomicU32::new(0),
        purge_queue: Lariv::new(1024),
    };
);


pub enum State {
    Unknown,
    Handshake,
    Status,
    Login,
    Play,
}

pub struct ConnectionList {
    // The connections, keyed with random values. The value also contains the connection id for ease of access.
    pub connections: DashMap<u32 ,Connection>,
    // The number of connections.
    pub connection_count: atomic::AtomicU32,
    // The queue of connections to be purged. This is used to store the connections to be dropped at the end of every tick.
    pub purge_queue: Lariv<u32>,
}
pub struct Connection {
    // The connection id.
    pub id: u32,
    // The socket.
    pub socket: tokio::net::TcpStream,
    // The player uuid, if the connection is authenticated.
    pub player_uuid: Option<uuid::Uuid>,
    // A queue of bytes to be sent to the client. Set up this way to allow for easy batching of packets & we can run the sender/receiver in parallel.
    pub send_queue: Lariv<Vec<u8>>,
    // A queue of bytes received from the client. Set up this way to allow for easy batching of packets & we can run the sender/receiver in parallel.
    pub recv_queue: Lariv<u8>,
    // State
    pub state: State,
}

pub async fn handle_connection(mut socket: tokio::net::TcpStream)  {
    let mut id = random();
    // check if we have a collision (1 in 4.2 billion chance) and if so, generate a new id
    while CONNECTIONS.connections.contains_key(&id) {
        id = random();
    }
    let conn = Connection {
        id,
        socket,
        player_uuid: None,
        send_queue: Lariv::new(1024),
        recv_queue: Lariv::new(1024),
        state: State::Unknown,
    };
    CONNECTIONS.connections.insert(id, conn);
    CONNECTIONS.connection_count.fetch_add(1, atomic::Ordering::Relaxed);
}