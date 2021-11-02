use crate::geometry::Vec3f;
use crate::material::Material;

pub trait Object {
    // returns tuple (b, dist)
    // b is true if ray from orig in direction of vector dir intersects with this object
    // dist is the distance along the ray that the first intersection occurs
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> (bool, f32);

    fn get_position(&self) -> &Vec3f;

    fn get_material(&self) -> &Material;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vec3f,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(c: Vec3f, r: f32, material: &Material) -> Self {
        Sphere { center: c, radius: r, material: material.clone() }
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

        if t0 < 0.0 { t0 = t1; }

        (t0 >= 0.0, t0)
    }

    fn get_position(&self) -> &Vec3f {
        &self.center
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}
