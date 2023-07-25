use crate::peer_id::PeerId;

use super::error::MessageError;

extern crate alloc;
use alloc::vec::Vec;
use postcard::{from_bytes, to_allocvec};
use rand::prelude::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    pub(crate) peer_id: PeerId,
    pub(crate) msg_id: u32,
    pub(crate) kind: Kind,
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

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Kind {
    Test(Test),
    Ack,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Test {
    pub(crate) peer_src_port: u16,
}
