use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlayerIdentity {
    //rename to name
    #[serde(rename = "name")] //vanilla uses name (at least in whitelist)
    pub username: String,
    #[serde(
        deserialize_with = "uuid_to_u128_encoded_uuid",
        serialize_with = "encoded_u128_uuid_to_uuid_str"
    )] //tested deserializing, not serializing
    pub uuid: u128,
}

impl PlayerIdentity {
    pub fn new(username: String, uuid: u128) -> Self {
        Self { username, uuid }
    }
}

fn uuid_to_u128_encoded_uuid<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let uuid_str: String = Deserialize::deserialize(deserializer)?;
    let id = uuid::Uuid::try_parse(&uuid_str)
        .map_err(|e| serde::de::Error::custom(format!("failed to parse UUID: {e}")))?;
    let bytes = id.as_bytes();
    let mut u = 0u128;
    for &b in bytes {
        u = (u << 8) | b as u128;
    }
    Ok(u)
}

fn encoded_u128_uuid_to_uuid_str<S>(uuid: &u128, serializer: S) -> Result<S::Ok, S::Error>
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
