use std::sync::{Arc, Mutex};

use iced::{executor, widget::{row, column}, Application, Command, Element, Theme};
use iced_lazy::Component;

use crate::{devices::{amplifier::Amplifier, oscillator::WaveTableOscillator}};

use super::components::{amplifier::AmplifierUI, oscillator::OscillatorUI};

pub struct SynthesizerUI {
    amplifier_ui: AmplifierUI,

    oscillators_ui: Vec<OscillatorUI>,
}

impl SynthesizerUI {
    pub fn new(amplifier_ui: AmplifierUI, oscillators_ui: Vec<OscillatorUI>) -> Self {
        Self {
            amplifier_ui,
            oscillators_ui,
        }
    }
}

///parameters to initialize a SynthesizerUI
pub struct Flags {
    amplifier_ui: AmplifierUI,
    oscillators_ui: Vec<OscillatorUI>,
}

impl Flags {
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
}

#[derive(Clone, Debug)]
pub enum Message {}


impl Application for SynthesizerUI {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                amplifier_ui: flags.amplifier_ui,
                oscillators_ui: flags.oscillators_ui,
            }, 
            Command::none(),
        )
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