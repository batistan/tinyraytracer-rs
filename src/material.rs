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

impl Clone for Material {
    fn clone(&self) -> Self {
        Material::new(&self.base_color.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.base_color = source.base_color.clone();
    }
}

