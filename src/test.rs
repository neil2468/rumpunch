use crate::message::Message;
use tokio::net::UdpSocket;

pub struct Test {}

impl Test {
    pub async fn test() -> anyhow::Result<()> {
        let socket = UdpSocket::bind(("0.0.0.0", 0)).await?;

        let message = Message {
            peer_id: "abc".into(),
            msg_id: Message::random_msg_id(),
            kind: crate::message::Kind::Test(crate::message::Test {
                peer_src_port: socket.local_addr()?.port(),
            }),
        };

        let data = message.to_allocvec()?;

        socket.send_to(&data, ("127.0.0.1", 4000)).await?;

        Ok(())
    }
}
