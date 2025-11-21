use thiserror::Error;
use ferrumc_core::identity::player_identity::PlayerProperty;

#[derive(Debug, Clone, Error)]
pub enum NetAuthenticationError {
    #[error("Failed to reach Mojang's authentication servers")]
    CouldNotReachMojang,

    #[error("The server has exceeded the rate limit allowed by Mojang")]
    RateLimitReached,

    #[error("The user could not be authenticated")]
    FailedToAuthenticate,
}

pub(crate) async fn authenticate_user(username: &str, server_id: &str) -> Result<Vec<PlayerProperty>, NetAuthenticationError> {
    Ok(Vec::new())
}