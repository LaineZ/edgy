use alloc::string::String;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::Rectangle,
    text::{renderer::TextRenderer, Alignment, Text},
};

use crate::{EventResult, UiContext};
use super::{Widget, WidgetEvent};

/// Re-export of type [SevenSegmentStyle] from [eg_seven_segment]
pub use eg_seven_segment::SevenSegmentStyle as SevenSegmentStyle;
/// Re-export of type [SevenSegmentStyleBuilder] from [eg_seven_segment]
pub use eg_seven_segment::SevenSegmentStyleBuilder as SevenSegmentStyleBuilder;

/// Seven segment widget basically a widigitized [eg_seven_segment] library
pub struct SevenSegmentWidget<C: PixelColor> {
    text: String,
    style: SevenSegmentStyle<C>,
}

impl<C> SevenSegmentWidget<C>
where
    C: PixelColor,
{
    pub fn new(text: String, style: SevenSegmentStyle<C>) -> Self {
        Self {
            text,
            style,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for SevenSegmentWidget<C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        let mut total_width = 0;
        let mut total_height = 0;
        let line_spacing = self.style.line_height() / 2;
    
        for line in self.text.lines() {
            let line_rect = self
                .style
                .measure_string(line, Point::zero(), embedded_graphics::text::Baseline::Top)
                .bounding_box;
    
            total_width = total_width.max(line_rect.size.width);
            total_height += line_rect.size.height + line_spacing;
        }
    
        Size::new(total_width, total_height)
    }

       fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        _event_args: WidgetEvent,
    ) -> EventResult {
        let mut position = rect.top_left;
        position.y += self.style.digit_size.height as i32;
        let text = Text::new(&self.text, position, self.style);
        let _ = text.draw(context.draw_target);
        EventResult::Pass
    }
}


/// Label widget
pub struct Label<'a, C: PixelColor> {
    text: String,
    style: MonoTextStyle<'a, C>,
    alignment: Alignment,
}

impl<'a, C> Label<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(text: String, alignment: Alignment, style: MonoTextStyle<'a, C>) -> Self {
        Self {
            text,
            alignment,
            style,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Label<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        let mut total_width = 0;
        let mut total_height = 0;
        let line_spacing = self.style.line_height() / 2;
    
        for line in self.text.lines() {
            let line_rect = self
                .style
                .measure_string(line, Point::zero(), embedded_graphics::text::Baseline::Top)
                .bounding_box;
    
            total_width = total_width.max(line_rect.size.width);
            total_height += line_rect.size.height + line_spacing;
        }
    
        Size::new(total_width, total_height)
    }

       fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        _event_args: WidgetEvent,
    ) -> EventResult {
        let mut position = rect.top_left;

        match self.alignment {
            Alignment::Left => {
                // do nothing, layout already draws from left
            }
            Alignment::Center => {
                position.x = rect.center().x;
            }
            Alignment::Right => {
                position.x += rect.size.width as i32;
            }
        }

        position.y += self.style.font.character_size.height as i32;
        let text = Text::with_alignment(&self.text, position, self.style, self.alignment);
        let _ = text.draw(context.draw_target);
        EventResult::Pass
    }
}
