use crate::audio::graph::{render_nodes, AudioDevice, AudioNode};

pub struct Amplifier {
    amplitude: f32,
}

impl Amplifier {
    pub fn new(amplitude: f32) -> Self {
        Self {
            amplitude,
        }
    }

    pub fn get_amplitude(&self) -> f32 {
        self.amplitude
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }
}

impl AudioDevice for Amplifier {
    fn render(&mut self, children: &Vec<AudioNode>, time: u64) -> f32 {
        let input = render_nodes(children, time);
        input * self.amplitude 
    }
}