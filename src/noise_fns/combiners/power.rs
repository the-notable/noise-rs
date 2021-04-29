use crate::noise_fns::NoiseFn;
use std::rc::Rc;

/// Noise function that raises the output value from the first source function
/// to the power of the output value of the second source function.
pub struct Power<T, const DIM: usize> {
    /// Outputs a value.
    pub source1: Rc<dyn NoiseFn<T, DIM>>,

    /// Outputs a value.
    pub source2: Rc<dyn NoiseFn<T, DIM>>,
}

impl<T, const DIM: usize> Power<T, DIM> {
    pub fn new(source1: Rc<dyn NoiseFn<T, DIM>>, source2: Rc<dyn NoiseFn<T, DIM>>) -> Self {
        Self { source1, source2 }
    }
}

impl<T, const DIM: usize> NoiseFn<T, DIM> for Power<T, DIM>
where
    T: Copy,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        (self.source1.get(point)).powf(self.source2.get(point))
    }
}
