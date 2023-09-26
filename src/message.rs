use std::net::SocketAddr;

use crate::{
    network_error::{NetworkError, NetworkErrorKind},
    types::MsgId,
};
use anyhow::anyhow;
use postcard::{from_bytes, to_stdvec};
use serde::{Deserialize, Serialize};

// TODO: define / calc a max datagram length so recv() can know size for
// rx_buffer and discard over long datagrams. Can only do this if PeerId
// has a max length. Also, postcard has variable length encoding and adds
// overhead; does this compilcate things?

// TODO: Make these messages serialise compatible with other languages &
// platforms.

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    /// Id of message, for matching with reply
    msg_id: MsgId,

    /// Payload kind
    kind: PayloadKind,

    /// Payload bytes
    payload: Vec<u8>,
}

impl Message {
    pub(crate) fn new<P>(msg_id: MsgId, payload: P) -> Self
    where
        P: Payload,
    {
        Self {
            msg_id,
            kind: P::KIND.clone(),
            payload: payload.to_bytes(),
        }
    }

    pub(crate) fn msg_id(&self) -> MsgId {
        self.msg_id
    }

    pub(crate) fn kind(&self) -> &PayloadKind {
        &self.kind
    }

    /// Serialise message to bytes
    ///
    /// # Panics
    ///
    /// Panics if serialisation fails.
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        // If this errors it's probably a bug
        to_stdvec(self).expect("error serialising message")
    }

    /// Deserialise from bytes
    pub(crate) fn from_bytes(s: &[u8]) -> Result<Self, NetworkError> {
        from_bytes(s).map_err(|e| NetworkErrorKind::Deserialize(e.into()).into())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[non_exhaustive]
pub(crate) enum PayloadKind {
    Ack,
    ProbeRequest,
    ProbeReply,
}

pub(crate) trait Payload: Serialize + for<'a> Deserialize<'a> {
    const KIND: PayloadKind;

    fn to_bytes(self) -> Vec<u8> {
        // If this errors it's probably a bug
        to_stdvec(&self).expect("Error serialising message payload")
    }

    fn from_message(message: &Message) -> Result<Self, NetworkError> {
        match message.kind() == &Self::KIND {
            true => Ok(from_bytes(&message.payload)
                .map_err(|e| NetworkErrorKind::Deserialize(e.into()))?),
            false => Err(NetworkErrorKind::Deserialize(anyhow!("Wrong message kind")).into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ProbeRequest {}

impl Payload for ProbeRequest {
    const KIND: PayloadKind = PayloadKind::ProbeRequest;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ProbeReply {
    pub(crate) public_addr: SocketAddr,
    // TODO: Also use XOR like STUN to see if NAT is rewriting data
}

impl Payload for ProbeReply {
    const KIND: PayloadKind = PayloadKind::ProbeReply;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn kinds_are_unique() {
        // Try to check that all the Payload::KIND values are unique
        let values = vec![ProbeRequest::KIND, ProbeReply::KIND];
        let set: HashSet<&PayloadKind> = HashSet::from_iter(values.iter());
        assert_eq!(values.len(), set.len());
    }
}
