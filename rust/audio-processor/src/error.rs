use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Empty audio data")]
    EmptyAudio,

    #[error("Insufficient audio data for processing")]
    InsufficientData,

    #[error("Invalid audio format")]
    InvalidFormat,

    #[error("Processing error: {0}")]
    ProcessingError(String),
}
