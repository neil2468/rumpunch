use crate::types::PeerId;
use dashmap::DashMap;
use std::fmt;

pub(crate) struct ConnectRequests {
    // key: (peer_a, peer_b)
    // value: true if both peers have requested a connect
    data: DashMap<(PeerId, PeerId), bool>,
}

impl fmt::Debug for ConnectRequests {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = fmt.debug_list();

        for val in &self.data {
            d.entry(&format!(
                "{} <{}> {}",
                val.key().0,
                if *val.value() { &"T" } else { &"F" },
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

    /// Process aand store a request to create a connection from one peer to
    /// another.
    ///
    /// Returns `true` if the both `from_peer` and `to_peer` have requested
    /// a connection to each other.
    pub(crate) fn process_request(&self, from_peer: PeerId, to_peer: PeerId) -> bool {
        let key_rev = (to_peer, from_peer);

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
