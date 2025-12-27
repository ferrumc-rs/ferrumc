use crate::errors::NetAuthenticationError;
use base64::Engine;
use ferrumc_core::identity::player_identity::PlayerProperty;
use ferrumc_net_encryption::minecraft_hex_digest;
use serde_derive::Deserialize;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

/// Authenticates the given player with Mojang's session server.
///
/// # Parameters
/// - `username`: The username of the player to be authenticated. Should be from the login start packet.
/// - `server_id`: The server id sent in the encryption request packet. In Minecraft 1.7+, this value should be an empty string.
/// - `shared_secret`: The decrypted shared secret returned by the client in an encryption response packet.
///
/// # Returns
/// `(String, Uuid, Vec<PlayerProperty>)`: The data returned by Mojang upon successful authentication.
/// - `String`: The official username of the player from Mojang.
/// - `Uuid`: The official uuid of the player from Mojang.
/// - `Vec<PlayerProperty>`: The player properties returned by Mojang.
///
/// # Error
/// A `NetAuthenticationError` upon an unsuccessful authentication. See the `NetAuthenticationError` enum for more information.
pub(crate) async fn authenticate_user(
    username: &str,
    server_id: &str,
    shared_secret: &[u8],
) -> Result<(String, Uuid, Vec<PlayerProperty>), NetAuthenticationError> {
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
        username,
        minecraft_hex_digest(server_id, shared_secret),
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|_| NetAuthenticationError::CouldNotReachMojang)?;

    match response.status().as_u16() {
        200 => Ok(()),
        204 => Err(NetAuthenticationError::FailedToAuthenticate),
        404 => Err(NetAuthenticationError::BadURL),
        429 => Err(NetAuthenticationError::RateLimitReached),
        code => Err(NetAuthenticationError::UnknownStatusError(code)),
    }?;

    let response = response
        .json::<MojangAuthResponse>()
        .await
        .map_err(|err| NetAuthenticationError::ParseError(Arc::new(err)))?;

    let username = response.name.clone();
    let uuid = Uuid::from_str(&response.id).map_err(|_| NetAuthenticationError::CorruptUuid)?;
    let mut properties: Vec<PlayerProperty> = vec![];

    for property in response.properties {
        properties.push(PlayerProperty {
            name: property.name,
            signature: if property.signature.is_empty() {
                None
            } else {
                Some(property.signature)
            },
            value: String::from_utf8(
                base64::engine::general_purpose::STANDARD
                    .decode(&property.value)
                    .map_err(|err| NetAuthenticationError::ParseError(Arc::new(err)))?,
            )
            .map_err(|err| NetAuthenticationError::ParseError(Arc::new(err)))?,
        })
    }

    Ok((username, uuid, properties))
}

// Helper structs to decode Mojang's json response
#[derive(Deserialize)]
struct MojangAuthResponse {
    name: String,
    id: String,
    properties: Vec<MojangAuthProperty>,
}

#[derive(Deserialize, Clone)]
struct MojangAuthProperty {
    name: String,
    value: String,
    signature: String,
}
