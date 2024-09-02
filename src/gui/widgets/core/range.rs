use super::{normal::Normal, normal_param::NormalParam};

#[derive(Debug, Copy, Clone)]
pub struct FloatRange {
    min: f32,
    max: f32,
    span: f32,
    span_recip: f32,
}

impl FloatRange {
    pub fn new(min: f32, max: f32) -> Self {
        assert!(max > min);

        let span = max - min;
        let span_recip = span.recip();

        Self {
            min,
            max,
            span,
            span_recip,
        }
    }

    fn constrain(&self, value: f32) -> f32 {
        if value <= self.min {
            self.min
        } else if value >= self.max {
            self.max
        } else {
            value
        }
    }

    pub fn normal_param(&self, value: f32, default: f32) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(value),
            default: self.map_to_normal(default),
        }
    }

    pub fn map_to_normal(&self, value: f32) -> Normal {
        let value = self.constrain(value);
        Normal::from_clipped((value - self.min) * self.span_recip)
    }

    pub fn unmap_to_value(&self, normal: Normal) -> f32 {
        (normal.as_f32() * self.span) + self.min
    }
}

impl Default for FloatRange {
    fn default() -> Self {
        FloatRange::new(0.0, 1.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IntRange {
    min: i32,
    max: i32,
    span: f32,
    span_recip: f32,
}

impl IntRange {
    pub fn new(min: i32, max: i32) -> Self {
        assert!(max > min);

        let span = (max - min) as f32;
        let span_recip = span.recip();

        Self {
            min,
            max,
            span,
            span_recip,
        }
    }

    fn constrain(&self, value: i32) -> i32 {
        if value <= self.min {
            self.min
        } else if value >= self.max {
            self.max
        } else {
            value
        }
    }

    pub fn normal_param(&self, value: i32, default: i32) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(value),
            default: self.map_to_normal(default),
        }
    }

    pub fn snapped(&self, normal: Normal) -> Normal {
        let value_int = self.unmap_to_value(normal);
        self.map_to_normal(value_int)
    }

    pub fn map_to_normal(&self, value: i32) -> Normal {
        let value = self.constrain(value);
        Normal::from_clipped((value - self.min) as f32 * self.span_recip)
    }

    pub fn unmap_to_value(&self, normal: Normal) -> i32 {
        (normal.as_f32() * self.span).round() as i32 + self.min
    }
}

impl Default for IntRange {
    fn default() -> Self {
        IntRange::new(0, 100)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LogDBRange {
    min: f32,
    max: f32,
    zero_position: Normal,
    min_recip: f32,
    max_recip: f32,
    zero_pos_recip: f32,
    one_minus_zero_position_recip: f32,
}

impl LogDBRange {
    pub fn new(min: f32, max: f32, zero_position: Normal) -> Self {
        assert!(max > min, "max must be greater than min");
        assert!(max >= 0.0, "max must be 0.0 or positive");
        assert!(min <= 0.0, "min must be 0.0 or negative");

        let min_recip = if min == 0.0 { 0.0 } else { min.recip() };

        let max_recip = if max == 0.0 { 0.0 } else { max.recip() };

        let zero_pos_recip = if zero_position.as_f32() == 0.0 {
            0.0
        } else {
            zero_position.as_f32().recip()
        };

        let one_minus_zero_position_recip = if zero_position.as_f32() == 0.0 {
            0.0
        } else {
            (1.0 - zero_position.as_f32()).recip()
        };

        Self {
            min,
            max,
            zero_position,
            min_recip,
            max_recip,
            zero_pos_recip,
            one_minus_zero_position_recip,
        }
    }

    fn constrain(&self, value: f32) -> f32 {
        if value <= self.min {
            self.min
        } else if value >= self.max {
            self.max
        } else {
            value
        }
    }

    pub fn normal_param(&self, value: f32, default: f32) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(value),
            default: self.map_to_normal(default),
        }
    }

