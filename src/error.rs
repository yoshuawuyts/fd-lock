use std::fmt;

/// Error categories.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it. It is used with the [`Error`] struct.
///
/// This list is non-exhaustive.
///
/// [`Error`]: std.struct.Error.html
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The lock is already held.
    Locked,
    /// Any error not part of this list.
    Other,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ErrorKind::Locked => "The fd is locked",
            ErrorKind::Other => "Generic error",
        };
        f.write_str(message)
    }
}

/// A specialized `Error` type.
#[derive(Debug)]
pub struct Error(ErrorKind);

impl Error {
    /// Access the [`ErrorKind`] member.
    ///
    /// [`ErrorKind`]: enum.ErrorKind.html
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error(kind)
    }
}
