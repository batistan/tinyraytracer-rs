use crate::geometry::{Vec3f, Vec4f};

#[derive(Debug)]
pub struct Material {
    base_color: Vec3f,
    // this is a misnomer
    // each value of this vector is the proportion of light reflected off the material differently
    // albedo[0] is the percentage of incident light which is reflected diffusely
    // albedo[1] is the percentage of incident light which is reflected specular-ly
    // albedo[2] is the percentage of incident light which is reflected the normal reflect way
    // albedo[3] is the percentage of incident light which is refracted
    albedo: Vec4f,
    specular_exponent: f32,
    refractive_index: f32,
}

impl Material {
    pub fn new(base_color: &Vec3f, albedo: &Vec3f, specular_exponent: f32, refractive_index: f32) -> Self {
        Material { base_color: base_color.clone(), albedo: albedo.clone(), specular_exponent, refractive_index }
    }

    pub fn color(&self) -> &Vec3f {
        &self.base_color
    }

    pub fn albedo(&self) -> &Vec3f {
        &self.albedo
    }

    pub fn specular_exponent(&self) -> f32 {
        self.specular_exponent
    }

    pub fn refractive_index(&self) -> f32 {
        self.refractive_index
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Material::new(&self.base_color.clone(), &self.albedo.clone(), self.specular_exponent, self.refractive_index)
    }

    fn clone_from(&mut self, source: &Self) {
        self.base_color = source.base_color.clone();
        self.albedo = source.albedo.clone();
        self.specular_exponent = source.specular_exponent;
        self.refractive_index = source.refractive_index;
    }
}
