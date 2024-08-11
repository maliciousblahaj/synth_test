use std::{f64::consts::{PI, TAU}, thread};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

struct WaveTable {    
    table: Vec<f64>,
    samples: usize,
    inverse_step: f64,
}

impl WaveTable {
    pub fn new<F>(function: F, samples: usize) -> Self 
        where F: Fn(f64) -> f64,
    {
        let mut lookup_table = Vec::with_capacity(samples);
        
        let step = TAU / (samples as f64);
        for i in 0..samples {
            lookup_table.push(function((i as f64) * step));
        }
        

        Self {
            table: lookup_table,
            samples,
            inverse_step: 1.0/step,
        }
    }

    pub fn lookup(&self, phase: f64) -> f64 {
        let phase = phase % TAU;
        let index = phase * self.inverse_step;

        let index_floor = index.floor().clamp(0.0, (self.samples-2) as f64);
        let index_floor_usize = (index_floor as usize).clamp(0, self.samples-2);

        lerp(self.table[index_floor_usize], self.table[index_floor_usize+1], index-index_floor)
    }
}

fn sine(x: f64) -> f64 {
    x.sin()
}

fn square(x: f64) -> f64 {
    if x < PI {
        1.0
    } else {
        -1.0
    }
}

fn main() {
    let sample_rate = 48000.0;
    let frequency = 131.0;

    let wavetable = WaveTable::new(square, 128);;

    let host = cpal::default_host();
    let device = host.default_output_device().expect("Failed to get default output device");
    let config = device.default_output_config().unwrap();

    let mut phase = 0.0;
    let phase_step = frequency * TAU / sample_rate;

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    *sample = wavetable.lookup(phase) as f32;
                    phase += phase_step;
                }
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream(
            &config.into(),
            move |data: &mut [i16], _| {
                for sample in data.iter_mut() {
                    *sample = (wavetable.lookup(phase) * i16::MAX as f64) as i16;
                    phase += phase_step;
                }
            },
            err_fn,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_output_stream(
            &config.into(),
            move |data: &mut [u16], _| {
                for sample in data.iter_mut() {
                    *sample = (wavetable.lookup(phase) * u16::MAX as f64) as u16;
                    phase += phase_step;
                }
            },
            err_fn,
            None,
        ),
        _ => panic!("help")
    }.unwrap();

    stream.play().unwrap();

    // Keep the thread alive to continue playback
    loop {
        thread::sleep(std::time::Duration::from_millis(100));
    }


    //idk
    /*
    let duration = Duration::from_secs_f64(1.0 / sample_rate as f64);
    loop {
        let start_time = Instant::now();

        //insert audio output logic here

        let elapsed_time = start_time.elapsed();
        if elapsed_time < duration {
            thread::sleep(duration - elapsed_time);
        }
    }*/
}


