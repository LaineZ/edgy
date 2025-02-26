use edgy::{
    widgets::{
        linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder},
        UiBuilder,
    },
    Theme, UiContext,
};
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    text::Alignment,
};
use embedded_graphics_simulator::{sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, Window};
use rand::Rng;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(2)
        .build();

    let mut window = Window::new("a bit edgy ui", &output_settings);
    let mut ui_ctx = UiContext::new(&mut display, Theme::hope_diamond());

    let mut points = Vec::new();
    let mut offset = Point::zero();
    let mut scale = 1.0;
    let mut rng = rand::rng();

    for x in 0..100 {
        let y = (x as f32).sin();
        points.push(Point::new(x, (y * rng.random_range(-3.0..3.0)) as i32));
    }

    loop {
        window.update(&ui_ctx.draw_target);

        for event in window.events() {
            match event {
                embedded_graphics_simulator::SimulatorEvent::Quit => {
                    std::process::exit(0);
                }
                embedded_graphics_simulator::SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat: _,
                } => {
                    if keycode == Keycode::F1 {
                        ui_ctx.debug_mode = !ui_ctx.debug_mode;
                    }

                    if keycode == Keycode::UP {
                        offset.y += 1;
                    }

                    if keycode == Keycode::DOWN {
                        offset.y -= 1;
                    }

                    if keycode == Keycode::EQUALS {
                        scale += 0.1;
                    }

                    if keycode == Keycode::MINUS {
                        scale -= 0.1;
                    }
                }
                _ => {}
            }
        }

        let mut ui = LinearLayoutBuilder::default()
            .vertical_alignment(LayoutAlignment::Stretch)
            .horizontal_alignment(LayoutAlignment::Stretch)
            .direction(LayoutDirection::Vertical);

        ui.label(
            format!("HORIZONTAL OFFSET: {} SCALE: {:.0}%", offset.y, scale * 100.0),
            Alignment::Center,
            MonoTextStyle::new(&FONT_4X6, Rgb888::WHITE),
        );
        ui.plot(points.clone(), scale, offset);

        ui_ctx.draw_target.clear(Rgb888::BLACK)?;
        ui_ctx.update(&mut ui.finish());
    }
}
