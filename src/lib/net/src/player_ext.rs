// use tracing::debug;

// use crate::{
//     connection::StreamWriter, packets::outgoing::disconnect::Disconnect, GlobalState, NetResult,
// };

// pub trait PlayerExt {
//     async fn disconnect_player(
//         &self,
//         conn_id: usize,
//         state: &GlobalState,
//         reason: String,
//     ) -> NetResult<()>;
// }

// // Seems like theres something wrong with the remove_all_components function for now, uncomment and modify this when the ECS is reworked a bit

// impl PlayerExt for StreamWriter {
//     async fn disconnect_player(
//         &self,
//         conn_id: usize,
//         state: &GlobalState,
//         reason: String,
//     ) -> NetResult<()> {
//         debug!("KICKING PLAYER");
//         let mut writer = state.universe.get_mut::<StreamWriter>(conn_id)?;
//         let packet = Disconnect::from_string(reason);
//         match writer
//             .send_packet(
//                 &packet,
//                 &ferrumc_net_codec::encode::NetEncodeOpts::WithLength,
//             )
//             .await
//         {
//             Ok(_) => {
//                 state.universe.remove_all_components(conn_id)?;
//                 debug!("Kicked player with entity ID : {:?}", conn_id);
//                 Ok(())
//             }
//             Err(err) => {
//                 debug!("Failed to kick player with entity ID : {:?}", conn_id);
//                 Err(err)
//             }
//         }
//     }
// }
