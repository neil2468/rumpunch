use crate::message::{Kind, Message};
use tokio::net::UdpSocket;
use tracing::debug;

pub struct Test {}

impl Test {
    pub async fn test() -> anyhow::Result<()> {
        let socket = UdpSocket::bind(("0.0.0.0", 0)).await?;

        let kind = Kind::Test {
            peer_src_port: socket.local_addr()?.port(),
        };
        let message = Message::with_random_msg_id("abc".into(), kind);
        debug!("message: {:?}", message);
        debug!("message.size_of(): {}", std::mem::size_of::<Message>());
        let data = message.to_bytes()?;
        debug!("data.size_of(): {}", data.len());

        let kind = Kind::Ack;
        let message = Message::with_random_msg_id("abc".into(), kind);
        debug!("message: {:?}", message);
        debug!("message.size_of(): {}", std::mem::size_of::<Message>());
        let data = message.to_bytes()?;
        debug!("data.size_of(): {}", data.len());

        socket.send_to(&data, ("127.0.0.1", 4000)).await?;

        Ok(())
    }
}
