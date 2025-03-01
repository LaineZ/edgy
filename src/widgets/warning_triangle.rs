use core::u32;

use crate::UiContext;

use super::Widget;
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    prelude::*,
    primitives::{self, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
};

/// Warning triangle...
pub struct WarningTriangle {
    min_size: Size,
    max_size: Size,
}

impl WarningTriangle {
    pub fn new(min_size: Size) -> Self {
        Self {
            min_size,
            max_size: Size::new(u32::MAX, u32::MAX),
        }
    }

    pub fn new_both_sizes(min_size: Size, max_size: Size) -> Self {
        Self { min_size, max_size }
    }
}

impl<'a, D, C> Widget<'a, D, C> for WarningTriangle
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        self.min_size
    }

    fn max_size(&mut self) -> Size {
        self.max_size
    }

    fn min_size(&mut self) -> Size {
        self.min_size
    }

    fn draw(&mut self, context: &mut crate::UiContext<'a, D, C>, rect: Rectangle) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(context.theme.warning)
            .build();

        let _ = primitives::Triangle::new(
            Point::new(rect.center().x, rect.top_left.y),
            Point::new(rect.top_left.x, rect.top_left.y + rect.size.height as i32),
            Point::new(
                rect.top_left.x + rect.size.width as i32,
                rect.top_left.y + rect.size.height as i32,
            ),
        )
        .into_styled(style)
        .draw(context.draw_target);

        let _ = Text::with_alignment(
            "cyka",
            Point::new(rect.center().x, rect.center().y + 6),
            MonoTextStyle::new(&FONT_4X6, context.theme.foreground),
            Alignment::Center
        )
        .draw(context.draw_target);
    }
}
