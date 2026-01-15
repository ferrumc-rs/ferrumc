use anyhow::Result;
use simd_json::prelude::{ValueAsArray, ValueAsObject, ValueAsScalar};
use std::env;
use std::fs;
use std::io::{Cursor, Write};
use std::path::Path;
use zip::ZipArchive;

// CONFIGURATION
// Since we set up the GitHub Action to release to 'latest', we can use this URL.
const API_URL: &str = "https://api.github.com/repos/ferrumc-rs/dashboard/releases";
const DASHBOARD_URL: &str =
    "https://github.com/ferrumc-rs/dashboard/releases/download/latest/ferrumc-dashboard.zip";
const API_KEY: &str =
    "github_pat_**no secrets**11AP6ULWQ00FwfV**no secrets**MudA7Yq_mHFFw3SmU4jNtl**no secrets**wdfvp229DyGeCF6stYxPwR0**no secrets**d0VqFsMO2LPEKVG9dZpGKp";

fn main() -> Result<()> {
    // Tell Cargo about our custom cfg flags to suppress warnings
    println!("cargo::rustc-check-cfg=cfg(dashboard_in_out_dir)");
    println!("cargo::rustc-check-cfg=cfg(dashboard_in_manifest_dir)");
    println!("cargo::rustc-check-cfg=cfg(dashboard_build)");

    // 1. Determine where to put the files.
    let (dest_dir, use_out_dir) = if let Ok(out_dir) = env::var("OUT_DIR") {
        let out_path = Path::new(&out_dir)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        // Check if OUT_DIR actually exists and is accessible
        if out_path.exists() {
            (out_path.join("dashboard"), true)
        } else {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
            (Path::new(&manifest_dir).join("dashboard-dist"), false)
        }
    } else {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
        (Path::new(&manifest_dir).join("dashboard-dist"), false)
    };

    // Emit which path we're using so include_dir! knows where to look
    if use_out_dir {
        println!("cargo:rustc-cfg=dashboard_in_out_dir");
    } else {
        println!("cargo:rustc-cfg=dashboard_in_manifest_dir");
    }

    let client = reqwest::blocking::Client::new();

    let api_req = client
        .get(API_URL)
        .header("User-Agent", "ferrumc-build-script")
        .header(
            "Authorization",
            format!("Bearer {}", API_KEY.replace("**no secrets**", "")),
        )
        .send();

    let api_resp = match api_req {
        Ok(response) => {
            if response.status().is_success() {
                response.json::<simd_json::value::owned::Value>()?
            } else {
                println!(
                    "cargo:warning=Failed to fetch release info, status: {}",
                    response.status()
                );
                return Err(anyhow::anyhow!("Failed to fetch release info"));
            }
        }
        Err(e) => {
            println!("cargo:warning=Error fetching release info: {}", e);
            return Err(anyhow::anyhow!("Error fetching release info: {}", e));
        }
    };

    let first_release = api_resp
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|val| val.as_object())
        .ok_or_else(|| anyhow::anyhow!("No releases found in API response"))?;

    let time_stamp = first_release
        .get("created_at")
        .and_then(|val| val.as_str())
        .ok_or_else(|| anyhow::anyhow!("No created_at field in release"))?;

    if dest_dir.exists() {
        let time_stamp_path = dest_dir.join("timestamp.txt");
        let existing_timestamp = fs::read_to_string(&time_stamp_path).unwrap_or_default();
        if existing_timestamp == time_stamp {
            println!("cargo:warning=Dashboard is up-to-date, skipping download.");
            println!("cargo:rustc-cfg=dashboard_build");
            return Ok(());
        }
    } else {
        println!("cargo:warning=Dashboard directory does not exist or using OUT_DIR, proceeding to download.");
    }

    // 2. Attempt to download the dashboard
    println!(
        "cargo:warning=Downloading Dashboard artifact from {}",
        DASHBOARD_URL
    );
    let download_result = client
        .get(DASHBOARD_URL)
        .header("User-Agent", "ferrumc-build-script")
        .header(
            "Authorization",
            format!("Bearer {}", API_KEY.replace("**no secrets**", "")),
        )
        .send();

    match download_result {
        Ok(response) if response.status().is_success() => {
            // Delete existing directory if it exists
            if dest_dir.exists() {
                fs::remove_dir_all(&dest_dir)?;
            }

            let content = response.bytes()?;

            // Extract the zip file
            let cursor = Cursor::new(content);
            let mut archive = ZipArchive::new(cursor)?;

            // Create the destination directory
            fs::create_dir_all(&dest_dir)?;

            // Extract all files
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = dest_dir.join(file.mangled_name());

                if file.is_dir() {
                    fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(parent) = outpath.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    let mut outfile = fs::File::create(&outpath)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
            }

            println!("cargo:warning=Dashboard extracted to {:?}", dest_dir);

            // Write the timestamp file
            let mut ts_file = fs::File::create(dest_dir.join("timestamp.txt"))?;
            ts_file.write_all(time_stamp.as_bytes())?;
        }
        _ => {
            // Download failed (no internet, request error, non-success status)
            if dest_dir.exists() {
                // Use existing dashboard files
                println!("cargo:warning=Download failed, using existing dashboard files.");
            } else {
                // Create fallback HTML
                println!(
                    "cargo:warning=Download failed and no existing dashboard. Creating fallback."
                );
                fs::create_dir_all(&dest_dir)?;
                let mut file = fs::File::create(dest_dir.join("index.html"))?;
                file.write_all(b"<h1>Dashboard Offline (Build failed to fetch)</h1>")?;
            }
        }
    }

    // Emit a cfg flag so the main code knows build.rs ran
    println!("cargo:rustc-cfg=dashboard_build");

    Ok(())
}
