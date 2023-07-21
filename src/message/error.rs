use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
};
#[derive(Debug)]
pub(crate) struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn error::Error>>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl error::Error for Error {
    /// The lower-level source of this error, if any.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_deref()
    }
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind, source: None }
    }

    pub(crate) fn with_source(kind: ErrorKind, source: impl error::Error + 'static) -> Self {
        Self {
            kind,
            source: Some(Box::new(source)),
        }
    }

    pub(crate) fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub(crate) enum ErrorKind {
    FromBytes,
    ToBytes,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use self::ErrorKind::*;
        match &self {
            FromBytes => write!(f, "failed to deserialize message from bytes"),
            ToBytes => write!(f, "failed to serialize message to bytes"),
        }
    }
}
