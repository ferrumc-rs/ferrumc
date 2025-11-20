pub mod errors;
pub mod formatting;

/// Gets the fully qualified path to the root of the project and joins it with the given path.
///
/// # Arguments
///
/// * `from_root` - The path to join with the root of the project
///
/// # Returns
///
/// The fully qualified path to the root of the project joined with the given path
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use ferrumc_utils::root;
///
/// assert!(Path::new(&root!("Cargo.toml")).exists());
/// ```
#[macro_export]
macro_rules! root {
    ($from_root:literal) => {{
        // Use CARGO_MANIFEST_DIR instead of file!()
        // This guarantees we get the path to the source code on disk,
        // even when running in a temporary doctest environment.
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let delimiter = std::path::MAIN_SEPARATOR;

        let root = manifest_dir
            .split(delimiter)
            .take_while(|&x| x != "src")
            .collect::<Vec<&str>>()
            .join(&delimiter.to_string());

        let path_from_root = std::path::Path::new($from_root);

        std::path::Path::new(&root)
            .join(path_from_root)
            .to_str()
            .expect("Failed to convert path to string")
            .to_string()
    }};
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_root() {
        assert!(Path::new(&root!("Cargo.toml")).exists());
    }
}
