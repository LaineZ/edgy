use alloc::vec::Vec;

use crate::{
    Color, font::{Font, Glyph}, framebuffer::FrameBuffer, geometry::{Point, Rectangle, Size},
};

#[derive(Clone, Copy, Debug)]
pub struct TextLayout {
    pub cursor: Point,
    pub bounding_box: Rectangle,
    pub lines: u16,
}

#[derive(Clone, Copy)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Clone, Copy)]
pub enum Wrap {
    None,
    Character,
    Word,
}

#[derive(Clone, Copy)]
pub struct LayoutOptions {
    pub wrap: Wrap,
    pub horizontal: HorizontalAlign,
    pub vertical: VerticalAlign,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            wrap: Wrap::None,
            horizontal: HorizontalAlign::Left,
            vertical: VerticalAlign::Top,
        }
    }
}

#[derive(Debug)]
struct Line<'a> {
    glyphs: Vec<&'a Glyph>,
    width: i32,
}

impl Font<'_> {
    pub fn glyph(&self, ch: char) -> Option<&Glyph> {
        self.glyphs.binary_search_by_key(&ch, |g| g.character).ok().map(|i| &self.glyphs[i])
    }

    pub fn bitmap<'a>(&'a self, glyph: &Glyph) -> &'a [u8] {
        let stride = glyph.width.div_ceil(8) as usize;
        let size = stride * glyph.height as usize;

        &self.bitmap[glyph.offset as usize..][..size]
    }

    pub fn measure_line(&self, text: &str) -> Size {
        let mut size = Size::new(0, self.line_height as u32);
        for ch in text.chars() {
            size.width += self.glyph(ch).unwrap_or(self.replacement()).advance_width as u32;
        }

        size
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

pub(crate) fn draw_glyph(
    fb: &mut FrameBuffer,
    pos: Point,
    font: &Font,
    glyph: &Glyph,
    color: Color,
) -> Rectangle {
    let stride = glyph.width.div_ceil(8) as usize;
    let size = stride * glyph.height as usize;
    let bitmap = &font.bitmap[glyph.offset as usize..][..size];

    let px = pos.x + glyph.x_offset as i32;
    let py = pos.y;

    for y in 0..glyph.height {
        for x in 0..glyph.width {
            if bit(bitmap, stride, x, y) {
                fb.set_pixel((px + x as i32) as u16, (py + y as i32) as u16, color);
            } else {
                //fb.set_pixel((px + x as i32) as u16, (py + y as i32) as u16, 3);
            }
        }
    }

    Rectangle::new(
        Point::new(px, py),
        Size::new(glyph.width as u32, glyph.height as u32),
    )
}

pub fn layout<F: FnMut(Point, &Glyph)>(
    font: &Font,
    position: Point,
    text: &str,
    mut f: F,
) -> TextLayout {
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
        Size::new(max_width as u32, lines as u32 * font.line_height as u32),
    );

    TextLayout {
        cursor: pos,
        bounding_box,
        lines,
    }
}

pub fn layout_bounds<F>(
    font: &Font,
    bounds: Rectangle,
    text: &str,
    options: &LayoutOptions,
    mut f: F,
) -> TextLayout
where
    F: FnMut(Point, &Glyph),
{
    let mut lines = Vec::<Line>::new();

    // PHASE ONE: Splitting text to lines

    let max_width = bounds.size.width as i32;

    let mut current = Line {
        glyphs: Vec::new(),
        width: 0,
    };

    match options.wrap {
        Wrap::None | Wrap::Character => {
            for ch in text.chars() {
                if ch == '\n' {
                    lines.push(current);
                    current = Line {
                        glyphs: Vec::new(),
                        width: 0,
                    };
                    continue;
                }

                let glyph = font.glyph(ch).unwrap_or(font.replacement());

                if matches!(options.wrap, Wrap::Character)
                    && current.width + glyph.advance_width as i32 > max_width
                    && !current.glyphs.is_empty()
                {
                    lines.push(current);

                    current = Line {
                        glyphs: Vec::new(),
                        width: 0,
                    };
                }

                current.width += glyph.advance_width as i32;
                current.glyphs.push(glyph);
            }
        }

        Wrap::Word => {
            for paragraph in text.split('\n') {
                let space = font.glyph(' ').unwrap_or(font.replacement());
                for word in paragraph.split(' ') {
                    let word_width = font.measure_line(word).width as i32;
                    let space_width = space.advance_width as i32;

                    let required = if current.glyphs.is_empty() {
                        word_width
                    } else {
                        space_width + word_width
                    };

                    if current.width + required > max_width && !current.glyphs.is_empty() {
                        lines.push(current);

                        current = Line {
                            glyphs: Vec::new(),
                            width: 0,
                        };
                    }

                    if !current.glyphs.is_empty() {
                        current.width += space.advance_width as i32;

                        current.glyphs.push(space);
                    }

                    for ch in word.chars() {
                        let glyph = font.glyph(ch).unwrap_or(font.replacement());

                        current.width += glyph.advance_width as i32;

                        current.glyphs.push(glyph);
                    }
                }

                lines.push(current);

                current = Line {
                    glyphs: Vec::new(),
                    width: 0,
                };
            }
        }
    }

    if !current.glyphs.is_empty() {
        lines.push(current);
    }

    // Alignment
    let text_height = lines.len() as i32 * font.line_height as i32;
    let text_width = lines.iter().map(|l| l.width).max().unwrap_or(0);

    let y_offset = match options.vertical {
        VerticalAlign::Top => 0,
        VerticalAlign::Center => (bounds.size.height as i32 - text_height) / 2,
        VerticalAlign::Bottom => bounds.size.height as i32 - text_height,
    };

    // PHASE TWO: Compute bounds

    let mut cursor = Point::new(bounds.top_left.x, bounds.top_left.y);

    for (line_index, line) in lines.iter().enumerate() {
        let x_offset = match options.horizontal {
            HorizontalAlign::Left => 0,
            HorizontalAlign::Center => (bounds.size.width as i32 - line.width) / 2,
            HorizontalAlign::Right => bounds.size.width as i32 - line.width,
        };
        let mut x = bounds.top_left.x + x_offset;
        let y = bounds.top_left.y + y_offset + line_index as i32 * font.line_height as i32;
        for glyph in &line.glyphs {
            f(Point::new(x, y), glyph);
            x += glyph.advance_width as i32;
        }

        cursor = Point::new(x, y);
    }

    TextLayout {
        cursor,
        lines: lines.len() as u16,
        bounding_box: Rectangle::new(bounds.top_left, Size::new(text_width as u32, text_height as u32)),
    }
}
