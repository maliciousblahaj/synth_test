use std::{f32::consts::TAU, sync::Arc};

use crate::{audio::graph::{AudioDevice, AudioNode}, synthesis::{waveforms::WaveForm, wavetable::WaveTable}};

#[derive(Clone)]
pub struct WaveTableOscillator {
    active: bool,
    sample_rate: Arc<u32>,
    wavetable: WaveTable,
    phase: f32,
    phase_increment: f32,
    amplitude: f32,
    frequency: f32,
}

impl WaveTableOscillator {
    pub fn new(sample_rate: Arc<u32>, wavetable: WaveTable) -> Self 
    {       
        Self {
            active: false,
            sample_rate,
            wavetable,
            phase: 0.0,
            phase_increment: 0.0,
            amplitude: 1.0,
            frequency: 0.0,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn decativate(&mut self) {
        self.active = false;
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.phase_increment = frequency*TAU / (self.sample_rate as f32);
        self.frequency = frequency;
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }

    pub fn get_waveform(&self) -> WaveForm {
        self.wavetable.get_waveform()
    }

    pub fn set_wavetable(&mut self, wavetable: WaveTable) {
        self.wavetable = wavetable;
    }

    /*
    pub fn get_sample(&mut self) -> f32 {
        if !self.active {
            return 0.0;
        }
        let sample = self.wavetable.lookup(self.phase);
        self.phase += self.phase_increment;
        self.phase %= TAU;
        sample*self.amplitude
    }*/
}

impl AudioDevice for WaveTableOscillator {
    fn render(&mut self, _children: &Vec<AudioNode>, time: u64) -> f32 {
        if !self.active {
            return 0.0; 
        }
        let phase = ((time as f32) * self.phase_increment) % TAU;
        self.wavetable.lookup(phase)
    }
}