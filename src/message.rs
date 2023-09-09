use crate::{message_error::MessageError, peer_id::PeerId};

extern crate alloc; // TODO: is'nt this old rust?
use postcard::{from_bytes, to_stdvec};
use rand::prelude::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    pub(crate) msg_id: u32,
    pub(crate) peer_id: PeerId,
    pub(crate) kind: Kind,
}

impl Message {
    pub(crate) fn new(msg_id: u32, peer_id: PeerId, kind: Kind) -> Self {
        Self {
            msg_id,
            peer_id,
            kind,
        }
    }

    pub(crate) fn with_random_msg_id(peer_id: PeerId, kind: Kind) -> Self {
        Self::new(random(), peer_id, kind)
    }

    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, MessageError> {
        to_stdvec(self).map_err(|e| MessageError::ToBytes(e.into()))
    }

    pub(crate) fn from_bytes(s: &[u8]) -> Result<Self, MessageError> {
        from_bytes(s).map_err(|e| MessageError::FromBytes(e.into()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Kind {
    Test { peer_src_port: u16 },
    Ack,
}
