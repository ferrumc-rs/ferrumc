use bitcode::{Decode, Encode};
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use std::io::Write;
use tracing::debug;

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

#[derive(NetEncode, Encode, Decode)]
pub struct RegistryEntry {
    pub id: String,
    pub data: PrefixedOptional<Vec<u8>>,
}

impl RegistryDataPacket {
    // TODO: bake this. and make it return just the bytes instead.
    pub fn get_registry_packets() -> Vec<Self> {
        let mut packets = vec![];

        debug!("CWD: {:?}", std::env::current_dir().unwrap());

        let dir_path = std::env::current_dir()
            .unwrap()
            .join(r#"assets\data\registries"#);

        let dir = match std::fs::read_dir(dir_path) {
            Ok(d) => d,
            Err(e) => {
                debug!("Failed to read registry directory: {}", e);
                return packets; // Return empty packets if the directory cannot be read
            }
        };

        for dir_entry in dir {
            let file_path = match dir_entry {
                Ok(f) => f.path(),
                Err(e) => {
                    debug!("Failed to read file in registry directory: {}", e);
                    continue; // Skip files that cannot be read
                }
            };


            let unparsed_data = match std::fs::read(file_path.clone()) {
                Ok(d) => d,
                Err(e) => {
                    debug!("Failed to read registry file {}: {}", file_path.to_str().unwrap(), e);
                    continue; // Skip files that cannot be read
                }
            };

            let extension = file_path.extension().and_then(|s| s.to_str()).unwrap();
            if extension != "bin" {
                continue;
            }

            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let registry_name = "minecraft:".to_string()
                + &*file_name
                .strip_suffix(".bin")
                .unwrap_or(file_name)
                .replace("worldgen", "worldgen/biome");
            let parsed_data: Vec<RegistryEntry> = bitcode::decode(&unparsed_data).unwrap();

            let registry_data_packet = RegistryDataPacket::new(
                registry_name,
                parsed_data,
            );
            packets.push(registry_data_packet);
        }

        packets
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::outgoing::registry_data::RegistryEntry;
    use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::io::Write;

    #[test]
    #[ignore]
    fn generate_nbt() {
        let json_file = include_bytes!("../../../../../../.etc/registry.json");
        let val: HashMap<String, HashMap<String, Value>> = serde_json::from_slice(json_file).unwrap();

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
