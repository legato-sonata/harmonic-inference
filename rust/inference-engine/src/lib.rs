use ndarray::Array1;

pub mod error;
pub mod model;
pub mod runtime;

pub use error::InferenceError;
pub use model::Model;
pub use runtime::InferenceRuntime;

pub struct InferenceEngine {
    _model: Model,
    runtime: InferenceRuntime,
}

impl InferenceEngine {
    pub fn new(model_path: &str) -> Result<Self, InferenceError> {
        let model = Model::load(model_path)?;
        let runtime = InferenceRuntime::new(&model)?;

        Ok(Self {
            _model: model,
            runtime,
        })
    }

    pub fn predict(&self, input: &Array1<f32>) -> Result<Array1<f32>, InferenceError> {
        self.runtime.forward(input)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_engine_creation() {
        // Mock test - would use actual model in production
    }
}
