use std::rc::Rc;
use std::rc::Weak;

use super::Light;
use super::Material;
use super::traceable::RayTraceable;

use std::collections::HashMap;

pub struct Scene<'a> {
    pub objects: Vec<&'a dyn RayTraceable>,
    pub materials: HashMap<String, Rc<Material>>,
    pub lights: Vec<Light>
}

impl Scene<'_> {
    pub fn new() -> Self {
        Self { 
            objects: Vec::<&dyn RayTraceable>::new(),
            materials: HashMap::new(),
            lights: Vec::new()
        }
    }

    pub fn find_material(&self, name: &str) -> Weak<Material> {
        Rc::downgrade(self.materials.get(name).unwrap())
    }
}