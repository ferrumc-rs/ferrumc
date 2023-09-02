use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;
use crate::server::packet::{decode_packet, handle_packet};

mod handshake;
mod packet;
mod status;


pub async fn run_server(listener: TcpListener) -> Result<()> {
    loop {
        let (socket, _) = listener.accept().await?;
        // println!("Connection established to {:?}", socket.peer_addr()?);

        tokio::spawn(async move {
            handle_client(socket).await.unwrap_or_else(|e| {
                println!("An error occurred while handling the client: {:?}", e);
            });
        });
    }
}

async fn handle_client(mut socket: TcpStream) -> Result<()> {
    let mut buf = vec![0u8; 1024];
    match socket.read(&mut buf).await {
        Ok(0) => {
            // Connection was closed
            println!("Connection closed");
        }
        Ok(n) => {
         // let text = String::from_utf8_lossy(&buf[0..n]);
         //    println!("Received: {:?}", &buf[0..n]);

            match decode_packet(&buf[0..n]).await{
                Ok(packet) => {
                    handle_packet(packet, &mut socket).await?;
                }
                Err(e) => println!("Error decoding packet: {:?}", e),
            }
        }
        Err(err) => {
            // Handle the error
            // println!("An error occurred: {:?}", err);
        }
    }

    Ok(())
}

// pub async fn run_server(listener: TcpListener) -> Result<(), Box<dyn std::error::Error + Send>> {
//     loop {
//         let (socket, _) = listener.accept().await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
//         println!("Connection established");
//
//         tokio::spawn(async move {
//             if let Err(e) = handle_client(socket).await {
//                 println!("An error occurred while handling the client: {:?}", e);
//             }
//         });
//     }
// }
//
// #[derive(Debug)]
// struct SendErrorWrapper(Box<dyn std::error::Error>);
//
// impl std::fmt::Display for SendErrorWrapper {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "SendErrorWrapper: {}", self.0)
//     }
// }
//
// impl std::error::Error for SendErrorWrapper {}
//
// pub async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error + Send>> {
//     let mut buf = vec![0u8; 1024];
//
//     match socket.read(&mut buf).await {
//         Ok(0) => {
//             // Connection was closed
//             println!("Connection closed");
//         }
//         Ok(n) => {
//             // Log the data received (for now, print it)
//             println!("Received: {:?}", &buf[0..n]);
//             // Send back "Hello, World!"
//             // socket.write_all(b"Hello, World!").await?;
//
//             let mut buf = [0; 1024];
//             // let _ = socket.read(&mut buf).await?;
//
//             println!("Received packet: {:?}", &buf);
//
//             match decode_packet(&buf).await {
//                 Ok(packet) => {
//                     handle_packet(packet, &mut socket).await?;
//                 }
//                 Err(e) => println!("Error decoding packet: {:?}", e),
//             }
//         }
//         Err(err) => {
//             // Handle the error
//             println!("An error occurred: {:?}", err);
//         }
//     }
//
//     Ok(())
// }

//             match decode_packet(&buf).await {
//                 Ok(packet) => {
//                     handle_packet(packet, &mut socket).await?;
//                 }
//                 Err(e) => println!("Error decoding packet: {:?}", e),
//             }