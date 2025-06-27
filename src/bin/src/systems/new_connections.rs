use bevy_ecs::prelude::{Commands, Res, Resource};
use crossbeam_channel::Receiver;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::conn::keepalive::KeepAliveTracker;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::connection::NewConnection;
use ferrumc_pdc::db::PdcDatabaseResource;
use std::time::SystemTime;
use tracing::{debug, error, trace};

#[derive(Resource)]
pub struct NewConnectionRecv(pub Receiver<NewConnection>);

pub fn accept_new_connections(
    mut cmd: Commands,
    new_connections: Res<NewConnectionRecv>,
    pdc_database: Res<PdcDatabaseResource>,
) {
    if new_connections.0.is_empty() {
        return;
    }
    let pdc_db = &pdc_database.database;
    while let Ok(new_connection) = new_connections.0.try_recv() {
        let return_sender = new_connection.entity_return;
        let pdc = pdc_db
            .load(format!("player:{}", new_connection.player_identity.uuid.as_u128()).as_str())
            .unwrap();

        debug!("Loaded PDC data from database :)");
        let entity = cmd.spawn((
            new_connection.stream,
            Position::default(),
            ChunkReceiver::default(),
            Rotation::default(),
            OnGround::default(),
            pdc,
            new_connection.player_identity,
            KeepAliveTracker {
                last_sent_keep_alive: 0,
                last_received_keep_alive: SystemTime::now(),
                has_received_keep_alive: true,
            },
        ));
        trace!("Spawned entity for new connection: {:?}", entity.id());
        if let Err(err) = return_sender.send(entity.id()) {
            error!(
                "Failed to send entity ID back to the networking thread: {:?}",
                err
            );
        }
    }
}
