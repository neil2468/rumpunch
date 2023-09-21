use crate::{
    network_error::{NetworkError, NetworkErrorKind},
    types::{MsgId, PeerId},
};
extern crate alloc; // TODO: is'nt this old rust?
use anyhow::anyhow;
use postcard::{from_bytes, to_stdvec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    /// Id of sender
    peer_id: PeerId,

    /// Id of message, for matching with reply
    msg_id: MsgId,

    /// Payload kind
    kind: PayloadKind,

    /// Payload bytes
    payload: Vec<u8>,
}

impl Message {
    pub(crate) fn new<P>(peer_id: PeerId, msg_id: MsgId, payload: P) -> Self
    where
        P: Payload,
    {
        Self {
            peer_id,
            msg_id,
            kind: P::KIND.clone(),
            payload: payload.to_bytes(),
        }
    }

    pub(crate) fn peer_id(&self) -> &PeerId {
        &self.peer_id
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
    /// Will panics if serialisation fails.
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        // If this errors it's probably a bug
        to_stdvec(self).expect("error serialising message")
    }

    /// Deserialise from bytes
    pub(crate) fn from_bytes(s: &[u8]) -> Result<Self, NetworkError> {
        from_bytes(s).map_err(|e| NetworkErrorKind::Deserialize(e.into()).into())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) enum PayloadKind {
    StartRequest,
    StartReply,
    SampleRequest,
    SampleReply,
    // StopRequest { connect_to: PeerId },
    // StopReply,
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
pub(crate) struct StartRequest {
    pub(crate) connect_to: PeerId,
}

impl Payload for StartRequest {
    const KIND: PayloadKind = PayloadKind::StartRequest;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct StartReply {
    pub(crate) can_continue: bool,
}

impl Payload for StartReply {
    const KIND: PayloadKind = PayloadKind::StartReply;
}

// TOOD: Also pass IP address in message, to see if NAT is rewriting packets
// Like the STUN protocol does?
// Include hash to see if what client sent is what we received
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SampleRequest {
    pub(crate) src_port: u16,
}

impl Payload for SampleRequest {
    const KIND: PayloadKind = PayloadKind::SampleRequest;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SampleReply {}

impl Payload for SampleReply {
    const KIND: PayloadKind = PayloadKind::SampleReply;
}
