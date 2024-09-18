//! # FerrumC Configuration Utilities - Unit Tests
//!
//! Contains unit tests for the configuration utilities.

use crate::server_config::DatabaseCompression;
use crate::{get_global_config, ServerConfig};
use std::fs::*;
use std::io::Write;

/// A struct to hold the test configuration file paths.
/// When drop is called, it will remove the files.
/// Prevents files from being left behind after tests.
struct TestFile {
    config_file: File,
    path: &'static str,
}

impl Drop for TestFile {
    fn drop(&mut self) {
        remove_file(self.path).expect("Unable to remove test config file.");
    }
}

/// A helper function to generate a sample configuration string in TOML format.
fn sample_config_toml() -> String {
    r#"
        host = "127.0.0.1"
        port = 25565
        motd = ["hi", "bye"]
        max_players = 100
        network_tick_rate = 20
        world = "default_world"
        network_compression_threshold = 512

        [database]
        cache_size = 4096
        compression = "fast"
        "#
    .to_string()
}

/// A helper function to generate an invalid configuration string in TOML format.
fn invalid_config_toml() -> String {
    r#"
        host = "
        port = 25565
        motd = ["hi", "bye"]
        max_players = 100
        network_tick_rate = 20
        "#
    .to_string()
}

/// Test a sample configuration file in TOML format.
#[test]
fn test_sample_config_toml() {
    // Write the sample config to a temporary file
    let config_str = sample_config_toml();
    let config_file_path = "./test_server_config.toml";

    // Write the configuration to the file
    // TestFile implements Drop, so the file will be removed after the test.
    let mut file = TestFile {
        config_file: File::create(config_file_path).expect("Unable to create test config file."),
        path: config_file_path,
    };
    file.config_file
        .write_all(config_str.as_bytes())
        .expect("Unable to write test config data.");

    // Load the configuration from the file
    let server_config = ServerConfig::new_no_prompt(Some(config_file_path))
        .expect("Failed to read configuration file.");

    // Test the get_global_config function
    let global_config = get_global_config().expect("Failed to get global configuration.");
    assert_eq!(global_config.host, "127.0.0.1");
    assert_eq!(global_config.port, 25565);
    assert_eq!(global_config.motd, vec!["hi", "bye"]);
    assert_eq!(global_config.max_players, 100);
    assert_eq!(global_config.network_tick_rate, 20);
    assert_eq!(global_config.world, "default_world");
    assert_eq!(global_config.network_compression_threshold, 512);
    assert_eq!(global_config.database.cache_size, 4096);
    assert!(matches!(
        global_config.database.compression,
        DatabaseCompression::Fast
    ));

    // Test the values in the ServerConfig struct
    assert_eq!(server_config.host, "127.0.0.1");
    assert_eq!(server_config.port, 25565);
    assert_eq!(server_config.motd, vec!["hi", "bye"]);
    assert_eq!(server_config.max_players, 100);
    assert_eq!(server_config.network_tick_rate, 20);
    assert_eq!(server_config.world, "default_world");
    assert_eq!(server_config.network_compression_threshold, 512);
    assert_eq!(server_config.database.cache_size, 4096);
    assert!(matches!(
        server_config.database.compression,
        DatabaseCompression::Fast
    ));
}

/// Test an invalid configuration file in TOML format.
#[test]
fn test_invalid_config_toml() {
    // Write the invalid config to a temporary file
    let config_str = invalid_config_toml();
    let config_file_path = "./test_invalid_config.toml";

    // Write the configuration to the file
    // TestFile implements Drop, so the file will be removed after the test.
    let mut file = TestFile {
        config_file: File::create(config_file_path).expect("Unable to create test config file."),
        path: config_file_path,
    };
    file.config_file
        .write_all(config_str.as_bytes())
        .expect("Unable to write test config data.");

    // Load the configuration from the file
    let server_config = ServerConfig::new_no_prompt(Some(config_file_path));

    // Test that the configuration could not be loaded
    assert!(server_config.is_err());

    println!("{}", server_config.err().unwrap());
}
