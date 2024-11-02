#[cfg(feature = "redb")]
pub mod redb;
#[cfg(feature = "rocksdb")]
pub mod rocksdb;

#[cfg(feature = "sled")]
pub mod sled;

#[cfg(feature = "surrealkv")]
pub mod surrealkv;

#[cfg(not(any(
    feature = "redb",
    feature = "rocksdb",
    feature = "sled",
    feature = "surrealkv"
)))]
compile_error!("At least one storage backend must be enabled");

pub fn backend_enabled(backend: &str) -> bool {
    match backend {
        "redb" => cfg!(feature = "redb"),
        "rocksdb" => cfg!(feature = "rocksdb"),
        "sled" => cfg!(feature = "sled"),
        "surrealkv" => cfg!(feature = "surrealkv"),
        _ => false,
    }
}
