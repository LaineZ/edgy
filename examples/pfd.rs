use edgy::widgets::grid_layout::GridLayoutBuilder;
use edgy::widgets::linear_layout::LayoutAlignment;
use edgy::{
    widgets::{
        linear_layout::{LayoutDirection, LinearLayoutBuilder},
        UiBuilder, WidgetObj,
    },
    UiContext,
};
use edgy::{Event, Theme};
use embedded_graphics::mono_font::iso_8859_5::FONT_5X7;
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_graphics_simulator::sdl2::Keycode;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pages {
    PFD = 0,
    Engine = 1,
}

pub fn demo_ui<'a, D>(theme: Theme<Rgb888>, page: Pages) -> WidgetObj<'a, D, Rgb888>
where
    D: DrawTarget<Color = Rgb888> + 'a,
{
    let style = MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE);
    let mut ui = GridLayoutBuilder::default()
        .add_column(100)
        .add_row(90)
        .add_row(10);

    let mut menu_layout = LinearLayoutBuilder::default()
        .aligment(LayoutAlignment::Stretch)
        .direction(LayoutDirection::Horizontal);
    menu_layout.button("PFD", theme, &FONT_5X7, move || todo!());
    menu_layout.button("ENG", theme, &FONT_5X7, move || todo!());

    match page {
        Pages::PFD => {
            ui.linear_layout(LayoutDirection::Vertical, LayoutAlignment::Start, |ui| {
                ui.linear_layout(LayoutDirection::Vertical, LayoutAlignment::Start, |ui| {
                    ui.label("ALTITUDE", style);
                    ui.label("SPEED", style);
                    ui.label("VSPEED", style);
                });
            });
        }
        Pages::Engine => {
            ui.linear_layout(LayoutDirection::Vertical, LayoutAlignment::Stretch, |ui| {
                ui.label("RPM", style);
                ui.label("MIXTURE", style);
                ui.linear_layout(
                    LayoutDirection::Horizontal,
                    LayoutAlignment::Stretch,
                    |ui| {
                        ui.button("BAT 1", theme, &FONT_5X7, move || todo!());
                        ui.button("BAT 2", theme, &FONT_5X7, move || todo!());
                        ui.button("ALTN 1", theme, &FONT_5X7, move || todo!());
                        ui.button("ALTN 2", theme, &FONT_5X7, move || todo!());
                    },
                );
                ui.linear_layout(
                    LayoutDirection::Horizontal,
                    LayoutAlignment::Stretch,
                    |ui| {
                        ui.button("MAGNETO", theme, &FONT_5X7, move || todo!());
                        ui.button("STARTER", theme, &FONT_5X7, move || todo!());
                    },
                );
            });
        }
    };

    ui.add_widget_obj(menu_layout.finish());

    ui.finish()
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(4)
        .max_fps(60)
        .build();
    let mut window = Window::new("a bit edgy ui", &output_settings);
    let debug_text_style = MonoTextStyle::new(&FONT_4X6, Rgb888::BLUE);
    let rect = Rectangle::new(Point::new(0, 0), display.size());
    let mut ui_ctx = UiContext::new(&mut display, rect, Theme::hope_diamond());
    let mut page = Pages::Engine;
    let mut mouse_pos = Point::new(0, 20);

    loop {
        let frame_render = std::time::Instant::now();
        window.update(&ui_ctx.draw_target);
        ui_ctx.draw_target.clear(Rgb888::BLACK)?;
        ui_ctx.last_event = Event::Hover(mouse_pos);


        for event in window.events() {
            match event {
                embedded_graphics_simulator::SimulatorEvent::Quit => {
                    std::process::exit(0);
                }
                embedded_graphics_simulator::SimulatorEvent::MouseButtonDown {
                    mouse_btn,
                    point,
                } => ui_ctx.last_event = Event::Active(point),
                embedded_graphics_simulator::SimulatorEvent::MouseMove { point } => {
                    mouse_pos = point;
                }
                embedded_graphics_simulator::SimulatorEvent::KeyDown {
                    keycode,
                    keymod,
                    repeat,
                } => {
                    if keycode == Keycode::F1 {
                        ui_ctx.debug_mode = !ui_ctx.debug_mode;
                    }

                    if keycode == Keycode::F2 {
                        if page == Pages::PFD {
                            page = Pages::Engine;
                        } else {
                            page = Pages::PFD;
                        }
                    }
                }
                _ => {}
            }
        }

        let ui_context_render = std::time::Instant::now();
        ui_ctx.update(&mut demo_ui(ui_ctx.theme, page));
        let seconds_ui = ui_context_render.elapsed().as_secs_f32();

        if ui_ctx.debug_mode {
            Text::new(
                &format!(
                    "edgy: {:.0} fps {:.1} ms",
                    1.0 / seconds_ui,
                    seconds_ui * 1000.0
                ),
                Point::new(2, 10),
                debug_text_style,
            )
            .draw(ui_ctx.draw_target)?;

            let seconds = frame_render.elapsed().as_secs_f32();
            Text::new(
                &format!("simu: {:.0} fps {:.1} ms", 1.0 / seconds, seconds * 1000.0),
                Point::new(2, 5),
                debug_text_style,
            )
            .draw(ui_ctx.draw_target)?;
        }
    }
}
