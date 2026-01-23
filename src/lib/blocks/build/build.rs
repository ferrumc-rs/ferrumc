mod simple;
mod complex;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use serde::Deserialize;
use crate::complex::generate_complex_blocks;
use crate::simple::generate_simple_block_enum;

#[derive(Deserialize, Debug)]
struct BlockState {
    name: String,
    properties: Option<HashMap<String, String>>,
}

struct ComplexBlock {
    name: String,
    properties: HashMap<String, String>,
}

#[derive(Deserialize)]
struct BuildConfig {
    name_overrides: HashMap<String, String>,
    block_overrides: HashMap<String, String>,
}

fn main() {
    println!("cargo:rerun-if-changed=../../../assets/data/blockstates.json");
    println!("cargo:rerun-if-changed=build_config.toml");

    let config = fs::read_to_string("build_config.toml").unwrap();
    let config = toml::from_str::<BuildConfig>(&config).unwrap();

    let file = fs::read_to_string("../../../assets/data/blockstates.json").unwrap();
    let blockstates: HashMap<String, BlockState> = serde_json::from_str(&file).unwrap();

    let mut simple_blocks = Vec::new();
    let mut complex_blocks = Vec::new();

    for (id, block_state) in blockstates {
        let id = id.parse::<u32>().expect("block id key should be an integer");

        if block_state.properties.is_none() {
            simple_blocks.push((id, block_state.name))
        } else {
            complex_blocks.push((id, ComplexBlock { name: block_state.name, properties: block_state.properties.unwrap() }))
        }
    }

    let simple_blocks = generate_simple_block_enum(simple_blocks);
    let complex_blocks = generate_complex_blocks(&config, complex_blocks);

    fs::write("src/simple_blocks.rs", format_code(&simple_blocks.to_string())).unwrap();
    fs::write("src/blocks.rs", format_code(&complex_blocks.to_string())).unwrap();
}

fn format_code(unformatted_code: &str) -> String {
    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn rustfmt process.");

    child
        .stdin
        .take()
        .expect("Failed to take rustfmt stdin")
        .write_all(unformatted_code.as_bytes())
        .expect("Failed to write to rustfmt stdin.");

    let output = child
        .wait_with_output()
        .expect("Failed to wait for rustfmt process.");

    if output.status.success() {
        String::from_utf8(output.stdout).expect("rustfmt output was not valid UTF-8.")
    } else {
        panic!(
            "rustfmt failed with status: {}\n--- stderr ---\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}