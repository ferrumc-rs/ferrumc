use deepsize::DeepSizeOf;
use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Clone, DeepSizeOf)]
pub struct Heightmaps {
    world_surface: ChunkHeightmap,
    motion_blocking: ChunkHeightmap,
}

#[derive(Clone, DeepSizeOf)]
pub struct ChunkHeightmap {
    data: Box<[i16]>,
}

impl ChunkHeightmap {
    const BITS_PER_ENTRY: u8 = 9;
    const ENTRIES: usize = 16 * 16;

    pub fn new() -> Self {
        Self {
            data: Box::new([0; 256]),
        }
    }

    pub fn set_height(&mut self, x: u8, z: u8, height: i16) {
        self.data[((z << 4) | x) as usize] = height;
    }

    pub fn get_height(&self, x: u8, z: u8) -> i16 {
        self.data[((z << 4) | x) as usize]
    }
}

#[derive(NetEncode)]
pub struct NetworkHeightmap {
    heightmap: VarInt,
    data: LengthPrefixedVec<u64>
}

impl Heightmaps {
    pub fn new() -> Self {
        Self {
            world_surface: ChunkHeightmap::new(),
            motion_blocking: ChunkHeightmap::new(),
        }
    }

    pub fn as_network(&self) -> LengthPrefixedVec<NetworkHeightmap> {
        let mut heightmaps = Vec::with_capacity(2);

        heightmaps.push(NetworkHeightmap {
            heightmap: VarInt(1),
            data: LengthPrefixedVec::new(vec![0; 37]),
        });

        heightmaps.push(NetworkHeightmap {
            heightmap: VarInt(4),
            data: LengthPrefixedVec::new(vec![0; 37]),
        });

        LengthPrefixedVec::new(heightmaps)
    }
}