use std::rc::Weak;
pub use super::*;

pub struct Chessboard {
    pub even_material: Weak<Material>,
    pub odd_material: Weak<Material>
}

impl RayTraceable for Chessboard {
    fn ray_intersect(&self, origin: &Vec3f, dir: &Vec3f) -> Option<HitInfo> {
        if dir.y.abs() > 1e-3  {
            let d = -(origin.y + 4.0) / dir.y; // the checkerboard plane has equation y = -4
            let hit = origin + dir * d;
            if d > 0.0 && hit.x.abs() < 10.0 && hit.z < -10.0 && hit.z > -30.0 {
                let material = if ((0.5 * hit.x + 1000.0) as i32 + (0.5 * hit.z) as i32) & 1 == 1 
                { self.odd_material.clone() } else { self.even_material.clone() };

                let normal = Vec3f::unit_y();
                return Some(HitInfo { distance: d, hitpoint: hit, normal, material });
            }
        } 
        None
    }
}