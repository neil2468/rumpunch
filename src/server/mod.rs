use crate::{message::Message, peer_id::PeerId};
use dashmap::DashMap;
use std::{net::IpAddr, panic, sync::Arc, time::Instant};
use tokio::{net::UdpSocket, task::JoinSet};
use tracing::{debug, error, trace, warn};

#[derive(Debug)]
struct TestEvent {
    timestamp: Instant,
    peer_src_port: u16,
    local_src_ip: IpAddr,
    local_src_port: u16,
}

#[derive(Debug)]
struct PeerData {
    events: Vec<TestEvent>,
}

pub struct RendezvousServer {
    ports: Vec<u16>,
    data: Arc<DashMap<PeerId, PeerData>>,
}

// TODO: Should this be built using the builder pattern?
impl RendezvousServer {
    pub fn new(ports: Vec<u16>) -> Self {
        debug!("ports: {:?}", ports);
        RendezvousServer {
            ports,
            data: Arc::new(DashMap::new()),
        }
    }

    pub async fn blocking_run(&self) {
        let mut set = JoinSet::new();

        set.spawn(Self::monitor_task(self.data.clone()));

        for port in &self.ports {
            set.spawn(Self::port_task(*port, self.data.clone()));
        }

        // Wait on tasks
        // If a task panics, re-throw the panic
        while let Some(res) = set.join_next().await {
            if let Err(err) = res {
                if err.is_panic() {
                    let panic_obj = err.into_panic();
                    error!(
                        "Unexpected task panic: {}",
                        panic_obj.downcast_ref::<&str>().unwrap_or(&"")
                    );
                    // Resume the panic on the main task
                    // Copied from `tokio::task::JoinError::into_panic()` docs
                    panic::resume_unwind(panic_obj);
                } else {
                    error!("Unexpected task error: {err:?}");
                }
            }
        }
    }

    async fn monitor_task(data: Arc<DashMap<PeerId, PeerData>>) -> anyhow::Result<()> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

            debug!("Data...");
            for item in data.iter() {
                let peer_id = item.key();
                let peer_data = item.value();
                debug!("  {peer_id:?}, {peer_data:?}");
            }
        }
    }

    // TODO: replace anyhow error with something more specific?
    async fn port_task(port: u16, data: Arc<DashMap<PeerId, PeerData>>) -> anyhow::Result<()> {
        debug!("Started. port = {port}");

        let socket = UdpSocket::bind(("0.0.0.0", port)).await?;
        let mut buf = [0; 10240]; // TODO: Is this large enough? Use a vec?

        loop {
            let (len, addr) = socket.recv_from(&mut buf).await?;

            debug!("Rx on {}: {:02x?}", port, &buf[..len]);
            debug!("dst_port: {}", socket.local_addr()?.port());
            debug!("local_src_ip: {}", addr.ip());
            debug!("local_src_port: {}", addr.port());

            // TODO: log Err case to tracing.
            if let Ok(message) = Message::from_bytes(&buf[..len]) {
                debug!("message: {:?}", message);

                match message.kind {
                    crate::message::Kind::Test { peer_src_port } => {
                        debug!("peer_src_port: {}", peer_src_port);
                    }
                    _ => warn!("Ignoring unexpected message: {:?}", message),
                }
            }
        }
    }
}
