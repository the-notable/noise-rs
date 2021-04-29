use crate::noise_fns::NoiseFn;
use std::rc::Rc;

/// Noise function that outputs the absolute value of the output value from the
/// source function.
pub struct Abs<T, const DIM: usize> {
    /// Outputs a value.
    pub source: Rc<dyn NoiseFn<T, DIM>>,
}

impl<T, const DIM: usize> Abs<T, DIM> {
    pub fn new(source: Rc<dyn NoiseFn<T, DIM>>) -> Self {
        Self { source }
    }
}

impl<T, const DIM: usize> NoiseFn<T, DIM> for Abs<T, DIM> {
    fn get(&self, point: [T; DIM]) -> f64 {
        (self.source.get(point)).abs()
    }
}
