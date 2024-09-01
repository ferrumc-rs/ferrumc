use std::time::Instant;

use ferrumc_codec::enc::NetEncode;
use ferrumc_codec::network_types::varint::VarInt;
#[cfg(not(test))]
use include_flate::flate;
use rand::random;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLockWriteGuard;
use tracing::{debug, info};
use uuid::Uuid;

use ferrumc_macros::{packet, NetDecode};

use crate::net::packets::outgoing::default_spawn_position::DefaultSpawnPosition;
use crate::net::packets::outgoing::keep_alive::KeepAlivePacketOut;
use crate::net::packets::outgoing::login_success::LoginSuccess;
use crate::net::packets::outgoing::synchronize_player_position::SynchronizePlayerPosition;
use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::net::State::Play;
use crate::state::GlobalState;
use crate::utils::components::keep_alive::KeepAlive;
use crate::utils::components::player::Player;
use crate::utils::components::rotation::Rotation;
use crate::utils::constants::init;
use crate::utils::encoding::position::Position;
use crate::utils::prelude::*;
use crate::Connection;
use crate::net::packets::outgoing::login_plugin_request::LoginPluginRequest;
use crate::net::systems::chunk_sender::ChunkSender;

/// The login start packet is sent by the client to the server to start the login process.
///
/// Server responds with [crate::net::packets::outgoing::login_success::LoginSuccess],
/// [crate::net::packets::outgoing::login_play::LoginPlay], and
/// [crate::net::packets::outgoing::default_spawn_position::DefaultSpawnPosition] packets in that order.
/// No response is required from the client while these are being sent.
///
/// This is the final stage in the login process. The client is now in the play state.
#[derive(NetDecode)]
#[packet(packet_id = 0x00, state = "login")]
pub struct LoginStart {
    pub username: String,
    pub uuid: u128,
}

// MAKE SURE YOU RUN THE TEST IN THE login_play.rs FILE TO GENERATE THE NBT FILE
// The NBT encoded data for the dimension codec. Using flate_include cos the codec file is like 40kb
#[cfg(not(test))]
flate!(pub static NBT_CODEC: [u8] from "./.etc/nbt_codec.nbt");

#[cfg(test)]
const NBT_CODEC: &[u8] = &[0u8; 1];

impl IncomingPacket for LoginStart {
    async fn handle(mut self, conn_id: ConnectionId, state: GlobalState) -> Result<()> {
        self.username = self.username.trim().to_string();

        let conn = state.connections.get_connection(conn_id)?;
        let mut conn = conn.write().await;


        self.send_login_success(&mut conn).await?;
        self.send_login_play(&mut conn).await?;
        self.send_spawn_position(&mut conn).await?;

        let data: i64 = random();
        let mut keep_alive = KeepAlive::new(Instant::now(), Instant::now(), data);
        self.send_keep_alive(&mut conn, &mut keep_alive).await?;
        self.update_world_state(&conn, keep_alive, state.clone())
            .await?;

        self.synchronize_player_position(state.clone(), &mut conn)
            .await?;


        let packet = LoginPluginRequest::server_brand("🦀".repeat(100)).await;
        conn.send_packet(packet).await?;

        info!("Player {} has joined the server", self.username);


        conn.state = Play;

        let entity = conn.id;
        // Drop connection to avoid deadlock with chunk sender since it also needs to write to the connection
        drop(conn);

        ChunkSender::send_chunks_to_player(state.clone(), entity).await?;



        Ok(())
    }
}

impl LoginStart {
    async fn send_login_success(&self, conn: &mut RwLockWriteGuard<'_, Connection>) -> Result<()> {
        debug!("LoginStart packet received");
        debug!("Username: {}", self.username);
        let uuid = Uuid::from_u128(self.uuid);
        debug!("UUID: {uuid}");

        let namespace_uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, "OfflinePlayer".as_bytes());
        let uuid = Uuid::new_v3(&namespace_uuid, self.username.as_bytes());

        let response = LoginSuccess::new_auto(
            uuid.as_bytes().into(),
            "OfflinePlayer".to_string(),
            VarInt::new(0),
            vec![],
        );

        let mut cursor = std::io::Cursor::new(Vec::new());
        response.net_encode(&mut cursor).await?;
        let response = cursor.into_inner();

        conn.socket.write_all(&*response).await?;
        Ok(())
    }

    async fn send_login_play(&self, conn: &mut RwLockWriteGuard<'_, Connection>) -> Result<()> {
        let play_packet = crate::net::packets::outgoing::login_play::LoginPlay {
            packet_id: VarInt::from(0x28),
            entity_id: 0,
            hardcore: false,
            gamemode: 1,
            previous_gamemode: -1,
            dimension_length: VarInt::new(1),
            dimension_names: vec!["minecraft:overworld".to_string()],
            registry_codec: NBT_CODEC.to_vec(),
            dimension_type: "minecraft:overworld".to_string(),
            dimension_name: "minecraft:overworld".to_string(),
            seed_hash: 0,
            max_players: VarInt::new(20),
            view_distance: VarInt::new(10),
            simulation_distance: VarInt::new(10),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            is_debug: false,
            is_flat: false,
            has_death_location: false,
            portal_cooldown: VarInt::new(0),
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        play_packet.net_encode(&mut cursor).await?;
        let play_packet = cursor.into_inner();

        conn.socket.write_all(&*play_packet).await?;
        Ok(())
    }

    async fn send_spawn_position(&self, conn: &mut RwLockWriteGuard<'_, Connection>) -> Result<()> {
        let player_position = Position {
            x: init::DEFAULT_SPAWN_X_POS,
            y: init::DEFAULT_SPAWN_Y_POS,
            z: init::DEFAULT_SPAWN_Z_POS,
        };
        let spawn_position = DefaultSpawnPosition::new_auto(player_position.clone(), 0.0);
        conn.send_packet(spawn_position).await?;
        Ok(())
    }

    async fn send_keep_alive(
        &self,
        conn: &mut RwLockWriteGuard<'_, Connection>,
        keep_alive: &mut KeepAlive,
    ) -> Result<()> {
        let keep_alive_outgoing: KeepAlivePacketOut = keep_alive.into();
        debug!("Sending keep alive packet {:?}", keep_alive.data);
        conn.send_packet(keep_alive_outgoing).await?;
        Ok(())
    }

    async fn update_world_state(
        &self,
        conn: &RwLockWriteGuard<'_, Connection>,
        keep_alive: KeepAlive,
        state: GlobalState,
    ) -> Result<()> {
        let entity = conn.id;

        let component_storage = state.world.get_component_storage();

        component_storage
            .insert(
                entity,
                Position::new(
                    init::DEFAULT_SPAWN_X_POS,
                    init::DEFAULT_SPAWN_Y_POS,
                    init::DEFAULT_SPAWN_Z_POS,
                ),
            )
            .insert(
                entity,
                Rotation::new(init::DEFAULT_SPAWN_YAW, init::DEFAULT_SPAWN_PITCH),
            )
            .insert(entity, keep_alive)
            .insert(entity, Player::new(self.uuid, self.username.clone()));

        Ok(())
    }
    async fn synchronize_player_position(
        &self,
        state: GlobalState,
        conn: &mut RwLockWriteGuard<'_, Connection>,
    ) -> Result<()> {
        let entity = conn.id;
        let component_storage = state.world.get_component_storage();

        let position = component_storage.get::<Position>(entity).await?;
        let rotation = component_storage.get::<Rotation>(entity).await?;

        let packet = SynchronizePlayerPosition::new(&*position, &*rotation);

        conn.send_packet(packet).await?;

        Ok(())
    }
}
