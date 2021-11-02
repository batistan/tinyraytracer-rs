/*
 * Credit to https://github.com/NeuroWhAI/tinyraytracer-rs/blob/master/src/vector.rs
 */

use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, PartialEq)]
pub struct VecRT<T> {
    _data: Vec<T>,
}

pub type Vec2f = VecRT<f32>;
pub type Vec3f = VecRT<f32>;
pub type Vec4f = VecRT<f32>;

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

    pub fn zero() -> Self {
        VecRT { _data: vec![0.0, 0.0, 0.0] }
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

impl<T> Neg for VecRT<T>
    where T: Copy + Neg<Output=T> {
    type Output = VecRT<T>;

    fn neg(self) -> Self::Output {
        let data = self._data.iter().map(| &val| { -val }).collect();

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

/// return direction of refraction given incoming ray and surface normal, as well as refractive index of material being entered
// this is literally https://en.wikipedia.org/wiki/Snell%27s_law#Vector_form implemented as code
pub fn refract(incident_ray: &Vec3f, normal: &Vec3f, refractive_index: f32) -> Vec3f {
    // cos of angle between incident and normal
    // bounded to -1, 1 since both rays are normalized
    let cos_in = -(normal.dot(incident_ray));

    // if cos_in is negative, ray is coming out from inside the object
    // swap the indices and invert the normal to get correct result
    let n_1 = if cos_in < 0.0 { 1f32 } else { refractive_index };
    let n_2 = if cos_in < 0.0 { refractive_index } else { 1f32 };
    let n = if cos_in < 0.0 { -normal.clone() } else { normal.clone() };
    let cos_corrected = cos_in.abs();

    let ref_index_ratio = n_2 / n_1;
    // sqrt of this is the cos of angle of refraction
    // remember that (1 - cos^2) == sin^2
    let k = 1.0 - ((ref_index_ratio * ref_index_ratio) * (1.0 - (cos_corrected * cos_corrected)));

    // if sin < 0, refraction angle is negative, so there is no refracted ray because total internal reflection
    return if k < 0.0 {
        Vec3f::zero()
    } else {
        let r_i = incident_ray * ref_index_ratio;
        let r_n = &n * ((ref_index_ratio * cos_corrected) - k.sqrt());

        (&r_i + &r_n).normalize()
    }
}

// sloppy but it works
pub fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

pub fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
