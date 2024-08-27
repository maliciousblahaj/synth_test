pub use crate::gui::widgets::audio_widgets::style::knob::{
    Appearance, ArcAppearance, ArcBipolarAppearance, CircleAppearance,
    CircleNotch, LineCap, LineNotch, ModRangeArcAppearance, NotchShape,
    StyleLength, StyleSheet, TextMarksAppearance, TickMarksAppearance,
    ValueArcAppearance,
};
use crate::gui::widgets::audio_widgets::{native::{text_marks, tick_marks}, ModulationRange};

struct ValueMarkers<'a> {
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
    tick_marks_style: Option<TickMarksAppearance>,
    text_marks_style: Option<TextMarksAppearance>,
    value_arc_style: Option<ValueArcAppearance>,
    mod_range_style_1: Option<ModRangeArcAppearance>,
    mod_range_style_2: Option<ModRangeArcAppearance>,
}