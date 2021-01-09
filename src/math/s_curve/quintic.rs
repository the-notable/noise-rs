use crate::math::s_curve::SCurve;
use num_traits::Float;
use vek::{Vec2, Vec3, Vec4};

/// Quintic Interpolation Trait
///
/// Interpolates the provided value according to the quintic S-curve function
/// 6x<sup>5</sup> - 15x<sup>4</sup> + 10x<sup>3</sup>. This creates a curve with endpoints (0,0)
/// and (1,1), and first and second derivatives of zero at the endpoints, allowing the curves to be
/// combined together without discontinuities.
pub trait Quintic: SCurve {
    fn quintic(&self) -> Self;
}

impl Quintic for f32 {
    fn quintic(&self) -> Self {
        self * self * self * (self * (self * 6.0 - 15.0) + 10.0)
    }
}

impl Quintic for f64 {
    fn quintic(&self) -> Self {
        self * self * self * (self * (self * 6.0 - 15.0) + 10.0)
    }
}

impl<T> Quintic for [T; 2]
where
    T: Float + Quintic,
{
    fn quintic(&self) -> Self {
        [self[0].quintic(), self[1].quintic()]
    }
}

impl<T> Quintic for [T; 3]
where
    T: Float + Quintic,
{
    fn quintic(&self) -> Self {
        [self[0].quintic(), self[1].quintic(), self[2].quintic()]
    }
}

impl<T> Quintic for [T; 4]
where
    T: Float + Quintic,
{
    fn quintic(&self) -> Self {
        [
            self[0].quintic(),
            self[1].quintic(),
            self[2].quintic(),
            self[3].quintic(),
        ]
    }
}

impl<T> Quintic for Vec2<T>
where
    T: Float + Quintic,
{
    fn quintic(&self) -> Self {
        self.map(|x| x.quintic())
    }
}

impl<T> Quintic for Vec3<T>
where
    T: Float + Quintic,
{
    fn quintic(&self) -> Self {
        self.map(|x| x.quintic())
    }
}

impl<T> Quintic for Vec4<T>
where
    T: Float + Quintic,
{
    fn quintic(&self) -> Self {
        self.map(|x| x.quintic())
    }
}

// #[inline(always)]
// pub fn quintic<F>(x: F) -> F
//     where
//         F: Float,
// {
//     x * x * x * (x * (x * F::from(6.0).unwrap() - F::from(15.0).unwrap()) + F::from(10.0).unwrap())
// }
