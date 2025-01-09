use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::{
    encode::{NetEncode, NetEncodeOpts, NetEncodeResult},
    net_types::var_int::VarInt,
};
use std::io::Write;

use crate::slot::NetworkSlot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Boots,
    Leggings,
    Chestplate,
    Helmet,
    Body,
}

impl NetEncode for EquipmentSlot {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        self.get_index().encode(writer, opts)
    }

    async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        self.get_index().encode_async(writer, opts).await
    }
}

impl EquipmentSlot {
    pub fn get_index(&self) -> u8 {
        match self {
            Self::MainHand => 0,
            Self::OffHand => 1,
            Self::Boots => 2,
            Self::Leggings => 3,
            Self::Chestplate => 4,
            Self::Helmet => 5,
            Self::Body => 6,
        }
    }
}

#[derive(NetEncode)]
pub struct Equipment {
    pub equipment: EquipmentSlot,
    pub slot: NetworkSlot,
}

impl Equipment {
    pub fn new(equipment: EquipmentSlot, slot: NetworkSlot) -> Self {
        Self { equipment, slot }
    }
}

#[derive(NetEncode)]
#[packet(packet_id = "set_equipment", state_id = "play")]
pub struct SetEquipmentPacket {
    pub conn_id: VarInt,
    pub equipment: Vec<Equipment>,
}

impl SetEquipmentPacket {
    pub fn new(conn_id: usize, equipment: Vec<Equipment>) -> Self {
        Self {
            conn_id: VarInt::new(conn_id as i32),
            equipment,
        }
    }
}
