use crate::Color;

pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
}

#[allow(dead_code)]
impl RenderTarget {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.data[(x + y * self.width as i32) as usize] = color.into();
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.data[x + y * self.width].into()
    }
}