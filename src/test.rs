use crate::{
    message::{ProbeReply, ProbeRequest},
    peer::Peer,
};

use std::net::{IpAddr, SocketAddr};
use tracing::{debug, info, warn};

struct ProbeData {
    local_addr: SocketAddr,
    public_addr: SocketAddr,
    server_addr: SocketAddr,
}

pub struct Test {}

impl Test {
    pub async fn test(
        server_ip: IpAddr,
        server_port_from: u16,
        server_port_to: u16,
    ) -> anyhow::Result<()> {
        let mut peer = Peer::new().await?;

        let mut server_addrs = Vec::new();
        for port in server_port_from..=server_port_to {
            let addr = SocketAddr::new(server_ip, port);
            server_addrs.push(addr);
        }

        // Send probes
        let mut data = Vec::<ProbeData>::new();
        let local_addr = peer.local_addr()?;
        for addr in server_addrs {
            for _ in 0..5 {
                let request = ProbeRequest {};
                // TODO: should not allow error to stop our loop

                let val = peer.send_receive::<_, ProbeReply>(request, addr).await;
                match val {
                    Ok(reply) => data.push(ProbeData {
                        local_addr,
                        public_addr: reply.public_addr,
                        server_addr: addr,
                    }),
                    Err(e) => warn!(?e, "Ignoring error"),
                }
            }
        }

        info!("Probe results (chronological)...");
        for d in data {
            info!(
                "  local, public, server = {:?}, {:?}, {:?}",
                d.local_addr, d.public_addr, d.server_addr
            );
        }

        // Analyse probes

        Ok(())
    }
}
