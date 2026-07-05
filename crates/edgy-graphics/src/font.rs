use crate::{framebuffer::FrameBuffer, geometry::{Point, Rectangle, Size}};

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

#[derive(Clone, Copy, Debug)]
pub struct TextLayout {
    pub cursor: Point,
    pub bounding_box: Rectangle,
    pub lines: u16,
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

    pub fn replacement(&self) -> &Glyph {
        self.glyph('�')
            .or_else(|| self.glyph('?'))
            .or_else(|| self.glyph(' '))
            .expect("font has no replacement glyph")
    }
}

#[inline(always)]
fn bit(bitmap: &[u8], stride: usize, x: u8, y: u8) -> bool {
    let byte = bitmap[y as usize * stride + x as usize / 8];
    let mask = 1 << (7 - (x % 8));

    byte & mask != 0
}

fn draw_glyph(fb: &mut FrameBuffer, pos: Point, font: &Font, glyph: &Glyph, color: u8) -> Rectangle {
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

    Rectangle::new(Point::new(px, py), Size::new(glyph.width as u32, glyph.height as u32))
}

pub fn layout<F: FnMut(Point, &Glyph)>(font: &Font, position: Point, text: &str, mut f: F) -> TextLayout {
    let mut pos = position;
    let mut lines = 1;
    let mut max_width = 0;

    for ch in text.chars() {
        match ch {
            '\n' => {
                max_width = max_width.max(pos.x - position.x);
                
                pos.x = position.x;
                pos.y += font.line_height as i32;
                lines += 1;
            }
            _ => {
                let glyph = font.glyph(ch).unwrap_or(font.replacement());
                
                f(pos, glyph);
                pos.x += glyph.advance_width as i32;
            }   
        }
    }

    max_width = max_width.max(pos.x - position.x);
    
    let bounding_box = Rectangle::new(
        Point::new(position.x, position.y - font.ascent as i32),
        Size::new(
            max_width as u32,
            lines as u32 * font.line_height as u32,
        ),
    );
    
    TextLayout { cursor: pos, bounding_box: bounding_box, lines }
}

pub fn text(fb: &mut FrameBuffer, position: Point, font: &Font, text: &str, color: u8) -> TextLayout {
    layout(font, position, text.as_ref(), |pos, glyph| {
        draw_glyph(fb, pos, font, glyph, color);
    })
}
