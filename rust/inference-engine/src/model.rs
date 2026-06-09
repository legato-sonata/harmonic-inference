use crate::InferenceError;

#[derive(Clone, Debug)]
pub struct Model {
    pub name: String,
    pub version: String,
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
}

impl Model {
    pub fn load(_path: &str) -> Result<Self, InferenceError> {
        Ok(Self {
            name: "default".to_string(),
            version: "0.1.0".to_string(),
            input_shape: vec![1, 64],
            output_shape: vec![1, 10],
        })
    }
}
