use crate::complex::ComplexBlock;
use fxhash::FxHashMap;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;

pub mod complex;
pub mod config;
pub mod simple;

#[derive(Deserialize, Debug)]
pub struct BlockState {
    name: String,
    properties: Option<HashMap<String, String>>,
}

#[allow(clippy::type_complexity)]
pub fn separate_blocks(
    input: HashMap<u32, BlockState>,
) -> (Vec<(u32, String)>, Vec<(u32, ComplexBlock)>) {
    let mut simple_blocks = Vec::new();
    let mut complex_blocks = Vec::new();

    for (id, state) in input.into_iter() {
        match state.properties {
            Some(properties) => complex_blocks.push((
                id,
                ComplexBlock {
                    name: state.name,
                    properties: FxHashMap::from_iter(properties.into_iter()),
                },
            )),
            None => simple_blocks.push((id, state.name)),
        }
    }

    (simple_blocks, complex_blocks)
}

pub fn format_code(unformatted_code: &str) -> String {
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
