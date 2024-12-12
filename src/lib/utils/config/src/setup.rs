use crate::statics::{write_whitelist_to_file, DEFAULT_CONFIG};
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
        write_whitelist_to_file();
    }
    if std::fs::exists(get_root_path().join("config.toml"))? {
        return Ok(());
    }
    let mut config_file = File::create(get_root_path().join("config.toml"))?;
    config_file.write_all(DEFAULT_CONFIG.as_bytes())?;
    if !std::fs::exists(get_root_path().join("import"))? {
        std::fs::create_dir(get_root_path().join("import"))?;
    }
    if !std::fs::exists(get_root_path().join("world"))? {
        std::fs::create_dir(get_root_path().join("world"))?;
    }
    Ok(())
}
