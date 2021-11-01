use crate::geometry::Vec3f;

pub struct Material {
    base_color: Vec3f,
    // this is a misnomer
    // each value of this vector is the proportion of light reflected off the material differently
    // albedo[0] is the percentage of incident light which is reflected diffusely
    // albedo[1] is the percentage of incident light which is reflected specular-ly
    // albedo[3] is the percentage of incident light which is reflected the normal reflect way
    albedo: Vec3f,
    specular_exponent: f32,
}

impl Material {
    pub fn new(base_color: &Vec3f, albedo: &Vec3f, specular_exponent: f32) -> Self {
        Material { base_color: base_color.clone(), albedo: albedo.clone(), specular_exponent }
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
