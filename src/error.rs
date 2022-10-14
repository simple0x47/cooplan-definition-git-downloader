use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    FailedToCloneRepository,
    FailedToOpenRepository,
    FailedToUpdateDefinitions,
    FailedToReadEnvironmentVariables,
    SourceMissingId,
    DefinitionsNotAvailable,
    SerializationFailure,
    ConnectionFailure,
    ChannelNotAvailable,
    DataWritingFailure,
    VersionReadFailure,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Error {
        Error {
            kind,
            message: message.to_string(),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
