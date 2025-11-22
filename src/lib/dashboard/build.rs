use anyhow::Result;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

// CONFIGURATION
// Since we set up the GitHub Action to release to 'latest', we can use this URL.
// Replace 'ferrumc-rs/dashboard' with your actual user/repo if different.
const DASHBOARD_URL: &str =
    "https://github.com/ferrumc-rs/dashboard/releases/download/latest/dashboard.min.html";

fn main() -> Result<()> {
    // 1. Determine where to put the file.
    // OUT_DIR is the standard place for build artifacts.
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("dashboard.min.html");

    // 2. Logic to decide when to download.
    // We check if the file exists. If it doesn't, we download.
    // We can also force a download by setting an ENV var: FORCE_UPDATE=1
    let should_download = std::env::var("FORCE_UPDATE").is_ok() || !dest_path.exists();

    if should_download {
        println!(
            "cargo:warning=Downloading Dashboard artifact from {}",
            DASHBOARD_URL
        );

        // delete existing file if it exists
        if dest_path.exists() {
            fs::remove_file(&dest_path)?;
        }

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(DASHBOARD_URL)
            .header("User-Agent", "ferrumc-build-script")
            .send()?;

        if response.status().is_success() {
            let content = response.bytes()?;
            let mut file = fs::File::create(&dest_path)?;
            file.write_all(&content)?;
        } else {
            // Fallback: If download fails (e.g., no internet, repo private),
            // create a dummy file so the build doesn't crash during dev.
            println!("cargo:warning=Failed to download dashboard. Using fallback.");
            let mut file = fs::File::create(&dest_path)?;
            file.write_all(b"<h1>Dashboard Offline (Build failed to fetch)</h1>")?;
        }
    }

    // 3. Tell Cargo to rerun this script if specific conditions change
    println!("cargo:rerun-if-env-changed=FORCE_UPDATE");

    Ok(())
}
