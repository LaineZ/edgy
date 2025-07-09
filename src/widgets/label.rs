use alloc::string::String;
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    prelude::*,
    primitives::Rectangle,
    text::{renderer::TextRenderer, Alignment, Baseline, Text, TextStyleBuilder},
};

use super::{Widget, WidgetEvent};
use crate::{EventResult, UiContext};

/// Re-export of type [SevenSegmentStyle] from [eg_seven_segment]
pub use eg_seven_segment::SevenSegmentStyle;
/// Re-export of type [SevenSegmentStyleBuilder] from [eg_seven_segment]
pub use eg_seven_segment::SevenSegmentStyleBuilder;

/// Seven segment widget. Basically a "widigitized" [eg_seven_segment] library
pub struct SevenSegmentWidget<C: PixelColor> {
    text: String,
    style: SevenSegmentStyle<C>,
}

impl<C> SevenSegmentWidget<C>
where
    C: PixelColor,
{
    pub fn new(text: String, style: SevenSegmentStyle<C>) -> Self {
        Self { text, style }
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

        for line in self.text.lines() {
            let line_rect = self
                .style
                .measure_string(
                    line,
                    Point::zero(),
                    embedded_graphics::text::Baseline::Bottom,
                )
                .bounding_box;

            total_width = total_width.max(line_rect.size.width);
            total_height += line_rect.size.height + self.style.segment_width;
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
        let _ = text.draw(&mut context.draw_target);
        EventResult::Pass
    }
}

/// Advanced label format options
#[derive(Clone, Copy)]
pub struct LabelOptions {
    /// Horizontal alignment for label
    pub alignment: Alignment,
    // Line height, left `None`` for auto-computation
    pub line_height: Option<u32>,
}

impl Default for LabelOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl LabelOptions {
    pub fn new() -> Self {
        Self {
            alignment: Alignment::Left,
            line_height: None,
        }
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn line_height(mut self, height: u32) -> Self {
        self.line_height = Some(height);
        self
    }
}

impl From<Alignment> for LabelOptions {
    fn from(value: Alignment) -> Self {
        Self {
            alignment: value,
            ..Self::new()
        }
    }
}

/// Label widget
pub struct Label<'a, C: PixelColor> {
    text: String,
    style: MonoTextStyle<'a, C>,
    options: LabelOptions,
}

impl<'a, C> Label<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new<S: Into<String>>(text: S, options: LabelOptions, font: &'a MonoFont) -> Self {
        Self {
            text: text.into(),
            options,
            style: MonoTextStyleBuilder::new().font(font).build(),
        }
    }

    pub fn new_with_style<S: Into<String>>(
        text: S,
        options: LabelOptions,
        style: MonoTextStyle<'a, C>,
    ) -> Self {
        Self {
            text: text.into(),
            options,
            style,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Label<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        if self.style.text_color.is_none() {
            self.style.text_color = Some(context.theme.label_color);
        }

        if self.text.is_empty() {
            return Size::zero();
        }

        let mut total_width = 0;
        let mut total_height = 0;

        let line_count = self.text.lines().into_iter().count();

        let line_spacing = if line_count > 1 {
            self.options.line_height.unwrap_or(self.style.line_height()) / 2
        } else {
            0
        };

        if line_count > 1 {
            // multiline case
            for (i, line) in self.text.lines().into_iter().enumerate() {
                let line_rect = self
                    .style
                    .measure_string(line, Point::zero(), embedded_graphics::text::Baseline::Top)
                    .bounding_box;

                total_width = total_width.max(line_rect.size.width);

                // do not count the last line, because this creates a bottom padding in the text and in general is very bad thing...
                if i != line_count - 1 {
                    total_height += line_rect.size.height + line_spacing;
                }
            }
        } else {
            // single line case
            let text_rect = self
                .style
                .measure_string(
                    &self.text,
                    Point::zero(),
                    embedded_graphics::text::Baseline::Top,
                )
                .bounding_box;
            total_height = text_rect.size.height;
            total_width = text_rect.size.width;
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

        match self.options.alignment {
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

        //position.y += self.style.font.character_size.height as i32;
        let text = Text::with_text_style(
            &self.text,
            position,
            self.style,
            TextStyleBuilder::new()
                .alignment(self.options.alignment)
                .baseline(Baseline::Top)
                .build(),
        );
        let _ = text.draw(&mut context.draw_target);
        EventResult::Pass
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        prelude::*,
        themes::hope_diamond::{self},
        widgets::linear_layout::LinearLayoutBuilder,
        SystemEvent,
    };
    use embedded_graphics::{
        mock_display::MockDisplay,
        mono_font::ascii::{FONT_10X20, FONT_4X6},
        pixelcolor::Rgb888,
    };

    #[test]
    fn single_line_size() {
        let display = MockDisplay::<Rgb888>::new();
        let mut ctx = UiContext::new(display, hope_diamond::apply());

        let label_size = Label::new(
            "DISPLAYING BEE!",
            LabelOptions::from(Alignment::Center),
            &FONT_10X20,
        )
        .size(&mut ctx, Size::new(320, 320));

        assert_eq!(label_size.width, 150);
        assert_eq!(label_size.height, 20);
    }

    #[test]
    fn multiline_size() {
        let display = MockDisplay::<Rgb888>::new();
        let mut ctx = UiContext::new(display, hope_diamond::apply());

        let label_size = Label::new(
            "At the heart is ocelot-brain - basically OpenComputers\nbut untied from Minecraft and packaged as a Scala library.\nThis makes Ocelot Desktop the most accurate emulator ever made.",
            LabelOptions::from(Alignment::Left),
            &FONT_4X6,
        )
        .size(&mut ctx, Size::new(320, 320));

        assert_eq!(label_size.width, 252);
        assert_eq!(label_size.height, 18);
    }

    #[test]
    fn empty_label_size() {
        let display = MockDisplay::<Rgb888>::new();
        let mut ctx = UiContext::new(display, hope_diamond::apply());

        let size = Label::new("", LabelOptions::from(Alignment::Left), &FONT_10X20)
            .size(&mut ctx, Size::new(320, 240));

        assert_eq!(size.width, 0);
        assert_eq!(size.height, 0);
    }

    #[test]
    fn center_alignment_draws_in_bounds() {
        let display = MockDisplay::<Rgb888>::new();
        let disp_size = display.size();
        let mut ctx = UiContext::new(display, hope_diamond::apply());

        let mut ui = LinearLayoutBuilder::default()
            .horizontal_alignment(LayoutAlignment::Center)
            .vertical_alignment(LayoutAlignment::Center)
            .direction(LayoutDirection::Vertical);

        ui.add_widget(Label::new(
            "text",
            LabelOptions::from(Alignment::Center),
            &FONT_10X20,
        ));
        let mut ui = ui.finish();

        ui.size(&mut ctx, disp_size);
        ui.layout(&mut ctx, Rectangle::new(Point::zero(), disp_size));
        ui.draw(&mut ctx, &SystemEvent::Idle);

        assert_eq!(ctx.draw_target.get_pixel(Point::new(0, 32)), None);
    }
}
