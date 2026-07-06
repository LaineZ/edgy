use crate::PixelFormat;

pub struct Image<'a> {
    pub bitmap: &'a [u8],
    pub width: u16,
    pub height: u16,
    pub format: PixelFormat,
    pub compress: bool,
}