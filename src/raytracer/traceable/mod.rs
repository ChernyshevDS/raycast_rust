use std::rc::Weak;

pub use super::vector::*;
pub use super::material::Material;

mod sphere;
pub use sphere::Sphere;

mod chessboard;
pub use chessboard::Chessboard;

pub struct HitInfo {
    pub distance: f32,
    pub hitpoint: Vec3f,
    pub normal: Vec3f,
    pub material: Weak<Material>
}

pub trait RayTraceable {
    fn ray_intersect(&self, origin: &Vec3f, dir: &Vec3f) -> Option<HitInfo>;
}