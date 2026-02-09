use std::env::current_exe;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum RootPathError {
    #[error("Failed to get the current executable location.")]
    IoError(#[from] std::io::Error),
    #[error("Failed to get the parent directory of the executable.")]
    NoParent,
}

/// Returns the root path of the executable.
/// e.g.
/// - If the executable is located at "D:/server/ferrumc.exe",
///   this function will return "D:/server".
///
/// # Examples
/// ```rust
/// use ferrumc_general_purpose::paths::get_root_path;
///
/// // Returns a PathBuf or panics if an error occurs
/// let root_path = get_root_path();
///
/// let favicon_path = root_path.join("icon.png");
/// ```
pub fn get_root_path() -> PathBuf {
    let exe_location = current_exe().expect("Failed to get the current executable location.");
    let exe_dir = exe_location
        .parent()
        .ok_or(RootPathError::NoParent)
        .expect("Failed to get the parent directory of the executable.");
    exe_dir.to_path_buf()
}
