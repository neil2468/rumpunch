// TODO: docs
// TODO: should this have a max length, so UDP packet max lengths can be known?
// UUIDs are 128 bits (16 bytes)? Public keys are sometimes XX bits?

//TODO: we have some clones of PeerId. Are the needed?
pub(crate) type PeerId = String;

pub(crate) type MsgId = u16;
