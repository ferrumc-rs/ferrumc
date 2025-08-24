use std::env::current_dir;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum RootPathError {
    #[error("Failed to get the current executable location.")]
    IoError(#[from] std::io::Error),
    #[error("Failed to get the parent directory of the executable.")]
    NoParent,
}

pub fn get_root_path() -> PathBuf {
    //! Returns the current working directory.
    //!
    //!
    //! # Panics
    //! - If the current working directory cannot be found.
    //!
    //! # Examples
    //! ```rust
    //! use ferrumc_general_purpose::paths::get_root_path;
    //!
    //! // Returns a PathBuf
    //! let root_path = get_root_path();
    //!
    //! let favicon_path = root_path.join("icon.png");
    //! ```
    //!
    current_dir().expect("Failed to get the current working directory")
}

pub trait BetterPathExt {
    fn better_display(&self) -> String;
}

impl BetterPathExt for PathBuf {
    fn better_display(&self) -> String {
        //! Returns a string representation of the path that is more readable.
        //! <br>
        //! e.g.
        //! If the path is `D:\\server\\world\\region\\r.0.0.mca`,
        //! <br>
        //! -> `D:/server/world/region/r.0.0.mca`.
        let path = self
            .to_string_lossy()
            .replace(r"\\?\", "") // Remove Windows extended path prefix
            .replace(r"\\", r"\"); // Normalize backslashes

        format!("`{path}`")
    }
}
