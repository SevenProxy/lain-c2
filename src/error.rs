use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Error de IO em: '{context}'")]
    Io {
        source: io::Error,
        context: String,
    },
}

impl ServerError {
    pub fn with_context(source: io::Error, context: String) -> Self {
        ServerError::Io { source, context }
    }
}
