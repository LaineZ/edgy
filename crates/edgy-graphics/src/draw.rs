use embedded_heatshrink::{HSDFinishRes, HSDPollRes, HSDSinkRes, HeatshrinkDecoder};

use crate::{
    font::Font,
    framebuffer::FrameBuffer,
    geometry::{Point, Rectangle},
    image::Image,
    text::{LayoutOptions, TextLayout},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct BasicStyle {
    pub border_width: u16,
    pub fill_color: Option<u8>,
    pub border_color: u8,
}

impl BasicStyle {
    pub fn with_fill(fill_color: u8) -> Self {
        Self {
            fill_color: Some(fill_color),
            ..Default::default()
        }
    }

    pub fn with_border(border_color: u8, border_width: u16) -> Self {
        Self {
            border_color,
            border_width,
            ..Default::default()
        }
    }

    pub fn new(fill_color: u8, border_color: u8, border_width: u16) -> Self {
        Self {
            border_color,
            border_width,
            fill_color: Some(fill_color),
        }
    }
}

fn stamp(fb: &mut FrameBuffer, position: Point, size: u16, color: u8) {
    // if size == 1 use just pixel setting instead
    if size == 1 {
        if position.x >= 0
            && position.y >= 0
            && position.x < fb.width() as i32
            && position.y < fb.height() as i32
        {
            fb.set_pixel(position.x as u16, position.y as u16, color);
        }
        return;
    }

    let r = size as i32 / 2;
    for yy in position.y - r..=position.y + r {
        if yy < 0 || yy >= fb.height() as i32 {
            continue;
        }

        for xx in position.x - r..=position.x + r {
            if xx < 0 || xx >= fb.width() as i32 {
                continue;
            }

            fb.set_pixel(xx as u16, yy as u16, color);
        }
    }
}

fn plot8(fb: &mut FrameBuffer, center_position: Point, position: Point, color: u8, thickness: u16) {
    stamp(
        fb,
        Point::new(
            center_position.x + position.x,
            center_position.y + position.y,
        ),
        thickness,
        color,
    );
    stamp(
        fb,
        Point::new(
            center_position.x - position.x,
            center_position.y + position.y,
        ),
        thickness,
        color,
    );
    stamp(
        fb,
        Point::new(
            center_position.x + position.x,
            center_position.y - position.y,
        ),
        thickness,
        color,
    );
    stamp(
        fb,
        Point::new(
            center_position.x - position.x,
            center_position.y - position.y,
        ),
        thickness,
        color,
    );

    stamp(
        fb,
        Point::new(
            center_position.x + position.y,
            center_position.y + position.x,
        ),
        thickness,
        color,
    );
    stamp(
        fb,
        Point::new(
            center_position.x - position.y,
            center_position.y + position.x,
        ),
        thickness,
        color,
    );
    stamp(
        fb,
        Point::new(
            center_position.x + position.y,
            center_position.y - position.x,
        ),
        thickness,
        color,
    );
    stamp(
        fb,
        Point::new(
            center_position.x - position.y,
            center_position.y - position.x,
        ),
        thickness,
        color,
    );
}

#[inline(always)]
pub(crate) fn hline(fb: &mut FrameBuffer, x0: i32, x1: i32, y: i32, color: u8) {
    if y < 0 || y >= fb.height() as i32 {
        return;
    }

    let x0 = x0.max(0);
    let x1 = x1.min(fb.width() as i32 - 1);

    for x in x0..=x1 {
        fb.set_pixel(x as u16, y as u16, color);
    }
}

fn fill_circle_impl(fb: &mut FrameBuffer, center: Point, radius: i32, color: u8) {
    let mut x = radius;
    let mut y = 0;
    let mut d = 1 - radius;

    while x >= y {
        hline(fb, center.x - x, center.x + x, center.y + y, color);
        hline(fb, center.x - x, center.x + x, center.y - y, color);

        hline(fb, center.x - y, center.x + y, center.y + x, color);
        hline(fb, center.x - y, center.x + y, center.y - x, color);

        y += 1;

        if d < 0 {
            d += 2 * y + 1;
        } else {
            x -= 1;
            d += 2 * (y - x) + 1;
        }
    }
}

fn circle_impl(fb: &mut FrameBuffer, center: Point, radius: i32, color: u8, thickness: u16) {
    let mut x = radius;
    let mut y = 0;
    let mut d = 1 - radius;

    while x >= y {
        plot8(fb, center, Point::new(x, y), color, thickness);

        y += 1;

        if d < 0 {
            d += 2 * y + 1;
        } else {
            x -= 1;
            d += 2 * (y - x) + 1;
        }
    }
}

pub fn line(fb: &mut FrameBuffer, first: Point, second: Point, color: u8, thickness: u16) {
    let mut x0 = first.x;
    let mut y0 = first.y;
    let x1 = second.x;
    let y1 = second.y;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };

    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;

    loop {
        stamp(fb, Point::new(x0, y0), thickness, color);
        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = err * 2;

        if e2 >= dy {
            err += dy;
            x0 += sx;
        }

        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn rect_impl(fb: &mut FrameBuffer, rect: Rectangle, color: u8, thickness: u16) {
    if rect.size.is_zero() {
        return;
    }

    let x = rect.top_left.x;
    let y = rect.top_left.y;
    let w = rect.size.width as i32;
    let h = rect.size.height as i32;

    line(
        fb,
        Point::new(x, y),
        Point::new(x + w - 1, y),
        color,
        thickness,
    );

    line(
        fb,
        Point::new(x, y),
        Point::new(x, y + h - 1),
        color,
        thickness,
    );

    line(
        fb,
        Point::new(x + w - 1, y),
        Point {
            x: x + w - 1,
            y: y + h - 1,
        },
        color,
        thickness,
    );

    line(
        fb,
        Point::new(x, y + h - 1),
        Point {
            x: x + w - 1,
            y: y + h - 1,
        },
        color,
        thickness,
    );
}

fn fill_rect_impl(fb: &mut FrameBuffer, rect: Rectangle, color: u8) {
    if rect.size.is_zero() {
        return;
    }

    let x0 = rect.top_left.x.max(0);
    let y0 = rect.top_left.y.max(0);

    let x1 = (rect.top_left.x + rect.size.width as i32).min(fb.width() as i32);
    let y1 = (rect.top_left.y + rect.size.height as i32).min(fb.height() as i32);

    for y in y0..y1 {
        for x in x0..x1 {
            fb.set_pixel(x as u16, y as u16, color);
        }
    }
}

pub fn rect(fb: &mut FrameBuffer, rect: Rectangle, style: BasicStyle) {
    if let Some(color) = style.fill_color {
        fill_rect_impl(fb, rect, color);
    }

    if style.border_width > 0 {
        rect_impl(fb, rect, style.border_color, style.border_width);
    }
}

pub fn circle(fb: &mut FrameBuffer, center: Point, radius: i32, style: BasicStyle) {
    if let Some(color) = style.fill_color {
        fill_circle_impl(fb, center, radius, color);
    }

    if style.border_width > 0 {
        circle_impl(fb, center, radius, style.border_color, style.border_width);
    }
}

/// Draws text using a simple layout.
pub fn text(
    fb: &mut FrameBuffer,
    position: Point,
    font: &Font,
    text: &str,
    color: u8,
) -> TextLayout {
    crate::text::layout(font, position, text, |pos, glyph| {
        crate::text::draw_glyph(fb, pos, font, glyph, color);
    })
}

/// Draws text using advanced layout options.
///
/// Unlike [`text`], this function performs heap allocations while processing
/// the text. If you only need to draw a simple string, prefer [`text`].
pub fn text_advanced(
    fb: &mut FrameBuffer,
    bounds: Rectangle,
    options: &LayoutOptions,
    font: &Font,
    text: &str,
    color: u8,
) -> TextLayout {
    crate::text::layout_bounds(font, bounds, text, options, |pos, glyph| {
        crate::text::draw_glyph(fb, pos, font, glyph, color);
    })
}

fn blit_bytes(
    fb: &mut FrameBuffer,
    position: Point,
    image: &Image,
    pixel: &mut usize,
    data: &[u8],
) {
    for byte in data {
        for i in 0..image.format.pixels_per_byte() {
            if *pixel >= image.width as usize * image.height as usize {
                return;
            }

            let value = image.format.unpack(*byte, i);

            let x = *pixel % image.width as usize;
            let y = *pixel / image.width as usize;
            
            let transparent = image.is_transparent(x as u16, y as u16);
            
            if !transparent {
                fb.set_pixel(
                    (position.x + x as i32) as u16,
                    (position.y + y as i32) as u16,
                    value,
                );   
            }

            *pixel += 1;
        }
    }
}

pub fn image(fb: &mut FrameBuffer, position: Point, image: &Image) {
    let mut pixel = 0usize;
    if !image.compress {
        blit_bytes(fb, position, image, &mut pixel, image.bitmap);
    } else {
        let mut decoder = HeatshrinkDecoder::new(32, 8, 4).unwrap();
        let mut input = image.bitmap;
        let mut buf: [u8; 32] = [0; 32];
        
        loop {
            if !input.is_empty() {
                match decoder.sink(input) {
                    HSDSinkRes::Ok(n) => {
                        input = &input[n..];
                    }
                    HSDSinkRes::Full => {
                    }
                    e => panic!("{e:?}"),
                }
            }

            loop {
                match decoder.poll(&mut buf) {
                    HSDPollRes::More(n) => {
                        blit_bytes(fb, position, image, &mut pixel, &buf[..n]);
                    }
                    HSDPollRes::Empty(n) => {
                        blit_bytes(fb, position, image, &mut pixel, &buf[..n]);
                        break;
                    }
                    
                    e => panic!("{e:?}"),
                }
            }


            if input.is_empty() {
                match decoder.finish() {
                    HSDFinishRes::Done => break,
                    _ => {}
                }
            }
        }
    }
}
