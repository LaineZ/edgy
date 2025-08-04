use crate::{
    EventResult, UiContext,
    prelude::LayoutDirection,
    style::{Part, SelectorKind, Style},
    widgets::{Widget, WidgetEvent},
};
use embedded_graphics::{
    prelude::{DrawTarget, PixelColor, Point, Size},
    primitives::{PrimitiveStyle, Rectangle, StrokeAlignment, StyledDrawable},
};

// TODO: Terminal size setting
pub struct BatteryStyle {
    pub direction: LayoutDirection,
}

impl BatteryStyle {
    pub fn new(direction: LayoutDirection) -> Self {
        Self { direction }
    }
}

/// Battery indicator widget, represents some kind of battery
pub struct Battery {
    /// Charge percentage 0-100
    pub charge_percentage: u8,
    /// Battery charge status
    pub charging: bool,
    size: Size,
    style: BatteryStyle,
}

impl Battery {
    pub fn new(charge_percentage: u8, charging: bool, size: Size, style: BatteryStyle) -> Self {
        Self {
            charge_percentage,
            charging,
            size: size.clamp(Size::new(5, 3), Size::new(u32::MAX, u32::MAX)),
            style,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Battery
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(
        &mut self,
        _context: &mut UiContext<'a, D, C>,
        _hint: Size,
        selectors: &[SelectorKind<'a>],
    ) -> Size {
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
        selectors: &[SelectorKind<'a>],
    ) -> EventResult {
        let resolved_style = context.resolve_style_static(selectors, Part::Main);

        match self.style.direction {
            LayoutDirection::Horizontal => {
                let terminal_width = resolved_style.stroke_width.unwrap_or(0);
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
                PrimitiveStyle::with_fill(resolved_style.stroke_color.unwrap_or(
                resolved_style.background_color.expect(
                    "Battery widget requires either stroke color or background color for drawing",
                ),
                ));
                // battery background
                let mut style: PrimitiveStyle<C> = resolved_style.primitive_style();
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
                    resolved_style.color.unwrap()
                } else {
                    resolved_style.accent_color.unwrap()
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
    // TODO: Tests
}
