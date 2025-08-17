use bitcode::{Decode, Encode};
use ferrumc_macros::{build_registry_packets, packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use lazy_static::lazy_static;

#[derive(NetEncode)]
#[packet(packet_id = "registry_data", state = "configuration")]
pub struct RegistryDataPacket {
    pub registry_id: String,
    pub entries: LengthPrefixedVec<RegistryEntry>,
}

impl RegistryDataPacket {
    pub fn new(registry_id: String, entries: Vec<RegistryEntry>) -> Self {
        Self {
            registry_id,
            entries: LengthPrefixedVec::new(entries),
        }
    }
}

lazy_static! {
    // This is a lazy static to ensure that the registry packets are only built once
    // and can be reused across multiple calls.
    pub static ref REGISTRY_PACKETS: Vec<RegistryDataPacket> = process_reg_packets();
}

fn process_reg_packets() -> Vec<RegistryDataPacket> {
    build_registry_packets!()
        .iter()
        .map(|(key, packets)| {
            let decoded: Vec<(String, Vec<u8>)> = bitcode::decode(packets).unwrap();
            RegistryDataPacket {
                registry_id: key.clone(),
                entries: LengthPrefixedVec::new(
                    decoded
                        .into_iter()
                        .map(|(id, data)| RegistryEntry {
                            id,
                            data: if data.is_empty() {
                                PrefixedOptional::None
                            } else {
                                PrefixedOptional::Some(data)
                            },
                        })
                        .collect(),
                ),
            }
        })
        .collect()
}

#[derive(NetEncode, Encode, Decode)]
pub struct RegistryEntry {
    pub id: String,
    pub data: PrefixedOptional<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use crate::packets::outgoing::registry_data::RegistryEntry;
    use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
    use indexmap::IndexMap;
    use serde_json::Value;
    use std::io::Write;

    #[test]
    #[ignore]
    fn generate_nbt() {
        let json_file = include_bytes!("../../../../../../assets/data/registry_packets.json");
        let val: IndexMap<String, IndexMap<String, Value>> =
            serde_json::from_slice(json_file).unwrap();

        for (key, value_set) in val {
            let mut packets = vec![];
            let key = key
                .replace("minecraft:", "")
                .replace("worldgen/biome", "worldgen");
            for (value_name, value) in &value_set {
                let mut nbt_data_buf = Vec::new();
                craftflow_nbt::to_writer(&mut nbt_data_buf, &value).unwrap();
                let packet = RegistryEntry {
                    id: value_name.clone(),
                    data: if nbt_data_buf.is_empty() {
                        PrefixedOptional::None
                    } else {
                        PrefixedOptional::Some(nbt_data_buf)
                    },
                };
                packets.push(packet);
            }
            let dump_folder = std::env::current_dir()
                .unwrap()
                .join(r#"..\..\..\assets\data\registries"#);
            std::fs::create_dir_all(&dump_folder).unwrap();
            let dump_path = std::env::current_dir()
                .unwrap()
                .join(r#"..\..\..\assets\data\registries"#)
                .join(format!("{key}.bin"));
            let encoded_packets = bitcode::encode(&packets);
            let mut open_file = std::fs::File::create(dump_path).unwrap();
            open_file.write_all(encoded_packets.as_slice()).unwrap();
        }
    }
}
