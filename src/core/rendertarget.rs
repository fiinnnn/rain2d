use crate::core::Color;

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

    pub fn clear(&mut self, color: Color) {
        for p in self.data.iter_mut() {
            *p = color.into();
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.data[(x + y * self.width as i32) as usize] = color.into();
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            return Some(self.data[x as usize + y as usize * self.width].into());
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::color::WHITE;

    #[test]
    fn test_new_rendertarget() {
        let target = RenderTarget::new(10, 10);
        assert_eq!(target.width, 10);
        assert_eq!(target.height, 10);
        assert_eq!(target.data.len(), 10 * 10);
        assert_eq!(target.data.iter().sum::<u32>(), 0);
    }

    #[test]
    fn test_clear() {
        let mut target = RenderTarget::new(10, 10);
        target.clear(WHITE);

        for p in target.data {
            assert_eq!(p, WHITE.into());
        }
    }

    #[test]
    fn test_set_pixel() {
        let mut target = RenderTarget::new(10, 10);
        let (x, y) = (5, 2);
        let color = WHITE;
        target.set_pixel(x, y, color);

        assert_eq!(target.data[x as usize + y as usize * 10], color.into());
    }

    #[test]
    fn test_get_pixel() {
        let mut target = RenderTarget::new(10, 10);
        let (x, y) = (7, 3);
        let color = WHITE;
        target.set_pixel(x, y, color);

        assert_eq!(target.get_pixel(x, y), Some(color));
        assert_eq!(target.get_pixel(100, 100), None);
    }
}