use crate::noise_fns::NoiseFn;

/// Noise function that applies a scaling factor and a bias to the output value
/// from the source function.
///
/// The function retrieves the output value from the source function, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
pub struct ScaleBias<'a, T, const DIM: usize> {
    /// Outputs a value.
    pub source: &'a Box<dyn NoiseFn<T, DIM>>,

    /// Scaling factor to apply to the output value from the source function.
    /// The default value is 1.0.
    pub scale: f64,

    /// Bias to apply to the scaled output value from the source function.
    /// The default value is 0.0.
    pub bias: f64,
}

impl<'a, T, const DIM: usize> ScaleBias<'a, T, DIM> {
    pub fn new(source: &'a Box<dyn NoiseFn<T, DIM>>) -> Self {
        Self {
            source,
            scale: 1.0,
            bias: 0.0,
        }
    }

    pub fn set_scale(self, scale: f64) -> Self {
        Self { scale, ..self }
    }

    pub fn set_bias(self, bias: f64) -> Self {
        Self { bias, ..self }
    }
}

impl<'a, T, const DIM: usize> NoiseFn<T, DIM> for ScaleBias<'a, T, DIM> {
    #[cfg(not(target_os = "emscripten"))]
    fn get(&self, point: [T; DIM]) -> f64 {
        (self.source.get(point)).mul_add(self.scale, self.bias)
    }

    #[cfg(target_os = "emscripten")]
    fn get(&self, point: [T; DIM]) -> f64 {
        (self.source.get(point) * self.scale) + self.bias
    }
}
