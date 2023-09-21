use crate::{
    message::{Message, Payload, PayloadKind},
    network_error::{NetworkError, NetworkErrorKind},
    types::{MsgId, PeerId},
};
use anyhow::anyhow;
use rand::random;
use std::{net::SocketAddr, time::Duration};
use tokio::{
    net::{lookup_host, ToSocketAddrs, UdpSocket},
    time::timeout,
};
use tracing::{debug, trace};

const ACK_TIMEOUT: Duration = Duration::from_millis(3000);
const RX_BUF_LEN: usize = 1024;

pub(crate) struct Peer {
    /// Id of this client
    id: PeerId,

    /// For creating message ids
    current_msg_id: MsgId,

    /// Udp socket
    socket: UdpSocket,

    /// Datagram receive buffer
    rx_buf: [u8; RX_BUF_LEN],
}

impl<'a> Peer {
    pub(crate) async fn new(id: PeerId) -> Result<Self, NetworkError> {
        let socket = UdpSocket::bind(("0.0.0.0", 0)).await?;
        Ok(Self {
            id,
            current_msg_id: random(),
            socket,
            rx_buf: [0u8; RX_BUF_LEN],
        })
    }

    pub(crate) fn local_addr(&self) -> Result<SocketAddr, NetworkError> {
        self.socket.local_addr().map_err(|e| e.into())
    }

    fn new_message<P: Payload>(&mut self, payload: P) -> Message {
        // Get next message id
        (self.current_msg_id, _) = self.current_msg_id.overflowing_add(1);

        // Create message
        Message::new(self.id.clone(), self.current_msg_id, payload)
    }

    /// Send a message and receive a reply
    pub(crate) async fn send_receive<S, R>(
        &mut self,
        send_payload: S,
        addr: SocketAddr,
    ) -> Result<R, NetworkError>
    where
        S: Payload,
        R: Payload,
    {
        let tx_msg = self.new_message(send_payload);
        debug!(?tx_msg, ?addr, "Sending message");
        let data = tx_msg.to_bytes();

        self.socket.connect(addr).await?;
        self.socket.send(&data).await?;

        // Try to receive a message within a timeout
        let res = timeout(ACK_TIMEOUT, self.receive(tx_msg.msg_id(), &R::KIND)).await;

        // Handle timeout errors
        let rx_msg = res.map_err(|e| NetworkErrorKind::SendReceive(e.into()))?;

        // Deserialise payload
        let rx_payload = R::from_message(&rx_msg)?;

        Ok(rx_payload)
    }

    /// Receives messages until an expected message is received.
    async fn receive(&mut self, msg_id: MsgId, expect_payload: &PayloadKind) -> Message {
        let mut msg: Option<Message> = None;

        while msg.is_none() {
            // Receive a datagram, log and ignore errors
            let len = match self.socket.recv(&mut self.rx_buf).await {
                Ok(len) => len,
                Err(e) => {
                    trace!(?e, "Ignoring receive error");
                    continue;
                }
            };

            // Deserialise message, log and ignore errors
            let rx_msg = match Message::from_bytes(&self.rx_buf[..len]) {
                Ok(msg) => msg,
                Err(e) => {
                    trace!(?e, "Ignoring deserialization error");
                    continue;
                }
            };

            // Validate msg_id
            if msg_id != rx_msg.msg_id() {
                trace!(?rx_msg, "Ignoring wrong msg_id");
                continue;
            }

            // Validate payload kind
            if expect_payload != rx_msg.kind() {
                trace!(?rx_msg, "Ignoring unexpected payload kind");
                continue;
            }

            // All okay
            trace!(?rx_msg, "Received okay");
            msg = Some(rx_msg);
        }

        msg.unwrap()
    }

    /// DNS resolve a hostname to a SocketAddr
    /// We use the first address resolved
    pub(crate) async fn lookup_host<T: ToSocketAddrs>(host: T) -> anyhow::Result<SocketAddr> {
        let mut iter = lookup_host(host).await?;
        match iter.next() {
            Some(a) => Ok(a),
            None => Err(anyhow!("Error looking up host's socket address")),
        }
    }
}
