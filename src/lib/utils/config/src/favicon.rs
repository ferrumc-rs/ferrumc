use std::fs::File;
use std::io::Read;
use base64::Engine;
use lazy_static::lazy_static;
use tracing::error;
use ferrumc_general_purpose::paths::get_root_path;

const BAKED_FAVICON: &[u8] = include_bytes!("../../../../../icon-64.png");

lazy_static! {
    static ref FAVICON_BASE64: String = {
        let encoded = base64::engine::general_purpose::STANDARD.encode(BAKED_FAVICON);
        format!("data:image/png;base64,{}", encoded)
    };

    static ref CUSTOM_FAVICON: Option<String> = {
        let icon_path = get_root_path().ok()?.join("icon.png");
        if icon_path.exists() {
            let mut file = match File::open(icon_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Could not open custom favicon file: {}", e);
                    return None;
                }
            };
            let mut icon = Vec::new();
            if let Err(e) = file.read_to_end(&mut icon) {
                eprintln!("Could not read custom favicon file: {}", e);
                return None;
            }
            let res = format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(icon));
            Some(res)
        } else {
            None
        }
    };
}
pub fn get_favicon_base64() -> &'static str {
    CUSTOM_FAVICON.as_ref().unwrap_or(&*FAVICON_BASE64).as_str()
}
