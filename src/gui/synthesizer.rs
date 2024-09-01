use std::sync::{Arc, Mutex};

use iced::{executor, widget::{column, row}, Element, Theme};

use crate::{devices::{amplifier::Amplifier, oscillator::WaveTableOscillator}};

use super::components::{amplifier::AmplifierUI, oscillator::OscillatorUI};


#[derive(Clone, Debug)]
pub enum Message {}


pub struct SynthesizerUI {
    amplifier_ui: AmplifierUI,
    oscillators_ui: Vec<OscillatorUI>,
}

impl SynthesizerUI {
    pub fn new(amplifier: Arc<Mutex<Box<Amplifier>>>, oscillators: Vec<Arc<Mutex<Box<WaveTableOscillator>>>>) -> Self {
        let mut oscillators_ui = Vec::with_capacity(oscillators.len());
        for oscillator in oscillators {
            oscillators_ui.push(OscillatorUI::new(oscillator));
        }

        Self {
            amplifier_ui: AmplifierUI::new(amplifier),
            oscillators_ui,
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        row![
            self.amplifier_ui,
        ].into()
    }
    
    fn title(&self) -> String {
        String::from("Synthesizer")
    }
}