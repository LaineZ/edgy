use edgy::Widget;
use edgy::{Label, StackLayout, StackLayoutDirection, UiContext};
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(2)
        .build();
    let mut window = Window::new("a bit edgy ui", &output_settings);
    let text_style = MonoTextStyle::new(&FONT_4X6, Rgb888::WHITE);

    loop {
        let frame_render = std::time::Instant::now();
        window.update(&display);
        display.clear(Rgb888::BLACK)?;

        let rect = Rectangle::new(Point::new(0, 20), display.size());
        let ui_context_render = std::time::Instant::now();
        let mut ui = UiContext::new(&mut display, rect);
        StackLayout::new(&mut ui, StackLayoutDirection::Vertical, |add_widget| {
            let mut label1 = Label::new::<SimulatorDisplay<Rgb888>>(
                MonoTextStyle::new(&FONT_4X6, Rgb888::RED),
                "Hello World",
            );

            let mut label2 = Label::new::<SimulatorDisplay<Rgb888>>(
                MonoTextStyle::new(&FONT_4X6, Rgb888::GREEN),
                "Goodbye World",
            );

            add_widget(&mut label1);
            add_widget(&mut label2);

            StackLayout::new(&mut ui, StackLayoutDirection::Horizontal, |add_widget| {
            });
        });


        let seconds_ui = ui_context_render.elapsed().as_millis();
        Text::new(
            &format!("edgy: {} ms", seconds_ui),
            Point::new(1, 10),
            text_style,
        )
        .draw(&mut display)?;

        let seconds = frame_render.elapsed().as_secs_f32();
        Text::new(
            &format!("window: {:.0} fps", 1.0 / seconds),
            Point::new(1, 5),
            text_style,
        )
        .draw(&mut display)?;


        for event in window.events() {
            match event {
                embedded_graphics_simulator::SimulatorEvent::Quit => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }
}
