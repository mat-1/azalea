#[derive(Debug)]
pub enum Error {
    InvalidTagType(u8),
    InvalidTag,
    WriteError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    UnexpectedEof,
    RootNotCompound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidTagType(id) => write!(f, "Invalid tag type: {id}"),
            Error::InvalidTag => write!(f, "Invalid tag"),
            Error::WriteError(e) => write!(f, "Write error: {e}"),
            Error::Utf8Error(e) => write!(f, "Utf8 error: {e}"),
            Error::UnexpectedEof => write!(f, "Unexpected EOF"),
            Error::RootNotCompound => write!(f, "Root tag is not a compound"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::WriteError(e)
    }
}
impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::Utf8Error(e)
    }
}
