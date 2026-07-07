use crate::PixelFormat;

pub struct Image<'a> {
    pub bitmap: &'a [u8],
    pub alpha: Option<&'a [u8]>,
    pub width: u16,
    pub height: u16,
    pub format: PixelFormat,
    pub compress: bool,
}

impl<'a> Image<'a> {
    pub fn is_transparent(&self, x: u16, y: u16) -> bool {
        if x >= self.width || y >= self.height {
            return false
        }
        
        let pixel = y as usize * self.width as usize + x as usize;
        
        let ppb = PixelFormat::Bpp1.pixels_per_byte() as usize;
        let bits = PixelFormat::Bpp1 as usize;

        let byte = pixel / ppb;
        let shift = (ppb - 1 - pixel % ppb) * bits;

        if let Some(alpha) = self.alpha {
            ((alpha[byte] >> shift) & PixelFormat::Bpp1.mask()) == 0   
        } else {
            false
        }
    }
}