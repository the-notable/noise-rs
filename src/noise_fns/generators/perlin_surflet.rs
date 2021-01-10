use crate::{
    gradient, math,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::{NoiseHasher, PermutationTable},
};
use vek::{Vec2, Vec3, Vec4};

/// Noise function that outputs 2/3/4-dimensional Perlin noise.
///
/// THis is a variant of original perlin noise, based on the principles of simplex noise to
/// calculate the values at a point using wavelets instead of interpolated gradients.
#[derive(Clone, Copy, Debug)]
pub struct Perlin {
    seed: u32,
    perm_table: PermutationTable,
}

impl Perlin {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new() -> Self {
        Self {
            seed: Self::DEFAULT_SEED,
            perm_table: PermutationTable::new(Self::DEFAULT_SEED),
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Seedable for Perlin {
    /// Sets the seed value for Perlin noise
    fn set_seed(self, seed: u32) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Self {
            seed,
            perm_table: PermutationTable::new(seed),
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional perlin noise
impl NoiseFn<[f64; 2]> for Perlin {
    fn get(&self, point: [f64; 2]) -> f64 {
        let point = Vec2::from(point);

        let floored = point.floor();
        let corner = floored.as_();
        let distance = point - floored;

        const SCALE_FACTOR: f64 = 3.160_493_827_160_493_7;

        #[inline(always)]
        fn surflet(hasher: &dyn NoiseHasher, corner: Vec2<isize>, distance: Vec2<f64>) -> f64 {
            let attn = 1.0 - distance.dot(distance);
            if attn > 0.0 {
                attn.powi(4) * distance.dot(Vec2::from(gradient::get2(hasher.hash(&corner))))
            } else {
                0.0
            }
        }

        let f00 = surflet(&self.perm_table, corner, distance);
        let f10 = surflet(&self.perm_table, corner + [1, 0], distance - [1.0, 0.0]);
        let f01 = surflet(&self.perm_table, corner + [0, 1], distance - [0.0, 1.0]);
        let f11 = surflet(
            &self.perm_table,
            corner + Vec2::one(),
            distance - Vec2::one(),
        );

        // Multiply by arbitrary value to scale to -1..1
        math::clamp((f00 + f10 + f01 + f11) * SCALE_FACTOR, -1.0, 1.0)
    }
}

/// 3-dimensional perlin noise
impl NoiseFn<[f64; 3]> for Perlin {
    fn get(&self, point: [f64; 3]) -> f64 {
        let point = Vec3::from(point);

        let floored = point.floor();
        let corner = floored.as_();
        let distance = point - floored;

        const SCALE_FACTOR: f64 = 3.889_855_325_553_107_4;

        #[inline(always)]
        fn surflet(hasher: &dyn NoiseHasher, corner: Vec3<isize>, distance: Vec3<f64>) -> f64 {
            let attn = 1.0 - distance.dot(distance);
            if attn > 0.0 {
                attn.powi(4) * distance.dot(Vec3::from(gradient::get3(hasher.hash(&corner))))
            } else {
                0.0
            }
        }

        let f000 = surflet(&self.perm_table, corner, distance);
        let f100 = surflet(
            &self.perm_table,
            corner + [1, 0, 0],
            distance - [1.0, 0.0, 0.0],
        );
        let f010 = surflet(
            &self.perm_table,
            corner + [0, 1, 0],
            distance - [0.0, 1.0, 0.0],
        );
        let f110 = surflet(
            &self.perm_table,
            corner + [1, 1, 0],
            distance - [1.0, 1.0, 0.0],
        );
        let f001 = surflet(
            &self.perm_table,
            corner + [0, 0, 1],
            distance - [0.0, 0.0, 1.0],
        );
        let f101 = surflet(
            &self.perm_table,
            corner + [1, 0, 1],
            distance - [1.0, 0.0, 1.0],
        );
        let f011 = surflet(
            &self.perm_table,
            corner + [0, 1, 1],
            distance - [0.0, 1.0, 1.0],
        );
        let f111 = surflet(
            &self.perm_table,
            corner + Vec2::one(),
            distance - Vec2::one(),
        );

        // Multiply by arbitrary value to scale to -1..1
        math::clamp(
            (f000 + f100 + f010 + f110 + f001 + f101 + f011 + f111) * SCALE_FACTOR,
            -1.0,
            1.0,
        )
    }
}

/// 4-dimensional perlin noise
impl NoiseFn<[f64; 4]> for Perlin {
    fn get(&self, point: [f64; 4]) -> f64 {
        let point = Vec4::from(point);

        let floored = point.floor();
        let corner = floored.as_();
        let distance = point - floored;

        const SCALE_FACTOR: f64 = 4.424_369_240_215_691;

        #[inline(always)]
        fn surflet(perm_table: &PermutationTable, corner: Vec4<isize>, distance: Vec4<f64>) -> f64 {
            let attn = 1.0 - distance.dot(distance);
            if attn > 0.0 {
                attn.powi(4) * distance.dot(Vec4::from(gradient::get4(perm_table.hash(&corner))))
            } else {
                0.0
            }
        }

        let f0000 = surflet(&self.perm_table, corner, distance);
        let f1000 = surflet(
            &self.perm_table,
            corner + [1, 0, 0, 0],
            distance - [1.0, 0.0, 0.0, 0.0],
        );
        let f0100 = surflet(
            &self.perm_table,
            corner + [0, 1, 0, 0],
            distance - [0.0, 1.0, 0.0, 0.0],
        );
        let f1100 = surflet(
            &self.perm_table,
            corner + [1, 1, 0, 0],
            distance - [1.0, 1.0, 0.0, 0.0],
        );
        let f0010 = surflet(
            &self.perm_table,
            corner + [0, 0, 1, 0],
            distance - [0.0, 0.0, 1.0, 0.0],
        );
        let f1010 = surflet(
            &self.perm_table,
            corner + [1, 0, 1, 0],
            distance - [1.0, 0.0, 1.0, 0.0],
        );
        let f0110 = surflet(
            &self.perm_table,
            corner + [0, 1, 1, 0],
            distance - [0.0, 1.0, 1.0, 0.0],
        );
        let f1110 = surflet(
            &self.perm_table,
            corner + [1, 1, 1, 0],
            distance - [1.0, 1.0, 1.0, 0.0],
        );
        let f0001 = surflet(
            &self.perm_table,
            corner + [0, 0, 0, 1],
            distance - [0.0, 0.0, 0.0, 1.0],
        );
        let f1001 = surflet(
            &self.perm_table,
            corner + [1, 0, 0, 1],
            distance - [1.0, 0.0, 0.0, 1.0],
        );
        let f0101 = surflet(
            &self.perm_table,
            corner + [0, 1, 0, 1],
            distance - [0.0, 1.0, 0.0, 1.0],
        );
        let f1101 = surflet(
            &self.perm_table,
            corner + [1, 1, 0, 1],
            distance - [1.0, 1.0, 0.0, 1.0],
        );
        let f0011 = surflet(
            &self.perm_table,
            corner + [0, 0, 1, 1],
            distance - [0.0, 0.0, 1.0, 1.0],
        );
        let f1011 = surflet(
            &self.perm_table,
            corner + [1, 0, 0, 0],
            distance - [1.0, 0.0, 0.0, 0.0],
        );
        let f0111 = surflet(
            &self.perm_table,
            corner + [1, 0, 0, 0],
            distance - [1.0, 0.0, 0.0, 0.0],
        );
        let f1111 = surflet(
            &self.perm_table,
            corner + [1, 0, 0, 0],
            distance - [1.0, 0.0, 0.0, 0.0],
        );

        // Multiply by arbitrary value to scale to -1..1
        math::clamp(
            (f0000
                + f1000
                + f0100
                + f1100
                + f0010
                + f1010
                + f0110
                + f1110
                + f0001
                + f1001
                + f0101
                + f1101
                + f0011
                + f1011
                + f0111
                + f1111)
                * SCALE_FACTOR,
            -1.0,
            1.0,
        )
    }
}
