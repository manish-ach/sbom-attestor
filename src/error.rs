use thiserror::Error;

#[derive(Debug, Error)]
pub enum AttestorError {
    #[error("IO Error")]
    Io(#[from] std::io::Error),

    #[error("Invalid Signature")]
    InvalidSignature,

    #[error("Key Error")]
    KeyError,
}
