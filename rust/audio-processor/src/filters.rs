use ndarray::Array1;

pub struct Filter;

impl Filter {
    pub fn apply_hann_window(signal: &mut [f32]) {
        let n = signal.len();
        for (i, sample) in signal.iter_mut().enumerate() {
            let window =
                0.5 * (1.0 - ((2.0 * std::f32::consts::PI * i as f32) / (n as f32 - 1.0)).cos());
            *sample *= window;
        }
    }

    pub fn apply_lowpass(signal: &[f32], cutoff_freq: f32) -> Array1<f32> {
        Array1::from(signal.to_vec())
    }
}
