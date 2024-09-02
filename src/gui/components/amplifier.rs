use std::sync::{Arc, Mutex};

use iced::widget::{column, component, container, text, Component};
use iced::Element;
use crate::gui::widgets::core::normal::Normal;
use crate::gui::widgets::core::normal_param::NormalParam;
use crate::gui::widgets::core::range::LogDBRange;
use crate::gui::widgets::knob::Knob;
use crate::{devices::amplifier::Amplifier, math::decibel_to_amplitude};


#[derive(Clone, Copy, Debug)]
pub enum AmplifierUIEvent {
    GainChanged(Normal),
}

pub struct AmplifierUI {
    amplifier: Arc<Mutex<Box<Amplifier>>>,

    gain_range: LogDBRange,
    gain_param: NormalParam,
}

impl AmplifierUI {
    pub fn new(amplifier: Arc<Mutex<Box<Amplifier>>>) -> Self {
        let guard = amplifier.lock().unwrap();
        let gain = guard.get_amplitude();
        drop(guard);

        let gain_range = LogDBRange::new(-56.0, 0.0, Normal::from_clipped(0.7));

        Self {
            amplifier,

            gain_range,
            gain_param: gain_range.normal_param(gain, -20.0),
        }
    }
}

impl<Message> Component<Message> for AmplifierUI
    where
        //Theme: 
        //iced::widget::container::Catalog +
        //iced::widget::text::Catalog,
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

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let gain_knob = Knob::new(
            self.gain_param,
            AmplifierUIEvent::GainChanged,
        );

        let gain_widget = container(
            column![
                text("gain"),
                gain_knob,
                text(format!("{:.1}db", self.gain_range.unmap_to_value(self.gain_param.value)))
            ].align_x(iced::Alignment::Center)
        )
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .height(100)
            .width(100);

        gain_widget.into()
    }
}

impl<'a, Message: 'a> From<AmplifierUI> for Element<'a, Message> {
    fn from(amplifier_ui: AmplifierUI) -> Self {
        component(amplifier_ui)
    }
}