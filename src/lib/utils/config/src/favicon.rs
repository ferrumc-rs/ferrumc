use base64::Engine;
use lazy_static::lazy_static;

const BAKED_FAVICON: &[u8] = include_bytes!("../../../../../icon-64.png");

lazy_static! {
    static ref FAVICON_BASE64: String = {
        let encoded = base64::engine::general_purpose::STANDARD.encode(BAKED_FAVICON);
        format!("data:image/png;base64,{}", encoded)
    };
}
pub fn get_favicon_base64() -> &'static str {
    FAVICON_BASE64.as_str()
}