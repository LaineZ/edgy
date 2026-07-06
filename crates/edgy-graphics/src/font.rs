#[derive(Clone, Copy, Debug)]
pub struct Glyph {
    pub character: char,
    pub offset: u16,
    pub width: u8,
    pub height: u8,
    pub x_offset: i8,
    pub y_offset: i8,
    pub advance_width: u8,
}

pub struct Font<'a> {
    pub glyphs: &'a [Glyph],
    pub bitmap: &'a [u8],
    pub line_height: u8,
    pub ascent: u8,
    pub descent: u8,
}
