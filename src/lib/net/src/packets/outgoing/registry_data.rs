use ferrumc_macros::{packet, NetEncode};
use ferrumc_nbt::{NBTSerializeOptions, NbtTape};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use std::io::Write;

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
        let registry_nbt_buf = include_bytes!("../../../../../../assets/data/registry.nbt");

        let mut tape = NbtTape::new(registry_nbt_buf);
        tape.parse();
        let mut serializer_machine = NbtTape::new(registry_nbt_buf);
        serializer_machine.parse();

        let root = tape.root.as_ref().map(|(_, b)| b).unwrap();
        let root = root.as_compound().unwrap();

        let mut packets = vec![];

        /*let (name, element) = &root[1];*/
        for (name, element) in root {
            // TOP LEVEL
            let element = element.as_compound().unwrap();

            let mut entries = vec![];
            for (name, element) in element {
                let mut data = vec![];
                element
                    .serialize_as_network(
                        &mut serializer_machine,
                        &mut data,
                        &NBTSerializeOptions::Network,
                    )
                    .unwrap_or_else(|_| panic!("Failed to serialize entry for {name}"));

                entries.push(RegistryEntry {
                    id: name,
                    data: if data.is_empty() {
                        None
                    } else {
                        Some(data)
                    },
                });
            }
            packets.push(RegistryDataPacket::new(name, entries));
        }

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
    use std::io::{Cursor, Write};

    #[test]
    #[ignore]
    fn generate_nbt() {
        let json_file = include_bytes!("../../../../../../.etc/registry.json");
        let val: HashMap<String, Value> = serde_json::from_slice(json_file).unwrap();

        let mut nbt = Cursor::new(Vec::<u8>::new());


        craftflow_nbt::to_writer(&mut nbt, &val).unwrap();
        let nbt = nbt.into_inner();
        println!("cwd {}", std::env::current_dir().unwrap().display());
        let dump_path = std::env::current_dir()
            .unwrap()
            .join(r#"..\..\..\assets\data\"#);
        println!("dump path {}", dump_path.display());
        let open_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(dump_path.join("registry.nbt"));
        //
        open_file.unwrap().write_all(nbt.as_slice()).unwrap();
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
