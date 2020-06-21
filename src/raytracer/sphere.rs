pub use super::raytraceable::*;

pub struct Sphere<'a> {
    pub center: Vec3f,
    pub radius: f32,
    pub material: &'a Material
}

impl RayTraceable for Sphere<'_> {
    fn ray_intersect(&self, origin: &Vec3f, dir: &Vec3f) -> Option<HitInfo> {
        let fwd: Vec3f = self.center - origin;
        let tca: f32 = fwd.dot(*dir);
        let d2: f32 = fwd.dot(fwd) - tca*tca;
        if d2 > self.radius * self.radius 
        {
            return None;
        }
        let thc: f32 = (self.radius*self.radius - d2).sqrt();
        let mut intersection = tca - thc;
        let t1 = tca + thc;
        if intersection < 0.0 {
            intersection = t1;
        } 
        if intersection < 0.0 {
            return None;
        }

        let hitpoint = origin + dir * intersection;
        let normal = (hitpoint - self.center).normalize();
        Some(HitInfo { distance: intersection, hitpoint, normal, material: self.material })
    }
}