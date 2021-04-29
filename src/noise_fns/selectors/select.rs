use crate::{
    math::{interpolate, s_curve::cubic::Cubic},
    noise_fns::NoiseFn,
};
use std::rc::Rc;
use std::marker::PhantomData;

/// Noise function that outputs the value selected from one of two source
/// functions chosen by the output value from a control function.
pub struct Select<T, N, const DIM: usize>
where
    T: NoiseFn<N, DIM>
{
    /// Outputs a value.
    pub source1: Rc<T>,

    /// Outputs a value.
    pub source2: Rc<T>,

    /// Determines the value to select. If the output value from
    /// the control function is within a range of values know as the _selection
    /// range_, this noise function outputs the value from `source2`.
    /// Otherwise, this noise function outputs the value from `source1`.
    pub control: Rc<T>,

    /// Bounds of the selection range. Default is 0.0 to 1.0.
    pub bounds: (f64, f64),

    /// Edge falloff value. Default is 0.0.
    pub falloff: f64,

    _marker: PhantomData<N>
}

impl<T: NoiseFn<N, DIM>, N, const DIM: usize> Select<T, N, DIM> {
    pub fn new(
        source1: Rc<T>,
        source2: Rc<T>,
        control: Rc<T>,
    ) -> Self {
        Select {
            source1,
            source2,
            control,
            bounds: (0.0, 1.0),
            falloff: 0.0,
            _marker: PhantomData
        }
    }

    pub fn set_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        Select {
            bounds: (lower_bound, upper_bound),
            ..self
        }
    }

    pub fn set_falloff(self, falloff: f64) -> Self {
        Select { falloff, ..self }
    }
}

impl<T: NoiseFn<R, DIM>, R, const DIM: usize> NoiseFn<R, DIM> for Select<T, R, DIM>
where
    R: Copy,
{
    fn get(&self, point: [R; DIM]) -> f64 {
        let control_value = self.control.get(point);
        let (lower, upper) = self.bounds;

        if self.falloff > 0.0 {
            match () {
                _ if control_value < (lower - self.falloff) => self.source1.get(point),
                _ if control_value < (lower + self.falloff) => {
                    let lower_curve = lower - self.falloff;
                    let upper_curve = lower + self.falloff;
                    let alpha =
                        ((control_value - lower_curve) / (upper_curve - lower_curve)).map_cubic();

                    interpolate::linear(self.source1.get(point), self.source2.get(point), alpha)
                }
                _ if control_value < (upper - self.falloff) => self.source2.get(point),
                _ if control_value < (upper + self.falloff) => {
                    let lower_curve = upper - self.falloff;
                    let upper_curve = upper + self.falloff;
                    let alpha =
                        ((control_value - lower_curve) / (upper_curve - lower_curve)).map_cubic();

                    interpolate::linear(self.source2.get(point), self.source1.get(point), alpha)
                }
                _ => self.source1.get(point),
            }
        } else if control_value < lower || control_value > upper {
            self.source1.get(point)
        } else {
            self.source2.get(point)
        }
    }
}
