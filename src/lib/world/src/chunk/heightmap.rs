use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use crate::errors::WorldError;
use crate::vanilla_chunk_format::VanillaHeightmaps;

#[derive(Default, Clone, DeepSizeOf, Encode, Decode)]
pub struct Heightmaps {
    pub world_surface: ChunkHeightmap,
    pub motion_blocking: ChunkHeightmap,
}

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct ChunkHeightmap {
    data: Box<[i16]>,
}

impl Default for ChunkHeightmap {
    fn default() -> Self {
        Self {
            data: vec![0; 256].into_boxed_slice(),
        }
    }
}

impl ChunkHeightmap {
    pub fn set_height(&mut self, x: u8, z: u8, height: i16) {
        self.data[((z << 4) | x) as usize] = height;
    }

    pub fn get_height(&self, x: u8, z: u8) -> i16 {
        self.data[((z << 4) | x) as usize]
    }
}

impl TryFrom<&VanillaHeightmaps> for Heightmaps {
    type Error = WorldError;

    fn try_from(value: &VanillaHeightmaps) -> Result<Self, Self::Error> {
        let convert_long_vec = |data: Vec<i64>| {
            ChunkHeightmap {
                data: data.into_iter().map(|v| v as i16).collect(),
            }
        };

        Ok(Self {
            world_surface: value.world_surface.clone().map(convert_long_vec).unwrap_or_default(),
            motion_blocking: value.motion_blocking.clone().map(convert_long_vec).unwrap_or_default(),
        })
    }
}

#[derive(NetEncode)]
pub struct NetworkHeightmap {
    heightmap: VarInt,
    data: LengthPrefixedVec<u64>,
}

impl Heightmaps {
    pub fn get_network_repr(
        heightmaps: &Option<Heightmaps>,
    ) -> LengthPrefixedVec<NetworkHeightmap> {
        const BITS_PER_ENTRY: usize = 9;
        const ENTRIES_PER_LONG: usize = 64 / 9;
        const NUMBER_OF_ENTRIES: usize = 16 * 16;
        const NUMBER_OF_LONGS: usize =
            NUMBER_OF_ENTRIES + (ENTRIES_PER_LONG - 1) / ENTRIES_PER_LONG;

        let mut world_surface = vec![0u64; NUMBER_OF_LONGS];
        let mut motion_blocking = vec![0u64; NUMBER_OF_LONGS];

        if let Some(heightmaps) = heightmaps.as_ref() {
            for (i, (&world_surface_val, &motion_blocking_val)) in heightmaps
                .world_surface
                .data
                .iter()
                .zip(heightmaps.motion_blocking.data.iter())
                .enumerate()
            {
                let entry_mask = (1u64 << BITS_PER_ENTRY) - 1;
                let long_index = i / ENTRIES_PER_LONG;
                let bit_index = i % ENTRIES_PER_LONG * BITS_PER_ENTRY;

                world_surface[long_index] &= !(entry_mask << bit_index);
                world_surface[long_index] |= (world_surface_val as u64) << bit_index;

                motion_blocking[long_index] &= !(entry_mask << bit_index);
                motion_blocking[long_index] |= (motion_blocking_val as u64) << bit_index;
            }
        }

        let heightmaps = vec![
            NetworkHeightmap {
                heightmap: VarInt(1),
                data: LengthPrefixedVec::new(world_surface),
            },
            NetworkHeightmap {
                heightmap: VarInt(4),
                data: LengthPrefixedVec::new(motion_blocking),
            },
        ];

        LengthPrefixedVec::new(heightmaps)
    }
}
