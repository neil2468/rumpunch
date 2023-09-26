mod port_task;

use self::port_task::PortTask;
use std::{
    net::{IpAddr, SocketAddr},
    panic,
};
use tokio::task::JoinSet;
use tracing::{error, trace};

const SERVER_BIND_IP: &str = "0.0.0.0";

pub struct RendezvousServer {
    ports: Vec<u16>,
}

// TODO: Should this be built using the builder pattern?
impl RendezvousServer {
    pub fn new(ports: Vec<u16>) -> Self {
        trace!("ports: {:?}", ports);
        RendezvousServer { ports }
    }

    // TODO: don't use anyhow::Error?
    pub async fn blocking_run(&self) -> anyhow::Result<()> {
        let mut set = JoinSet::new();

        set.spawn(Self::monitor_task());

        let bind_ip: IpAddr = SERVER_BIND_IP.parse()?;
        let mut bind_addr = SocketAddr::from((bind_ip, 0));

        for port in &self.ports {
            bind_addr.set_port(*port);
            let mut obj = PortTask::new(&bind_addr).await?;
            set.spawn(async move { obj.main_loop().await });
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

        Ok(())
    }

    async fn monitor_task() -> anyhow::Result<()> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
            error!("Implement me");
        }
    }
}
