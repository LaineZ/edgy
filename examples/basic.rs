use edgy::widgets::margin::Margin;

use edgy::Theme;
use edgy::{
    widgets::{
        linear_layout::{LayoutDirection, LinearLayoutBuilder},
        UiBuilder, WidgetObj,
    },
    UiContext,
};
use embedded_graphics::mono_font::iso_8859_10::FONT_10X20;
use embedded_graphics::primitives::{PrimitiveStyle, PrimitiveStyleBuilder};
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
    let style = MonoTextStyle::new(&FONT_4X6, Rgb888::WHITE);
    let mut style2 = MonoTextStyle::new(&FONT_10X20, Rgb888::CSS_WHEAT);
    let mut button_style = PrimitiveStyleBuilder::new().fill_color(Rgb888::BLUE);
    let mut ui = LinearLayoutBuilder {
        direction: LayoutDirection::VerticalFill,
        children: Vec::new(),
    };
    ui.linear_layout(LayoutDirection::HorizontalFill, |ui| {
        ui.label("hello", style);
        ui.label("world", style);
        ui.button("click me", button_style.build(), style2, move || {
            panic!("ты сдох")
        });
        ui.button("click me", button_style.build(), style2, move || {
            panic!("ты сдох")
        });
        ui.label("goodbye...", style);
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
        let mut ui_ctx = UiContext::new(&mut display, rect, Theme::hope_diamond());
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
