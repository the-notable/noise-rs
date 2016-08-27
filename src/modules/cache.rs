// Copyright 2016 The Noise-rs Developers.
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

use num_traits::Float;
use math::{Point2, Point3, Point4};
use NoiseModule;
use std::cell::{Cell, RefCell};

/// Noise module that caches the last output value generated by the source
/// module.
///
/// If the input coordinates pased to `Cache::get` are equal to the previous
/// call, the module returns the cached result of the previous call to
/// `Source::get`. Otherwise, `Source::get` is called with the new coordinates,
/// overwriting the cache with the result, and returning the result to the
/// caller.
///
/// Caching a noise module is useful if it is used as a source module for
/// multiple noise modules. If a source module is not cached, the source
/// module will redundantly calculate the same output value once for each
/// noise module in which it is included.
#[derive(Clone, Debug)]
pub struct Cache<Source, T>
    where T: Float,
{
    /// Outputs the value to be cached.
    pub source: Source,

    value: Cell<Option<T>>,

    point: RefCell<Vec<T>>,
}

impl<Source, T> Cache<Source, T>
    where T: Float,
{
    pub fn new(source: Source) -> Cache<Source, T> {
        Cache {
            source: source,
            value: Cell::new(None),
            point: RefCell::new(Vec::new()),
        }
    }
}

impl<Source, T> NoiseModule<Point2<T>> for Cache<Source, T>
    where Source: NoiseModule<Point2<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point2<T>) -> Self::Output {
        match self.value.get() {
            Some(value) if *self.point.borrow() == point => value,
            Some(_) | None => {
                let value = self.source.get(point);
                self.value.set(Some(value));

                *self.point.borrow_mut() = point.to_vec();

                value
            }
        }
    }
}

impl<Source, T> NoiseModule<Point3<T>> for Cache<Source, T>
    where Source: NoiseModule<Point3<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point3<T>) -> Self::Output {
        match self.value.get() {
            Some(value) if *self.point.borrow() == point => value,
            Some(_) | None => {
                let value = self.source.get(point);
                self.value.set(Some(value));

                *self.point.borrow_mut() = point.to_vec();

                value
            }
        }
    }
}

impl<Source, T> NoiseModule<Point4<T>> for Cache<Source, T>
    where Source: NoiseModule<Point4<T>, Output = T>,
          T: Float,
{
    type Output = T;

    fn get(&self, point: Point4<T>) -> Self::Output {
        match self.value.get() {
            Some(value) if *self.point.borrow() == point => value,
            Some(_) | None => {
                let value = self.source.get(point);
                self.value.set(Some(value));

                *self.point.borrow_mut() = point.to_vec();

                value
            }
        }
    }
}
