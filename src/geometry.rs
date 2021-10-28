/*
 * Credit to https://github.com/NeuroWhAI/tinyraytracer-rs/blob/master/src/vector.rs
 */

use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug)]
pub struct VecRT<T> {
    _data: Vec<T>,
}

pub type Vec2f = VecRT<f32>;
pub type Vec3f = VecRT<f32>;

impl<T> VecRT<T>
    where T: Clone + Default {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, Default::default());

        VecRT { _data: data }
    }

    pub fn from(values: &[T]) -> Self {
        let data = Vec::from(values);

        VecRT { _data: data }
    }
}

impl<T> VecRT<T>
    where T: Copy + Default + Add<Output=T> + Mul<Output=T> {
    pub fn dot(&self, rhs: &VecRT<T>) -> T {
        self._data.iter().zip(rhs._data.iter())
            .map(|(&l, &r)| l * r)
            .fold(Default::default(), |acc, v| acc + v)
    }
}

impl VecRT<f32> {
    pub fn new3f(x: f32, y: f32, z: f32) -> Self {
        VecRT { _data: vec![x, y, z] }
    }

    pub fn magnitude(&self) -> f32 {
        self._data.iter()
            .map(|v| v * v)
            .fold(0.0_f32, |acc, v| acc + v)
            .sqrt()
    }

    pub fn normalize(&self) -> VecRT<f32> {
        let mag = self.magnitude();
        let data = self._data.iter()
            .map(|v| v / mag)
            .collect();

        VecRT { _data: data }
    }
}

impl<T> Clone for VecRT<T>
    where T: Clone {
    fn clone(&self) -> Self {
        let data = self._data.clone();

        VecRT { _data: data }
    }
}

impl<T> Index<i32> for VecRT<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        &self._data[index as usize]
    }
}

impl<T> IndexMut<i32> for VecRT<T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        self._data.index_mut(index as usize)
    }
}

impl<T> Sub for &VecRT<T>
    where T: Copy + Sub<Output=T> {
    type Output = VecRT<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let data = self._data.iter().zip(rhs._data.iter())
            .map(|(&l, &r)| l - r)
            .collect();

        VecRT { _data: data }
    }
}

impl<T> Add for &VecRT<T>
    where T: Copy + Add<Output=T> {
    type Output = VecRT<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let data = self._data.iter().zip(rhs._data.iter())
            .map(|(&l, &r)| l + r)
            .collect();

        VecRT { _data: data }
    }
}

impl<T> Mul<T> for &VecRT<T>
    where T: Copy + Mul<Output=T> {
    type Output = VecRT<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let data = self._data.iter()
            .map(|&v| v * rhs)
            .collect();

        VecRT { _data: data }
    }
}

// TODO what
pub fn reflect(i: &Vec2f, n: &Vec2f) -> Vec2f {
    i - &(&(n * 2.0) * (i.dot(n)))
}

// sloppy but it works
pub fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

pub fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
