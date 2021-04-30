use crate::noise_fns::NoiseFn;
use std::rc::Rc;
use crate::WrapRc;

/// Noise function that outputs the larger of the two output values from two source
/// functions.
pub struct Max<T, const DIM: usize> {
    /// Outputs a value.
    pub source1: Rc<dyn NoiseFn<T, DIM>>,

    /// Outputs a value.
    pub source2: Rc<dyn NoiseFn<T, DIM>>,
}

impl<T, const DIM: usize> Max<T, DIM> {
    pub fn new(source1: Rc<dyn NoiseFn<T, DIM>>, source2: Rc<dyn NoiseFn<T, DIM>>) -> Self {
        Self { source1, source2 }
    }
}

impl<T, const DIM: usize> WrapRc for Max<T, DIM> {}

impl<T, const DIM: usize> NoiseFn<T, DIM> for Max<T, DIM>
where
    T: Copy,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        (self.source1.get(point)).max(self.source2.get(point))
    }
}
