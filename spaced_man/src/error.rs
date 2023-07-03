use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Error {
    kind: ErrorKind
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {

}

impl std::error::Error for Error {}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {{
            
        }}")
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}