#[derive(Clone, Copy)]
pub struct ColorF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl ColorF {
    pub const BLACK: ColorF = ColorF{ r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE: ColorF = ColorF{ r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const RED: ColorF =   ColorF{ r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: ColorF = ColorF{ r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: ColorF =  ColorF{ r: 0.0, g: 0.0, b: 1.0, a: 1.0 };

    pub fn new() -> ColorF {
        ColorF{ r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> ColorF {
        ColorF{ r, g, b, a: 1.0 }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> ColorF {
        ColorF{ r, g, b, a }
    }
}