// Copyright 2015 The noise-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::rand::{Rand, Rng, SeedableRng, XorShiftRng};
use std::num::SignedInt;

use math;

const TABLE_SIZE: usize = 256;

#[allow(missing_copy_implementations)]
pub struct Seed {
    values: [u8; TABLE_SIZE],
}

impl Rand for Seed {
    fn rand<R: Rng>(rng: &mut R) -> Seed {
        let mut seq: Vec<u8> = ::std::iter::range_inclusive(0, (TABLE_SIZE - 1) as u8).collect();
        rng.shuffle(&mut *seq);

        // It's unfortunate that this double-initializes the array, but Rust doesn't currently provide a
        // clean way to do this in one pass. Hopefully won't matter, as Seed creation will usually be a
        // one-time event.
        let mut seed = Seed { values: [0; TABLE_SIZE] };
        let seq_it = seq.iter();
        for (x, y) in seed.values.iter_mut().zip(seq_it) { *x = *y }
        seed
    }
}

impl Seed {
    pub fn new(seed: u32) -> Seed {
        let mut rng: XorShiftRng = SeedableRng::from_seed([1, seed, seed, seed]);
        rng.gen()
    }

    #[inline(always)]
    pub fn get1<T: SignedInt>(&self, x: T) -> usize {
        let x: usize = math::cast(x & math::cast(0xff));
        self.values[x] as usize
    }

    #[inline(always)]
    pub fn get2<T: SignedInt>(&self, pos: math::Point2<T>) -> usize {
        let y: usize = math::cast(pos[1] & math::cast(0xff));
        self.values[self.get1(pos[0]) ^ y] as usize
    }

    #[inline(always)]
    pub fn get3<T: SignedInt>(&self, pos: math::Point3<T>) -> usize {
        let z: usize = math::cast(pos[2] & math::cast(0xff));
        self.values[self.get2([pos[0], pos[1]]) ^ z] as usize
    }

    #[inline(always)]
    pub fn get4<T: SignedInt>(&self, pos: math::Point4<T>) -> usize {
        let w: usize = math::cast(pos[3] & math::cast(0xff));
        self.values[self.get3([pos[0], pos[1], pos[2]]) ^ w] as usize
    }
}

#[cfg(test)]
mod tests {
    use std::rand::random;
    use perlin::perlin3;
    use super::Seed;

    #[test]
    fn test_random_seed() {
        let _ = perlin3::<f32>(&random(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_negative_params() {
        let _ = perlin3::<f32>(&Seed::new(0), &[-1.0, 2.0, 3.0]);
    }
}
