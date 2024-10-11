use tokio::io::AsyncWriteExt;
use tracing::info;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::event_handler;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::GlobalState;
use ferrumc_net::packets::incoming::handshake::{HandshakeEvent};
use ferrumc_net::packets::outgoing::status_response::OutgoingStatusResponse;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};

#[event_handler]
async fn handle_handshake(
    handshake_event: HandshakeEvent,
    state: GlobalState,
) -> Result<HandshakeEvent, NetError> {
    info!("Handling handshake event: {:?}", handshake_event.handshake);

    let mut out_stream = state
        .universe
        .get_mut::<StreamWriter>(handshake_event.conn_id)?; 
    
    
    // Check if next state is status.
    if handshake_event.handshake.next_state == 1 {
        // Send status response
        let packet = OutgoingStatusResponse::new(EXAMPLE_JSON.to_string());
        let mut buf = Vec::new();
        packet.encode(&mut buf, &NetEncodeOpts::WithLength)?;
        
        out_stream.write_all(&buf).await?;
    } else {
        // Send login response
    }
    
    
    
    Ok(handshake_event)
}



const EXAMPLE_JSON: &str = r#"{
    "version": {
        "name": "1.19.4",
        "protocol": 762
    },
    "players": {
        "max": 100,
        "online": 5,
        "sample": [
            {
                "name": "thinkofdeath",
                "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
            }
        ]
    },
    "description": {
        "text": "Hello, world!"
    },
    "favicon": "data:image/png;base64,<data>",
    "enforcesSecureChat": false
}"#;