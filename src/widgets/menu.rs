use crate::{
    margin,
    themes::DynamicStyle,
    widgets::{margin_layout::Margin, WidgetEvent},
    Event, EventResult, UiContext,
};

use super::Widget;
use alloc::vec::Vec;
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, StrokeAlignment, StyledDrawable},
    text::{renderer::TextRenderer, Baseline, Text},
};

pub struct MenuEntryStyle<'a, C: PixelColor> {
    pub padding: Margin,
    pub font: &'a MonoFont<'a>,
    style: Option<DynamicStyle<C>>,
}

impl<'a, C: PixelColor> MenuEntryStyle<'a, C> {
    pub fn new(font: &'a MonoFont<'a>) -> Self {
        Self {
            padding: margin!(0),
            font,
            style: None,
        }
    }

    pub fn new_with_style(font: &'a MonoFont<'a>, style: DynamicStyle<C>) -> Self {
        Self {
            padding: margin!(0),
            font,
            style: Some(style),
        }
    }

    pub fn new_with_padding(
        padding: Margin,
        font: &'a MonoFont<'a>,
        style: DynamicStyle<C>,
    ) -> Self {
        Self {
            padding,
            font,
            style: Some(style),
        }
    }

    fn get_font_style(&self, event: &Event) -> MonoTextStyle<'a, C> {
        MonoTextStyle::new(
            self.font,
            self.style
                .expect("No style was set")
                .style(event)
                .foreground_color
                .expect("Foreground color is needed for drawing menu entry!"),
        )
    }
}

pub struct Menu<'a, P: AsRef<str> + Eq, C: PixelColor> {
    entries: Vec<P>,
    selected: P,
    style: MenuEntryStyle<'a, C>,
}

impl<'a, P: AsRef<str> + Eq, C: PixelColor> Menu<'a, P, C> {
    pub fn new(entries: Vec<P>, selected: P, style: MenuEntryStyle<'a, C>) -> Self {
        Self {
            entries,
            selected,
            style,
        }
    }
}

impl<'a, D, C, P> Widget<'a, D, C> for Menu<'a, P, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
    P: AsRef<str> + Eq + 'a,
{
    fn is_interactive(&mut self) -> bool {
        true
    }

    fn size(&mut self, context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        if self.style.style.is_none() {
            self.style.style = Some(context.theme.button_style);
        }
        hint
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
    ) -> EventResult {
        let mut y_offset = 0;
        for entry in self.entries.iter() {
            let text_height = self
                .style
                .get_font_style(event_args.event)
                .measure_string(entry.as_ref(), rect.top_left, Baseline::Top)
                .bounding_box
                .size
                .height;

            let mut style: PrimitiveStyle<C> =
                self.style.style.unwrap().style(event_args.event).into();

            style.stroke_alignment = StrokeAlignment::Inside;

            let rect_background = Rectangle::new(
                Point::new(rect.top_left.x, rect.top_left.y + y_offset),
                Size::new(rect.size.width, text_height + style.stroke_width * 2),
            );

            let _ = rect_background.draw_styled(&style.into(), &mut context.draw_target);
            let _ = Text::new(
                entry.as_ref(),
                Point::new(
                    rect_background.top_left.x + style.stroke_width as i32,
                    rect_background.center().y + style.stroke_width as i32,
                ),
                self.style.get_font_style(event_args.event),
            )
            .draw(&mut context.draw_target);

            y_offset += rect_background.size.height as i32;
        }

        EventResult::Pass
    }
}
