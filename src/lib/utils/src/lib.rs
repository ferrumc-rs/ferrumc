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
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

        // Walk up until we find the workspace root (defined by having a Cargo.lock or .git)
        // Or just assume the logic you had before but applied to MANIFEST_DIR
        let mut root = manifest_dir;

        // Attempt to strip src/lib/... if present, or just find the git root
        for _ in 0..5 {
            if root.join("Cargo.lock").exists() || root.join(".git").exists() {
                break;
            }
            if let Some(parent) = root.parent() {
                root = parent;
            } else {
                break;
            }
        }

        root.join($from_root).to_str().unwrap().to_string()
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
