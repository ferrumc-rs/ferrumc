use config::ConfigError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    TomlSe(#[from] toml::ser::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("Connection not found: {0}")]
    ConnectionNotFound(u32),
    #[error("Invalid packet id: {0}")]
    InvalidPacketId(u32),
    #[error("Invalid state: {0:x}")]
    InvalidState(i32),
    #[error("Invalid Connection Metadata: {0}")]
    InvalidConnectionMetadata(String),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    ECSError(#[from] crate::ecs::error::Error),

    #[error(transparent)]
    FastAnvilError(#[from] fastanvil::Error),
    #[error("Chunk at ({0}, {1}) not found")]
    ChunkNotFound(i32, i32),

    #[error(transparent)]
    SimdNbtError(#[from] simdnbt::Error),
    #[error("Invalid NBT: {0}")]
    InvalidNbt(String),
    #[error(transparent)]
    NbtDeserializeError(#[from] simdnbt::DeserializeError),
}
