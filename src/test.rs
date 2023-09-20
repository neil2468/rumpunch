use crate::{
    client::Client,
    message::{SampleReply, SampleRequest, StartReply, StartRequest},
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::debug;

const SERVER_1: &str = "127.0.0.1:4000";
const SERVER_2: &str = "127.0.0.1:4001";

pub struct Test {}

impl Test {
    pub async fn test(this_peer_id: &str, that_peer_id: &str) -> anyhow::Result<()> {
        let mut client = Client::new(this_peer_id.into()).await?;

        let server_1_addr = Client::lookup_host(SERVER_1).await?;
        let server_2_addr = Client::lookup_host(SERVER_2).await?;

        ///////////////////////////
        // Try to start
        ///////////////////////////

        // TODO: this should timeout
        loop {
            let payload = StartRequest {
                connect_to: that_peer_id.into(),
            };

            let rx_payload: StartReply = client.send_receive(payload, server_1_addr).await?;
            debug!(?rx_payload, "received reply");

            if rx_payload.can_continue {
                break;
            }

            debug!("sleeping");
            sleep(Duration::from_millis(3000)).await;
        }

        ///////////////////////////
        // Send samples
        ///////////////////////////

        let payload = SampleRequest {
            src_port: client.local_addr()?.port(),
        };
        let _: SampleReply = client.send_receive(payload.clone(), server_1_addr).await?;
        let _: SampleReply = client.send_receive(payload.clone(), server_1_addr).await?;
        let _: SampleReply = client.send_receive(payload.clone(), server_2_addr).await?;
        let _: SampleReply = client.send_receive(payload, server_2_addr).await?;

        // let payload = Payload::Test {
        //     peer_src_port: client.socket_local_addr()?.port(),
        // };

        // client.send_ack(payload.clone(), server_1_addr).await?;
        // client.send_ack(payload.clone(), server_1_addr).await?;
        // client.send_ack(payload.clone(), server_2_addr).await?;
        // client.send_ack(payload, server_2_addr).await?;

        Ok(())
    }
}
