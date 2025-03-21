use crate::NetResult;
use ferrumc_config::statics::get_global_config;
use std::net::TcpListener;
use tracing::{debug, error};

pub fn create_server_listener() -> NetResult<TcpListener> {
    let config = get_global_config();
    let server_addy = format!("{}:{}", config.host, config.port);
    let server_addy = server_addy.as_str();

    debug!("Trying to bind to {}", server_addy);

    // let listener = TcpListener::bind(server_addy)?;
    let listener = match TcpListener::bind(server_addy) {
        Ok(l) => l,
        Err(e) => {
            error!("Failed to bind to addy: {}", server_addy);
            error!("Perhaps the port {} is already in use?", config.port);

            return Err(e.into());
        }
    };

    Ok(listener)
}
