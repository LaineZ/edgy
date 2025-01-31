use edgy::widgets::margin::Margin;

use edgy::{
    widgets::{
        linear_layout::{LayoutDirection, LinearLayoutBuilder},
        UiBuilder, WidgetObj,
    },
    UiContext,
};
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

pub fn demo_ui<'a, D>() -> WidgetObj<'a, D, Rgb888>
where
    D: DrawTarget<Color = Rgb888> + 'a,
{
    let mut ui = LinearLayoutBuilder {
        direction: LayoutDirection::Horizontal,
        children: Vec::new(),
    };
    ui.label("hello", Rgb888::RED);
    ui.label("world", Rgb888::RED);
    ui.margin_layout(edgy::margin!(5, 5), |ui| {
        ui.linear_layout(LayoutDirection::Vertical, |ui| {
            ui.label("good", Rgb888::WHITE);
            ui.label("friend", Rgb888::BLUE);
        });
    });

    ui.finish()
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(2)
        .max_fps(60)
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
        demo_ui().draw(&mut ui_ctx, rect);

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
