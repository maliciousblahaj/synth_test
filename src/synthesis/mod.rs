pub mod waveforms;
pub mod wavetable;

/*
pub struct Synthesizer {
    sample_rate: u32,
    oscillators: Vec<Arc<Mutex<WaveTableOscillator>>>,
    amplitude: f32,
}
impl Synthesizer {
    pub fn new(sample_rate: u32, oscillators: Vec<Arc<Mutex<WaveTableOscillator>>>) -> Self {
        let mut oscillators = oscillators;
        for osc in oscillators.iter_mut() {
            assert_eq!(osc.lock().unwrap().get_sample_rate(), sample_rate);
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
            //sample += oscillator.get_sample();
            sample += 0.0;
        }

        sample * self.amplitude
    }

    pub fn get_oscillators(&self) -> &Vec<WaveTableOscillator> {
        &self.oscillators
    }

    pub fn get_oscillators_mut(&mut self) -> &mut Vec<WaveTableOscillator> {
        &mut self.oscillators
    }

    /// will panic if id is not valid
    pub fn get_oscillator_pitch(&self, id: usize) -> f32 {
        self.oscillators[id].get_frequency()
    }

    /// will panic if id is not valid
    pub fn set_oscillator_pitch(&mut self, id: usize, frequency: f32) {
        self.oscillators[id].set_frequency(frequency);
    }

    /// will panic if id is not valid
    pub fn set_oscillator_wavetable(&mut self, id: usize, wavetable: WaveTable) {
        self.oscillators[id].set_wavetable(wavetable);
    }
}
*/