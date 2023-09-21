use crate::types::PeerId;
use dashmap::DashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
enum ConnectState {
    StartRequested,
    AllStartsRequested,
}

pub(crate) struct ConnectRequests {
    // key: (peer_a, peer_b)
    data: DashMap<(PeerId, PeerId), ConnectState>,
}

impl fmt::Debug for ConnectRequests {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = fmt.debug_list();

        for val in &self.data {
            d.entry(&format!(
                "{} <{:?}> {}",
                val.key().0,
                *val.value(),
                val.key().1,
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
    pub(crate) fn handle_start_request(&self, from_peer: PeerId, to_peer: PeerId) -> bool {
        // TODO: We should only need referneces to the PeerIds , for comparison
        let key_rev = (to_peer, from_peer);

        // Case: the other peer has already requested a connection
        if let Some(mut val) = self.data.get_mut(&key_rev) {
            *val.value_mut() = ConnectState::AllStartsRequested;
            return true;
        }

        // Case: this peer has previously requested a connection and
        // is waiting on the other peer
        let key = (key_rev.1, key_rev.0);
        if let Some(val) = self.data.get(&key) {
            return *val == ConnectState::AllStartsRequested;
        }

        // Case: this is the first time either peer has requested a connection
        self.data.insert(key, ConnectState::StartRequested);
        false
    }

    /// Record a request to stop a connection from one peer to another.
    pub(crate) fn handle_stop_request(&self, from_peer: PeerId, to_peer: PeerId) {
        // TODO: We should only need referneces to the PeerIds , for comparison
        let key = (from_peer, to_peer);
        if self.data.remove(&key).is_none() {
            let key_rev = (key.1, key.0);
            self.data.remove(&key_rev);
        }
    }
}

// TODO: Persist state on disk.
/// Store for state of server.
#[derive(Debug)]
pub(crate) struct State {
    pub(crate) connect_requests: ConnectRequests,
}
impl State {
    pub(crate) fn new() -> Self {
        Self {
            connect_requests: ConnectRequests::new(),
        }
    }
}
