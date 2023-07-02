use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    inner: anyhow::Error
}

impl Error {
    pub fn into_inner(self) -> anyhow::Error {
        self.inner
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self { inner: value }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.root_cause().source()
    }
}

pub type Result<T> = core::result::Result<T, Error>;