    pub fn map_to_normal(&self, value: f32) -> Normal {
        let value = self.constrain(value);
        if value == 0.0 {
            self.zero_position
        } else if value < 0.0 {
            if self.min >= 0.0 {
                return Normal::MIN;
            }
            let neg_normal = value * self.min_recip;

            let log_normal = 1.0 - neg_normal.sqrt();

            Normal::from_clipped(log_normal * self.zero_position.as_f32())
        } else {
            if self.max <= 0.0 {
                return Normal::MAX;
            }
            let pos_normal = value * self.max_recip;

            let log_normal = pos_normal.sqrt();

            Normal::from_clipped(
                (log_normal * (1.0 - self.zero_position.as_f32()))
                    + self.zero_position.as_f32(),
            )
        }
    }

    pub fn unmap_to_value(&self, normal: Normal) -> f32 {
        if normal == self.zero_position {
            0.0
        } else if normal < self.zero_position {
            if self.min >= 0.0 {
                return self.min;
            }
            let neg_normal = 1.0 - (normal.as_f32() * self.zero_pos_recip);

            let log_normal = 1.0 - (neg_normal * neg_normal);

            (1.0 - log_normal) * self.min
        } else {
            if self.zero_position.as_f32() == 1.0 || self.max <= 0.0 {
                return self.max;
            }
            let pos_normal = (normal.as_f32() - self.zero_position.as_f32())
                * self.one_minus_zero_position_recip;

            let log_normal = pos_normal * pos_normal;

            log_normal * self.max
        }
    }
}

impl Default for LogDBRange {
    fn default() -> Self {
        LogDBRange::new(-12.0, 12.0, Normal::CENTER)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FreqRange {
    min: f32,
    max: f32,
    spectrum_normal_span: f32,
    spectrum_normal_span_recip: f32,
    min_spectrum_normal: Normal,
}

impl FreqRange {
    pub fn new(min: f32, max: f32) -> Self {
        assert!(max > min);

        let mut min = min;
        if min < 20.0 {
            min = 20.0;
        }

        let mut max = max;
        if max > 20480.0 {
            max = 20480.0;
        }

        let min_spectrum_normal = frequency_to_normal(min);
        let max_spectrum_normal = frequency_to_normal(max);

        let spectrum_normal_span =
            max_spectrum_normal.as_f32() - min_spectrum_normal.as_f32();

        let spectrum_normal_span_recip = spectrum_normal_span.recip();

        Self {
            min,
            max,
            spectrum_normal_span,
            min_spectrum_normal,
            spectrum_normal_span_recip,
        }
    }

    fn constrain(&self, value: f32) -> f32 {
        if value <= self.min {
            self.min
        } else if value >= self.max {
            self.max
        } else {
            value
        }
    }

    pub fn normal_param(&self, value: f32, default: f32) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(value),
            default: self.map_to_normal(default),
        }
    }

    pub fn map_to_normal(&self, value: f32) -> Normal {
        let value = self.constrain(value);
        let spectrum_normal = frequency_to_normal(value);
        Normal::from_clipped(
            (spectrum_normal.as_f32() - self.min_spectrum_normal.as_f32())
                * self.spectrum_normal_span_recip,
        )
    }

    pub fn unmap_to_value(&self, normal: Normal) -> f32 {
        let spectrum_normal = Normal::from_clipped(
            normal.as_f32() * self.spectrum_normal_span
                + self.min_spectrum_normal.as_f32(),
        );

        normal_to_frequency(spectrum_normal)
    }
}

impl Default for FreqRange {
    fn default() -> Self {
        FreqRange::new(20.0, 20_480.0)
    }
}

fn normal_to_frequency(value: Normal) -> f32 {
    40.0 * 2.0_f32.powf((10.0 * value.as_f32()) - 1.0)
}

fn frequency_to_normal(freq: f32) -> Normal {
    Normal::from_clipped(((freq / 40.0).log2() + 1.0) * 0.1)
}