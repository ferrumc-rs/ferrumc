use std::sync::Arc;
use crate::{NetResult, ServerState};
use ferrumc_config::statics::get_global_config;
use tokio::net::TcpListener;
use tracing::{debug, error, info, info_span, Instrument};
use crate::connection::handle_connection;

pub async fn create_server_listener() -> NetResult<TcpListener> {
    let config = get_global_config();
    let server_addy = format!("{}:{}", config.host, config.port);

    debug!("Trying to bind to {}", server_addy);

    // let listener = TcpListener::bind(server_addy)?;
    let listener = match TcpListener::bind(&server_addy).await {
        Ok(l) => l,
        Err(e) => {
            error!("Failed to bind to addy: {}", server_addy);
            error!("Perhaps the port {} is already in use?", config.port);

            return Err(e.into());
        }
    };

    Ok(listener)
}

pub async fn listen(net_state: Arc<ServerState>, tcp_listener: TcpListener) -> NetResult<()> {
    info!("Server is listening on [{}]", tcp_listener.local_addr()?);

    
    loop {
        let (stream, _) = tcp_listener.accept().await?;
        let addy = stream.peer_addr()?;
        debug!("Accepted connection from: {}", addy);
        tokio::task::spawn(
            handle_connection(Arc::clone(&net_state), stream)
                .instrument(info_span!("conn", %addy).or_current())
        );
    }


    #[allow(unreachable_code)]
    Ok(())
}
