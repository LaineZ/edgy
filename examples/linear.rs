use edgy::{
    margin, widgets::{
        linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder},
        UiBuilder, WidgetObj,
    }, SystemEvent, Theme, UiContext
};
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::{
    mono_font::{ascii::FONT_5X8, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
    text::{self},
};
use embedded_graphics_simulator::{sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, Window};

const PANEL_STYLE: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .fill_color(Rgb888::new(21, 14, 16))
    .stroke_color(Rgb888::new(39, 39, 57))
    .stroke_width(1)
    .build();

pub fn demo_ui<'a, D>(theme: Theme<Rgb888>) -> WidgetObj<'a, D, Rgb888>
where
    D: DrawTarget<Color = Rgb888> + 'a,
{
    let mut layout_main = LinearLayoutBuilder::default()
        .horizontal_alignment(LayoutAlignment::Stretch)
        .vertical_alignment(LayoutAlignment::Stretch)
        .direction(LayoutDirection::Vertical);

    let seven_segment_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(16, 24))
        .segment_width(2)
        .digit_spacing(4)
        .segment_color(Rgb888::WHITE)
        .inactive_segment_color(Rgb888::new(10, 5, 10))
        .build();

    layout_main.margin_layout_styled(margin!(5), PANEL_STYLE, |ui| {
        // ui.vertical_linear_layout(LayoutAlignment::Start, |ui| {
        //     ui.label(
        //         "TEMPERATURE C",
        //         text::Alignment::Left,
        //         MonoTextStyle::new(&FONT_5X8, theme.foreground),
        //     );
        //     ui.seven_segment("25.0", seven_segment_style);
        // });
        ui.button("TEST", &FONT_5X8, || {});
    });

    layout_main.button("TEST", &FONT_5X8, || {});

    layout_main.margin_layout_styled(margin!(5), PANEL_STYLE, |ui| {
        ui.vertical_linear_layout(LayoutAlignment::Start, |ui| {
            ui.label(
                "R. HUMIDITY %",
                text::Alignment::Left,
                MonoTextStyle::new(&FONT_5X8, theme.foreground),
            );
            ui.seven_segment("100", seven_segment_style);
        });
    });

    layout_main.finish()
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(160, 128));

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(2)
        .build();

    let mut window = Window::new("a bit edgy ui", &output_settings);
    let mut ui_ctx = UiContext::new(&mut display, Theme::hope_diamond());

    loop {
        window.update(&ui_ctx.draw_target);

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
                    if keycode == Keycode::F1 {
                        ui_ctx.debug_mode = !ui_ctx.debug_mode;
                    }
                }
                _ => {}
            }
        }

        ui_ctx.draw_target.clear(Rgb888::BLACK)?;
        ui_ctx.update(&mut demo_ui(ui_ctx.theme));
    }
}
