#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use cgmath::prelude::*;
use cgmath::Vector3;
use cgmath::Vector2;

type Vec3f = Vector3<f32>;
type Vec2f = Vector2<f32>;
type ColorF = frame::ColorF;

mod frame;

const FOV: f32 = std::f32::consts::FRAC_PI_4;

fn main() {
    let mut framebuffer = frame::Frame::new(1024, 768);

    let ivory = Material { 
        albedo: Vec2f{ x:0.6, y: 0.3 }, 
        diffuse_color: ColorF::rgb(0.4, 0.4, 0.3),
        specularity: 50.0
    };
    let red_rubber = Material {
        albedo: Vec2f{ x:0.9, y: 0.1 }, 
        diffuse_color: ColorF::rgb(0.3, 0.1, 0.1),
        specularity: 10.0
    };

    let mut spheres = Vec::new();
    spheres.push(Sphere { center: Vector3::new(-3.0,  0.0, -16.0), radius: 2.0, material:      &ivory });
    spheres.push(Sphere { center: Vector3::new(-1.0, -1.5, -12.0), radius: 2.0, material: &red_rubber });
    spheres.push(Sphere { center: Vector3::new( 1.5, -0.5, -18.0), radius: 3.0, material: &red_rubber });
    spheres.push(Sphere { center: Vector3::new( 7.0,  5.0, -18.0), radius: 4.0, material:      &ivory });

    let mut lights = Vec::new();
    lights.push(Light { position: Vector3::new(-20.0, 20.0,  20.0), intensity: 1.5 });
    lights.push(Light { position: Vector3::new( 30.0, 50.0, -25.0), intensity: 1.8 });
    lights.push(Light { position: Vector3::new( 30.0, 20.0,  30.0), intensity: 1.7 });

    render_scene(&mut framebuffer, &spheres, &lights);

    save_as_ppm(&framebuffer).unwrap();
}

/*float light_distance = (lights[i].position - point).norm();
Vec3f shadow_orig = light_dir*N < 0 ? point - N*1e-3 : point + N*1e-3; // checking if the point lies in the shadow of the lights[i]
Vec3f shadow_pt, shadow_N;
Material tmpmaterial;
if (scene_intersect(shadow_orig, light_dir, spheres, shadow_pt, shadow_N, tmpmaterial) && (shadow_pt-shadow_orig).norm() < light_distance)
    continue;*/

fn cast_ray(orig: &Vec3f, dir: &Vec3f, objects: &Vec<Sphere>, lights: &Vec<Light>) -> frame::ColorF {
    match scene_intersect(orig, dir, objects) {
        Some(hit) => {
            let mut diffuse_intensity = 0.0;
            let mut specular_intensity = 0.0;
            for light in lights {
                let light_dir = (light.position - hit.hitpoint).normalize();
                let light_distance = (light.position - hit.hitpoint).magnitude();
                let shadow_orig = if light_dir.dot(hit.normal) < 0.0 
                         { hit.hitpoint - hit.normal * 1e-3 } 
                    else { hit.hitpoint + hit.normal * 1e-3 };

                match scene_intersect(&shadow_orig, &light_dir, objects) {
                    Some(shadow_hit) => {
                        if (shadow_hit.hitpoint - shadow_orig).magnitude() < light_distance {
                            continue;
                        }
                    },
                    None => {}
                }

                diffuse_intensity += light.intensity * light_dir.dot(hit.normal).max(0.0);
                specular_intensity += reflect(light_dir, hit.normal).dot(*dir).max(0.0).powf(hit.material.specularity) * light.intensity;
            }
            let diffuse = hit.material.diffuse_color * diffuse_intensity * hit.material.albedo.x;
            let specular = ColorF::WHITE * specular_intensity * hit.material.albedo.y;
            diffuse + specular
        },
        None => frame::ColorF::rgb(0.2, 0.7, 0.8) 
    }
}

