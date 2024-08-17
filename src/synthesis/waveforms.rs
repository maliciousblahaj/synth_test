use std::f32::consts::{PI, TAU};

/*
 * these waveforms are made to initialize wavetables, and potentially not optimized for real time performance
 */

#[allow(unused)]
pub fn sine(x: f32) -> f32 {
    x.sin()
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

#[allow(unused)]
pub fn triangle(x: f32) -> f32 {
    let x = x + 0.5*PI;
    4.0 * (x/TAU - (x/TAU + 0.5).floor()).abs() - 1.0
}