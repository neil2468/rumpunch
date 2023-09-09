use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub(crate) enum MessageError {
    #[error("failed to deserialize message from bytes")]
    FromBytes(#[source] anyhow::Error),

    #[error("failed to serialize message to bytes")]
    ToBytes(#[source] anyhow::Error),
}
