use std::sync::{Arc, Mutex};

use iced::{executor, widget::{column, container, pick_list, row, text}, Application, Command, Element, Theme};
use iced_audio::{FloatRange, FreqRange, Knob, Normal, NormalParam};

use crate::synthesis::{math::amplitude_to_decibel, waveforms::WaveForm, Synthesizer, WaveTable};

pub struct SynthesizerUI {
    synthesizer: Arc<Mutex<Synthesizer>>,


    gain_range: FloatRange,
    gain_param: NormalParam,

    pitch_range: FreqRange,
    pitch_param: NormalParam,

    waveform: Option<WaveForm>,
}

///parameters to initialize a SynthesizerUI
pub struct Flags {
    synthesizer: Arc<Mutex<Synthesizer>>,
}

impl Flags {
    pub fn new(synthesizer: Arc<Mutex<Synthesizer>>) -> Self {
        Self {
            synthesizer,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    GainChanged(Normal),
    PitchChanged(Normal),
    WaveFormSelected(WaveForm),
}

impl Application for SynthesizerUI {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let guard = flags.synthesizer.lock().unwrap();
        let gain = amplitude_to_decibel(guard.get_amplitude());
        let pitch = guard.get_osc1_pitch();
        drop(guard);

        let gain_range =  FloatRange::new(-50.0, 0.0);
        
        let pitch_range = FreqRange::new(20.0, 20000.0);

        (Self {
            synthesizer: flags.synthesizer,

            gain_range,
            gain_param: gain_range.normal_param(gain, -20.0),

            pitch_range,
            pitch_param: pitch_range.normal_param(pitch, 220.0),

            waveform: None,
        }, Command::none())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::GainChanged(normal) => {
                self.gain_param.update(normal);
                let mut guard = self.synthesizer.lock().unwrap();
                guard.set_gain(self.gain_range.unmap_to_value(normal));
                drop(guard);
                println!("{}", self.gain_range.unmap_to_value(normal));
            },
            Message::PitchChanged(normal) => {
                self.pitch_param.update(normal);
                let mut guard = self.synthesizer.lock().unwrap();
                guard.set_osc1_pitch(self.pitch_range.unmap_to_value(normal));
                drop(guard);
                println!("{}", self.pitch_range.unmap_to_value(normal));
            },
            Message::WaveFormSelected(waveform) => {
                let wavetable = WaveTable::from_fn(waveform.get_fn(), 128);
                let mut guard = self.synthesizer.lock().unwrap();
                guard.set_osc1_wavetable(wavetable);
                drop(guard);
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

        let pitch_knob = Knob::new(
            self.pitch_param,
            Message::PitchChanged,
        );

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