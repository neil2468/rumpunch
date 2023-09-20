mod port_task;
mod state;

use self::{port_task::PortTask, state::State};
use std::{panic, sync::Arc};
use tokio::task::JoinSet;
use tracing::{debug, error, trace};

const SERVER_ID: &str = "server";

pub struct RendezvousServer {
    ports: Vec<u16>,
    state: Arc<State>,
}

// TODO: Should this be built using the builder pattern?
impl RendezvousServer {
    pub fn new(ports: Vec<u16>) -> Self {
        trace!("ports: {:?}", ports);
        RendezvousServer {
            ports,
            state: Arc::new(State::new()),
        }
    }

    // TODO: don't use anyhow::Error?
    pub async fn blocking_run(&self) -> anyhow::Result<()> {
        let mut set = JoinSet::new();

        set.spawn(Self::monitor_task(self.state.clone()));

        for port in &self.ports {
            let mut obj = PortTask::new(port.clone(), self.state.clone(), SERVER_ID.into()).await?;
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

    async fn monitor_task(state: Arc<State>) -> anyhow::Result<()> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

            debug!(?state);
        }
    }
}
