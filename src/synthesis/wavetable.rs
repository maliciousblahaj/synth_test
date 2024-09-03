use std::f32::consts::TAU;

use crate::math::lerp;

use super::waveforms::WaveForm;

#[derive(Clone, Debug)]
pub struct WaveTable {
    table: Vec<f32>,
    waveform: WaveForm,
    inverse_step: f32,
}

impl WaveTable {
    pub fn new(table: Vec<f32>, waveform: WaveForm) -> Self {
        Self {
            waveform,
            inverse_step: (table.len() as f32)/TAU,
            table,
        }
    }

    pub fn from_waveform(waveform: WaveForm, length: usize) -> Self {
        let mut table = Vec::with_capacity(length);
        let step = TAU / (length as f32);
        let function = waveform.get_fn();

        for i in 0..length {
            table.push(function((i as f32) * step));
        }


        Self {
            waveform,
            table,
            inverse_step: (length as f32)/TAU,
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
            waveform: WaveForm::Table,
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
    
    pub fn get_waveform(&self) -> WaveForm {
        self.waveform
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}