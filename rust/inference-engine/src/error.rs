use thiserror::Error;

#[derive(Error, Debug)]
pub enum InferenceError {
    #[error("Model loading failed: {0}")]
    ModelLoadError(String),

    #[error("Inference failed: {0}")]
    InferenceFailed(String),

    #[error("Invalid input dimensions")]
    InvalidInput,
}
