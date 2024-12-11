use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerIdentity {
    //rename to name
    #[serde(rename = "name")] //vanilla support (at least in whitelist)
    pub username: String,
    #[serde(
        deserialize_with = "uuid_str_to_u128",
        serialize_with = "uuid_u128_to_str"
    )]
    pub uuid: u128,
}

impl PlayerIdentity {
    pub fn new(username: String, uuid: u128) -> Self {
        Self { username, uuid }
    }
}

fn uuid_str_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let uuid_str: String = Deserialize::deserialize(deserializer)?;
    let id = uuid::Uuid::parse_str(&uuid_str)
        .map_err(|e| serde::de::Error::custom(format!("Failed to parse UUID: {e}")))?;
    let bytes = id.as_bytes();
    let mut u = 0u128;
    for &b in bytes {
        u = (u << 8) | b as u128;
    }
    Ok(u)
}

fn uuid_u128_to_str<S>(uuid: &u128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut bytes = [0u8; 16];
    for i in 0..16 {
        bytes[15 - i] = (*uuid >> (i * 8)) as u8;
    }
    let id = uuid::Uuid::from_bytes(bytes);
    serializer.serialize_str(&id.hyphenated().to_string())
}
