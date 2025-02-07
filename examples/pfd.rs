use std::cell::RefCell;
use std::mem;

use edgy::widgets::grid_layout::GridLayoutBuilder;
use edgy::widgets::linear_layout::LayoutAlignment;
use edgy::{
    widgets::{
        linear_layout::{LayoutDirection, LinearLayoutBuilder},
        UiBuilder, WidgetObj,
    },
    UiContext,
};
use edgy::{SystemEvent, Theme};
use embedded_graphics::mono_font::iso_8859_5::FONT_5X7;
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};
use embedded_graphics_simulator::sdl2::Keycode;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pages {
    PFD = 0,
    Engine = 1,
}

impl From<u8> for Pages {
    fn from(value: u8) -> Self {
        unsafe { mem::transmute(value) }
    }
}

pub struct UiState {
    page: Pages,
    engine: bool,
    magneto: bool,
    battery1: bool,
    battery2: bool,
    alternator1: bool,
    alternator2: bool,
}

impl UiState {
    fn cycle_page(&mut self) {
        let current_page_index = self.page as u8;
        if current_page_index <= 0 {
            self.page = Pages::from((current_page_index + 1).clamp(0, 1));
        } else {
            self.page = Pages::PFD;
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            page: Pages::PFD,
            engine: false,
            magneto: false,
            battery1: false,
            battery2: false,
            alternator1: false,
            alternator2: false,
        }
    }
}

pub fn demo_ui<'a, D>(state: &'a RefCell<&'a mut UiState>) -> WidgetObj<'a, D, Rgb888>
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
    menu_layout.toggle_button(
        "PFD",
        &FONT_5X7,
        state.borrow().page == Pages::PFD,
        move |_| {
            state.borrow_mut().page = Pages::PFD;
        },
    );
    menu_layout.toggle_button(
        "ENGINE",
        &FONT_5X7,
        state.borrow().page == Pages::Engine,
        move |_| {
            state.borrow_mut().page = Pages::Engine;
        },
    );
    match state.borrow().page {
        Pages::PFD => {
            ui.vertical_linear_layout(LayoutAlignment::Start, |ui| {
                ui.vertical_linear_layout(LayoutAlignment::Start, |ui| {
                    ui.label("ALTITUDE", style);
                    ui.label("SPEED", style);
                    ui.label("VSPEED", style);
                });
            });
        }
        Pages::Engine => {
            ui.grid_layout([50, 50].into(), [100].into(), |ui| {
                ui.vertical_linear_layout(LayoutAlignment::Stretch, |ui| {
                    if state.borrow().engine {
                        ui.label("engine started", style);
                    }
                });

                ui.vertical_linear_layout(LayoutAlignment::Stretch, |ui| {
                    ui.horizontal_linear_layout(LayoutAlignment::Stretch, |ui| {
                        ui.toggle_button(
                            "BAT 1",
                            &FONT_5X7,
                            state.borrow().battery1,
                            move |value| {
                                state.borrow_mut().battery1 = value;
                            },
                        );
                        ui.toggle_button(
                            "BAT 2",
                            &FONT_5X7,
                            state.borrow().battery2,
                            move |value| {
                                state.borrow_mut().battery2 = value;
                            },
                        );
                        ui.toggle_button(
                            "ALTN 1",
                            &FONT_5X7,
                            state.borrow().alternator1,
                            move |value| {
                                state.borrow_mut().alternator1 = value;
                            },
                        );
                        ui.toggle_button(
                            "ALTN 2",
                            &FONT_5X7,
                            state.borrow().alternator2,
                            move |value| {
                                state.borrow_mut().alternator2 = value;
                            },
                        );
                    });
                    ui.horizontal_linear_layout(LayoutAlignment::Stretch, |ui| {
                        ui.toggle_button(
                            "MAGNETO",
                            &FONT_5X7,
                            state.borrow().magneto,
                            move |value| {
                                state.borrow_mut().magneto = value;
                            },
                        );
                        ui.button("STARTER", &FONT_5X7, move || todo!());
                    });
                });
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
        .scale(3)
        .max_fps(60)
        .build();
    let mut window = Window::new("a bit edgy ui", &output_settings);
    let debug_text_style = MonoTextStyle::new(&FONT_4X6, Rgb888::BLUE);

    let mut ui_ctx = UiContext::new(&mut display, Theme::hope_diamond());
    let mut default_state = UiState::default();
    let state = &RefCell::new(&mut default_state);

    loop {
        let frame_render = std::time::Instant::now();
        window.update(&ui_ctx.draw_target);
        ui_ctx.draw_target.clear(Rgb888::BLACK)?;

        for event in window.events() {
            match event {
                embedded_graphics_simulator::SimulatorEvent::Quit => {
                    std::process::exit(0);
                }
                embedded_graphics_simulator::SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point,
                } => ui_ctx.push_event(SystemEvent::Active(point)),
                embedded_graphics_simulator::SimulatorEvent::MouseMove { point } => {
                    ui_ctx.push_event(SystemEvent::Move(point));
                }
                embedded_graphics_simulator::SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat: _,
                } => {
                    if keycode == Keycode::Tab {
                        ui_ctx.next_widget();
                    }

                    if keycode == Keycode::Return {
                        ui_ctx.activate_selected_widget();
                    }

                    if keycode == Keycode::F1 {
                        ui_ctx.debug_mode = !ui_ctx.debug_mode;
                    }

                    if keycode == Keycode::F2 {
                        state.borrow_mut().cycle_page();
                    }
                }
                _ => {}
            }
        }

        let ui_context_render = std::time::Instant::now();
        ui_ctx.update(&mut demo_ui(state));
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
