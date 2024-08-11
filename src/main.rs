use std::{error::Error, f32::consts::{PI, TAU}, time::Duration};

use rodio::{OutputStream, Source};

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

struct WaveTable {
    table: Vec<f32>,
    inverse_step: f32,
}

impl WaveTable {
    pub fn new<F>(function: F, length: usize) -> Self
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

struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: WaveTable,
    index: f32,
    index_increment: f32,
}

impl WaveTableOscillator {
    pub fn new(sample_rate: u32, wave_table: WaveTable) -> Self 
    {       
        Self {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency*TAU / (self.sample_rate as f32);
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.wave_table.lookup(self.index);
        self.index += self.index_increment;
        self.index %= TAU;
        sample
    }
}

impl Iterator for WaveTableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample())
    }
}

impl Source for WaveTableOscillator {
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
    let frequency = 220.0;

    let wavetable_size = 256;
    let wavetable = WaveTable::new(sine, wavetable_size);

    let mut oscillator = WaveTableOscillator::new(sample_rate, wavetable);
    oscillator.set_frequency(frequency);

    let (_stream, stream_handle) = OutputStream::try_default()?;

    let _result = stream_handle.play_raw(oscillator.convert_samples());

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


