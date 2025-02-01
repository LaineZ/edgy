use std::u32;

use edgy::widgets::linear_layout::LayoutAlignment;
use edgy::Theme;
use edgy::{
    widgets::{
        linear_layout::{LayoutDirection, LinearLayoutBuilder},
        UiBuilder, WidgetObj,
    },
    UiContext,
};
use embedded_graphics::mono_font::iso_8859_5::FONT_5X7;
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

pub enum Pages {
    PFD = 0,
    Engine = 1,
}

pub fn demo_ui<'a, D>(theme: Theme<Rgb888>, page: Pages) -> WidgetObj<'a, D, Rgb888>
where
    D: DrawTarget<Color = Rgb888> + 'a,
{
    let style = MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE);
    let mut ui = LinearLayoutBuilder::default()
        .aligment(LayoutAlignment::End)
        .direction(LayoutDirection::Vertical);

    let mut menu_layout = LinearLayoutBuilder::default()
        .aligment(LayoutAlignment::Stretch)
        .direction(LayoutDirection::Horizontal)
        .max_size(Size::new(u32::MAX, 40));
    menu_layout.button("PFD", theme, &FONT_5X7, move || todo!());
    menu_layout.button("ENG", theme, &FONT_5X7, move || todo!());

    match page {
        Pages::PFD => {
            ui.linear_layout(LayoutDirection::Vertical, LayoutAlignment::Start, |ui| {
                ui.linear_layout(LayoutDirection::Vertical, LayoutAlignment::Start, |ui| {
                    ui.label("альтитуд-хуитьюд", style);
                    ui.label("скорость-хуёрость", style);
                    ui.label("вертикалка", style);
                });
            });
        }
        Pages::Engine => {
            ui.linear_layout(
                LayoutDirection::Vertical,
                LayoutAlignment::Stretch,
                |ui| {
                    ui.label("эрпики", style);
                    ui.linear_layout(LayoutDirection::Horizontal, LayoutAlignment::Center, |ui| {
                        ui.button("START ENG", theme, &FONT_5X7, move || todo!());
                    });
                },
            );
        }
    };

    ui.add_widget_obj(menu_layout.finish());


    ui.finish()
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(3)
        .max_fps(60)
        .build();
    let mut window = Window::new("a bit edgy ui", &output_settings);
    let debug_text_style = MonoTextStyle::new(&FONT_4X6, Rgb888::RED);

    loop {
        let frame_render = std::time::Instant::now();
        window.update(&display);
        display.clear(Rgb888::BLACK)?;

        let rect = Rectangle::new(Point::new(0, 0), display.size());
        let ui_context_render = std::time::Instant::now();
        let mut ui_ctx = UiContext::new(&mut display, rect, Theme::hope_diamond());
        demo_ui(ui_ctx.theme, Pages::PFD).draw(&mut ui_ctx, rect);

        let seconds_ui = ui_context_render.elapsed().as_millis();
        Text::new(
            &format!("ui: {} ms", seconds_ui),
            Point::new(0, 10),
            debug_text_style,
        )
        .draw(&mut display)?;

        let seconds = frame_render.elapsed().as_secs_f32();
        Text::new(
            &format!("{:.0} fps", 1.0 / seconds),
            Point::new(0, 5),
            debug_text_style,
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
