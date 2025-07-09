use edgy::{
    margin,
    themes::{self, DynamicStyle, Theme, WidgetStyle},
    widgets::{
        button::{Button, ButtonGeneric}, linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder}, UiBuilder, WidgetObject
    },
    SystemEvent, UiContext,
};
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::{
    mono_font::{iso_8859_5::FONT_5X8},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
    text::{self},
};
use embedded_graphics_simulator::{sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, Window};


use std::sync::OnceLock;

const fn color_u32_to_rgb(color: u32) -> Rgb888 {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    Rgb888::new(r, g, b)
}

pub const FOREGROUND_COLOR: Rgb888 = Rgb888::WHITE;
pub const FOREGROUND2_COLOR: Rgb888 = color_u32_to_rgb(0x363636);
pub const FOREGROUND3_COLOR: Rgb888 = color_u32_to_rgb(0x555555);
pub const FOREGROUND4_COLOR: Rgb888 = color_u32_to_rgb(0x767676);

pub const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;
pub const BACKGROUND2_COLOR: Rgb888 = color_u32_to_rgb(0xdcdcdc);
pub const BACKGROUND3_COLOR: Rgb888 = color_u32_to_rgb(0xb9b9b9);
pub const BACKGROUND4_COLOR: Rgb888 = color_u32_to_rgb(0x979797);

pub const FAULT_COLOR: Rgb888 = color_u32_to_rgb(0xdc0000);
pub const WARN_COLOR: Rgb888 = color_u32_to_rgb(0xffb900);
pub const ACCENT_COLOR: Rgb888 = color_u32_to_rgb(0x004597);

const BASE_BUTTON: WidgetStyle<Rgb888> = WidgetStyle::new()
    .background_color(BACKGROUND2_COLOR)
    .storke(1, BACKGROUND3_COLOR)
    .foreground_color(FOREGROUND_COLOR)
    .accent_color(ACCENT_COLOR);

pub const MENU_BUTTON_STYLE: DynamicStyle<Rgb888> = DynamicStyle {
    active: BASE_BUTTON.background_color(FOREGROUND3_COLOR),
    drag: BASE_BUTTON.background_color(BACKGROUND2_COLOR),
    focus: BASE_BUTTON.background_color(BACKGROUND3_COLOR),
    idle: BASE_BUTTON,
};

pub const PANEL_STYLE: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .fill_color(BACKGROUND_COLOR)
    .stroke_color(BACKGROUND2_COLOR)
    .stroke_width(1)
    .build();

static STYLE: OnceLock<Theme<Rgb888>> = OnceLock::new();

pub fn theme() -> Theme<Rgb888> {
    *STYLE.get_or_init(|| {
        let mut theme = themes::hope_diamond::apply();
        theme.label_color = FOREGROUND_COLOR;
        theme
    })
}

pub fn demo_ui<'a, D>() -> WidgetObject<'a, D, Rgb888>
where
    D: DrawTarget<Color = Rgb888> + 'a,
{
    let mut ui = LinearLayoutBuilder::default()
        .horizontal_alignment(LayoutAlignment::Stretch)
        .vertical_alignment(LayoutAlignment::Stretch)
        .direction(LayoutDirection::Vertical);

    let seven_segment_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(12, 32))
        .segment_width(2)
        .digit_spacing(2)
        .segment_color(Rgb888::WHITE)
        .inactive_segment_color(Rgb888::new(10, 5, 10))
        .build();

    ui.horizontal_linear_layout(LayoutAlignment::Stretch, |ui| {
        ui.margin_layout_styled(margin!(5), PANEL_STYLE, |ui| {
            let mut layout = LinearLayoutBuilder::default()
                .direction(LayoutDirection::Vertical)
                .horizontal_alignment(LayoutAlignment::Stretch);

            layout.label("ТЕМПЕРАТУРА C", text::Alignment::Left, &FONT_5X8);
            layout.seven_segment(format!("37.51"), seven_segment_style);
            layout.label(
                format!("Цель: {} C", 37.0),
                text::Alignment::Left,
                &FONT_5X8,
            );

            ui.add_widget_obj(layout.finish());
        });

        ui.margin_layout_styled(margin!(5), PANEL_STYLE, |ui| {
            let mut layout = LinearLayoutBuilder::default()
                .direction(LayoutDirection::Vertical)
                .horizontal_alignment(LayoutAlignment::Stretch);

            layout.label("О. ВЛАЖНОСТЬ %", text::Alignment::Left, &FONT_5X8);
            layout.seven_segment(format!("0"), seven_segment_style);
            layout.label(format!("Цель: 100%"), text::Alignment::Left, &FONT_5X8);

            ui.add_widget_obj(layout.finish());
        });
    });

    let style = ButtonGeneric::new(
                &FONT_5X8,
                text::Alignment::Left,
                MENU_BUTTON_STYLE,
                6
            );

    ui.add_widget(Button::new_styled("тест".to_string(), style, Box::new(|| {

    })));

    ui.finish()
}

fn main() -> Result<(), core::convert::Infallible> {
    let display = SimulatorDisplay::<Rgb888>::new(Size::new(160, 128));

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(2)
        .build();

    let mut window = Window::new("a bit edgy ui", &output_settings);
    let mut ui_ctx = UiContext::new(display, themes::hope_diamond::apply());

    loop {
        window.update(&mut ui_ctx.draw_target);

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
                        ui_ctx.toggle_debug_mode();
                    }
                }
                _ => {}
            }
        }

        ui_ctx.draw_target.clear(Rgb888::BLACK)?;
        ui_ctx.update(demo_ui());
    }
}
