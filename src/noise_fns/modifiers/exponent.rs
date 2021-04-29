use crate::{math::scale_shift, noise_fns::NoiseFn};
use std::rc::Rc;

/// Noise function that maps the output value from the source function onto an
/// exponential curve.
///
/// Because most noise functions will output values that range from -1.0 to 1.0,
/// this noise function first normalizes the output value (the range becomes 0.0
/// to 1.0), maps that value onto an exponential curve, then rescales that
/// value back to the original range.
pub struct Exponent<T, const DIM: usize> {
    /// Outputs a value.
    pub source: Rc<dyn NoiseFn<T, DIM>>,

    /// Exponent to apply to the output value from the source function. Default
    /// is 1.0.
    pub exponent: f64,
}

impl<T, const DIM: usize> Exponent<T, DIM> {
    pub fn new(source: Rc<dyn NoiseFn<T, DIM>>) -> Self {
        Self {
            source,
            exponent: 1.0,
        }
    }

    pub fn set_exponent(self, exponent: f64) -> Self {
        Self { exponent, ..self }
    }
}

impl<T, const DIM: usize> NoiseFn<T, DIM> for Exponent<T, DIM> {
    fn get(&self, point: [T; DIM]) -> f64 {
        let mut value = self.source.get(point);
        value = (value + 1.0) / 2.0;
        value = value.abs();
        value = value.powf(self.exponent);
        scale_shift(value, 2.0)
    }
}
