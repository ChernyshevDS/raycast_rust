use super::colorf::ColorF;

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

    pub fn data_mut(&mut self) -> &mut Vec<ColorF> {
        &mut self.data
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