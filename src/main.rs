use std::{error::Error, sync::{Arc, Mutex}};

use iced::Settings;
use rodio::OutputStream;
use wavetable_synthesizer::{audio::graph::{AudioDevice, AudioGraph, AudioNode}, devices::{amplifier::Amplifier, oscillator::{self, WaveTableOscillator}}, gui::synthesizer::SynthesizerUI, synthesis::{waveforms::WaveForm, wavetable::WaveTable}};

fn main() -> Result<(), Box<dyn Error>> {
    let sample_rate = Arc::new(48000);
    let base_frequency = 50.0;

    let wavetable = WaveTable::from_fn(WaveForm::Square.get_fn(), 128);

    let oscillator_blueprint = WaveTableOscillator::new(sample_rate, wavetable);

    let mut oscillators: Vec<Arc<Mutex<Box<dyn AudioDevice>>>> = Vec::new();
    for i in 1..=1 {
        let mut osc = oscillator_blueprint.clone();
        osc.set_frequency(base_frequency*(i as f32));
        oscillators.push(Arc::new(Mutex::new(Box::new(osc))));
    }

    let mut amplifier = Arc::new(Mutex::new(Box::new(
        Amplifier::new(0.7)
    )));

    
    let oscillator_nodes = oscillators.iter()
        .map(|osc| {
            let osc = osc.clone(); 
            AudioNode::new(osc.clone(), Vec::new())
        })
        .collect();

    let amplifier_node = AudioNode::new(amplifier.clone(), oscillator_nodes);

    let audio_graph = AudioGraph::new(
        sample_rate,

    );

    let synth = Arc::new(Mutex::new(
        SynthesizerUI::new(
            amplifier,
            oscillators,
        )
    ));

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


