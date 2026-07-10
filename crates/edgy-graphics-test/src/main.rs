#![allow(dead_code)]

use edgy_graphics::{
    PixelFormat,
    draw::{self, BasicStyle},
    framebuffer::FrameBuffer,
    geometry::{Point, Rectangle, Size},
    parse_palette_rgb888,
    polygon::{PolygonData, fill_polygon},
};
use image::RgbImage;

use crate::fonts::unscii::UNSCII;

pub mod fonts;
pub mod transparent_gradient;

fn basic_test(fb: &mut FrameBuffer) {
    // background
    draw::rect(
        fb,
        Rectangle::new(Point::<i32>::zero(), Size::new(320, 240)),
        BasicStyle::with_fill(0),
    );

    // grid
    for x in (0..320).step_by(32) {
        draw::line(fb, Point::new(x, 0), Point::new(x, 239), 1, 1);
    }

    for y in (0..240).step_by(32) {
        draw::line(fb, Point::new(0, y), Point::new(319, y), 1, 1);
    }

    // lines
    for i in 0..16 {
        draw::line(fb, Point::new(10, 10), Point::new(160 + i * 8, 120), 2, 1);
    }

    // thick lines
    draw::line(fb, Point::new(20, 180), Point::new(300, 180), 3, 2);
    draw::line(fb, Point::new(20, 190), Point::new(300, 220), 4, 4);
    draw::line(fb, Point::new(20, 220), Point::new(300, 150), 5, 8);

    // rectangles
    draw::rect(
        fb,
        Rectangle::new(Point::new(180, 20), Size::new(120, 80)),
        BasicStyle::new(2, 6, 3),
    );

    draw::rect(
        fb,
        Rectangle::new(Point::new(200, 40), Size::new(40, 40)),
        BasicStyle::with_fill(7),
    );

    // concentric circles
    for r in (8..70).step_by(8) {
        draw::circle(
            fb,
            Point::new(80, 120),
            r,
            BasicStyle::with_border((r / 8 % 7 + 1) as u8, 1),
        );
    }

    // thick circles
    draw::circle(fb, Point::new(250, 160), 40, BasicStyle::with_border(3, 6));

    draw::circle(fb, Point::new(250, 160), 20, BasicStyle::with_fill(5));
}

fn image_test(fb: &mut FrameBuffer) {
    draw::image(
        fb,
        Point::new(0, 0),
        &transparent_gradient::TRANSPARENT_GRADIENT,
    );
}

fn clipping_test(fb: &mut FrameBuffer) {
    let clipping = Rectangle::new(Point::new(10, 50), Size::new(120, 80));

    let scroll_y = 100;

    fb.with_clip(clipping, |fb| {
        for i in 0..1000 {
            let y = clipping.top_left.y + (i as i32 * 16) - scroll_y;

            let item = Rectangle::new(
                Point::new(clipping.top_left.x, y),
                Size::new(clipping.size.width, 16),
            );

            draw::rect(fb, item, BasicStyle::with_fill(1));
            draw::rect(fb, item, BasicStyle::with_border(4, 1));

            draw::text(
                fb,
                item.top_left + Point::new(4, 0),
                &UNSCII,
                &format!("Item {}", i),
                7,
            );
        }
    });
}

fn polygon_test(fb: &mut FrameBuffer) {
    let mut data = PolygonData::<128>::default();

    let cx = 220.0;
    let cy = 60.0;
    let scale = 1.1;
    let resolution = 128;

    for i in 0..resolution {
        let t = i as f32 / resolution as f32 * 2.0 * std::f32::consts::PI;

        let x = 16.0 * t.sin().powi(3);
        let y = 13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();

        let _ = data
            .points
            .push(Point::new((cx + x * scale) as i32, (cy - y * scale) as i32));
    }

    fill_polygon(fb, &mut data, 2);

    for point in data.points {
        draw::circle(fb, point, 1, BasicStyle::with_fill(4));
    }
}

const COLORS: [u32; 256] = parse_palette_rgb888::<256>(include_str!("vga13h.hex"));

fn main() {
    let mut fb = FrameBuffer::new(320, 240, PixelFormat::Bpp8);
    let mut img = RgbImage::new(fb.width() as u32, fb.height() as u32);

    for color in COLORS {
        println!("{:#08x}", color);
    }

    image_test(&mut fb);

    for y in 0..fb.height() {
        for x in 0..fb.width() {
            let color = COLORS[fb.get_pixel(x, y) as usize];
            let [b, g, r, _] = color.to_le_bytes();

            img.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
        }
    }
    img.save("graphics_test.png").unwrap();
}
