use std::u32;

use crate::{
    prelude::LayoutDirection,
    themes::WidgetStyle,
    widgets::{Widget, WidgetEvent},
    EventResult, UiContext,
};
use embedded_graphics::{
    prelude::{DrawTarget, PixelColor, Point, Size},
    primitives::{PrimitiveStyle, Rectangle, StrokeAlignment, StyledDrawable},
};

// TODO: Terminal size setting
pub struct BatteryStyle<C: PixelColor> {
    pub style: WidgetStyle<C>,
    pub direction: LayoutDirection,
}

impl<C: PixelColor> BatteryStyle<C> {
    pub fn new(style: WidgetStyle<C>, direction: LayoutDirection) -> Self {
        Self { style, direction }
    }
}

/// Battery indicator widget, represents some kind of battery
pub struct Battery<C: PixelColor> {
    /// Charge percentage 0-100
    pub charge_percentage: u8,
    /// Battery charge status
    pub charging: bool,
    size: Size,
    style: BatteryStyle<C>,
}

impl<'a, C> Battery<C>
where
    C: PixelColor + 'a,
{
    pub fn new(charge_percentage: u8, charging: bool, size: Size, style: BatteryStyle<C>) -> Self {
        Self {
            charge_percentage,
            charging,
            size: size.clamp(Size::new(5, 3), Size::new(u32::MAX, u32::MAX)),
            style,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Battery<C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        self.size
    }

    fn min_size(&mut self) -> Size {
        self.size
    }

    fn max_size(&mut self) -> Size {
        self.size
    }

    fn draw(
        &mut self,
        context: &mut crate::UiContext<'a, D, C>,
        rect: Rectangle,
        _event_args: WidgetEvent,
    ) -> EventResult {
        match self.style.direction {
            LayoutDirection::Horizontal => {
                let terminal_width = self.style.style.stroke_width;
                let terminal_height: u32 = if (rect.size.height as i32 / 2) & 1 == 0 {
                    rect.size.height / 2
                } else {
                    rect.size.height / 2 + 1
                };

                let battery = Rectangle::new(
                    rect.top_left,
                    Size::new(rect.size.width - terminal_width, rect.size.height),
                );

                // terminal
                let terminal_y =
                    battery.top_left.y + (battery.size.height as i32 - terminal_height as i32) / 2;
                let battery_termianl = Rectangle::new(
                    Point::new(
                        battery.top_left.x + rect.size.width as i32 - terminal_width as i32,
                        terminal_y,
                    ),
                    Size::new(terminal_width, terminal_height),
                );

                let battery_terminal_style =
            PrimitiveStyle::with_fill(self.style.style.stroke_color.unwrap_or(
                self.style.style.background_color.expect(
                    "Battery widget requires either stroke color or background color for drawing",
                ),
            ));
                // battery background
                let mut style: PrimitiveStyle<C> = self.style.style.into();
                style.stroke_alignment = StrokeAlignment::Inside;

                let _ = battery.draw_styled(&style, &mut context.draw_target);
                let _ =
                    battery_termianl.draw_styled(&battery_terminal_style, &mut context.draw_target);

                // charge rect

                let max_width = battery.size.width - style.stroke_width * 2;
                let clamped_charge = self.charge_percentage.clamp(0, 100) as u32;
                let fill_width = max_width * clamped_charge / 100;

                let charge_rect = Rectangle::new(
                    Point::new(
                        battery.top_left.x + style.stroke_width as i32,
                        battery.top_left.y + style.stroke_width as i32,
                    ),
                    Size::new(fill_width, battery.size.height - style.stroke_width * 2),
                );

                let color = if self.charging {
                    self.style.style.foreground_color.unwrap()
                } else {
                    self.style.style.accent_color.unwrap()
                };

                let _ = charge_rect
                    .draw_styled(&PrimitiveStyle::with_fill(color), &mut context.draw_target);
            }
            LayoutDirection::Vertical => todo!(),
        }
        EventResult::Pass
    }
}

#[cfg(test)]
mod tests {
    use crate::themes::WidgetStyle;
    use crate::widgets::battery::{Battery, BatteryStyle};
    use crate::widgets::linear_layout::LinearLayoutBuilder;
    use crate::SystemEvent;
    use crate::{prelude::*, themes::hope_diamond, UiContext};
    use embedded_graphics::geometry::OriginDimensions;
    use embedded_graphics::prelude::{Point, RgbColor, Size};
    use embedded_graphics::primitives::Rectangle;
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};

    const BATTERY_STYLE: WidgetStyle<Rgb888> = WidgetStyle::new()
        .background_color(Rgb888::WHITE)
        .foreground_color(Rgb888::RED)
        .accent_color(Rgb888::RED);

    #[test]
    fn battery_small_terminal_uneven() {
        let mut display = MockDisplay::<Rgb888>::new();
        let disp_size = display.size();
        display.set_allow_overdraw(true);
        let mut ctx = UiContext::new(display, hope_diamond::apply());

        let mut ui = LinearLayoutBuilder::default()
            .horizontal_alignment(LayoutAlignment::Start)
            .vertical_alignment(LayoutAlignment::Start)
            .direction(LayoutDirection::Vertical);

        ui.add_widget(Battery::new(
            50,
            false,
            Size::new(7, 3),
            BatteryStyle::new(BATTERY_STYLE, LayoutDirection::Horizontal),
        ));
        let mut ui = ui.finish();

        ui.size(&mut ctx, disp_size);
        ui.layout(&mut ctx, Rectangle::new(Point::zero(), disp_size));
        ui.draw(&mut ctx, &SystemEvent::Idle);

        assert_eq!(
            ctx.draw_target.get_pixel(Point::new(6, 1)),
            Some(Rgb888::WHITE)
        );
    }

    #[test]
    fn battery_small_terminal_even() {
        let mut display = MockDisplay::<Rgb888>::new();
        let disp_size = display.size();
        display.set_allow_overdraw(true);
        let mut ctx = UiContext::new(display, hope_diamond::apply());

        let mut ui = LinearLayoutBuilder::default()
            .horizontal_alignment(LayoutAlignment::Start)
            .vertical_alignment(LayoutAlignment::Start)
            .direction(LayoutDirection::Vertical);

        ui.add_widget(Battery::new(
            50,
            false,
            Size::new(7, 3),
            BatteryStyle::new(BATTERY_STYLE, LayoutDirection::Horizontal),
        ));
        let mut ui = ui.finish();

        ui.size(&mut ctx, disp_size);
        ui.layout(&mut ctx, Rectangle::new(Point::zero(), disp_size));
        ui.draw(&mut ctx, &SystemEvent::Idle);

        assert_eq!(ctx.draw_target.get_pixel(Point::new(12, 1)), None);
        assert_eq!(ctx.draw_target.get_pixel(Point::new(12, 6)), None);
    }
}
