use std::fs::File;
use std::io::Read;
use std::sync::OnceLock;
use base64::Engine;
use lazy_static::lazy_static;
use tracing::error;
use ferrumc_general_purpose::paths::get_root_path;

const BAKED_FAVICON: &[u8] = include_bytes!("../../../../../icon-64.png");
static CUSTOM_FAVICON: OnceLock<String> = OnceLock::new();

lazy_static! {
    static ref FAVICON_BASE64: String = {
        let encoded = base64::engine::general_purpose::STANDARD.encode(BAKED_FAVICON);
        format!("data:image/png;base64,{}", encoded)
    };
}
pub fn get_favicon_base64() -> String {
    match get_custom_favicon_base64() {
        Some(res) => {
            res.clone()
        }
        None => FAVICON_BASE64.clone()
    }
}

fn get_custom_favicon_base64() -> Option<String> {
    match CUSTOM_FAVICON.get() {
        Some(res) => Some(res.clone()),
        None => {
            let path = get_root_path().expect("Could not get root").join("favicon.png");
            if path.exists() {
                let mut file = match File::open(path) {
                    Ok(file) => file,
                    Err(e) => {
                        error!("Could not open favicon file: {}", e);
                        return None;
                    }
                };
                let mut favicon = Vec::new();
                if let Err(e) = file.read_to_end(&mut favicon) {
                    error!("Could not read favicon file: {}", e);
                    return None;
                }
                let encoded = base64::engine::general_purpose::STANDARD.encode(&favicon);
                match CUSTOM_FAVICON.set(Some(format!("data:image/png;base64,{}", encoded))?) {
                    Ok(_) => Some(format!("data:image/png;base64,{}", encoded)),
                    Err(_) => None
                }
            } else {
                None
            }
        }
    }
}