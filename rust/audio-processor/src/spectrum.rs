use ndarray::Array1;

pub struct SpectralAnalyzer;

impl SpectralAnalyzer {
    pub fn compute_power_spectrum(spectrum: &Array1<f32>) -> Array1<f32> {
        spectrum.mapv(|x| x.powi(2))
    }

    pub fn peak_detection(spectrum: &Array1<f32>, threshold: f32) -> Vec<usize> {
        spectrum
            .iter()
            .enumerate()
            .filter_map(|(i, &val)| if val > threshold { Some(i) } else { None })
            .collect()
    }
}
