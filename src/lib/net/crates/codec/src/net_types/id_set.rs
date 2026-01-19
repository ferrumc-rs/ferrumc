use ferrumc_net_codec::net_types::var_int::VarInt;

pub struct IDSet {
    pub id_type: VarInt,
    pub tag_name: Option<String>,
    pub ids: Option<Vec<VarInt>>,
}
