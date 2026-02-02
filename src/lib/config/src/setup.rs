use crate::server_config::DEFAULT_CONFIG;
use crate::whitelist::create_blank_whitelist_file;
use ferrumc_general_purpose::paths::get_root_path;
use std::fs::File;
use std::io::Write;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SetupError {
    #[error("Could not write the config file: {0}")]
    WriteError(std::io::Error),
    #[error("Could not read the config file: {0}")]
    ReadError(std::io::Error),
    #[error("IO error: {0}")]
    IoError(std::io::Error),
}

impl From<std::io::Error> for SetupError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => SetupError::ReadError(e),
            std::io::ErrorKind::PermissionDenied => SetupError::WriteError(e),
            std::io::ErrorKind::AlreadyExists => SetupError::WriteError(e),
            _ => SetupError::IoError(e),
        }
    }
}

pub fn setup() -> Result<(), SetupError> {
    if !std::fs::exists(get_root_path().join("whitelist.txt"))? {
        create_blank_whitelist_file();
    }
    if !std::fs::exists(get_root_path().join("configs"))? {
        std::fs::create_dir(get_root_path().join("configs"))?;
    }
    if std::fs::exists(get_root_path().join("configs").join("config.toml"))? {
        return Ok(());
    }
    let mut config_file = File::create(get_root_path().join("configs").join("config.toml"))?;
    let modified_config = DEFAULT_CONFIG.replace(
        "This will be replaced with a random \
    string on first run, do not change this text. If you see this outside of the default config, \
    something is wrong.",
        &generate_random_secret(),
    );
    config_file.write_all(modified_config.as_bytes())?;
    if !std::fs::exists(get_root_path().join("import"))? {
        std::fs::create_dir(get_root_path().join("import"))?;
    }
    if !std::fs::exists(get_root_path().join("world"))? {
        std::fs::create_dir(get_root_path().join("world"))?;
    }
    Ok(())
}

fn generate_random_secret() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz\
        0123456789)(*&^%$#@!~";
    const SECRET_LEN: usize = 32;
    let mut rng = rand::thread_rng();

    let secret: String = (0..SECRET_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    secret
}
