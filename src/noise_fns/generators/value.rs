use crate::{
    math::{interpolate::LinearInterpolate, s_curve::quintic::Quintic},
    noise_fns::{NoiseFn, Seedable},
    permutationtable::{NoiseHasher, PermutationTable},
};
use vek::{Vec2, Vec3, Vec4};

/// Noise function that outputs 2/3/4-dimensional Value noise.
#[derive(Clone, Copy, Debug)]
pub struct Value {
    seed: u32,
    perm_table: PermutationTable,
}

impl Value {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new() -> Self {
        Self {
            seed: Self::DEFAULT_SEED,
            perm_table: PermutationTable::new(Self::DEFAULT_SEED),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::new()
    }
}

impl Seedable for Value {
    /// Sets the seed value for Value noise
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

/// 2-dimensional value noise
impl NoiseFn<[f64; 2]> for Value {
    fn get(&self, point: [f64; 2]) -> f64 {
        let point = Vec2::from(point);

        let floored: Vec2<f64> = point.floor();
        let corner = floored.as_();

        let weight = (point - floored).quintic();

        fn get(hasher: &dyn NoiseHasher, corner: Vec2<isize>) -> f64 {
            hasher.hash(&corner) as f64 / 255.0
        }

        let f00 = get(&self.perm_table, corner);
        let f10 = get(&self.perm_table, corner + [1, 0]);
        let f01 = get(&self.perm_table, corner + [0, 1]);
        let f11 = get(&self.perm_table, corner + [1, 1]);

        let d0 = weight.x.lerp(f00, f10);
        let d1 = weight.x.lerp(f01, f11);
        let d = weight.y.lerp(d0, d1);

        d * 2.0 - 1.0
    }
}

/// 3-dimensional value noise
impl NoiseFn<[f64; 3]> for Value {
    fn get(&self, point: [f64; 3]) -> f64 {
        let point = Vec3::from(point);

        let floored: Vec3<f64> = point.floor();
        let corner = floored.as_();

        let weight = (point - floored).quintic();

        fn get(hasher: &dyn NoiseHasher, corner: Vec3<isize>) -> f64 {
            hasher.hash(&corner) as f64 / 255.0
        }

        let f000 = get(&self.perm_table, corner);
        let f100 = get(&self.perm_table, corner + [1, 0, 0]);
        let f010 = get(&self.perm_table, corner + [0, 1, 0]);
        let f110 = get(&self.perm_table, corner + [1, 1, 0]);
        let f001 = get(&self.perm_table, corner + [0, 0, 1]);
        let f101 = get(&self.perm_table, corner + [1, 0, 1]);
        let f011 = get(&self.perm_table, corner + [0, 1, 1]);
        let f111 = get(&self.perm_table, corner + Vec2::one());

        let d00 = weight.x.lerp(f000, f100);
        let d01 = weight.x.lerp(f001, f101);
        let d10 = weight.x.lerp(f010, f110);
        let d11 = weight.x.lerp(f011, f111);
        let d0 = weight.y.lerp(d00, d10);
        let d1 = weight.y.lerp(d01, d11);
        let d = weight.z.lerp(d0, d1);

        d * 2.0 - 1.0
    }
}

/// 4-dimensional value noise
impl NoiseFn<[f64; 4]> for Value {
    fn get(&self, point: [f64; 4]) -> f64 {
        let point = Vec4::from(point);

        let floored: Vec4<f64> = point.floor();
        let corner = floored.as_();

        let weight = (point - floored).quintic();

        fn get(hasher: &dyn NoiseHasher, corner: Vec4<isize>) -> f64 {
            hasher.hash(&corner) as f64 / 255.0
        }

        let f0000 = get(&self.perm_table, corner);
        let f1000 = get(&self.perm_table, corner + [1, 0, 0, 0]);
        let f0100 = get(&self.perm_table, corner + [0, 1, 0, 0]);
        let f1100 = get(&self.perm_table, corner + [1, 1, 0, 0]);
        let f0010 = get(&self.perm_table, corner + [0, 0, 1, 0]);
        let f1010 = get(&self.perm_table, corner + [1, 0, 1, 0]);
        let f0110 = get(&self.perm_table, corner + [0, 1, 1, 0]);
        let f1110 = get(&self.perm_table, corner + [1, 1, 1, 0]);
        let f0001 = get(&self.perm_table, corner + [0, 0, 0, 1]);
        let f1001 = get(&self.perm_table, corner + [1, 0, 0, 1]);
        let f0101 = get(&self.perm_table, corner + [0, 1, 0, 1]);
        let f1101 = get(&self.perm_table, corner + [1, 1, 0, 1]);
        let f0011 = get(&self.perm_table, corner + [0, 0, 1, 1]);
        let f1011 = get(&self.perm_table, corner + [1, 0, 1, 1]);
        let f0111 = get(&self.perm_table, corner + [0, 1, 1, 1]);
        let f1111 = get(&self.perm_table, corner + Vec4::one());

        let d000 = weight.x.lerp(f0000, f1000);
        let d100 = weight.x.lerp(f0100, f1100);
        let d010 = weight.x.lerp(f0010, f1010);
        let d110 = weight.x.lerp(f0110, f1110);
        let d001 = weight.x.lerp(f0001, f1001);
        let d101 = weight.x.lerp(f0101, f1101);
        let d011 = weight.x.lerp(f0011, f1011);
        let d111 = weight.x.lerp(f0111, f1111);
        let d00 = weight.y.lerp(d000, d100);
        let d10 = weight.y.lerp(d010, d110);
        let d01 = weight.y.lerp(d001, d101);
        let d11 = weight.y.lerp(d011, d111);
        let d0 = weight.z.lerp(d00, d10);
        let d1 = weight.z.lerp(d01, d11);
        let d = weight.w.lerp(d0, d1);

        d * 2.0 - 1.0
    }
}
