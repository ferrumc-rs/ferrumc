use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum NetAuthenticationError {
    #[error("Failed to reach Mojang's authentication servers")]
    CouldNotReachMojang,

    #[error("The server has exceeded the rate limit allowed by Mojang")]
    RateLimitReached,

    #[error("The user could not be authenticated")]
    FailedToAuthenticate,
}