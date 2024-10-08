#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Normal {
    value: f32,
}

impl Default for Normal {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl Normal {
    pub const MIN: Self = Self { value: 0.0 };
    pub const CENTER: Self = Self { value: 0.5 };
    pub const MAX: Self = Self { value: 1.0 };

    pub fn from_clipped(value: f32) -> Self {
        Self {
            value: {
                if value < 0.0 {
                    0.0
                } else if value > 1.0 {
                    1.0
                } else {
                    value
                }
            },
        }
    }


    pub fn set_clipped(&mut self, value: f32) {
        *self = Normal::from_clipped(value)
    }

    pub fn as_f32(&self) -> f32 {
        self.value
    }

    pub fn scale(&self, scalar: f32) -> f32 {
        self.value * scalar
    }
}

impl From<Normal> for f32 {
    fn from(normal: Normal) -> f32 {
        normal.value
    }
}