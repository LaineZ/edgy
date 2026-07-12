use std::time::{Duration, Instant};

use edgy_desktop::{SoftbufferWindow, WindowProperties, fonts::govno::UNSCII};
use edgy_ui::{Prop, UiContext, graphics::{self, geometry::{Point, Size}, parse_palette_rgb888}, widgets::label::TypographyLabel};

const COLORS: [u32; 256] = parse_palette_rgb888::<256>(include_str!("vga13h.hex"));

fn main() {
    let mut window = SoftbufferWindow::new(WindowProperties::default(), COLORS.to_vec());
    let mut frames = 0;
    let mut fps = 0;
    let mut last = Instant::now();
    let mut ui: UiContext<i32> = UiContext::new(Size::new(800, 600));

    let mut pizda = String::from("temperature");

    ui.build(|b| {
        b.column(|b| {

            fn pizda() -> String {
                String::from("govno")
            }
            
            b.add(TypographyLabel::new(Prop::Binding(pizda), Prop::Value(&UNSCII), Prop::Value(5)));
            b.add(TypographyLabel::new(Prop::Value(String::from("How are you?")), Prop::Value(&UNSCII), Prop::Value(4)));
        });
    });
    
    window.run(move |framebuffer| {
        frames += 1;

        if last.elapsed() >= Duration::from_secs(1) {
            fps = frames;
            frames = 0;
            last = Instant::now();
        }
        framebuffer.clear();
        ui.resize(Size::new(framebuffer.width() as u32, framebuffer.height() as u32));
        ui.layout();
        ui.draw(framebuffer);
        graphics::draw::text(framebuffer, Point::new(10, 10), &UNSCII, &format!("FPS: {}", fps), 3);
        //panic!();

    }).unwrap();
}
