use crate::geometry::Vec3f;

pub struct Material {
    base_color: Vec3f,
}

impl Material {
    pub fn new(base_color: &Vec3f) -> Self {
        Material { base_color: base_color.clone() }
    }

    pub fn color(&self) -> &Vec3f {
        return &self.base_color;
    }
}

pub(crate) static IVORY: Material = Material::new(&Vec3f::from_slice(&[0.4f32, 0.4f32, 0.3f32]));
pub(crate) static RUBBER: Material = Material::new(&Vec3f::from_slice(&[0.3f32, 0.1f32, 0.1f32]));
