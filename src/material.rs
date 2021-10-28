use crate::geometry::{Vec2f, Vec3f};

pub struct Material {
    base_color: Vec3f,
    albedo: Vec2f,
    specular_exponent: f32
}


impl Material {
    pub fn new(base_color: &Vec3f, albedo: &Vec2f, specular_exponent: f32) -> Self {
        Material { base_color: base_color.clone(), albedo: albedo.clone(), specular_exponent }
    }

    pub fn color(&self) -> &Vec3f {
        &self.base_color
    }

    pub fn albedo(&self) -> &Vec2f {
        &self.albedo
    }

    pub fn specular_exponent(&self) -> f32 {
        self.specular_exponent
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Material::new(&self.base_color.clone(), &self.albedo.clone(), self.specular_exponent)
    }

    fn clone_from(&mut self, source: &Self) {
        self.base_color = source.base_color.clone();
        self.albedo = source.albedo.clone();
        self.specular_exponent = source.specular_exponent;
    }
}

