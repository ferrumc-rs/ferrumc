use std::sync::OnceLock;
use base64::Engine;
use tracing::error;
use ferrumc_general_purpose::paths::get_root_path;

static FAVICON_BASE64: OnceLock<String> = OnceLock::new();


/// Get the favicon as a base64 string.
/// Reads the icon.png file from the same directory as the executable.
/// And caches the value for further calls.
pub fn get_favicon_base64<'a>() -> &'a str {
    FAVICON_BASE64.get_or_init(read_favicon_from_file)
}

const FAVICON_PATH: &str = "icon.png";

fn read_favicon_from_file() -> String {
    let root_path = get_root_path().unwrap_or_else(|e| {
        let message = format!("Failed to get root path: {}", e);
        error!("{message}");
        panic!("{message}");
    });

    let favicon_path = root_path.join(FAVICON_PATH);

    let favicon = match std::fs::read(favicon_path) {
        Ok(favicon) => favicon,
        Err(e) => {
            error!("Failed to read favicon file: {}", e);
            return String::from("");
        }
    };

    let encoded = base64::engine::general_purpose::STANDARD.encode(favicon.as_slice());

    format!("data:image/png;base64,{}", encoded)
}