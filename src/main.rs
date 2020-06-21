#![allow(dead_code)]

mod raytracer;

use self::raytracer::frame::Frame;
use self::raytracer::colorf::ColorF;
use self::raytracer::light::Light;
use self::raytracer::material::Material;
use self::raytracer::sphere::Sphere;
use self::raytracer::chessboard::Chessboard;
use self::raytracer::raytraceable::*;

use self::raytracer::writer::save_as_ppm;

const FOV: f32 = std::f32::consts::FRAC_PI_4;
const MAX_REFLECTION: i32 = 4;

fn main() {
    let mut framebuffer = Frame::new(1024, 768);

    let ivory = Material { 
        refract: 1.0,
        albedo: Vec4f{ x:0.6, y: 0.3, z: 0.1, w: 0.0 }, 
        diffuse_color: ColorF::rgb(0.4, 0.4, 0.3),
        specularity: 50.0
    };
    let glass = Material { 
        refract: 1.5,
        albedo: Vec4f{ x:0.0, y: 0.5, z: 0.1, w: 0.8 }, 
        diffuse_color: ColorF::rgb(0.6, 0.7, 0.8),
        specularity: 125.0
    };
    let red_rubber = Material {
        refract: 1.0,
        albedo: Vec4f{ x:0.9, y: 0.1, z:0.0, w: 0.0 }, 
        diffuse_color: ColorF::rgb(0.3, 0.1, 0.1),
        specularity: 10.0
    };
    let mirror = Material {
        refract: 1.0,
        albedo: Vec4f{ x:0.0, y: 10.0, z:0.8, w: 0.0 }, 
        diffuse_color: ColorF::WHITE,
        specularity: 1425.0
    };

    let s1 = Sphere { center: Vec3f::new(-3.0,  0.0, -16.0), radius: 2.0, material:      &ivory };
    let s2 = Sphere { center: Vec3f::new(-1.0, -1.5, -12.0), radius: 2.0, material:      &glass };
    let s3 = Sphere { center: Vec3f::new( 1.5, -0.5, -18.0), radius: 3.0, material: &red_rubber };
    let s4 = Sphere { center: Vec3f::new( 7.0,  5.0, -18.0), radius: 4.0, material:     &mirror };
    
    let even_chess = Material {
        refract: 1.0,
        albedo: Vec4f{ x:0.9, y: 0.1, z:0.0, w: 0.0 }, 
        diffuse_color: ColorF::rgb(0.3, 0.3, 0.3),
        specularity: 10.0
    };
    let odd_chess = Material {
        refract: 1.0,
        albedo: Vec4f{ x:0.9, y: 0.1, z:0.0, w: 0.0 }, 
        diffuse_color: ColorF::rgb(0.3, 0.2, 0.1),
        specularity: 10.0
    };
    let brd = Chessboard { even_material: &even_chess, odd_material: &odd_chess };

    let mut spheres = Vec::<&dyn RayTraceable>::new();
    spheres.push(&s1);
    spheres.push(&s2);
    spheres.push(&s3);
    spheres.push(&s4);
    spheres.push(&brd);

    let mut lights = Vec::new();
    lights.push(Light { position: Vec3f::new(-20.0, 20.0,  20.0), intensity: 1.5 });
    lights.push(Light { position: Vec3f::new( 30.0, 50.0, -25.0), intensity: 1.8 });
    lights.push(Light { position: Vec3f::new( 30.0, 20.0,  30.0), intensity: 1.7 });

    render_scene(&mut framebuffer, &spheres, &lights);

    save_as_ppm(&framebuffer, "result.ppm").unwrap();
}

fn cast_ray(orig: &Vec3f, dir: &Vec3f, objects: &Vec<&dyn RayTraceable>, lights: &Vec<Light>, depth: i32) -> ColorF {
    let maybe_hit = scene_intersect(orig, dir, objects);
    
    if maybe_hit.is_none() || depth > MAX_REFLECTION {
        ColorF::rgb(0.2, 0.7, 0.8)  //background
    } else {
        let hit = maybe_hit.unwrap();
        // offset the original point to avoid occlusion by the object itself
        let hitpoint_in = hit.hitpoint - hit.normal * 1e-3;
        let hitpoint_out= hit.hitpoint + hit.normal * 1e-3;

        let reflect_dir = reflect(*dir, hit.normal).normalize();
        let reflect_orig = if reflect_dir.dot(hit.normal) < 0.0 { hitpoint_in } else { hitpoint_out } ;
        let reflect_color = cast_ray(&reflect_orig, &reflect_dir, objects, lights, depth + 1);

        let refract_dir = refract(*dir, hit.normal, hit.material.refract).normalize();
        let refract_orig = if refract_dir.dot(hit.normal) < 0.0 { hitpoint_in } else {hitpoint_out};
        let refract_color = cast_ray(&refract_orig, &refract_dir, objects, lights, depth + 1);

        let mut diffuse_intensity = 0.0;
        let mut specular_intensity = 0.0;
        for light in lights {
            let light_dir = (light.position - hit.hitpoint).normalize();
            let light_distance = (light.position - hit.hitpoint).magnitude();
            let shadow_orig = if light_dir.dot(hit.normal) < 0.0 
                        { hit.hitpoint - hit.normal * 1e-3 } 
                else { hit.hitpoint + hit.normal * 1e-3 };

            if let Some(shadow_hit) = scene_intersect(&shadow_orig, &light_dir, objects) {
                if (shadow_hit.hitpoint - shadow_orig).magnitude() < light_distance {
                    continue;
                }
            }

            diffuse_intensity += light.intensity * light_dir.dot(hit.normal).max(0.0);
            specular_intensity += reflect(light_dir, hit.normal).dot(*dir).max(0.0).powf(hit.material.specularity) * light.intensity;
        }
        let diffuse = hit.material.diffuse_color * diffuse_intensity * hit.material.albedo.x;
        let specular = ColorF::WHITE * specular_intensity * hit.material.albedo.y;
        let mirror = reflect_color * hit.material.albedo.z;
        let refract = refract_color * hit.material.albedo.w;
        diffuse + specular + mirror + refract
    }
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

fn render_scene(framebuffer: &mut Frame, objects: &Vec<&dyn RayTraceable>, lights: &Vec<Light>) {
    let height = framebuffer.height(); 
    let width = framebuffer.width();
    let fovtan = (FOV / 2.0).tan();
    for j in 0..height {
        for i in 0..width {
            let x =  (2.0 * (i as f32 + 0.5) / (width as f32)  - 1.0) * fovtan * (width as f32) / (height as f32);
            let y = -(2.0 * (j as f32 + 0.5) / (height as f32) - 1.0) * fovtan;
            let dir: Vec3f = Vec3f::new(x, y, -1.0).normalize();
            let color = cast_ray(&Vec3f::zero(), &dir, objects, lights, 0);
            framebuffer.set_pixel(i, j, &color);
        }
    }
}

fn scene_intersect<'a>(orig: &Vec3f, dir: &Vec3f, spheres: &'a Vec<&dyn RayTraceable>) -> Option<HitInfo<'a>> {
    let mut spheres_dist = std::f32::MAX;
    let mut info: Option<HitInfo<'_>> = None;

    for sphere in spheres {
        if let Some(hit) = sphere.ray_intersect(orig, dir) {
            if hit.distance < spheres_dist {
                spheres_dist = hit.distance;
                info = Some(hit);
            }
        }
    }
    info
}
