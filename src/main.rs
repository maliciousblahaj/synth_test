use std::{error::Error, f32::consts::{PI, TAU}, time::Duration};

use rodio::{OutputStream, Source};

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

#[derive(Clone)]
struct WaveTable {
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
struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: WaveTable,
    index: f32,
    index_increment: f32,
    amplitude: f32
}

impl WaveTableOscillator {
    pub fn new(sample_rate: u32, wave_table: WaveTable) -> Self 
    {       
        Self {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
            amplitude: 1.0,
        }
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency*TAU / (self.sample_rate as f32);
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.wave_table.lookup(self.index);
        self.index += self.index_increment;
        self.index %= TAU;
        sample*self.amplitude
    }
}

struct Synthesizer {
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

    pub fn set_gain(&mut self, gain: f32) {
        self.amplitude = decibel_to_amplitude(gain);
    }
}

/// Converts gain in decibel to a number to multiply the waveforms with
fn decibel_to_amplitude(gain: f32) -> f32 {
    10f32.powf(gain/20f32)
}

impl Iterator for Synthesizer {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let mut sample = 0.0;
        for oscillator in self.oscillators.iter_mut() {
            sample += oscillator.get_sample();
        }

        return Some(sample * self.amplitude)
    }
}

impl Source for Synthesizer {
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


#[allow(unused)]
fn sine(x: f32) -> f32 {
    x.sin()
}

#[allow(unused)]
fn square(x: f32) -> f32 {
    if x <= PI {
        1.0
    } else {
        -1.0
    }
}

#[allow(unused)]
fn saw(x: f32) -> f32 {
    if x <= PI {
        (x*2.0)/PI - 1.0
    } else {
        ((x-PI)*2.0)/PI - 1.0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sample_rate = 48000;
    let base_frequency = 22.5;

    let wavetable = WaveTable::from_fn(saw, 256);

    let oscillator_blueprint = WaveTableOscillator::new(sample_rate, wavetable);

    let mut oscillators = Vec::new();
    for i in 1..=11 {
        let mut osc = oscillator_blueprint.clone();
        osc.set_frequency(base_frequency*(i as f32));
        oscillators.push(osc)
    }

    let synth = Synthesizer::new(sample_rate, oscillators);

    let (_stream, stream_handle) = OutputStream::try_default()?;

    let _result = stream_handle.play_raw(synth.convert_samples());

    loop {
        std::thread::sleep(Duration::from_millis(100));
    }

    //idk
    /*
    let duration = Duration::from_secs_f32(1.0 / sample_rate as f32);
    loop {
        let start_time = Instant::now();

        //insert audio output logic here

        let elapsed_time = start_time.elapsed();
        if elapsed_time < duration {
            thread::sleep(duration - elapsed_time);
        }
    }*/
}


