use crate::{framebuffer::FrameBuffer, geometry::Point};

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
}

impl Font<'_> {
    pub fn glyph(&self, ch: char) -> Option<&Glyph> {
        self.glyphs.iter().find(|g| g.character == ch)
    }

    pub fn bitmap<'a>(&'a self, glyph: &Glyph) -> &'a [u8] {
        let stride = glyph.width.div_ceil(8) as usize;
        let size = stride * glyph.height as usize;

        &self.bitmap[glyph.offset as usize..][..size]
    }
}

#[inline(always)]
fn bit(bitmap: &[u8], stride: usize, x: u8, y: u8) -> bool {
    let byte = bitmap[y as usize * stride + x as usize / 8];
    let mask = 1 << (7 - (x % 8));

    byte & mask != 0
}

fn draw_glyph(fb: &mut FrameBuffer, pos: Point, font: &Font, glyph: &Glyph, color: u8) {
    let stride = glyph.width.div_ceil(8) as usize;
    let size = stride * glyph.height as usize;
    let bitmap = &font.bitmap[glyph.offset as usize..][..size];

    let px = pos.x + glyph.x_offset as i32;
    let py = pos.y - glyph.y_offset as i32 - glyph.height as i32;

    for y in 0..glyph.height {
        for x in 0..glyph.width {
            if bit(bitmap, stride, x, y) {
                fb.set_pixel(
                    (px + x as i32) as u16,
                    (py + y as i32) as u16,
                    color,
                );
            }
        }
    }
}

pub fn text<S: AsRef<str>>(fb: &mut FrameBuffer, position: Point, font: &Font, text: S, color: u8) {
    let mut pos = position;
    for ch in text.as_ref().chars() {
        if let Some(glyph) = font.glyph(ch) {
            draw_glyph(fb, pos, font, glyph, color);
            pos.x += glyph.advance_width as i32;
        }
    }
}
