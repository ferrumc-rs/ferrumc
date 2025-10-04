use std::{net::{IpAddr, Ipv4Addr, SocketAddrV4, UdpSocket}, time::{Duration, Instant}};

use bevy_ecs::prelude::*;
use ferrumc_config::server_config::get_global_config;
use local_ip_address::local_ip;
use rand::prelude::IndexedRandom;
use tracing::error;

#[derive(Resource)]
pub struct LanPinger {
    last_sent: Instant,
    socket: UdpSocket,
    addr: SocketAddrV4,
    local_ip: IpAddr,
}

const INTERVAL: Duration = Duration::from_millis(1500);

impl LanPinger {
    pub fn new() -> Result<Self> {
        const ADDR: Ipv4Addr = Ipv4Addr::new(224, 0, 2, 60); // magic mojank numbers
        const PORT: u16 = 4445;
        
        Ok(Self { 
            last_sent: Instant::now() - INTERVAL,
            socket: UdpSocket::bind("0.0.0.0:0")?,
            addr: SocketAddrV4::new(ADDR, PORT),
            local_ip: local_ip()?
        })
    }
    
    pub fn announcement(&self) -> String {
        let cfg = get_global_config();
        let motd = cfg.motd.choose(&mut rand::rng()).unwrap();
        let addr = self.local_ip;
        let port = cfg.port;
        
        format!("[MOTD]{motd}[/MOTD][AD]{addr}:{port}[/AD]")
    }
}

pub fn lan_pinger(mut pinger: ResMut<LanPinger>) {
    let now = Instant::now();
    let elapsed = now.duration_since(pinger.last_sent);
    if elapsed < INTERVAL {
        return
    }
    pinger.last_sent = now;
    
    let announcement = pinger.announcement();
    
    if let Err(err) = pinger.socket.send_to(announcement.as_bytes(), pinger.addr) {
        error!("Failed sending LAN UDP Packet: {err}")
    }
}
