use super::MC_VERSION;
use reqwest::blocking::Client;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DECOMPILER_URL: &str = "https://github.com/MaxPixelStudios/MinecraftDecompiler/releases/download/v3.3.2/MinecraftDecompiler.jar";

pub fn prepare_remapped_jar(temp_dir: &Path) -> PathBuf {
    let remapped_jar = temp_dir.join(format!("server-{}-remapped.jar", MC_VERSION));

    if remapped_jar.exists() {
        println!("Found cached remapped JAR.");
        return remapped_jar;
    }

    println!("Remapped JAR not found. Starting Decompilation process...");

    // 1. Download Decompiler
    let decompiler_path = temp_dir.join("MinecraftDecompiler.jar");
    if !decompiler_path.exists() {
        println!("Downloading Decompiler...");
        let mut resp = Client::new()
            .get(DECOMPILER_URL)
            .send()
            .expect("Failed to download decompiler");
        let mut file = fs::File::create(&decompiler_path).unwrap();
        resp.copy_to(&mut file).unwrap();
    }

    // 2. Run Decompiler
    println!("Running MinecraftDecompiler (This will take a while)...");
    let status = Command::new("java")
        .current_dir(temp_dir)
        .arg("-jar")
        .arg(&decompiler_path)
        .arg("--version")
        .arg(MC_VERSION)
        .arg("--side")
        .arg("SERVER")
        .arg("--decompile")
        .arg("--output")
        .arg(&remapped_jar)
        .arg("--decompiled-output")
        .arg(temp_dir.join("src_out"))
        .status()
        .expect("Failed to run Decompiler");

    if !status.success() {
        panic!("Decompilation failed!");
    }

    if !remapped_jar.exists() {
        panic!("Decompiler finished but remapped JAR is missing!");
    }

    remapped_jar
}
