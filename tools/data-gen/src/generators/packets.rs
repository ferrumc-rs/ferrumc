use crate::utils;
use heck::ToShoutySnakeCase;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

// --- JSON Structures (Vanilla Report) ---
#[derive(Deserialize)]
struct PacketReport {
    handshake: DirectionMap,
    status: DirectionMap,
    login: DirectionMap,
    configuration: DirectionMap,
    play: DirectionMap,
}

#[derive(Deserialize)]
struct DirectionMap {
    #[serde(default)]
    clientbound: HashMap<String, PacketDef>,
    #[serde(default)]
    serverbound: HashMap<String, PacketDef>,
}

#[derive(Deserialize)]
struct PacketDef {
    protocol_id: u8,
}

// --- Generator Logic ---

pub fn generate(input_path: &Path, output_path: &Path) {
    println!("   ... Parsing Packets from {:?}", input_path);
    let content = fs::read_to_string(input_path).expect("Failed to read packets.json");
    let report: PacketReport =
        serde_json::from_str(&content).expect("Failed to parse packets.json");

    let mut file = utils::create_file(output_path);

    writeln!(file, "//Packet IDs based on the Vanilla Server Report").unwrap();
    writeln!(file, "").unwrap();

    let write_group = |writer: &mut fs::File, state_prefix: &str, dir_map: &DirectionMap| {
        // 1. Clientbound
        let mut client_keys: Vec<_> = dir_map.clientbound.keys().collect();
        client_keys.sort();

        for name in client_keys {
            let def = &dir_map.clientbound[name];
            let clean_name = name.replace("minecraft:", "");
            let const_name =
                format!("{}_CLIENTBOUND_{}", state_prefix, clean_name).to_shouty_snake_case();

            // Use `writer` instead of `file`
            writeln!(
                writer,
                "pub const {}: u8 = {:#04X};",
                const_name, def.protocol_id
            )
            .unwrap();
        }

        // 2. Serverbound
        let mut server_keys: Vec<_> = dir_map.serverbound.keys().collect();
        server_keys.sort();

        for name in server_keys {
            let def = &dir_map.serverbound[name];
            let clean_name = name.replace("minecraft:", "");
            let const_name =
                format!("{}_SERVERBOUND_{}", state_prefix, clean_name).to_shouty_snake_case();

            writeln!(
                writer,
                "pub const {}: u8 = {:#04X};",
                const_name, def.protocol_id
            )
            .unwrap();
        }
        writeln!(writer, "").unwrap();
    };

    writeln!(file, "// --- HANDSHAKE ---").unwrap();
    write_group(&mut file, "HANDSHAKE", &report.handshake);

    writeln!(file, "// --- STATUS ---").unwrap();
    write_group(&mut file, "STATUS", &report.status);

    writeln!(file, "// --- LOGIN ---").unwrap();
    write_group(&mut file, "LOGIN", &report.login);

    writeln!(file, "// --- CONFIGURATION ---").unwrap();
    write_group(&mut file, "CONFIGURATION", &report.configuration);

    writeln!(file, "// --- PLAY ---").unwrap();
    write_group(&mut file, "PLAY", &report.play);
}
