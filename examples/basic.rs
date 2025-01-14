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
        let mut ui_ctx = UiContext::new(&mut display, rect);

        let mut stack = StackLayout::new(StackLayoutDirection::Vertical, |add| {
            add(Box::new(Label::new::<SimulatorDisplay<Rgb888>>(
                text_style,
                "Hello world",
            )));
            add(Box::new(Label::new::<SimulatorDisplay<Rgb888>>(
                text_style,
                "Nested layout!",
            )));

            add(Box::new(StackLayout::new(
                StackLayoutDirection::Horizontal,
                |add| {
                    add(Box::new(Label::new::<SimulatorDisplay<Rgb888>>(
                        text_style, "Widget 1",
                    )));
                    add(Box::new(Label::new::<SimulatorDisplay<Rgb888>>(
                        text_style, "Widget 2",
                    )));
                },
            )));
        });

        stack.draw(&mut ui_ctx);

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
