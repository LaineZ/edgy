use alloc::{boxed::Box, string::String};
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, StrokeAlignment},
    text::{renderer::TextRenderer, Alignment, Baseline, Text, TextStyleBuilder},
};

use crate::{
    style::{Style, NULL_FONT},
    themes::DynamicStyle,
    Event, EventResult, UiContext,
};

use super::{Widget, WidgetEvent};

/// Generic button style and drawing implementation
#[derive(Clone, Copy)]
pub struct ButtonGeneric {
    text_size: Size,
}

impl ButtonGeneric {
    pub fn new() -> Self {
        Self {
            text_size: Size::zero(),
        }
    }

    pub fn size<'a, C: PixelColor>(&mut self, text: &str, resolved_style: &Style<'a, C>) -> Size {
        let text_style: MonoTextStyle<'a, C> = resolved_style.character_style();
        self.text_size = text_style
            .measure_string(text, Point::zero(), embedded_graphics::text::Baseline::Top)
            .bounding_box
            .size;

        Size::new(
            self.text_size.width + 2 * resolved_style.padding.unwrap_or_default(),
            self.text_size.height + 2 * resolved_style.padding.unwrap_or_default(),
        )
    }

    pub fn draw<'a, C: PixelColor, D: DrawTarget<Color = C>>(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        resolved_style: &Style<'a, C>,
        text: &str,
    ) {
        const TEXT_BASELINE: Baseline = Baseline::Middle;
        let converted_style = resolved_style.primitive_style();
        let character_style = resolved_style.character_style();

        let styled_rect = rect.into_styled(converted_style);
        let _ = styled_rect.draw(&mut context.draw_target);

        let text = match resolved_style.text_alignment.unwrap_or(Alignment::Left) {
            Alignment::Left => Text::with_baseline(
                text,
                Point::new(
                    rect.top_left.x + resolved_style.padding.unwrap_or_default() as i32,
                    rect.center().y,
                ),
                character_style,
                TEXT_BASELINE,
            ),
            Alignment::Center => {
                let text_style = TextStyleBuilder::new()
                    .alignment(resolved_style.text_alignment.unwrap_or(Alignment::Left))
                    .baseline(TEXT_BASELINE);
                Text::with_text_style(text, rect.center(), character_style, text_style.build())
            }
            Alignment::Right => {
                let text_width = self.text_size.width as i32;
                let x_pos = rect.top_left.x + rect.size.width as i32
                    - text_width
                    - resolved_style.padding.unwrap_or_default() as i32;
                Text::with_baseline(
                    text,
                    Point::new(x_pos, rect.center().y),
                    character_style,
                    TEXT_BASELINE,
                )
            }
        };

        let _ = text.draw(&mut context.draw_target);
    }
}

/// Button widget
pub struct Button<'a> {
    base: ButtonGeneric,
    text: String,
    callback: Box<dyn FnMut() + 'a>,
}

impl<'a> Button<'a> {
    pub fn new(text: String, callback: Box<dyn FnMut() + 'a>) -> Self {
        Self {
            base: ButtonGeneric::new(),
            text,
            callback,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Button<'a>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(
        &mut self,
        _context: &mut UiContext<'a, D, C>,
        _hint: Size,
        resolved_style: &Style<'a, C>,
    ) -> Size {
        self.base.size(&self.text, resolved_style)
    }

    fn is_interactive(&mut self) -> bool {
        true
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
        resolved_style: &Style<'a, C>,
    ) -> EventResult {
        let event_result = match event_args.event {
            Event::Focus => EventResult::Stop,
            Event::Active(_) | Event::Drag(_) => {
                context.focused_element = event_args.id;
                (self.callback)();
                EventResult::Stop
            }
            _ => EventResult::Pass,
        };

        self.base.draw(context, rect, resolved_style, &self.text);
        event_result
    }
}

#[cfg(test)]
mod tests {
    use crate::styles::apply_default_debug_style;
    use crate::styles::hope_diamond::{HOPE_DIAMOND, HOPE_DIAMOND_COLOR_BACKGROUND};
    use crate::widgets::linear_layout::LinearLayoutBuilder;
    use crate::SystemEvent;
    use crate::{prelude::*, UiContext};
    use embedded_graphics::geometry::OriginDimensions;
    use embedded_graphics::prelude::Point;
    use embedded_graphics::primitives::Rectangle;
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};

    #[test]
    fn button_render() {
        let mut display = MockDisplay::<Rgb888>::new();
        let disp_size = display.size();
        display.set_allow_overdraw(true);
        let mut ctx = UiContext::new(display, HOPE_DIAMOND.to_vec(), apply_default_debug_style());

        let mut ui = LinearLayoutBuilder::default()
            .horizontal_alignment(LayoutAlignment::Center)
            .vertical_alignment(LayoutAlignment::Center)
            .direction(LayoutDirection::Vertical);

        ui.button("pidor", || {});
        let mut ui = ui.finish(&[]);

        ui.size(&mut ctx, disp_size);
        ui.layout(&mut ctx, Rectangle::new(Point::zero(), disp_size));
        ui.draw(&mut ctx, &SystemEvent::Idle);

        assert_eq!(
            ctx.draw_target.get_pixel(Point::new(22, 28)),
            Some(HOPE_DIAMOND_COLOR_BACKGROUND)
        );
    }
}
