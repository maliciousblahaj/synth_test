use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, fmt::Debug, ops::DerefMut, panic::RefUnwindSafe, rc::Rc, sync::{Arc, Mutex}, time::Duration};
use crate::{Error, Result, error::AudioError};
use rodio::Source;

//pub type AudioNodeBox = Box<AudioNode<dyn AudioDevice>>;


pub struct AudioGraph {
    nodes: HashMap<u32, AudioNode>,
    master_node: AudioNode,
    sample_rate: u32,
    //time: u64, //current time in samples
    current_id: u32, //for getting a unique identifier to every node
}

impl AudioGraph {
    pub fn new(sample_rate: u32) -> Self {
        let mut master_node =
            AudioNode::new(
                Rc::new(Box::new(MasterOutput::new())),
                Vec::new(),
            );
        master_node.set_id(0);
        let mut nodes = HashMap::new();
        nodes.insert(master_node.id.unwrap_or(0), master_node.clone());
        Self {
            nodes,
            master_node,
            sample_rate,
            //time: 0,
            current_id: 0,
        }
    }

    pub fn render(&mut self, time: u64) -> f32 {
        0.0
    }

    pub fn get_node(&self, id: u32) -> Result<&AudioNode> {
        self.nodes.get(&id).ok_or(Error::Audio(AudioError::AudioGraphInvalidId(id)))
    }

    pub fn get_uid(&mut self) -> u32 {
        self.current_id += 1;
        self.current_id
    }
}

/*
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
*/

#[derive(Clone)]
pub struct AudioNode {
    device: Rc<Box<dyn AudioDevice>>, //dyn AudioDevice
    children: Vec<AudioNode>,
    id: Option<u32>,
}

impl AudioNode {
    pub fn new(device: Rc<Box<dyn AudioDevice>>, children: Vec<AudioNode>) -> Self {
        Self {
            device,
            children,
            id: None,
        }
    }

    fn set_id(&mut self, id: u32) {
        self.id = Some(id);
    }

    pub fn render(&self, time: u64) -> f32 {
        self.device.render(&self.children, time)
    }
}

pub trait AudioDevice {
    fn render(&self, children: &Vec<AudioNode>, time: u64) -> f32;
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
    fn render(&self, children: &Vec<AudioNode>, time: u64) -> f32 {
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