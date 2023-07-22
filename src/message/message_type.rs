use super::error::MessageError;

extern crate alloc;
use alloc::vec::Vec;
use postcard::{from_bytes, to_allocvec};
use rand::prelude::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub(crate) enum Message {
    PingReq(PingReq),
    PingRes(PingRes),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub(crate) struct PingReq {
    pub(crate) msg_id: u32,
    pub(crate) from_peer_id: String,
    pub(crate) to_peer_id: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub(crate) struct PingRes {
    pub(crate) _dummy: bool,
    pub(crate) msg_id: u32,
}

impl Message {
    pub(crate) fn random_msg_id() -> u32 {
        random()
    }

    pub(crate) fn to_allocvec(&self) -> Result<Vec<u8>, MessageError> {
        to_allocvec(self).map_err(|e| MessageError::ToBytes(e.into()))
    }

    pub(crate) fn from_bytes(s: &[u8]) -> Result<Self, MessageError> {
        from_bytes(s).map_err(|e| MessageError::FromBytes(e.into()))
    }
}

// TODO: add tests
