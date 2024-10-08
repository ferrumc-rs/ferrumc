use crate::NetResult;
use ferrumc_config::get_global_config;
use std::net::TcpListener;
use tracing::{debug, error, info};

pub fn create_server_listener() -> NetResult<TcpListener> {
    let config = get_global_config();
    let server_addy = format!("{}:{}", config.host, config.port);

    debug!("Trying to bind to {}", server_addy);

    // let listener = TcpListener::bind(server_addy)?;
    let listener = match TcpListener::bind(&server_addy) {
        Ok(l) => l,
        Err(e) => {
            error!("Failed to bind to addy: {}", server_addy);
            error!("Perhaps the port {} is already in use?", config.port);

            return Err(e.into());
        },
    };

    Ok(listener)
}

pub async fn listen(tcp_listener: TcpListener) -> NetResult<()> {
    info!("Server is listening on [{}]", tcp_listener.local_addr()?);

    Ok(())
}
