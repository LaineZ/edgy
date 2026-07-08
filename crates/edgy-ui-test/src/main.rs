use edgy_ui::{UiContext, edgy_graphics::{PixelFormat, framebuffer::FrameBuffer, parse_palette_rgb888}, widgets::{WidgetObject, label::TypographyLabel}};
use image::RgbImage;

use crate::fonts::unscii::UNSCII;

const COLORS: [u32; 256] = parse_palette_rgb888::<256>(include_str!("vga13h.hex"));
pub mod fonts;


fn main() {
    let mut ui = UiContext::<i32>::new(FrameBuffer::new(320, 240, PixelFormat::Bpp8));
    let mut img = RgbImage::new(ui.framebuffer.width() as u32, ui.framebuffer.height() as u32);
    let root = WidgetObject::new(Box::new(TypographyLabel::new(UNSCII, "Pizda", 1)));
    
    ui.update(root);

    
    for y in 0..ui.framebuffer.height() {
        for x in 0..ui.framebuffer.width() {
            let color = COLORS[ui.framebuffer.get_pixel(x, y) as usize];
            let [b, g, r, _] = color.to_le_bytes();

            img.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
        }
    }
    
    img.save("ui_test.png").unwrap();
}
