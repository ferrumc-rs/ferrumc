use ferrumc_macros::{packet, NetEncode};
use ferrumc_nbt::NbtTape;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use std::io::{Read, Write};
use tracing::debug;

#[derive(NetEncode)]
#[packet(packet_id = "registry_data", state = "configuration")]
pub struct RegistryDataPacket<'a> {
    pub registry_id: &'a str,
    pub entries: LengthPrefixedVec<RegistryEntry<'a>>,
}

impl<'a> RegistryDataPacket<'a> {
    pub fn new(registry_id: &'a str, entries: Vec<RegistryEntry<'a>>) -> Self {
        Self {
            registry_id,
            entries: LengthPrefixedVec::new(entries),
        }
    }
}

#[derive(NetEncode)]
pub struct RegistryEntry<'a> {
    pub id: &'a str,
    pub data: Option<Vec<u8>>,
}

impl RegistryDataPacket<'_> {
    // TODO: bake this. and make it return just the bytes instead.
    pub fn get_registry_packets() -> Vec<Self> {
        let mut packets = vec![];

        let dir = std::env::current_dir()
            .unwrap()
            .join(r#"..\..\..\assets\data\registries"#);

        let mut registry_nbt_buf = vec![].as_slice();

        for file_name in dir.iter() {
            let file_name = file_name.to_str().unwrap().split('/').next_back().unwrap();
            let registry_name = ("minecraft:".to_string()
                + &*file_name
                .clone()
                .strip_suffix(".nbt")
                .unwrap_or(file_name)
                .replace("worldgen", "worldgen/biome"));
            debug!("Opening registry file: {file_name}");
            registry_nbt_buf = std::fs::read(file_name).unwrap().as_slice();
            if registry_nbt_buf.is_empty() {
                continue; // Skip empty files
            }
            let mut entries = vec![];

            let mut tape = NbtTape::new(registry_nbt_buf);

            tape.parse();

            if tape.root.is_none() {
                debug!("No root found in registry file: {file_name}");
                continue; // Skip files without a root
            }
            let tape = tape.root.as_ref().unwrap().1.as_compound().unwrap();

            let mut serializer_machine = NbtTape::new(registry_nbt_buf);
            serializer_machine.parse();

            for (name, element) in tape {
                let mut data = vec![];
                element
                    .serialize_as_network(
                        &mut serializer_machine,
                        &mut data,
                        &ferrumc_nbt::NBTSerializeOptions::Network,
                    )
                    .unwrap_or_else(|_| panic!("Failed to serialize entry for {name}"));

                let entry = RegistryEntry {
                    id: name,
                    data: if data.is_empty() { None } else { Some(data) },
                };
                entries.push(entry);
            }
            packets.push(RegistryDataPacket::new(
                registry_name.clone().as_str(),
                entries,
            ));
        }

        // let mut tape = NbtTape::new(registry_nbt_buf);
        // tape.parse();
        // let mut serializer_machine = NbtTape::new(registry_nbt_buf);
        // serializer_machine.parse();
        //
        // let root = tape.root.as_ref().map(|(_, b)| b).unwrap();
        // let root = root.as_compound().unwrap();
        //
        // /*let (name, element) = &root[1];*/
        // for (name, element) in root {
        //     // TOP LEVEL
        //     let element = element.as_compound().unwrap();
        //
        //     let mut entries = vec![];
        //     for (name, element) in element {
        //         let mut data = vec![];
        //         element
        //             .serialize_as_network(
        //                 &mut serializer_machine,
        //                 &mut data,
        //                 &NBTSerializeOptions::Network,
        //             )
        //             .unwrap_or_else(|_| panic!("Failed to serialize entry for {name}"));
        //
        //         entries.push(RegistryEntry {
        //             id: name,
        //             data: if data.is_empty() { None } else { Some(data) },
        //         });
        //     }
        //     packets.push(RegistryDataPacket::new(name, entries));
        // }

        packets
    }
}

pub const fn get_registry_packets() -> &'static [u8] {
    include_bytes!("../../../../../../.etc/registry.packet")
}

#[cfg(test)]
mod tests {
    use super::RegistryDataPacket;
    use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
    use serde_json::Value;
    use std::collections::HashMap;
    use std::io::{Cursor, Write};

    #[test]
    #[ignore]
    fn generate_nbt() {
        let json_file = include_bytes!("../../../../../../.etc/registry.json");
        let val: HashMap<String, Value> = serde_json::from_slice(json_file).unwrap();

        for (key, value) in val {
            let key = key
                .replace("minecraft:", "")
                .replace("worldgen/biome", "worldgen");
            let mut nbt = Cursor::new(Vec::<u8>::new());
            craftflow_nbt::to_writer(&mut nbt, &value).unwrap();
            let nbt = nbt.into_inner();
            let dump_path = std::env::current_dir()
                .unwrap()
                .join(r#"..\..\..\assets\data\registries"#)
                .join(format!("{key}.nbt"));
            println!("dump path {}", dump_path.display());
            let mut open_file = std::fs::File::create(dump_path).unwrap();
            open_file.write_all(nbt.as_slice()).unwrap();
        }
    }

    #[test]
    #[ignore]
    fn generate_packet() {
        let registry_data = RegistryDataPacket::get_registry_packets();
        let mut buffer = Vec::new();
        for packet in registry_data {
            packet
                .encode(&mut buffer, &NetEncodeOpts::WithLength)
                .unwrap();
        }
        std::fs::write(
            r#"D:\Minecraft\framework\ferrumc\ferrumc-2_0\ferrumc\.etc/registry.packet"#,
            buffer,
        )
            .unwrap();
    }
}
