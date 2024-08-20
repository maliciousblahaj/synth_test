use std::{sync::{Arc, Mutex}, time::Duration};

use rodio::Source;
use synthesis::Synthesizer;

pub mod gui;
pub mod synthesis;
pub mod error;

pub use error::{Error, Result};

#[allow(unused)]
pub mod audio;

pub struct AudioSource {
    synthesizer: Arc<Mutex<Synthesizer>>
}

impl AudioSource {
    pub fn new(synthesizer: Arc<Mutex<Synthesizer>>) -> Self {
        Self {
            synthesizer
        }
    }
}

impl Iterator for AudioSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let mut guard = self.synthesizer.lock().unwrap();
        let sample = guard.get_sample();
        drop(guard);

        Some(sample)
    }
}

impl Source for AudioSource {
    fn channels(&self) -> u16 {
        return 1;
    }
    
    fn sample_rate(&self) -> u32 {
        self.synthesizer.lock().unwrap().get_sample_rate()
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}