use std::env;

fn main() {
    if cfg!(not(target_os = "windows")) {
        return;
    }

    if cfg!(debug_assertions) {
        return;
    }

    println!("cargo:rerun-if-changed=build/icon.ico");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut res = winres::WindowsResource::new();
    res.set_icon(&format!("{}/build/icon.ico", manifest_dir));
    res.compile().unwrap();
}
