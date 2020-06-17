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

#[derive(Clone)]
pub struct Frame {
    data: Vec<ColorF>,
    width: usize,
    height: usize
}
    
impl Frame {
    pub fn new(width: usize, height: usize) -> Frame {
        let cont = vec![ColorF::BLACK; width * height];
        Frame {data: cont, width, height}
    }

    pub fn data(&self) -> &Vec<ColorF>{
        &self.data
    }

    #[inline]
    pub fn width(&self) -> usize { self.width }

    #[inline]
    pub fn height(&self) -> usize { self.height }

    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> &ColorF {
        let index = self.get_index(x, y);
        &self.data[index]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &ColorF) {
        let index = self.get_index(x, y);
        let c = &mut self.data[index];
        *c = *color;
    }

    pub fn fill(&mut self, color: &ColorF) {
        for e in &mut self.data {
            *e = *color;
        }
    }

    #[inline]
    fn get_index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
}
