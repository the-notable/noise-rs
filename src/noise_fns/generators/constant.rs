use crate::noise_fns::NoiseFn;
use crate::WrapRc;

/// Noise function that outputs a constant value.
///
/// This function takes a input, value, and returns that input for all points,
/// producing a constant-valued field.
///
/// This function is not very useful by itself, but can be used as a source
/// function for other noise functions.
#[derive(Clone, Copy, Debug)]
pub struct Constant {
    /// Constant value.
    pub value: f64,
}

impl Constant {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl WrapRc for Constant {}

impl<T: Copy, const N: usize> NoiseFn<T, N> for Constant {
    fn get(&self, _point: [T; N]) -> f64 {
        self.value
    }
}
