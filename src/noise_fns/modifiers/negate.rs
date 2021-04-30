use crate::noise_fns::NoiseFn;
use std::rc::Rc;
use crate::WrapRc;

/// Noise function that negates the output value from the source function.
pub struct Negate<T, const DIM: usize> {
    /// Outputs a value.
    pub source: Rc<dyn NoiseFn<T, DIM>>,
}

impl<T, const DIM: usize> Negate<T, DIM> {
    pub fn new(source: Rc<dyn NoiseFn<T, DIM>>) -> Self {
        Negate { source }
    }
}

impl<T, const DIM: usize> WrapRc for Negate<T, DIM> {}

impl<T, const DIM: usize> NoiseFn<T, DIM> for Negate<T, DIM> {
    fn get(&self, point: [T; DIM]) -> f64 {
        -self.source.get(point)
    }
}
