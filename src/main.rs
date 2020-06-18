#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use cgmath::prelude::*;
use cgmath::Vector3;

type Vec3f = Vector3<f32>;

mod frame;

const FOV: f32 = std::f32::consts::FRAC_PI_2;

fn main() {
    let mut framebuffer = frame::Frame::new(1024, 768);
    let sphere = Sphere { center: Vector3::new(-3.0, 0.0, -16.0), radius: 2.0 };

    render(&mut framebuffer, &sphere);

    save_as_ppm(&framebuffer).unwrap();
}

fn render_gradient(framebuffer: &mut frame::Frame){
    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            let color = frame::ColorF::rgb(y as f32 / framebuffer.height() as f32,
                                           x as f32 / framebuffer.width() as f32, 
                                           0.0);
            framebuffer.set_pixel(x, y, &color);
        }
    }
}

fn cast_ray(orig: &Vec3f, dir: &Vec3f, sphere: &Sphere) -> frame::ColorF {
    let mut sphere_dist = std::f32::MAX;
    if !sphere.ray_intersect(orig, dir, &mut sphere_dist) {
        return frame::ColorF::rgb(0.2, 0.7, 0.8);
    }
    return frame::ColorF::rgb(0.4, 0.4, 0.3);
}

fn render(framebuffer: &mut frame::Frame, sphere: &Sphere) {
    let height = framebuffer.height(); 
    let width = framebuffer.width();
    let fovtan = (FOV / 2.0).tan();
    for j in 0..height {
        for i in 0..width {
            let x =  (2.0 * (i as f32 + 0.5) / (width as f32)  - 1.0) * fovtan * (width as f32) / (height as f32);
            let y = -(2.0 * (j as f32 + 0.5) / (height as f32) - 1.0) * fovtan;
            let dir: Vec3f = Vector3::new(x, y, -1.0).normalize();
            let color = cast_ray(&Vector3::zero(), &dir, sphere);
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

pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32
}

impl Sphere {
    pub fn ray_intersect(&self, origin: &Vec3f, dir: &Vec3f, step: &mut f32) -> bool {
        let L: Vec3f = self.center - origin;
        let tca: f32 = L.dot(*dir);
        let d2: f32 = L.dot(L) - tca*tca;
        if d2 > self.radius * self.radius 
        {
            return false;
        }
        let thc: f32 = (self.radius*self.radius - d2).sqrt();
        *step = tca - thc;
        let t1 = tca + thc;
        if *step < 0.0 {
            *step = t1;
        } 
        if *step < 0.0 {
            return false;
        }
        return true;
    }
}