use std::sync::{Arc, Mutex};

use iced::{executor, widget::{column, container, pick_list, row, text}, Application, Command, Element, Theme};
use iced_audio::{FloatRange, FreqRange, Knob, Normal, NormalParam};

use crate::synthesis::{math::amplitude_to_decibel, waveforms::WaveForm, Synthesizer, WaveTable};

pub struct SynthesizerUI {
    synthesizer: Arc<Synthesizer>,


    gain_range: FloatRange,
    gain_param: NormalParam,

    oscillators_ui: Vec<OscillatorUi>,
}

pub struct OscillatorUi {
    pitch_range: FreqRange,
    pitch_param: NormalParam,

    waveform: Option<WaveForm>,
}

impl OscillatorUi {
    pub fn new(pitch_range: FreqRange, pitch_param: NormalParam, waveform: Option<WaveForm>) -> Self {
        Self {
            pitch_range,
            pitch_param,
            waveform
        }
    }
}

///parameters to initialize a SynthesizerUI
pub struct Flags {
    synthesizer: Arc<Synthesizer>,
}

impl Flags {
    pub fn new(synthesizer: Arc<Synthesizer>) -> Self {
        Self {
            synthesizer,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    GainChanged(Normal),
    PitchChanged(usize, Normal),
    WaveFormSelected(usize, WaveForm),
}

impl Application for SynthesizerUI {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut oscillators_ui: Vec<_> = Vec::new();

        let synthesizer = flags.synthesizer;

        let gain = amplitude_to_decibel(synthesizer.get_amplitude());
        for oscillator in synthesizer.get_oscillators() {
            let pitch_range = FreqRange::new(20.0, 20000.0);
            oscillators_ui.push(
                OscillatorUi::new(
                    pitch_range, 
                    pitch_range.normal_param(oscillator.get_frequency(), 220.0),
                    None,
                )
            );
        }

        let gain_range =  FloatRange::new(-50.0, 0.0);
        
        (Self {
            synthesizer,

            gain_range,
            gain_param: gain_range.normal_param(gain, -20.0),

            oscillators_ui,
        }, Command::none())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::GainChanged(normal) => {
                self.gain_param.update(normal);
                let new_gain = self.gain_range.unmap_to_value(normal);
                
                self.synthesizer.set_gain(new_gain);
            },
            Message::PitchChanged(oscillator_id, normal) => {
                let oscillator_ui = &mut self.oscillators_ui[oscillator_id];
                oscillator_ui.pitch_param.update(normal);
                let new_pitch = oscillator_ui.pitch_range.unmap_to_value(normal);
                
                self.synthesizer.set_oscillator_pitch(oscillator_id, new_pitch);
            },
            Message::WaveFormSelected(oscillator_id, waveform) => {
                let wavetable = WaveTable::from_fn(waveform.get_fn(), 128);
                self.synthesizer.set_oscillator_wavetable(oscillator_id, wavetable);
                println!("{waveform:?}");
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let gain_knob = Knob::new(
            self.gain_param,
            Message::GainChanged,
        );

        let gain_widget = container(
            column![
                text("gain"),
                gain_knob,
                text(format!("{:.1}db", self.gain_range.unmap_to_value(self.gain_param.value)))
            ].align_items(iced::Alignment::Center)
        )
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .height(100)
            .width(100);

        let pitch_knobs = {
            let mut pitch_knobs = Vec::new();
            for (id, oscillator_ui) in self.oscillators_ui.iter().enumerate() {
                pitch_knobs.push( 
                    Knob::new(
                        self.pitch_param,
                        Message::PitchChanged)
                );
            }
           pitch_knobs
        };

        let pitch_widget = container(
            column![
                text("pitch"),
                pitch_knob,
                text(format!("{:.1}hz", self.pitch_range.unmap_to_value(self.pitch_param.value)))
            ].align_items(iced::Alignment::Center)
        )
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .height(100)
            .width(100);

        let waveform_list = pick_list(
            &WaveForm::ALL[..], 
            self.waveform, 
            Message::WaveFormSelected
        );

        let waveform_widget = container(waveform_list)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .height(100)
            .width(100);
        
        row![
            gain_widget,
            pitch_widget,
            waveform_widget
        ].into()
    }
    
    fn title(&self) -> String {
        String::from("Synthesizer")
    }
}