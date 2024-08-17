use std::{error::Error, sync::{Arc, Mutex}};

use iced::{Application, Settings};
use rodio::{OutputStream, Source};
use wavetable_synthesizer::{synthesis::{waveforms::sine, Synthesizer, WaveTable, WaveTableOscillator}, ui::{Flags, SynthesizerUI}, AudioSource};

fn main() -> Result<(), Box<dyn Error>> {
    let sample_rate = 48000;
    let base_frequency = 440.0;

    let wavetable = WaveTable::from_fn(sine, 256);

    let oscillator_blueprint = WaveTableOscillator::new(sample_rate, wavetable);

    let mut oscillators = Vec::new();
    for i in 1..=1 {
        let mut osc = oscillator_blueprint.clone();
        osc.set_frequency(base_frequency*(i as f32));
        oscillators.push(osc)
    }

    let mut synth = Synthesizer::new(sample_rate, oscillators);
    synth.set_gain(-20.0);
    let synth = Arc::new(Mutex::new(synth));

    let audio_source = AudioSource::new(synth.clone());

    let (_stream, stream_handle) = OutputStream::try_default()?;


    let _result = stream_handle.play_raw(audio_source.convert_samples());

    let ui_settings = Settings::with_flags(
        Flags::new(synth)
    );

    SynthesizerUI::run(ui_settings)?;

    Ok(())

    //idk
    /*
    loop {
        std::thread::sleep(Duration::from_millis(100));
    */
}


