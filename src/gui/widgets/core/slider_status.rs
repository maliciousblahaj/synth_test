/// Moved status for the virtual sliders.
///
/// This allows tracking the virtual slider actual movements
/// thus preventing some events from unnecessary being emitted.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum SliderStatus {
    Moved,
    #[default]
    Unchanged,
}

impl SliderStatus {
    /// Sets the slider as moved.
    pub fn set_moved(&mut self) {
        *self = SliderStatus::Moved;
    }
}