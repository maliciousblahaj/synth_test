use std::sync::{Arc, Mutex};

use iced::widget::{self, column, container, text};
use crate::gui::widgets::audio_widgets::{style::knob, FloatRange, Knob, Normal, NormalParam};

use crate::{devices::amplifier::Amplifier, math::decibel_to_amplitude};


#[derive(Clone, Copy, Debug)]
pub enum AmplifierUIEvent {
    GainChanged(Normal),
}

pub struct AmplifierUI {
    amplifier: Arc<Mutex<Box<Amplifier>>>,

    gain_range: FloatRange,
    gain_param: NormalParam,
}

impl AmplifierUI {
    pub fn new(amplifier: Arc<Mutex<Box<Amplifier>>>) -> Self {
        let guard = amplifier.lock().unwrap();
        let gain = guard.get_amplitude();
        drop(guard);

        let gain_range = FloatRange::new(-50.0, 0.0);

        Self {
            amplifier,

            gain_range,
            gain_param: gain_range.normal_param(gain, -20.0),
        }
    }
}

impl<Message, Renderer> Component<Message, Renderer> for AmplifierUI
    where
        Renderer: iced_native::text::Renderer + 'static,
        Renderer::Theme: knob::StyleSheet + widget::text::StyleSheet,
{
    type State = ();
    type Event = AmplifierUIEvent;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            AmplifierUIEvent::GainChanged(normal) => {
                self.gain_param.update(normal);
                let new_gain = self.gain_range.unmap_to_value(normal);
                
                self.amplifier
                    .lock()
                    .unwrap()
                    .set_amplitude(decibel_to_amplitude(new_gain));
            }
        }
        None
    }

    fn view(&self, state: &Self::State) -> iced_native::Element<'_, Self::Event, Renderer> {
        let gain_knob = Knob::new(
            self.gain_param,
            AmplifierUIEvent::GainChanged,
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

        gain_widget.into()
    }
}