use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;

pub mod network;

#[derive(Default, Clone, DeepSizeOf, Encode, Decode)]
pub(crate) enum LightStorage {
    #[default]
    Empty,
    Full,
    Mixed {
        light_data: Box<[u8]>,
    },
}

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct SectionLightData {
    sky_light: LightStorage,
    block_light: LightStorage,
}

impl Default for SectionLightData {
    fn default() -> Self {
        Self {
            sky_light: LightStorage::Full,
            block_light: LightStorage::default(),
        }
    }
}

impl From<Vec<i8>> for LightStorage {
    fn from(data: Vec<i8>) -> Self {
        if data.len() != 2048 {
            Self::Empty
        } else {
            let mut all_on = true;
            let mut all_off = true;

            for b in data.iter() {
                if *b != i8::MAX { all_on = false };
                if *b != i8::MIN { all_off = false };
            }

            if all_on { Self::Full }
            else if all_off { Self::Empty }
            else {
                Self::Mixed { light_data: data.into_iter().map(|v| v as u8).collect() }
            }
        }
    }
}

impl SectionLightData {
    pub fn with_data(sky_light: LightStorage, block_light: LightStorage) -> Self {
        Self {
            sky_light,
            block_light,
        }
    }

    #[inline]
    pub fn contains_sky_light(&self) -> bool {
        self.sky_light.contains_light()
    }

    #[inline]
    pub fn contains_block_light(&self) -> bool {
        self.block_light.contains_light()
    }
}

impl LightStorage {
    #[inline]
    pub fn contains_light(&self) -> bool {
        match self {
            LightStorage::Empty => false,
            LightStorage::Full => true,
            LightStorage::Mixed { .. } => true,
        }
    }
}
