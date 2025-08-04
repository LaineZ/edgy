use alloc::string::String;
use embedded_graphics::{
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    prelude::*,
    primitives::Rectangle,
    text::{renderer::TextRenderer, Alignment, Baseline, Text, TextStyleBuilder},
};

use super::{Widget, WidgetEvent};
use crate::{style::{Part, SelectorKind}, EventResult, UiContext};

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
    pub fn new<S: Into<String>>(text: S, style: SevenSegmentStyle<C>) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for SevenSegmentWidget<C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(
        &mut self,
        _context: &mut UiContext<'a, D, C>,
        _hint: Size,
        _selectors: &[SelectorKind<'a>],
    ) -> Size {
        let mut total_width = 0;
        let mut total_height = 0;

        for line in self.text.lines() {
            let line_rect = self
                .style
                .measure_string(line, Point::zero(), Baseline::Top)
                .bounding_box;

            total_width = total_width.max(line_rect.size.width);
            total_height += line_rect.size.height;
        }

        Size::new(total_width, total_height)
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        _event_args: WidgetEvent,
        selectors: &[SelectorKind<'a>],
    ) -> EventResult {
        let text = Text::with_baseline(&self.text, rect.top_left, self.style, Baseline::Top);
        let _ = text.draw(&mut context.draw_target);
        EventResult::Pass
    }
}

/// Label widget
pub struct Label<'a, C: PixelColor> {
    text: String,
    style: MonoTextStyle<'a, C>,
}

impl<'a, C> Label<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            style: MonoTextStyleBuilder::new().build(),
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Label<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        _hint: Size,
        selectors: &[SelectorKind<'a>]
    ) -> Size {
        let resolved_style = context.resolve_style_static(selectors, Part::Main);
        let font = resolved_style.font.unwrap();
        let text_style = MonoTextStyle::new(font, resolved_style.color.unwrap());
        let line_height = resolved_style
            .line_height
            .unwrap_or(text_style.line_height());

        if self.text.is_empty() {
            return Size::zero();
        }

        let mut total_width = 0;
        let mut total_height = 0;
        let line_count = self.text.lines().into_iter().count();

        let line_spacing = if line_count > 1 { line_height } else { 0 };

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
        selectors: &[SelectorKind<'a>],
    ) -> EventResult {
        let resolved_style = context.resolve_style_static(selectors, Part::Main);
        let mut position = rect.top_left;
        let alignment = resolved_style.text_alignment.unwrap_or(Alignment::Left);

        match alignment {
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
                .alignment(alignment)
                .baseline(Baseline::Top)
                .build(),
        );
        let _ = text.draw(&mut context.draw_target);
        EventResult::Pass
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::{
    //     prelude::*,
    //     style::{resolve_style, Modifier, Selector, SelectorKind, Part, StyleRule, Tag},
    //     styles::{apply_default_debug_style, hope_diamond::HOPE_DIAMOND},
    //     widgets::linear_layout::LinearLayoutBuilder,
    //     SystemEvent,
    // };
    // use embedded_graphics::{
    //     mock_display::MockDisplay,
    //     mono_font::ascii::FONT_10X20,
    //     pixelcolor::Rgb888,
    // };

    // #[test]
    // fn single_line_size() {
    //     const TEST_STYLE: Style<'static, Rgb888> = Style {
    //         color: Some(Rgb888::WHITE),
    //         font: Some(&FONT_10X20),
    //         text_alignment: Some(Alignment::Center),
    //         ..Style::default()
    //     };

    //     let display = MockDisplay::<Rgb888>::new();
    //     let mut ctx = UiContext::new(display, HOPE_DIAMOND.to_vec(), apply_default_debug_style());

    //     let label_size =
    //         Label::new("DISPLAYING BEE!").size(&mut ctx, Size::new(320, 320), &TEST_STYLE);

    //     assert_eq!(label_size.width, 150);
    //     assert_eq!(label_size.height, 20);
    // }

    // #[test]
    // fn multiline_size() {
    //     let display = MockDisplay::<Rgb888>::new();
    //     let mut ctx = UiContext::new(display, HOPE_DIAMOND.to_vec(), apply_default_debug_style());
    //     let styleshit = resolve_style(
    //         &[SelectorKind::Tag(Tag::Label)],
    //         &ctx.stylesheet,
    //         Modifier::None,
    //         Part::Main
    //     );

    //     let label_size = Label::new(
    //         "At the heart is ocelot-brain - basically OpenComputers\nbut untied from Minecraft and packaged as a Scala library.\nThis makes Ocelot Desktop the most accurate emulator ever made.",
    //     ).size(&mut ctx, Size::new(320, 320), &styleshit);
    //     assert_eq!(label_size.width, 252);
    //     assert_eq!(label_size.height, 18);
    // }

    // #[test]
    // fn empty_label_size() {
    //     let display = MockDisplay::<Rgb888>::new();
    //     let mut ctx = UiContext::new(display, HOPE_DIAMOND.to_vec(), apply_default_debug_style());
    //     const TEST_STYLE: Style<'static, Rgb888> = Style {
    //         color: Some(Rgb888::WHITE),
    //         font: Some(&FONT_10X20),
    //         text_alignment: Some(Alignment::Left),
    //         ..Style::default()
    //     };

    //     let size = Label::new("").size(&mut ctx, Size::new(320, 240), &TEST_STYLE);

    //     assert_eq!(size.width, 0);
    //     assert_eq!(size.height, 0);
    // }

    // #[test]
    // fn center_alignment_draws_in_bounds() {
    //     let display = MockDisplay::<Rgb888>::new();
    //     let disp_size = display.size();
    //     let mut styles = HOPE_DIAMOND.to_vec();

    //     styles.push(StyleRule::new(
    //         Selector {
    //             modifier: Modifier::None,
    //             part: Part::Main,
    //             kind: SelectorKind::Id("label"),
    //         },
    //         Style {
    //             font: Some(&FONT_10X20),
    //             ..Style::default()
    //         },
    //     ));
    //     let mut ctx = UiContext::new(display, styles, apply_default_debug_style());

    //     let mut ui = LinearLayoutBuilder::default()
    //         .horizontal_alignment(LayoutAlignment::Center)
    //         .vertical_alignment(LayoutAlignment::Center)
    //         .direction(LayoutDirection::Vertical);

    //     ui.add_widget(Label::new("text"), &[SelectorKind::Id("label")]);
    //     let mut ui = ui.finish(&[]);

    //     ui.size(&mut ctx, disp_size);
    //     ui.layout(&mut ctx, Rectangle::new(Point::zero(), disp_size));
    //     ui.draw(&mut ctx, &SystemEvent::Idle);

    //     assert_eq!(ctx.draw_target.get_pixel(Point::new(0, 32)), None);
    // }
}
