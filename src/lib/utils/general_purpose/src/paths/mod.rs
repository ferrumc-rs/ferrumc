use std::env::current_exe;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum RootPathError {
    #[error("Failed to get the current executable location.")]
    IoError(#[from] std::io::Error),
    #[error("Failed to get the parent directory of the executable.")]
    NoParent,
}

pub fn get_root_path() -> Result<PathBuf, RootPathError> {
    //! Returns the root path of the executable.
    //! e.g.
    //! - If the executable is located at "D:/server/ferrumc.exe",
    //!   this function will return "D:/server".
    //!
    //!
    //! # Errors
    //! - If the current executable location cannot be found. (RootPathError::IoError)
    //! - If the parent directory of the executable cannot be found. (RootPathError::NoParent)
    //!
    //! # Examples
    //! ```rust
    //! use ferrumc_general_purpose::paths::get_root_path;
    //!
    //! // Returns a Result<PathBuf, RootPathError>
    //! let root_path = get_root_path().expect("Failed to get root path.");
    //!
    //! let favicon_path = root_path.join("icon.png");
    //! ```
    //!
    let exe_location = current_exe()?;
    let exe_dir = exe_location.parent().ok_or(RootPathError::NoParent)?;

    Ok(exe_dir.to_path_buf())
}
