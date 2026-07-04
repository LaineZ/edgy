use edgy_graphics::{
    draw::{BasicStyle, circle, line, rect}, font, fonts::{ARIAL}, framebuffer::{FrameBuffer, FramebufferFormat}, geometry::{Point, Rectangle, Size},
};
use image::RgbImage;

const PALETTE: [[u8; 3]; 8] = [
    [30, 30, 30],    // 0
    [255, 255, 255], // 1
    [255, 0, 0],     // 2
    [0, 255, 0],     // 3
    [0, 128, 255],   // 4
    [255, 255, 0],   // 5
    [255, 0, 255],   // 6
    [0, 255, 255],   // 7
];

fn main() {
    let mut fb = FrameBuffer::new(320, 240, FramebufferFormat::Bpp8);

    // background
    rect(
        &mut fb,
        Rectangle::new(Point::zero(), Size::new(320, 240)),
        BasicStyle::with_fill(0),
    );

    // grid
    for x in (0..320).step_by(32) {
        line(&mut fb, Point::new(x, 0), Point::new(x, 239), 1, 1);
    }

    for y in (0..240).step_by(32) {
        line(&mut fb, Point::new(0, y), Point::new(319, y), 1, 1);
    }

    // lines
    for i in 0..16 {
        line(
            &mut fb,
            Point::new(10, 10),
            Point::new(160 + i * 8, 120),
            2,
            1,
        );
    }

    // thick lines
    line(&mut fb, Point::new(20, 180), Point::new(300, 180), 3, 2);
    line(&mut fb, Point::new(20, 190), Point::new(300, 220), 4, 4);
    line(&mut fb, Point::new(20, 220), Point::new(300, 150), 5, 8);

    // rectangles
    rect(
        &mut fb,
        Rectangle::new(Point::new(180, 20), Size::new(120, 80)),
        BasicStyle::new(2, 6, 3),
    );

    rect(
        &mut fb,
        Rectangle::new(Point::new(200, 40), Size::new(40, 40)),
        BasicStyle::with_fill(7),
    );

    // concentric circles
    for r in (8..70).step_by(8) {
        circle(
            &mut fb,
            Point::new(80, 120),
            r,
            BasicStyle::with_border((r / 8 % 7 + 1) as u8, 1),
        );
    }

    // thick circles
    circle(
        &mut fb,
        Point::new(250, 160),
        40,
        BasicStyle::with_border(3, 6),
    );

    circle(&mut fb, Point::new(250, 160), 20, BasicStyle::with_fill(5));

    // font

    font::text(&mut fb, Point::new(10, 50), &ARIAL, "HELLO WORLD", 5);

    let mut img = RgbImage::new(fb.width() as u32, fb.height() as u32);

    for y in 0..fb.height() {
        for x in 0..fb.width() {
            let c = PALETTE[fb.get_pixel(x, y) as usize];
            img.put_pixel(x as u32, y as u32, image::Rgb(c));
        }
    }

    img.save("graphics_test.png").unwrap();
}
