use ecou_spectral_rs::{NoiseGate, SpectralPlugin};

fn main() {
    // TODO (good first issue): load a WAV file, compute the STFT, apply plugins.
    let mut frames = vec![vec![0.1_f32, 0.9, 0.2]];
    NoiseGate { threshold: 0.5 }.process(&mut frames);
    println!("{:?}", frames);
}
