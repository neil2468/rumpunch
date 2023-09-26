use crate::types::PeerId;
use dashmap::{DashMap, DashSet};
use std::{fmt, net::SocketAddr, sync::Mutex};

pub(crate) struct ConnectRequests {
    // key: (peer_a, peer_b), created when peer_a requests connection to peer_b
    // value: true if peer_b has also requested a connection to peer_a
    data: DashMap<(PeerId, PeerId), bool>,
}

impl fmt::Debug for ConnectRequests {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = fmt.debug_list();

        for val in &self.data {
            d.entry(&format!(
                "{} <{:?}> {}",
                val.key().0,
                val.value(),
                val.key().0,
            ));
        }

        d.finish()
    }
}

impl ConnectRequests {
    fn new() -> Self {
        Self {
            data: DashMap::new(),
        }
    }

    /// Record a request to start a connection from one peer to another.
    ///
    /// Returns `true` if both `from_peer` and `to_peer` have requested
    /// a connection to each other.
    pub(crate) fn handle_start_request(&self, from_peer: &PeerId, to_peer: &PeerId) -> bool {
        // TODO: Remove these clones. The API of DashMap does not make it easy.
        // See https://stackoverflow.com/questions/45786717/how-to-implement-hashmap-with-two-keys/45795699#45795699
        let key_rev = (to_peer.clone(), from_peer.clone());

        // Case: the other peer has already requested a connection
        if let Some(mut val) = self.data.get_mut(&key_rev) {
            *val.value_mut() = true;
            return true;
        }

        // Case: this peer has previously requested a connection and
        // is waiting on the other peer
        let key = (key_rev.1, key_rev.0);
        if let Some(val) = self.data.get(&key) {
            return *val;
        }

        // Case: this is the first time either peer has requested a connection
        self.data.insert(key, false);
        false
    }

    /// Record a request to stop a connection between peers.
    pub(crate) fn handle_stop_request(&self, from_peer: &PeerId, to_peer: &PeerId) {
        // TODO: Remove these clones. The API of DashMap does not make it easy.
        // See https://stackoverflow.com/questions/45786717/how-to-implement-hashmap-with-two-keys/45795699#45795699
        let key = (from_peer.clone(), to_peer.clone());
        if self.data.remove(&key).is_none() {
            let key_rev = (key.1, key.0);
            self.data.remove(&key_rev);
        }
    }
}

#[derive(Debug)]
pub(crate) struct Sample {
    /// Peer that sent the sample
    pub(crate) peer_id: PeerId,

    /// What the server received as the source address of the message
    pub(crate) peer_addr: SocketAddr,

    /// Connection ID this sample relates to
    pub(crate) connection_id: u32,

    /// The local port number the peer used to send the message
    pub(crate) src_port: u16,

    /// Message sequence number
    pub(crate) seq_number: u16,
}

#[derive(Debug)]
pub(crate) struct Samples {
    data: Mutex<Vec<Sample>>,
}

impl Samples {
    pub(crate) fn new() -> Self {
        Self {
            data: Mutex::new(Vec::new()),
        }
    }

    pub(crate) fn insert_sample(&self, sample: Sample) {
        let mut data = self.data.lock().expect("Error with mutex for samples");
        data.push(sample);
    }
}

// TODO: Persist state on disk.
// TODO: Expire data (for example connection requests)
/// Store for state of server.
#[derive(Debug)]
pub(crate) struct State {
    pub(crate) connect_requests: ConnectRequests,
    pub(crate) connect_ids: DashSet<u32>,
    pub(crate) samples: Samples,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            connect_requests: ConnectRequests::new(),
            connect_ids: DashSet::new(),
            samples: Samples::new(),
        }
    }
}
