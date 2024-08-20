use std::sync::Arc;

pub struct AudioGraph {
    master_node: AudioNode
}

impl AudioGraph {
    pub fn new() -> Self {
        Self {
            master_node: AudioNode::new(
                Box::new(MasterOutput::new()),
                Vec::new(),
            ),
        }
    }
}

pub struct AudioNode {
    device: Box<dyn AudioDevice>,
    children: Vec<AudioNode>,
}

impl AudioNode {
    pub fn new(device: Box<dyn AudioDevice>, children: Vec<AudioNode>) -> Self {
        Self {
            device,
            children
        }
    }
}

pub trait AudioDevice {
    fn get_sample(&mut self, x: f32) -> f32;
}

pub struct MasterOutput {
    amplitude: f32,
}

impl MasterOutput {
    pub fn new() -> Self {
        Self {
            amplitude: 1.0
        }
    }
    
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn get_amplitude(&self) -> f32 {
        self.amplitude
    }
}

impl AudioDevice for MasterOutput {
    fn get_sample(&mut self, x: f32) -> f32 {
        x * self.amplitude
    }
}

/*
pub enum NodeType {
    Output,
    Oscillator,
    Amplifier,
}
*/