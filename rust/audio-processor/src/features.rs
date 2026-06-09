use ndarray::Array1;

pub struct FeatureExtractor;

impl FeatureExtractor {
    pub fn extract_mfcc(_spectrum: &Array1<f32>, n_mfcc: usize) -> Array1<f32> {
        Array1::zeros(n_mfcc)
    }

    pub fn extract_chroma(_spectrum: &Array1<f32>) -> Array1<f32> {
        Array1::zeros(12)
    }

    pub fn extract_spectral_features(_spectrum: &Array1<f32>) -> Array1<f32> {
        Array1::zeros(8)
    }
}
