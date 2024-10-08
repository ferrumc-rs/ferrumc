use ferrumc_net_types::var_int::VarInt;
use std::io::Cursor;

pub mod errors;

pub type NetPacketResult<T> = Result<T, errors::NetPacketError>;

pub struct PacketSkeleton<'a> {
    pub length: VarInt,
    pub packet_id: u8,
    pub payload: &'a [u8],
}
pub fn read_packet_headers<'a>(
    _reader: &mut Cursor<&'a [u8]>,
) -> NetPacketResult<PacketSkeleton<'a>> {
    unimplemented!()
}
