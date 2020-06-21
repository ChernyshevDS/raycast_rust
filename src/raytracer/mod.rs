pub mod colorf;
pub mod frame;
pub mod light;
pub mod material;
pub mod traceable;
pub mod vector;
pub mod writer;
pub mod scene;

use vector::*;
use traceable::*;
use colorf::ColorF;
use light::Light;
use frame::Frame;
use scene::Scene;

use rayon::prelude::*;

const FOV: f32 = std::f32::consts::FRAC_PI_4;
const MAX_REFLECTION: i32 = 4;

pub fn render_scene(framebuffer: &mut Frame, scene: &Scene) {
    let height = framebuffer.height(); 
    let width = framebuffer.width();
    let fovtan = (FOV / 2.0).tan();

    framebuffer.data_mut()
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, out)| {
            let yi = i / width;
            let xi = i - (yi * width);
           
            //*out = rgss_sample(xi, yi, fovtan, width as f32, height as f32, scene);
            *out = subpixel_sample(xi, yi, 0.0, 0.0, fovtan, width as f32, height as f32, scene);
        });
}

fn rgss_sample(x: usize, y: usize, fovtan: f32, width: f32, height: f32, scene: &Scene) -> ColorF {
    let ds = 3.0 / 8.0;
    let db = 1.0 / 8.0;
    let c1 = subpixel_sample(x, y, db, ds, fovtan, width, height, scene);
    let c2 = subpixel_sample(x, y, ds, -db, fovtan, width, height, scene);
    let c3 = subpixel_sample(x, y, -db, -ds, fovtan, width, height, scene);
    let c4 = subpixel_sample(x, y, -ds, db, fovtan, width, height, scene);
    (c1 + c2 + c3 + c4) / 4.0
}

//px, py = [-1..1]
fn subpixel_sample(x: usize, y: usize, px: f32, py: f32, fovtan: f32, width: f32, height: f32, scene: &Scene) -> ColorF {
    let xi = x as f32 + (px / 2.0);
    let yi = y as f32 + (py / 2.0);
    let x =  (2.0 * (xi + 0.5) / width  - 1.0) * fovtan * width / height;
    let y = -(2.0 * (yi + 0.5) / height - 1.0) * fovtan;
    let dir: Vec3f = Vec3f::new(x, y, -1.0).normalize();
    let color = cast_ray(&Vec3f::zero(), &dir, scene, 0);
    color
}

fn cast_ray(orig: &Vec3f, dir: &Vec3f, scene: &Scene, depth: i32) -> ColorF {
    let maybe_hit = scene_intersect(orig, dir, &scene.objects);
    
    if maybe_hit.is_none() || depth > MAX_REFLECTION {
        ColorF::rgb(0.2, 0.7, 0.8)  //background
    } else {
        let hit = maybe_hit.unwrap();
        // offset the original point to avoid occlusion by the object itself
        let hitpoint_in = hit.hitpoint - hit.normal * 1e-3;
        let hitpoint_out= hit.hitpoint + hit.normal * 1e-3;

        let reflect_dir = reflect(*dir, hit.normal).normalize();
        let reflect_orig = if reflect_dir.dot(hit.normal) < 0.0 { hitpoint_in } else { hitpoint_out } ;
        let reflect_color = cast_ray(&reflect_orig, &reflect_dir, scene, depth + 1);

        let refract_dir = refract(*dir, hit.normal, hit.material.upgrade().unwrap().refract).normalize();
        let refract_orig = if refract_dir.dot(hit.normal) < 0.0 { hitpoint_in } else {hitpoint_out};
        let refract_color = cast_ray(&refract_orig, &refract_dir, scene, depth + 1);

        let mut diffuse_intensity = 0.0;
        let mut specular_intensity = 0.0;
        let material = hit.material.upgrade().unwrap();

        for light in scene.lights.iter() {
            let light_dir = (light.position - hit.hitpoint).normalize();
            let light_distance = (light.position - hit.hitpoint).magnitude();
            let shadow_orig = if light_dir.dot(hit.normal) < 0.0 
                        { hit.hitpoint - hit.normal * 1e-3 } 
                else { hit.hitpoint + hit.normal * 1e-3 };

            if let Some(shadow_hit) = scene_intersect(&shadow_orig, &light_dir, &scene.objects) {
                if (shadow_hit.hitpoint - shadow_orig).magnitude() < light_distance {
                    continue;
                }
            }

            diffuse_intensity += light.intensity * light_dir.dot(hit.normal).max(0.0);
            specular_intensity += reflect(light_dir, hit.normal).dot(*dir).max(0.0).powf(material.specularity) * light.intensity;
        }
        let diffuse = material.diffuse_color * diffuse_intensity * material.albedo.x;
        let specular = ColorF::WHITE * specular_intensity * material.albedo.y;
        let mirror = reflect_color * material.albedo.z;
        let refract = refract_color * material.albedo.w;
        diffuse + specular + mirror + refract
    }
}

fn scene_intersect<'a>(orig: &Vec3f, dir: &Vec3f, objects: &'a Vec<&dyn RayTraceable>) -> Option<HitInfo> {
    let mut spheres_dist = std::f32::MAX;
    let mut info: Option<HitInfo> = None;

    for scene_object in objects {
        if let Some(hit) = scene_object.ray_intersect(orig, dir) {
            if hit.distance < spheres_dist {
                spheres_dist = hit.distance;
                info = Some(hit);
            }
        }
    }
    info
}

fn refract(vec: Vec3f, normal: Vec3f, index: f32) -> Vec3f {
    let mut cosi = vec.dot(normal).min(1.0).max(-1.0) * -1.0;
    let mut etai = 1.0;
    let mut etat = index;
    let mut n = normal;
    // if the ray is inside the object, swap the indices and invert the normal to get the correct result
    if cosi < 0.0 { 
        cosi = -cosi;
        std::mem::swap(&mut etai, &mut etat); 
        n = -normal;
    }
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    return if k < 0.0 { Vec3f::zero() } else { vec * eta + n * (eta * cosi - k.sqrt()) };
}

fn reflect(vec: Vec3f, relative_to: Vec3f) -> Vec3f {
    return vec - relative_to * 2.0 * (vec.dot(relative_to));
}
