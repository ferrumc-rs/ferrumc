use std::io::Read;
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    let _ = Command::new("cargo")
        .args(["generate-lockfile"])
        .stdout(Stdio::piped())
        .spawn()?;
    // Get the version of the binary crate
    let mut child = Command::new("cargo")
        .args(["pkgid", "--package", "ferrumc"])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut version = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut version)?;
    }

    let version = version.split('@').collect::<Vec<&str>>()[1];

    // Set env vars used for the server brand string
    println!("cargo:rustc-env=FERRUMC_VERSION={}", version);
    println!(
        "cargo:rustc-env=BUILD_TYPE={}",
        if std::env::var("PROFILE").unwrap() == "debug" {
            " DEBUG"
        } else {
            ""
        }
    );
    println!("cargo:rerun-if-changed=../../bin/Cargo.toml");
    Ok(())
}
