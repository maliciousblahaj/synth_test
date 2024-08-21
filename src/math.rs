pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

/// Converts gain in decibel to a number to multiply the waveforms with
pub fn decibel_to_amplitude(gain: f32) -> f32 {
    10f32.powf(gain/20f32)
}

/// Converts amplitude to gain in decibel
pub fn amplitude_to_decibel(gain: f32) -> f32 {
    20.0 * gain.log10()
}