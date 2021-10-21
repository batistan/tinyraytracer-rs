/*
 * Credit to https://github.com/NeuroWhAI/tinyraytracer-rs/blob/master/src/vector.rs
 */

use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug)]
pub struct vec<T> {
    _data: Vec<T>,
}

pub type Vec2f = vec<f32>;
pub type Vec3f = vec<f32>;
pub type Vec3i = vec<i32>;
pub type Vec4f = vec<f32>;

impl<T> vec<T>
    where T: Clone + Default {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, Default::default());

        vec { _data: data }
    }

    pub fn from_slice(values: &[T]) -> Self {
        let data = Vec::from(values);

        vec { _data: data }
    }
}

impl<T> vec<T>
    where T: Copy + Default + Add<Output=T> + Mul<Output=T> {
    pub fn dot(&self, rhs: &vec<T>) -> T {
        self._data.iter().zip(rhs._data.iter())
            .map(|(&l, &r)| l * r)
            .fold(Default::default(), |acc, v| acc + v)
    }
}

impl vec<f32> {
    pub fn norm(&self) -> f32 {
        self._data.iter()
            .map(|v| v * v)
            .fold(0.0_f32, |acc, v| acc + v)
            .sqrt()
    }

    pub fn normalize(&self) -> vec<f32> {
        let norm = self.norm();
        let data = self._data.iter()
            .map(|v| v / norm)
            .collect();

        vec { _data: data }
    }
}

impl<T> Clone for vec<T>
    where T: Clone {
    fn clone(&self) -> Self {
        let data = self._data.clone();

        vec { _data: data }
    }
}

impl<T> Index<i32> for vec<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        return &self._data[index as usize];
    }
}

impl<T> IndexMut<i32> for vec<T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        return self._data.index_mut(index as usize);
    }
}

impl<T> Sub for &vec<T>
    where T: Copy + Sub<Output=T> {
    type Output = vec<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let data = self._data.iter().zip(rhs._data.iter())
            .map(|(&l, &r)| l - r)
            .collect();

        vec { _data: data }
    }
}

impl<T> Add for &vec<T>
    where T: Copy + Add<Output=T> {
    type Output = vec<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let data = self._data.iter().zip(rhs._data.iter())
            .map(|(&l, &r)| l + r)
            .collect();

        vec { _data: data }
    }
}

impl<T> Mul<T> for &vec<T>
    where T: Copy + Mul<Output=T> {
    type Output = vec<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let data = self._data.iter()
            .map(|&v| v * rhs)
            .collect();

        vec { _data: data }
    }
}

pub fn reflect(i: &Vec2f, n: &Vec2f) -> Vec2f {
    i - &(&(n * 2.0) * (i.dot(n)))
}

pub trait Object {
    // returns tuple (b, dist)
    // b is true if ray from orig in direction of vector dir intersects with this object
    // dist is the distance along the ray that the first intersection occurs
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> (bool, f32);
}

pub struct Sphere {
    center: Vec3f,
    radius: f32,
}

impl Sphere {
    pub fn new(c: Vec3f, r: f32) -> Self {
        Sphere { center: c, radius: r }
    }
}

impl Object for Sphere {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> (bool, f32) {
        let l = &self.center - orig;
        let tca = &l.dot(dir);
        let d2 = l.dot(&l) - (tca * tca);
        if d2 > (self.radius * self.radius) { return (false, 0f32); }

        let thc = (self.radius * self.radius - d2).sqrt();

        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0f32 { t0 = t1; }

        return (!(t0 < 0.0), t0);
    }
}

// sloppy but it works
pub fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

pub fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
