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
    ($from_root:literal) => {
        {
            let depth = if cfg!(windows) {
                file!().split("\\").count()
            } else {
                file!().split("/").count()
            };
            
            let mut root = "../".repeat(depth-2);
            root.push_str($from_root);
            if cfg!(windows) {
                root.replace("/", "\\")
            } else {
                root
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_root() {
        assert!(Path::new(&root!("Cargo.toml")).exists());
    }
}
