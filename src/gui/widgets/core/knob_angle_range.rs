use super::math::{PI_OVER_180, TWO_PI};

pub const DEFAULT_ANGLE_MIN: f32 = 30.0 * PI_OVER_180;
pub const DEFAULT_ANGLE_MAX: f32 = (360.0 - 30.0) * PI_OVER_180;

#[derive(Debug, Clone)]
pub struct KnobAngleRange {
    min: f32,
    max: f32,
}

impl std::default::Default for KnobAngleRange {
    fn default() -> Self {
        Self {
            min: DEFAULT_ANGLE_MIN,
            max: DEFAULT_ANGLE_MAX,
        }
    }
}

impl KnobAngleRange {
    pub fn from_deg(min: f32, max: f32) -> Self {
        let min_rad = min * PI_OVER_180;
        let max_rad = max * PI_OVER_180;

        Self::from_rad(min_rad, max_rad)
    }

    pub fn from_rad(min: f32, max: f32) -> Self {
        debug_assert!(min <= max);

        let mut min = min;
        let mut max = max;

        if !(0.0..TWO_PI).contains(&min) {
            min = 0.0;
        }
        if !(0.0..TWO_PI).contains(&max) {
            max = 0.0;
        }

        Self { min, max }
    }

    /// returns the minimum angle (between `0.0` and `TWO_PI` in radians)
    pub fn min(&self) -> f32 {
        self.min
    }
    /// returns the maximum angle (between `0.0` and `TWO_PI` in radians)
    pub fn max(&self) -> f32 {
        self.max
    }
}