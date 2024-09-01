//an iced 0.12 implementation of the widgets i need from iced_audio

pub mod core;
pub mod graphics;
pub mod style;
pub mod native;

pub use core::*;


mod platform {
    #[doc(no_inline)]
    pub use crate::gui::widgets::audio_widgets::graphics::knob;

    //#[doc(no_inline)]
    //pub use knob::Knob;
    /*
    #[doc(no_inline)]
    pub use crate::gui::widgets::audio_widgets::graphics::{
        h_slider, knob, mod_range_input, ramp, text_marks, tick_marks,
        v_slider, xy_pad,
    };

    #[doc(no_inline)]
    pub use {
        h_slider::HSlider, knob::Knob, mod_range_input::ModRangeInput,
        ramp::Ramp, v_slider::VSlider, xy_pad::XYPad,
    };*/
}

#[doc(no_inline)]
pub use platform::*;

use iced::Renderer;