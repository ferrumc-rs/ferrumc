pub mod errors;

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
        let delimiter = if cfg!(windows) { "\\" } else { "/" };
        let root = std::path::absolute(file!())
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let root = root
            .split(delimiter)
            .take_while(|&x| x != "src")
            .collect::<Vec<&str>>()
            .join(delimiter);
        let path_from_root = std::path::Path::new($from_root);
        std::path::absolute(root)
            .unwrap()
            .join(path_from_root)
            .to_str()
            .unwrap()
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
