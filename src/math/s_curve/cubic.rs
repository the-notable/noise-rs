use crate::math::s_curve::SCurve;
use num_traits::Float;
use vek::{Vec2, Vec3, Vec4};

/// Cubic S-Curve
///
/// Maps the provided value onto the cubic S-curve function -2x<sup>3</sup> + 3x<sup>2</sup>.
/// This creates a curve with endpoints (0,0) and (1,1), and a first derivative of zero at the
/// endpoints, allowing the curves to be combined together without discontinuities.
pub trait Cubic: SCurve {
    fn cubic(&self) -> Self;
}

impl Cubic for f32 {
    fn cubic(&self) -> Self {
        self * self * (3.0 - (self * 2.0))
    }
}

impl Cubic for f64 {
    fn cubic(&self) -> Self {
        self * self * (3.0 - (self * 2.0))
    }
}

impl<T> Cubic for [T; 2]
where
    T: Float + Cubic,
{
    fn cubic(&self) -> Self {
        [self[0].cubic(), self[1].cubic()]
    }
}

impl<T> Cubic for [T; 3]
where
    T: Float + Cubic,
{
    fn cubic(&self) -> Self {
        [self[0].cubic(), self[1].cubic(), self[2].cubic()]
    }
}

impl<T> Cubic for [T; 4]
where
    T: Float + Cubic,
{
    fn cubic(&self) -> Self {
        [
            self[0].cubic(),
            self[1].cubic(),
            self[2].cubic(),
            self[3].cubic(),
        ]
    }
}

impl<T> Cubic for Vec2<T>
where
    T: Float + Cubic,
{
    fn cubic(&self) -> Self {
        self.map(|x| x.cubic())
    }
}

impl<T> Cubic for Vec3<T>
where
    T: Float + Cubic,
{
    fn cubic(&self) -> Self {
        self.map(|x| x.cubic())
    }
}

impl<T> Cubic for Vec4<T>
where
    T: Float + Cubic,
{
    fn cubic(&self) -> Self {
        self.map(|x| x.cubic())
    }
}
