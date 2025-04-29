use crate::errors::NetError;
use ferrumc_config::statics::get_global_config;
use tokio::net::TcpListener;
use tracing::{debug, error};

pub async fn create_server_listener() -> Result<TcpListener, NetError> {
    let config = get_global_config();
    let server_addy = format!("{}:{}", config.host, config.port);
    let server_addy = server_addy.as_str();

    debug!("Trying to bind to {}", server_addy);

    let listener = match TcpListener::bind(server_addy).await {
        Ok(l) => Ok::<TcpListener, std::io::Error>(l),
        Err(e) => {
            error!("Failed to bind to addy: {}", server_addy);
            error!("Perhaps the port {} is already in use?", config.port);

            Err(e)
        }
    };

    Ok(listener?)
}
