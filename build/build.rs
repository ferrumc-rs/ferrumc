use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut res = winres::WindowsResource::new();
        res.set_icon(&format!("{}/build/icon.ico", manifest_dir));
        res.compile().unwrap();
    }
}