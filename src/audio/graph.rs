use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, ops::DerefMut, panic::RefUnwindSafe, sync::{Arc, Mutex}, time::Duration};
use crate::{Error, Result, error::AudioError};
use rodio::Source;



pub struct AudioGraph {
    nodes: HashMap<u32, AudioNode>,
    master_node: AudioNode,
    sample_rate: Arc<u32>,
    time: u64, //time in samples
}

impl AudioGraph {
    pub fn new(sample_rate: Arc<u32>) -> Self {
        let master_node =
            AudioNode::new(
                0,
                Arc::new(Mutex::new(Box::new(MasterOutput::new()))),
                Vec::new(),
            );
        let mut nodes = HashMap::new();
        nodes.insert(master_node.id, master_node.clone());
        Self {
            nodes,
            master_node,
            sample_rate,
            time: 0,
        }
    }

    pub fn render(&mut self, time: u64) -> f32 {
        0.0
    }

    pub fn get_node(&self, id: u32) -> Result<&AudioNode> {
        self.nodes.get(&id).ok_or(Error::Audio(AudioError::AudioGraphInvalidId(id)))
    }
}

impl Iterator for AudioGraph {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let sample = Some(self.render(self.time));
        self.time += 1;
        sample
    }
}

impl Source for AudioGraph {
    fn channels(&self) -> u16 {
        return 1;
    }
    
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

#[derive(Clone)]
pub struct AudioNode {
    id: u32,
    device: Arc<Mutex<Box<dyn AudioDevice>>>,
    children: Vec<AudioNode>,
}

impl AudioNode {
    pub fn new(id: u32, device: Arc<Mutex<Box<dyn AudioDevice>>>, children: Vec<AudioNode>) -> Self {
        Self {
            id,
            device,
            children
        }
    }

    pub fn render(&mut self, time: u64) -> f32 {
        self.device
            .lock()
            .unwrap()
            .render(&self.children, time)
    }
}

pub trait AudioDevice {
    fn render(&mut self, children: &Vec<AudioNode>, time: u64) -> f32;
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
    fn render(&mut self, children: &Vec<AudioNode>, time: u64) -> f32 {
        render_nodes(children, time) * self.amplitude
    }
}

pub fn render_nodes(audio_nodes: &Vec<AudioNode>, time: u64) -> f32 {
    let mut sample = 0.0;
    for node in audio_nodes {
        sample += node.render(time);
    }
    sample
}

/*
pub enum NodeType {
    Output,
    Oscillator,
    Amplifier,
}
*/