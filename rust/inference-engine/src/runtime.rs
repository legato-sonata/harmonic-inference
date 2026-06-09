use crate::{InferenceError, Model};
use ndarray::Array1;

pub struct InferenceRuntime {
    _model: Model,
}

impl InferenceRuntime {
    pub fn new(model: &Model) -> Result<Self, InferenceError> {
        Ok(Self {
            _model: model.clone(),
        })
    }

    pub fn forward(&self, _input: &Array1<f32>) -> Result<Array1<f32>, InferenceError> {
        Ok(Array1::zeros(10))
    }
}

impl Clone for InferenceRuntime {
    fn clone(&self) -> Self {
        Self {
            _model: self._model.clone(),
        }
    }
}
