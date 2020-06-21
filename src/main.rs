#![allow(dead_code)]

use std::rc::Rc;

mod raytracer;

use self::raytracer::frame::Frame;
use self::raytracer::colorf::ColorF;
use self::raytracer::light::Light;
use self::raytracer::scene::Scene;
use self::raytracer::material::Material;
use self::raytracer::traceable::*;

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

    let mut scene = Scene::new();
    
    scene.materials.insert("ivory".to_string(), Rc::new(ivory));
    scene.materials.insert("glass".to_string(), Rc::new(glass));
    scene.materials.insert("red_rubber".to_string(), Rc::new(red_rubber));
    scene.materials.insert("mirror".to_string(), Rc::new(mirror));
    scene.materials.insert("even_chess".to_string(), Rc::new(even_chess));
    scene.materials.insert("odd_chess".to_string(), Rc::new(odd_chess));

    let s1 = Sphere { center: Vec3f::new(-3.0,  0.0, -16.0), radius: 2.0, material: scene.find_material("ivory") };
    let s2 = Sphere { center: Vec3f::new(-1.0, -1.5, -12.0), radius: 2.0, material: scene.find_material("glass")  };
    let s3 = Sphere { center: Vec3f::new( 1.5, -0.5, -18.0), radius: 3.0, material: scene.find_material("red_rubber") };
    let s4 = Sphere { center: Vec3f::new( 7.0,  5.0, -18.0), radius: 4.0, material: scene.find_material("mirror") };
    let brd = Chessboard { 
        even_material: scene.find_material("even_chess"), 
        odd_material: scene.find_material("odd_chess") 
    };

    scene.objects.push(&s1);
    scene.objects.push(&s2);
    scene.objects.push(&s3);
    scene.objects.push(&s4);
    scene.objects.push(&brd);

    scene.lights.push(Light { position: Vec3f::new(-20.0, 20.0,  20.0), intensity: 1.5 });
    scene.lights.push(Light { position: Vec3f::new( 30.0, 50.0, -25.0), intensity: 1.8 });
    scene.lights.push(Light { position: Vec3f::new( 30.0, 20.0,  30.0), intensity: 1.7 });

    raytracer::render_scene(&mut framebuffer, &scene);

    raytracer::writer::save_as_ppm(&framebuffer, "result.ppm").unwrap();
}
