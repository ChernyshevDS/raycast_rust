pub use super::vector::*;
pub use super::material::Material;

pub struct HitInfo<'a> {
    pub distance: f32,
    pub hitpoint: Vec3f,
    pub normal: Vec3f,
    pub material: &'a Material
}

pub trait RayTraceable {
    fn ray_intersect(&self, origin: &Vec3f, dir: &Vec3f) -> Option<HitInfo>;
}