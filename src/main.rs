use std::{error::Error, sync::{Arc, Mutex}};

use iced::{Application, Settings};
use rodio::{OutputStream, Source};
use wavetable_synthesizer::{gui::{Flags, SynthesizerUI}, synthesis::{waveforms::{saw, square, WaveForm}, Synthesizer, WaveTable, WaveTableOscillator}, AudioSource};

fn main() -> Result<(), Box<dyn Error>> {
    let sample_rate = 48000;
    let base_frequency = 50.0;

    let wavetable = WaveTable::from_fn(WaveForm::Square.get_fn(), 128);

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


