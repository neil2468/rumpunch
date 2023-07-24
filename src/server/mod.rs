use tokio::task::JoinSet;
use tracing::debug;

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

        for p in &self.ports {
            set.spawn(Self::task(*p));
        }

        while set.join_next().await.is_some() {}
    }

    async fn task(port: u16) {
        // TODO: should return a Result for errors?
        debug!("Task started for port {}", port);

        tokio::time::sleep(tokio::time::Duration::from_millis(2500)).await;
    }
}
