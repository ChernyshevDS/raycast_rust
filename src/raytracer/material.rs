use super::vector::Vec4f;
use super::colorf::ColorF;

pub struct Material {
    pub refract: f32,
    pub albedo: Vec4f,
    pub diffuse_color: ColorF,
    pub specularity: f32
}