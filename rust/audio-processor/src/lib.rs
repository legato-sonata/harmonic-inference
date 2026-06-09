use ndarray::{Array1, Array2};
use rustfft::FftPlanner;
use std::sync::Arc;

pub mod error;
pub mod features;
pub mod filters;
pub mod spectrum;

pub use error::AudioError;
pub use features::FeatureExtractor;
pub use spectrum::SpectralAnalyzer;

const SAMPLE_RATE: usize = 44100;
const FFT_SIZE: usize = 2048;

pub struct AudioProcessor {
    sample_rate: usize,
    fft_size: usize,
    planner: Arc<FftPlanner<f32>>,
}

impl AudioProcessor {
    pub fn new(sample_rate: usize, fft_size: usize) -> Self {
        Self {
            sample_rate,
            fft_size,
            planner: Arc::new(FftPlanner::new()),
        }
    }

    pub fn default() -> Self {
        Self::new(SAMPLE_RATE, FFT_SIZE)
    }

    pub fn process_audio(&self, samples: &[f32]) -> Result<ProcessedAudio, AudioError> {
        if samples.is_empty() {
            return Err(AudioError::EmptyAudio);
        }

        let spectrum = self.compute_spectrum(samples)?;
        let features = self.extract_features(&spectrum)?;

        Ok(ProcessedAudio {
            spectrum,
            features,
            duration_ms: (samples.len() as f32 / self.sample_rate as f32) * 1000.0,
        })
    }

    fn compute_spectrum(&self, samples: &[f32]) -> Result<Array1<f32>, AudioError> {
        if samples.len() < self.fft_size {
            return Err(AudioError::InsufficientData);
        }
        // Placeholder spectrum computation
        Ok(Array1::zeros(self.fft_size / 2))
    }

    fn extract_features(&self, spectrum: &Array1<f32>) -> Result<Array1<f32>, AudioError> {
        // Placeholder feature extraction
        Ok(Array1::zeros(64))
    }
}

pub struct ProcessedAudio {
    pub spectrum: Array1<f32>,
    pub features: Array1<f32>,
    pub duration_ms: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let processor = AudioProcessor::default();
        assert_eq!(processor.sample_rate, SAMPLE_RATE);
        assert_eq!(processor.fft_size, FFT_SIZE);
    }

    #[test]
    fn test_empty_audio_error() {
        let processor = AudioProcessor::default();
        let result = processor.process_audio(&[]);
        assert!(result.is_err());
    }
}