fn reflect(vec: Vec3f, relative_to: Vec3f) -> Vec3f {
    return vec - relative_to * 2.0 * (vec.dot(relative_to));
}

fn render_scene(framebuffer: &mut frame::Frame, objects: &Vec<Sphere>, lights: &Vec<Light>) {
    let height = framebuffer.height(); 
    let width = framebuffer.width();
    let fovtan = (FOV / 2.0).tan();
    for j in 0..height {
        for i in 0..width {
            let x =  (2.0 * (i as f32 + 0.5) / (width as f32)  - 1.0) * fovtan * (width as f32) / (height as f32);
            let y = -(2.0 * (j as f32 + 0.5) / (height as f32) - 1.0) * fovtan;
            let dir: Vec3f = Vector3::new(x, y, -1.0).normalize();
            let color = cast_ray(&Vector3::zero(), &dir, objects, lights);
            framebuffer.set_pixel(i, j, &color);
        }
    }
}

fn clamp_to_byte(v: f32) -> u8 {
    if v > 1.0 { return 255; }
    else if v < 0.0 { return 0; }
    else { return (v * 255.0).round() as u8 }
}

#[test]
fn test_clamp_to_byte() {
    assert_eq!(clamp_to_byte(0.0), 0);
    assert_eq!(clamp_to_byte(1.0), 255);
    
    assert_eq!(clamp_to_byte(0.5), 128);
    assert_eq!(clamp_to_byte(0.7), 179);

    assert_eq!(clamp_to_byte(-15.0), 0);
    assert_eq!(clamp_to_byte(12345.0), 255);

    assert_eq!(clamp_to_byte(f32::NAN), 0);
    assert_eq!(clamp_to_byte(f32::INFINITY), 255);
    assert_eq!(clamp_to_byte(f32::NEG_INFINITY), 0);
}

fn save_as_ppm(frame: &frame::Frame) -> std::io::Result<()>{
    let mut f = File::create("result.ppm")?;

    let header = format!("P3\n{} {}\n255\n", frame.width(), frame.height());
    f.write_all(header.as_bytes())?;
    
    let mut content = String::with_capacity(frame.width() * frame.height() * 10);
    for x in frame.data() {
        let r = clamp_to_byte(x.r);
        let g = clamp_to_byte(x.g);
        let b = clamp_to_byte(x.b);
        let s = format!("{} {} {}\n", r, g, b);
        content.push_str(&s);
    }
    f.write_all(content.as_bytes())?;
    f.sync_all()?;

    Ok(())
}

pub trait RayTraceable {
    fn ray_intersect(&self, origin: &Vec3f, dir: &Vec3f) -> Option<f32>;
}

pub struct Sphere<'a> {
    pub center: Vec3f,
    pub radius: f32,
    pub material: &'a Material
}

impl RayTraceable for Sphere<'_> {
    fn ray_intersect(&self, origin: &Vec3f, dir: &Vec3f) -> Option<f32> {
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
        return Some(intersection);
    }
}

pub struct Material {
    pub albedo: Vec2f,
    pub diffuse_color: ColorF,
    pub specularity: f32
}

pub struct HitInfo<'a> {
    pub hitpoint: Vec3f,
    pub normal: Vec3f,
    pub material: &'a Material
}

fn scene_intersect<'a>(orig: &Vec3f, dir: &Vec3f, spheres: &'a Vec<Sphere>) -> Option<HitInfo<'a>> {
    let mut spheres_dist = std::f32::MAX;
    let mut info: Option<HitInfo<'_>> = None;

    for sphere in spheres {
        if let Some(inter) = sphere.ray_intersect(orig, dir) {
            if inter < spheres_dist {
                spheres_dist = inter;
                let hitpoint = orig + dir * inter;
                let normal = (hitpoint - sphere.center).normalize();
                info = Some(HitInfo { hitpoint, normal, material: sphere.material });
            }
        }
    }
    info
}

pub struct Light {
    position: Vec3f,
    intensity: f32
}