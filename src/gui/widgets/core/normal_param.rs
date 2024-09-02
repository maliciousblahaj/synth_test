use super::normal::Normal;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NormalParam {
    pub value: Normal,
    pub default: Normal,
}


impl Default for NormalParam {
    fn default() -> Self {
        Self {
            value: Normal::MIN,
            default: Normal::MIN
        }
    }
}

impl NormalParam {
    #[inline]
    pub fn update(&mut self, normal: Normal) {
        self.value = normal;
    }
}