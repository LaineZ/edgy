use alloc::vec;
use alloc::vec::Vec;

use crate::{
    PixelFormat,
    geometry::{Point, Rectangle, Size},
};

pub struct FrameBuffer {
    format: PixelFormat,
    width: u16,
    height: u16,
    pub data: Vec<u8>,
    clip: Rectangle,
}

impl FrameBuffer {
    pub fn new(width: u16, height: u16, format: PixelFormat) -> Self {
        let pixels = width as usize * height as usize;
        let ppb = format.pixels_per_byte();
        let bytes = pixels.div_ceil(ppb.into());

        Self {
            format,
            width,
            height,
            data: vec![0; bytes],
            clip: Rectangle::new(Point::<i32>::zero(), Size::new(width as u32, height as u32)),
        }
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> u8 {
        let pixel = y as usize * self.width as usize + x as usize;
        let ppb = self.format.pixels_per_byte() as usize;
        let bits = self.format as usize;

        let byte = pixel / ppb;
        let shift = (ppb - 1 - pixel % ppb) * bits;

        (self.data[byte] >> shift) & self.format.mask()
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, value: u8) {
        if x > self.width() || y > self.height() {
            return;
        }

        if !self.clip.contains(Point::new(x as i32, y as i32)) {
            return;
        }

        let pixel = y as usize * self.width as usize + x as usize;
        let ppb = self.format.pixels_per_byte() as usize;
        let bits = self.format as usize;

        let byte = pixel / ppb;
        let shift = (ppb - 1 - pixel % ppb) * bits;

        let mask = self.format.mask() << shift;

        self.data[byte] &= !mask;
        self.data[byte] |= (value & self.format.mask()) << shift;
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn with_clip<R>(&mut self, clip: Rectangle, f: impl FnOnce(&mut Self) -> R) -> R {
        let old = self.clip;
        self.clip = self.clip.intersection(&clip);
        let r = f(self);
        self.clip = old;
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bpp1() {
        let mut fb = FrameBuffer::new(8, 1, PixelFormat::Bpp1);

        fb.set_pixel(0, 0, 1);
        fb.set_pixel(1, 0, 0);
        fb.set_pixel(2, 0, 1);
        fb.set_pixel(7, 0, 1);

        assert_eq!(fb.get_pixel(0, 0), 1);
        assert_eq!(fb.get_pixel(1, 0), 0);
        assert_eq!(fb.get_pixel(2, 0), 1);
        assert_eq!(fb.get_pixel(7, 0), 1);
    }

    #[test]
    fn bpp2() {
        let mut fb = FrameBuffer::new(4, 1, PixelFormat::Bpp2);

        fb.set_pixel(0, 0, 0);
        fb.set_pixel(1, 0, 1);
        fb.set_pixel(2, 0, 2);
        fb.set_pixel(3, 0, 3);

        assert_eq!(fb.get_pixel(0, 0), 0);
        assert_eq!(fb.get_pixel(1, 0), 1);
        assert_eq!(fb.get_pixel(2, 0), 2);
        assert_eq!(fb.get_pixel(3, 0), 3);
    }

    #[test]
    fn bpp4() {
        let mut fb = FrameBuffer::new(2, 1, PixelFormat::Bpp4);

        fb.set_pixel(0, 0, 0xA);
        fb.set_pixel(1, 0, 0x5);

        assert_eq!(fb.get_pixel(0, 0), 0xA);
        assert_eq!(fb.get_pixel(1, 0), 0x5);
    }

    #[test]
    fn bpp8() {
        let mut fb = FrameBuffer::new(4, 1, PixelFormat::Bpp8);

        fb.set_pixel(0, 0, 10);
        fb.set_pixel(1, 0, 20);
        fb.set_pixel(2, 0, 30);
        fb.set_pixel(3, 0, 40);

        assert_eq!(fb.get_pixel(0, 0), 10);
        assert_eq!(fb.get_pixel(1, 0), 20);
        assert_eq!(fb.get_pixel(2, 0), 30);
        assert_eq!(fb.get_pixel(3, 0), 40);
    }

    #[test]
    fn test_razyob() {
        let mut fb = FrameBuffer::new(8, 1, PixelFormat::Bpp1);

        for x in 0..8 {
            fb.set_pixel(x, 0, 0);
        }

        fb.set_pixel(3, 0, 1);

        for x in 0..8 {
            let expected = if x == 3 { 1 } else { 0 };
            assert_eq!(fb.get_pixel(x, 0), expected);
        }
    }
}
