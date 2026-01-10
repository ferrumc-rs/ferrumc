use crate::errors::NetError;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_world::chunk::light::network::NetworkLightData;
use ferrumc_world::chunk::network::NetworkChunk;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkPos;

#[derive(NetEncode)]
pub struct BlockEntity {
    pub xz: u8,
    pub y: u16,
    pub entity_type: VarInt,
    pub nbt: Vec<u8>,
}

#[derive(NetEncode)]
pub struct NetHeightmap {
    // Define the structure of your heightmaps here
    pub id: VarInt,
    pub data: LengthPrefixedVec<i64>,
}

#[derive(NetEncode)]
#[packet(packet_id = "level_chunk_with_light", state = "play")]
pub struct ChunkAndLightData<'chunk> {
    pub chunk_x: i32,
    pub chunk_z: i32,
    // The binary nbt data
    pub chunk_data: NetworkChunk,
    pub block_entities: LengthPrefixedVec<BlockEntity>,
    pub light_data: NetworkLightData<'chunk>,
}

impl<'chunk> ChunkAndLightData<'chunk> {
    pub fn from_chunk(pos: ChunkPos, chunk: &'chunk Chunk) -> Result<Self, NetError> {
        Ok(ChunkAndLightData::<'chunk> {
            chunk_x: pos.x(),
            chunk_z: pos.z(),
            chunk_data: NetworkChunk::try_from(chunk)?,
            block_entities: LengthPrefixedVec::new(Vec::new()),
            light_data: NetworkLightData::from(chunk),
        })
    }
}
