use std::sync::{Arc, Mutex};

use iced::{widget::{self, column, container, pick_list, row, text}, Element};
use crate::gui::widgets::audio_widgets::{knob, native::knob::Renderer, FreqRange, Knob, Normal, NormalParam};
use iced_lazy::Component;

use crate::{devices::oscillator::{self, WaveTableOscillator}, synthesis::{waveforms::WaveForm, wavetable::WaveTable}};


#[derive(Clone, Copy, Debug)]
pub enum OscillatorUIEvent {
    PitchChanged(Normal),
    WaveFormSelected(WaveForm),
}

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
Renderer::Theme: knob::StyleSheet + widget::text::StyleSheet,
    */

impl<Message, Renderer> Component<Message, Renderer> for OscillatorUI
where
    Renderer: 
        iced_native::text::Renderer + 'static,
    Renderer::Theme:
        iced::widget::container::StyleSheet +
        iced::widget::pick_list::StyleSheet +
        iced::widget::scrollable::StyleSheet,
    <Renderer as iced_native::Renderer>::Theme: iced::overlay::menu::StyleSheet,
    <<Renderer as iced_native::Renderer>::Theme as iced::overlay::menu::StyleSheet>::Style: From<<<Renderer as iced_native::Renderer>::Theme as iced::widget::pick_list::StyleSheet>::Style>,
    /*  iced_audio::knob::StyleSheet +
        iced::widget::text::StyleSheet +
        iced::widget::container::StyleSheet + 
        iced::widget::scrollable::StyleSheet + 
        iced::overlay::menu::StyleSheet + 
        iced::widget::pick_list::StyleSheet,*/
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

    fn view(&self, state: &Self::State) -> iced_native::Element<'_, Self::Event, Renderer> {
        let pitch_knob = Knob::new(
            self.pitch_param,
            OscillatorUIEvent::PitchChanged
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
            Some(self.waveform), 
            OscillatorUIEvent::WaveFormSelected
        );

        let waveform_widget = container(waveform_list)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .height(100)
            .width(100);
        
        //row![
        //    pitch_widget,
        //    waveform_widget
        //].into()
        pitch_widget.into()
    }
}

impl<'a, Message, Renderer> From<OscillatorUI>
        for Element<'a, Message, Renderer>
    where
        Message: 'a,
        Renderer:
            iced_native::text::Renderer + 'static,
        /*Renderer::Theme:
            iced_audio::knob::StyleSheet +
            iced::widget::text::StyleSheet +
            iced::widget::container::StyleSheet + 
            iced::widget::scrollable::StyleSheet + 
            iced::overlay::menu::StyleSheet + 
            iced::widget::pick_list::StyleSheet,*/
    {
        fn from(oscillator_ui: OscillatorUI) -> Self {
            iced_lazy::component(oscillator_ui)
        }
    }