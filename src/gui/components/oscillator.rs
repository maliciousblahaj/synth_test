use std::sync::{Arc, Mutex};

use iced::{widget::{column, row, container, pick_list, text, Component, component}, Element};

use crate::{gui::widgets::{core::{normal::Normal, normal_param::NormalParam, range::FreqRange}, knob::Knob}, synthesis::wavetable::WaveTable};
use crate::{devices::oscillator::WaveTableOscillator, synthesis::waveforms::WaveForm};


#[derive(Clone, Copy, Debug)]
pub enum OscillatorUIEvent {
    PitchChanged(Normal),
    WaveFormSelected(WaveForm),
}

#[derive(Clone, Debug)]
pub struct OscillatorUI {
    oscillator: Arc<Mutex<Box<WaveTableOscillator>>>,

    pitch_range: FreqRange,
    pitch_param: NormalParam,

    waveform: WaveForm,
}

impl OscillatorUI {
    pub fn new(oscillator: Arc<Mutex<Box<WaveTableOscillator>>>) -> Self {
        let guard = oscillator.lock().unwrap();
        let frequency = guard.get_frequency();
        let waveform = guard.get_waveform();
        drop(guard);

        let pitch_range = FreqRange::new(20.0, 20000.0);

        Self {
            oscillator,
            pitch_range,
            pitch_param: pitch_range.normal_param(frequency, 220.0),
            waveform,
        }
    }
}

/*
Renderer: iced_native::text::Renderer + 'static,
Renderer::Theme: knob::Catalog + widget::text::Catalog,
    */

impl<Message> Component<Message> for OscillatorUI
    /*  iced_audio::knob::Catalog +
        iced::widget::text::Catalog +
        iced::widget::container::Catalog + 
        iced::widget::scrollable::Catalog + 
        iced::overlay::menu::Catalog + 
        iced::widget::pick_list::Catalog,*/
{
    type State = ();
    type Event = OscillatorUIEvent;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            OscillatorUIEvent::PitchChanged(normal) => {
                self.pitch_param.update(normal);
                let new_pitch = self.pitch_range.unmap_to_value(normal);
                self.oscillator.lock().unwrap().set_frequency(new_pitch);
            },
            OscillatorUIEvent::WaveFormSelected(waveform) => {
                println!("{waveform:?}");
                let wavetable = WaveTable::from_waveform(waveform, 128);
                self.oscillator.lock().unwrap().set_wavetable(wavetable);
            },
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let pitch_knob = Knob::new(
            self.pitch_param,
            OscillatorUIEvent::PitchChanged
        );
        let pitch_widget = container(
            column![
                text("pitch"),
                pitch_knob,
                text(format!("{:.1}hz", self.pitch_range.unmap_to_value(self.pitch_param.value)))
            ].align_x(iced::Alignment::Center)
        )
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .height(100)
            .width(100);

        let waveform_list = pick_list(
            &WaveForm::ALL[..], 
            Some(self.waveform), 
            OscillatorUIEvent::WaveFormSelected
        );

        let waveform_widget = container(waveform_list)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .height(100)
            .width(100);
        
        row![
            pitch_widget,
            waveform_widget
        ].into()
        //pitch_widget.into()
    }
}

impl<'a, Message: 'a> From<OscillatorUI> for Element<'a, Message> {
    fn from(oscillator_ui: OscillatorUI) -> Self {
        component(oscillator_ui)
    }
}