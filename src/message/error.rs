use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

pub(crate) struct MessageError {
    kind: MessageErrorKind,
}

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Error for MessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::MessageErrorKind::*;
        match &self.kind {
            FromBytes(e) => Some(e),
            ToBytes(e) => Some(e),
        }
    }
}

impl Debug for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}

impl From<MessageErrorKind> for MessageError {
    fn from(kind: MessageErrorKind) -> Self {
        Self { kind }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub(crate) enum MessageErrorKind {
    FromBytes(postcard::Error),
    ToBytes(postcard::Error),
}

impl Display for MessageErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use self::MessageErrorKind::*;
        match &self {
            FromBytes(_) => write!(f, "failed to deserialize message from bytes"),
            ToBytes(_) => write!(f, "failed to serialize message to bytes"),
        }
    }
}
