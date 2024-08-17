use std::sync::{Arc, Mutex};

use iced::{executor, widget::slider, Application, Command, Element, Sandbox, Theme};

use crate::synthesis::{math::amplitude_to_decibel, Synthesizer};

pub struct SynthesizerUI {
    synthesizer: Arc<Mutex<Synthesizer>>,
    gain: f32,
}

///parameters to initialize a SynthesizerUI
pub struct Flags {
    synthesizer: Arc<Mutex<Synthesizer>>,
}

impl Flags {
    pub fn new(synthesizer: Arc<Mutex<Synthesizer>>) -> Self {
        Self {
            synthesizer
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    GainChanged(f32)
}

impl Application for SynthesizerUI {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let guard = flags.synthesizer.lock().unwrap();
        let gain = amplitude_to_decibel(guard.get_amplitude());
        drop(guard);
        
        (Self {
            synthesizer: flags.synthesizer,
            gain
        }, Command::none())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::GainChanged(new_gain) => {
                self.gain = new_gain;
                let mut guard = self.synthesizer.lock().unwrap();
                guard.set_gain(new_gain);
                drop(guard);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        slider(
            -50.0..=0.0, 
            self.gain, 
            Message::GainChanged
        ).into()
    }
    
    fn title(&self) -> String {
        String::from("Synthesizer")
    }
    

    

}