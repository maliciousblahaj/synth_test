use std::f32::consts::TAU;

use math::{decibel_to_amplitude, lerp};

pub mod waveforms;
pub mod math;

#[derive(Clone)]
pub struct WaveTable {
    table: Vec<f32>,
    inverse_step: f32,
}

impl WaveTable {
    pub fn new(table: Vec<f32>) -> Self {
        Self {
            inverse_step: (table.len() as f32)/TAU,
            table,
        }
    }

    pub fn from_fn<F>(function: F, length: usize) -> Self
        where F: Fn(f32) -> f32,
    {
        let mut table = Vec::with_capacity(length);

        let step = TAU / (length as f32);
        for i in 0..length {
            table.push(function((i as f32) * step));
        }

        Self {
            table,
            inverse_step: (length as f32)/TAU,
        }
    }

    pub fn lookup(&self, phase: f32) -> f32 {
        let phase = phase % TAU;
        let index = phase * self.inverse_step;

        let index_trunc = index as usize;
        let next_index_trunc = (index_trunc+1) % self.len();

        lerp(self.table[index_trunc], self.table[next_index_trunc], index-(index_trunc as f32))
    }
    
    pub fn len(&self) -> usize {
        self.table.len()
    }
}

#[derive(Clone)]
pub struct WaveTableOscillator {
    sample_rate: u32,
    wavetable: WaveTable,
    index: f32,
    index_increment: f32,
    amplitude: f32,
    frequency: f32,
}

impl WaveTableOscillator {
    pub fn new(sample_rate: u32, wavetable: WaveTable) -> Self 
    {       
        Self {
            sample_rate,
            wavetable,
            index: 0.0,
            index_increment: 0.0,
            amplitude: 1.0,
            frequency: 0.0,
        }
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency*TAU / (self.sample_rate as f32);
        self.frequency = frequency;
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }

    pub fn set_wavetable(&mut self, wavetable: WaveTable) {
        self.wavetable = wavetable;
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.wavetable.lookup(self.index);
        self.index += self.index_increment;
        self.index %= TAU;
        sample*self.amplitude
    }
}

pub struct Synthesizer {
    sample_rate: u32,
    oscillators: Vec<WaveTableOscillator>,
    amplitude: f32,
}

impl Synthesizer {
    pub fn new(sample_rate: u32, oscillators: Vec<WaveTableOscillator>) -> Self {
        let mut oscillators = oscillators;
        for oscillator in oscillators.iter_mut() {
            oscillator.sample_rate = sample_rate;
        }

        Self {
            sample_rate,
            oscillators,
            amplitude: decibel_to_amplitude(-20.0),
        }
    }

    pub fn get_amplitude(&self) -> f32 {
        self.amplitude
    } 

    pub fn set_gain(&mut self, gain: f32) {
        self.amplitude = decibel_to_amplitude(gain);
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn get_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        for oscillator in self.oscillators.iter_mut() {
            sample += oscillator.get_sample();
        }

        sample * self.amplitude
    }

    //temporary function 1
    pub fn get_osc1_pitch(&self) -> f32 {
        self.oscillators[0].get_frequency()
    }

    //temporary function 2
    pub fn set_osc1_pitch(&mut self, frequency: f32) {
        self.oscillators[0].set_frequency(frequency);
    }

    //temporary function 3
    pub fn set_osc1_wavetable(&mut self, wavetable: WaveTable) {
        self.oscillators[0].set_wavetable(wavetable)
    }
}