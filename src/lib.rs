//! Plugin contract: a plugin transforms a spectrogram in place.
//! `frames[t][f]` is the magnitude at time frame `t`, frequency bin `f`.

pub trait SpectralPlugin {
    fn id(&self) -> &str;
    fn process(&self, frames: &mut [Vec<f32>]);
}

/// Example plugin: silences every bin below `threshold`.
pub struct NoiseGate {
    pub threshold: f32,
}

impl SpectralPlugin for NoiseGate {
    fn id(&self) -> &str {
        "noise-gate"
    }
    fn process(&self, frames: &mut [Vec<f32>]) {
        for frame in frames.iter_mut() {
            for bin in frame.iter_mut() {
                if *bin < self.threshold {
                    *bin = 0.0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_silences_quiet_bins() {
        let mut frames = vec![vec![0.1, 0.9, 0.2]];
        NoiseGate { threshold: 0.5 }.process(&mut frames);
        assert_eq!(frames, vec![vec![0.0, 0.9, 0.0]]);
    }
}
