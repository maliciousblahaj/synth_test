use std::f32::consts::{PI, TAU};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum WaveForm {
    Sine,
    Triangle,
    Square,
    Saw,
    Table,
}

/*
 * these waveforms are made to initialize wavetables, and potentially not optimized for real time performance
 */

#[allow(unused)]
pub fn sine(x: f32) -> f32 {
    x.sin()
}

#[allow(unused)]
pub fn triangle(x: f32) -> f32 {
    let x = x + 0.5*PI;
    4.0 * (x/TAU - (x/TAU + 0.5).floor()).abs() - 1.0
}

#[allow(unused)]
pub fn square(x: f32) -> f32 {
    let x = x % TAU;
    if x <= PI {
        1.0
    } else {
        -1.0
    }
}

#[allow(unused)]
pub fn saw(x: f32) -> f32 {
    ((x + PI) / PI) % 2.0 - 1.0
}



impl WaveForm {
    pub const ALL: [Self; 4] = [
        Self::Sine,
        Self::Triangle,
        Self::Square,
        Self::Saw
    ];
    
    pub fn get_fn(&self) -> fn(f32) -> f32 {
        match self {
            Self::Sine => sine,
            Self::Triangle => triangle,
            Self::Square => square,
            Self::Saw => saw,
            Self::Table => panic!("Invalid get_fn from WaveTable"),
        }
    }
}

impl std::fmt::Display for WaveForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}",
            match self {
                Self::Sine => "Sine",
                Self::Triangle => "Triangle",
                Self::Square => "Square",
                Self::Saw => "Saw",
                Self::Table => "WaveTable",
            }
        )
    }
}