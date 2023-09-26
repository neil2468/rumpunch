use crate::{
    message::{ProbeReply, ProbeRequest},
    peer::Peer,
};

use std::net::SocketAddr;
use tracing::{debug, info};

const SERVER_1: &str = "127.0.0.1:4000";
const SERVER_2: &str = "127.0.0.1:4001";
const SERVER_3: &str = "127.0.0.1:4002";

struct ProbeData {
    local_addr: SocketAddr,
    public_addr: SocketAddr,
}

pub struct Test {}

impl Test {
    pub async fn test() -> anyhow::Result<()> {
        let mut peer = Peer::new().await?;

        let mut server_addrs = Vec::new();
        server_addrs.push(Peer::lookup_host(SERVER_1).await?);
        server_addrs.push(Peer::lookup_host(SERVER_2).await?);
        server_addrs.push(Peer::lookup_host(SERVER_3).await?);

        // TODO: rewrite all

        // Send probes
        let mut data = Vec::<ProbeData>::new();

        let local_addr = peer.local_addr()?;
        for addr in server_addrs {
            for _ in 0..5 {
                let request = ProbeRequest {};
                // TODO: should not allow error to stop our loop
                let reply: ProbeReply = peer.send_receive(request, addr).await?;
                debug!(?reply, "Received ProbeReply");

                data.push(ProbeData {
                    local_addr,
                    public_addr: reply.public_addr,
                });
            }
        }

        info!("Probe results...");
        for d in data {
            info!("  local, public = {:?}, {:?}", d.local_addr, d.public_addr);
        }

        // let mut connection_id = None;
        // loop {
        //     let payload = StartRequest {
        //         connect_to: that_peer_id.into(),
        //     };

        //     let server_addr = match this_peer_id == "bob" {
        //         true => server_2_addr,
        //         false => server_1_addr,
        //     };

        //     let rx_payload: StartReply = client.send_receive(payload, server_addr).await?;
        //     debug!(?rx_payload, "received reply");

        //     connection_id = rx_payload.connection_id;
        //     if connection_id.is_some() {
        //         break;
        //     }

        //     debug!("sleeping");
        //     sleep(Duration::from_millis(3000)).await;
        // }

        // ///////////////////////////
        // // Send samples
        // ///////////////////////////
        // let connection_id = connection_id.unwrap();

        // let src_port = client.local_addr()?.port();
        // for seq_number in 0..10 {
        //     let payload = SampleRequest {
        //         connection_id,
        //         src_port,
        //         seq_number,
        //     };

        //     let server_addr = match seq_number > 4 {
        //         true => server_1_addr,
        //         false => server_2_addr,
        //     };

        //     let _: Ack = client.send_receive(payload, server_addr).await?;
        // }

        // sleep(Duration::from_millis(3000)).await;

        // // let payload = Payload::Test {
        // //     peer_src_port: client.socket_local_addr()?.port(),
        // // };

        // // client.send_ack(payload.clone(), server_1_addr).await?;
        // // client.send_ack(payload.clone(), server_1_addr).await?;
        // // client.send_ack(payload.clone(), server_2_addr).await?;
        // // client.send_ack(payload, server_2_addr).await?;

        // let payload = StopRequest {
        //     connect_to: that_peer_id.into(),
        // };
        // let rx_payload: Ack = client.send_receive(payload, server_1_addr).await?;
        // debug!(?rx_payload, "received reply");

        Ok(())
    }
}
