use std::panic;
use tokio::task::JoinSet;
use tracing::{debug, error};

pub struct RendezvousServer {
    ports: Vec<u16>,
}

// TODO: Should this be built using the builder pattern?
impl RendezvousServer {
    pub fn new(ports: Vec<u16>) -> Self {
        debug!("ports: {:?}", ports);
        RendezvousServer { ports }
    }

    pub async fn blocking_run(&self) {
        let mut set = JoinSet::new();
        set.spawn(Self::good_task());
        set.spawn(Self::bad_task());

        // Wait on tasks
        // If a task panics, re-throw the panic
        while let Some(res) = set.join_next().await {
            if let Err(join_error) = res {
                if join_error.is_panic() {
                    let tmp = join_error.into_panic();
                    error!(
                        "Unexpected task panic: {}",
                        tmp.downcast_ref::<&str>().unwrap_or(&"")
                    );
                    // Resume the panic on the main task
                    // Copied from `tokio::task::JoinError::into_panic()` docs
                    panic::resume_unwind(tmp);
                } else {
                    error!("Unexpected task error: {join_error:?}");
                }
            }
        }
    }

    async fn good_task() {
        debug!("Started");
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        debug!("Finished");
    }

    async fn bad_task() {
        debug!("Started");
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
        panic!("bad_task() panicked");
    }
}
