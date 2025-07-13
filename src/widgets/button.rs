use alloc::{boxed::Box, string::String};
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, StrokeAlignment},
    text::{renderer::TextRenderer, Alignment, Baseline, Text, TextStyleBuilder},
};

use crate::{themes::DynamicStyle, Event, EventResult, UiContext};

use super::{Widget, WidgetEvent};

/// Generic button style and drawing implementation
#[derive(Clone, Copy)]
pub struct ButtonGeneric<'a, C: PixelColor> {
    text_style: Option<MonoTextStyle<'a, C>>,
    font: &'a MonoFont<'a>,
    text_alignment: Alignment,
    pub padding: u32,
    pub style: DynamicStyle<C>,
}

impl<'a, C> ButtonGeneric<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(
        font: &'a MonoFont,
        text_alignment: Alignment,
        style: DynamicStyle<C>,
        padding: u32,
    ) -> Self {
        Self {
            font,
            style,
            padding: padding,
            text_alignment,
            text_style: None,
        }
    }

    pub fn size(&mut self, text: &str) -> Size {
        let base_style = self.style.style(&Event::Idle);

        self.text_style = Some(MonoTextStyle::new(
            self.font,
            base_style
                .foreground_color
                .expect("Button must have a foreground color for drawing"),
        ));

        let text_size = self
            .text_style
            .unwrap()
            .measure_string(text, Point::zero(), embedded_graphics::text::Baseline::Top)
            .bounding_box
            .size;

        Size::new(
            text_size.width + 2 * self.padding,
            text_size.height + 2 * self.padding,
        )
    }

    pub fn draw<D: DrawTarget<Color = C>>(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event: &Event,
        text: &str,
    ) {
        const TEXT_BASELINE: Baseline = Baseline::Middle;
        let mut converted_style: PrimitiveStyle<C> = self.style.style(event).into();
        converted_style.stroke_alignment = StrokeAlignment::Inside;
        let styled_rect = rect.into_styled(converted_style);
        let _ = styled_rect.draw(&mut context.draw_target);

        if let Some(style) = self.text_style {
            let text = match self.text_alignment {
                Alignment::Left => Text::with_baseline(
                    text,
                    Point::new(rect.top_left.x + self.padding as i32, rect.center().y),
                    style,
                    TEXT_BASELINE,
                ),
                Alignment::Center => {
                    let text_style = TextStyleBuilder::new()
                        .alignment(self.text_alignment)
                        .baseline(TEXT_BASELINE);
                    Text::with_text_style(text, rect.center(), style, text_style.build())
                }
                Alignment::Right => {
                    let text_width = text.len() as i32 * style.font.character_size.width as i32;
                    let x_pos =
                        rect.top_left.x + rect.size.width as i32 - text_width - self.padding as i32;
                    Text::with_baseline(
                        text,
                        Point::new(x_pos, rect.center().y),
                        style,
                        TEXT_BASELINE,
                    )
                }
            };

            let _ = text.draw(&mut context.draw_target);
        }
    }
}

/// Button widget
pub struct Button<'a, C: PixelColor> {
    base: ButtonGeneric<'a, C>,
    text: String,
    callback: Box<dyn FnMut() + 'a>,
}

impl<'a, C> Button<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new_styled(
        text: String,
        style: ButtonGeneric<'a, C>,
        callback: Box<dyn FnMut() + 'a>,
    ) -> Self {
        Self {
            base: style,
            text,
            callback,
        }
    }

    pub fn new(text: String, font: &'a MonoFont, callback: Box<dyn FnMut() + 'a>) -> Self {
        Self {
            base: ButtonGeneric::new(
                font,
                Alignment::Center,
                DynamicStyle {
                    active: Default::default(),
                    drag: Default::default(),
                    focus: Default::default(),
                    idle: Default::default(),
                },
                6,
            ),
            text,
            callback,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Button<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        let style = self.base.style.style(&Event::Idle);
        if style.foreground_color.is_none() && style.background_color.is_none() {
            self.base.style = context.theme.button_style;
        }

        self.base.size(&self.text)
    }

    fn is_interactive(&mut self) -> bool {
        true
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
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

        self.base.draw(context, rect, event_args.event, &self.text);
        event_result
    }
}

#[cfg(test)]
mod tests {
    use crate::widgets::linear_layout::LinearLayoutBuilder;
    use crate::SystemEvent;
    use crate::{prelude::*, themes::hope_diamond, UiContext};
    use embedded_graphics::geometry::OriginDimensions;
    use embedded_graphics::mono_font::ascii::FONT_4X6;
    use embedded_graphics::prelude::Point;
    use embedded_graphics::primitives::Rectangle;
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};

    #[test]
    fn button_render() {
        let mut display = MockDisplay::<Rgb888>::new();
        let disp_size = display.size();
        display.set_allow_overdraw(true);
        let mut ctx = UiContext::new(display, hope_diamond::apply());

        let mut ui = LinearLayoutBuilder::default()
            .horizontal_alignment(LayoutAlignment::Center)
            .vertical_alignment(LayoutAlignment::Center)
            .direction(LayoutDirection::Vertical);

        ui.button("pidor", &FONT_4X6, || {});
        let mut ui = ui.finish();

        ui.size(&mut ctx, disp_size);
        ui.layout(&mut ctx, Rectangle::new(Point::zero(), disp_size));
        ui.draw(&mut ctx, &SystemEvent::Idle);

        assert_eq!(
            ctx.draw_target.get_pixel(Point::new(22, 28)),
            ctx.theme.button_style.idle.background_color
        );
    }
}
