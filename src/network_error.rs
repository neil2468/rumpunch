use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
};

/// Errors that can happend when networking
///
/// Design guided by https://sabrinajewson.org/blog/errors.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct NetworkError {
    kind: NetworkErrorKind,
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "network error: {}",
            format!("{:?}", self.kind).to_lowercase()
        )
    }
}

impl Error for NetworkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use NetworkErrorKind::*;
        match &self.kind {
            SendReceive(e) => Some(e.as_ref()),
            Io(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<io::Error> for NetworkError {
    fn from(value: io::Error) -> Self {
        NetworkError {
            kind: NetworkErrorKind::Io(value.into()),
        }
    }
}

impl From<NetworkErrorKind> for NetworkError {
    fn from(value: NetworkErrorKind) -> Self {
        Self { kind: value }
    }
}

#[derive(Debug)]
pub enum NetworkErrorKind {
    /// Error during sending or receiving
    SendReceive(anyhow::Error),

    /// Error deserialising from bytes
    Deserialize(anyhow::Error),

    /// A low-level IO error
    Io(anyhow::Error),
}
