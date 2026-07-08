use std::time::{Duration, Instant};

use edgy_desktop::{SoftbufferWindow, WindowProperties, fonts::unscii::UNSCII};
use edgy_ui::{edgy_graphics::parse_palette_rgb888, widgets::{label::{typography_label}}};


const COLORS: [u32; 256] = parse_palette_rgb888::<256>(include_str!("vga13h.hex"));

fn main() {
    let mut window = SoftbufferWindow::<i32>::new(WindowProperties::default(), COLORS.to_vec());
    let mut frames = 0;
    let mut fps = 0;
    let mut last = Instant::now();
    
    window.run(move |ctx| {
        frames += 1;

        if last.elapsed() >= Duration::from_secs(1) {
            fps = frames;
            frames = 0;
            last = Instant::now();
        }

        ctx.framebuffer.clear();

        let root = typography_label(&UNSCII, format!("FPS: {}", fps), 1);
        ctx.update(root);
    }).unwrap();
}