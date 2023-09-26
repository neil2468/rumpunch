use crate::{
    message::{Message, Payload, PayloadKind, ProbeReply, ProbeRequest},
    network_error::NetworkError,
    types::MsgId,
};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::{debug, trace, warn};

const RX_BUF_LEN: usize = 1024;

pub(crate) struct PortTask {
    port: u16,
    socket: UdpSocket,
    rx_buf: [u8; RX_BUF_LEN],
}

impl PortTask {
    pub(crate) async fn new(bind_addr: &SocketAddr) -> Result<Self, NetworkError> {
        let socket = UdpSocket::bind(bind_addr).await?;

        Ok(Self {
            port: socket.local_addr()?.port(), // TODO: is this needed?
            socket,
            rx_buf: [0u8; RX_BUF_LEN],
        })
    }

    /// Run main loop of task.
    pub(crate) async fn main_loop(&mut self) -> ! {
        trace!(?self.port, "PortTask running main loop");
        loop {
            // Wait for datagram
            let (len, peer_addr) = match self.socket.recv_from(&mut self.rx_buf).await {
                Ok((len, peer_addr)) => (len, peer_addr),
                Err(e) => {
                    debug!(?e, "Ignoring receive error");
                    continue;
                }
            };

            // TODO: Implement rate limiting by source SocketAddr

            trace!(
                "Rx on {} from {}: {:02x?}",
                self.port,
                peer_addr,
                &self.rx_buf[..len]
            );

            if let Ok(message) = Message::from_bytes(&self.rx_buf[..len]) {
                debug!(?message);
                if let Err(e) = self.handle_message(&peer_addr, &message).await {
                    warn!("Ignoring error {:?}", e);
                }
            }
        }
    }

    async fn handle_message(
        &mut self,
        peer_addr: &SocketAddr,
        message: &Message,
    ) -> Result<(), NetworkError> {
        // TODO: All of this will change

        match message.kind() {
            PayloadKind::ProbeRequest => {
                // Deserialise request
                let request = ProbeRequest::from_message(message)?;
                trace!(?request, "Received ProbeRequest");

                // Reply
                let payload = ProbeReply {
                    public_addr: peer_addr.clone(), // TODO: Can we avoid this clone?
                };
                self.send_reply(peer_addr, message.msg_id(), payload).await;
            }
            _ => trace!(?message, "Ignoring unexpected message"),
        }

        Ok(())

        // match message.kind() {
        //     PayloadKind::StartRequest => {
        //         let payload = StartRequest::from_message(message)?;
        //         debug!(?payload);

        //         // Process request
        //         let can_continue = self
        //             .state
        //             .connect_requests
        //             .handle_start_request(&payload.connect_to);

        //         // TODO: START HERE. This is wrong. Connection ID should only be
        //         // crated once per peer pair.

        //         // If needed, allocate a connection_id
        //         let connection_id = match can_continue {
        //             true => Some(self.new_connection_id()),
        //             false => None,
        //         };

        //         // Send reply
        //         let payload = StartReply { connection_id };
        //         self.send_reply(peer_addr, message.msg_id(), payload).await;

        //         Ok(())
        //     }
        //     PayloadKind::StopRequest => {
        //         let payload = StopRequest::from_message(message)?;
        //         debug!(?payload);

        //         // Process request
        //         self.state
        //             .connect_requests
        //             .handle_stop_request(message.peer_id(), &payload.connect_to);

        //         // Send reply
        //         self.send_reply(peer_addr, message.msg_id(), Ack {}).await;

        //         Ok(())
        //     }
        //     PayloadKind::SampleRequest => {
        //         let payload = SampleRequest::from_message(message)?;
        //         debug!(?payload);

        //         // TODO: handle properly
        //         // TODO: START HERE store samples in state

        //         let sample = Sample {
        //             peer_id: message.peer_id().clone(),
        //             peer_addr: peer_addr.clone(),
        //             connection_id: payload.connection_id,
        //             src_port: payload.src_port,
        //             seq_number: payload.seq_number,
        //         };

        //         self.state.samples.insert_sample(sample);

        //         // Send reply
        //         self.send_reply(peer_addr, message.msg_id(), Ack {}).await;

        //         Ok(())
        //     }

        //     // TODO: implement other messages
        //     _ => Err(NetworkErrorKind::Protocol(anyhow!(
        //         "ignoring unexpected message {:?}",
        //         message
        //     ))
        //     .into()),
        // }
    }

    async fn send_reply<P>(&self, peer_addr: &SocketAddr, msg_id: MsgId, payload: P)
    where
        P: Payload,
    {
        // Ignore send errors. We expect the client to retry.
        let data = Message::new(msg_id, payload).to_bytes();
        let _ = self.socket.send_to(&data, peer_addr).await;
    }
}

// TODO: add test for rate limiting? and ignoring bad datagrams
