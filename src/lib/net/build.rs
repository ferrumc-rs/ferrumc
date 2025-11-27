use std::io::Read;
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    let mut child = Command::new("cargo")
        .args(&[
            "pkgid",
            "--package",
            "ferrumc",
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut version = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut version)?;
    }

    let version = version.split('@').collect::<Vec<&str>>()[1];

    println!("cargo:rustc-env=FERRUMC_VERSION={}", version);
    println!("cargo:rerun-if-changed=../../bin/Cargo.toml");
    Ok(())
}