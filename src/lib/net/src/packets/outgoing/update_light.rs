use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_world::chunk::light::network::NetworkLightData;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkPos;

#[derive(NetEncode)]
#[packet(packet_id = "light_update", state = "play")]
pub struct UpdateLightPacket<'chunk> {
    pub chunk_x: VarInt,
    pub chunk_z: VarInt,
    pub light_data: NetworkLightData<'chunk>,
}

impl<'chunk> UpdateLightPacket<'chunk> {
    pub fn from_chunk(pos: ChunkPos, chunk: &'chunk Chunk) -> Self {
        Self {
            chunk_x: VarInt::new(pos.x()),
            chunk_z: VarInt::new(pos.z()),
            light_data: NetworkLightData::from(chunk),
        }
    }
}
