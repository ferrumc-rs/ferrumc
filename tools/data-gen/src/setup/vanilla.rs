use super::{MANIFEST_URL, MC_VERSION};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Deserialize)]
struct Manifest {
    versions: Vec<ManifestVersion>,
}
#[derive(Deserialize)]
struct ManifestVersion {
    id: String,
    url: String,
}
#[derive(Deserialize)]
struct VersionDetails {
    downloads: Downloads,
}
#[derive(Deserialize)]
struct Downloads {
    server: DownloadEntry,
}
#[derive(Deserialize)]
struct DownloadEntry {
    url: String,
}

pub fn download_server_jar(temp_dir: &Path) -> PathBuf {
    let jar_path = temp_dir.join("server.jar");
    if jar_path.exists() {
        println!("Found cached server.jar");
        return jar_path;
    }

    println!("Fetching manifest for {}...", MC_VERSION);
    let client = Client::new();

    // 1. Get Manifest
    let manifest: Manifest = client
        .get(MANIFEST_URL)
        .send()
        .expect("Failed to fetch manifest")
        .json()
        .expect("Failed to parse manifest");

    let version = manifest
        .versions
        .iter()
        .find(|v| v.id == MC_VERSION)
        .expect("Version not found in manifest");

    // 2. Get Version Details
    let details: VersionDetails = client
        .get(&version.url)
        .send()
        .expect("Failed to fetch version details")
        .json()
        .expect("Failed to parse version details");

    // 3. Download JAR
    println!("Downloading server.jar...");
    let mut resp = client
        .get(details.downloads.server.url)
        .send()
        .expect("Failed to download JAR");

    let mut file = fs::File::create(&jar_path).expect("Failed to create JAR file");
    resp.copy_to(&mut file).expect("Failed to write JAR");

    jar_path
}

pub fn run_java_generator(jar_path: &Path, cwd: &Path) {
    let report_file = cwd.join("generated/reports/blocks.json");
    if report_file.exists() {
        println!("Found cached Vanilla Reports. Skipping generation.");
        return;
    }

    println!("Running Vanilla Data Generator (--reports)...");

    Command::new("java")
        .current_dir(cwd)
        .arg("-DbundlerMainClass=net.minecraft.data.Main")
        .arg("-jar")
        .arg(jar_path)
        .arg("--reports")
        .output() // capture output to keep terminal clean, or use status() to see it
        .expect("Failed to run Java");

    println!("Vanilla reports generated.");
}
