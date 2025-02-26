use edgy::{
    widgets::{
        linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder},
        UiBuilder, WidgetObj,
    },
    Theme, UiContext,
};
use embedded_graphics::{
    mono_font::{iso_8859_10::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    text::Alignment,
};
use embedded_graphics_simulator::{sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, Window};
use tinybmp::Bmp;

pub fn demo_ui<'a, D, I>(image: &'a I) -> WidgetObj<'a, D, Rgb888>
where
    I: ImageDrawable<Color = Rgb888>,
    D: DrawTarget<Color = Rgb888> + 'a,
{
    let mut ui = LinearLayoutBuilder::default()
        .horizontal_alignment(LayoutAlignment::Center)
        .vertical_alignment(LayoutAlignment::Center)
        .direction(LayoutDirection::Vertical);

    ui.label(
        "DISPLAYING BEE!",
        Alignment::Center,
        MonoTextStyle::new(&FONT_10X20, Rgb888::WHITE),
    );
    ui.image(image);
    ui.finish()
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(256, 286));

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(2)
        .build();

    let mut window = Window::new("a bit edgy ui", &output_settings);
    let mut ui_ctx = UiContext::new(&mut display, Theme::hope_diamond());

    let bmp = Bmp::<Rgb888>::from_slice(include_bytes!("bee.bmp")).unwrap();
    println!(
        "bitmap: {} pixels: {}",
        bmp.bounding_box().size,
        bmp.pixels().count()
    );

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
                }
                _ => {}
            }
        }

        ui_ctx.draw_target.clear(Rgb888::BLACK)?;
        ui_ctx.update(&mut demo_ui(&bmp));
    }
}
