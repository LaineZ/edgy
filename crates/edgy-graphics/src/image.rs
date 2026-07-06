use crate::PixelFormat;

pub struct Image<'a> {
    pub bitmap: &'a [u8],
    pub width: u16,
    pub height: u16,
    pub format: PixelFormat,
    pub compress: bool,
}

impl<'a> Image<'a> {
    pub fn get_pixel(&self, x: u16, y: u16) -> u8 {
        assert_eq!(self.compress, false);
        let pixel = y as usize * self.width as usize + x as usize;
        let ppb = self.format.pixels_per_byte() as usize;
        let bits = self.format as usize;

        let byte = pixel / ppb;
        let shift = (ppb - 1 - pixel % ppb) * bits;

        (self.bitmap[byte] >> shift) & self.format.mask()
    }
}
