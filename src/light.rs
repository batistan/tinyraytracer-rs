use crate::geometry::Vec3f;

pub struct Light {
    position: Vec3f,
    intensity: f32,
}

impl Light {
    pub fn new (p: &Vec3f, i: f32) -> Self {
        Light { position: p.clone(), intensity: i }
    }

    pub fn get_position(&self) -> &Vec3f {
        &self.position
    }

    pub fn get_intensity(&self) -> f32 {
        self.intensity
    }

}
