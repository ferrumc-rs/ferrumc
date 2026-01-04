use anyhow::Result;
use std::env;
use std::fs;
use std::io::{Cursor, Write};
use std::path::Path;
use zip::ZipArchive;

// CONFIGURATION
// Since we set up the GitHub Action to release to 'latest', we can use this URL.
const DASHBOARD_URL: &str =
    "https://github.com/ferrumc-rs/dashboard/releases/download/latest/ferrumc-dashboard.zip";

fn main() -> Result<()> {
    // Tell Cargo about our custom cfg flags to suppress warnings
    println!("cargo::rustc-check-cfg=cfg(dashboard_in_out_dir)");
    println!("cargo::rustc-check-cfg=cfg(dashboard_in_manifest_dir)");
    println!("cargo::rustc-check-cfg=cfg(dashboard_build)");
    
    // 1. Determine where to put the files.
    // Try OUT_DIR first (standard for build artifacts), fall back to CARGO_MANIFEST_DIR
    // because proc macros like include_dir! don't always see OUT_DIR reliably.
    let (dest_dir, zip_path, use_out_dir) = if let Ok(out_dir) = env::var("OUT_DIR") {
        let out_path = Path::new(&out_dir);
        // Check if OUT_DIR actually exists and is accessible
        if out_path.exists() {
            (
                out_path.join("dashboard"),
                out_path.join("dashboard.zip"),
                true,
            )
        } else {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
            (
                Path::new(&manifest_dir).join("dashboard-dist"),
                Path::new(&manifest_dir).join("dashboard.zip"),
                false,
            )
        }
    } else {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
        (
            Path::new(&manifest_dir).join("dashboard-dist"),
            Path::new(&manifest_dir).join("dashboard.zip"),
            false,
        )
    };
    
    // Emit which path we're using so include_dir! knows where to look
    if use_out_dir {
        println!("cargo:rustc-cfg=dashboard_in_out_dir");
    } else {
        println!("cargo:rustc-cfg=dashboard_in_manifest_dir");
    }

    // 2. Logic to decide when to download.
    // We check if the directory exists. If it doesn't, we download.
    // We can also force a download by setting an ENV var: FORCE_UPDATE=1
    let should_download = std::env::var("FORCE_UPDATE").is_ok() || !dest_dir.exists();

    if should_download {
        println!(
            "cargo:warning=Downloading Dashboard artifact from {}",
            DASHBOARD_URL
        );

        // Delete existing directory if it exists
        if dest_dir.exists() {
            fs::remove_dir_all(&dest_dir)?;
        }

        // Delete existing zip if it exists
        if zip_path.exists() {
            fs::remove_file(&zip_path)?;
        }

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(DASHBOARD_URL)
            .header("User-Agent", "ferrumc-build-script")
            .send()?;

        if response.status().is_success() {
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
        } else {
            // Fallback: If download fails (e.g., no internet, repo private),
            // create a dummy file so the build doesn't crash during dev.
            println!("cargo:warning=Failed to download dashboard. Using fallback.");
            fs::create_dir_all(&dest_dir)?;
            let mut file = fs::File::create(dest_dir.join("index.html"))?;
            file.write_all(b"<h1>Dashboard Offline (Build failed to fetch)</h1>")?;
        }
    }

    // 3. Tell Cargo to rerun this script if specific conditions change
    println!("cargo:rerun-if-env-changed=FORCE_UPDATE");
    
    // 4. Emit a cfg flag so the main code knows build.rs ran
    println!("cargo:rustc-cfg=dashboard_build");
    
    // 5. Rerun if the dashboard-dist directory is deleted
    println!("cargo:rerun-if-changed=dashboard-dist");

    Ok(())
}
