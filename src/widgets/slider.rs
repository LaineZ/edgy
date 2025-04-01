
use alloc::boxed::Box;
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use super::{Widget, WidgetEvent};
use crate::{themes::DynamicStyle, Event, EventResult, SystemEvent, UiContext};

#[derive(Clone, Copy, Default)]
pub struct SliderStyle<C: PixelColor> {
    pub track_style: DynamicStyle<C>,
    pub handle_style: DynamicStyle<C>,
    pub track_height: u32,
    pub handle_size: Size,
}

impl<C: PixelColor> SliderStyle<C> {
    pub fn new(
        track_style: DynamicStyle<C>,
        handle_style: DynamicStyle<C>,
        track_height: u32,
        handle_size: Size,
    ) -> Self {
        Self {
            track_style,
            handle_style,
            track_height,
            handle_size,
        }
    }
}

/// Slider
pub struct Slider<'a, C: PixelColor> {
    value: f32,
    callback: Box<dyn FnMut(f32) + 'a>,
    style: Option<SliderStyle<C>>,
}

impl<'a, C> Slider<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(value: f32, callback: Box<dyn FnMut(f32) + 'a>) -> Self {
        Self {
            value,
            callback,
            style: None,
        }
    }

    pub fn new_with_style(
        style: SliderStyle<C>,
        value: f32,
        callback: Box<dyn FnMut(f32) + 'a>,
    ) -> Self {
        Self {
            value,
            callback,
            style: Some(style),
        }
    }

    fn pos_to_value(&mut self, rect: Rectangle, position: Point) {
        let relative_pos = (position.x - rect.top_left.x) as f32 / rect.size.width as f32;
        self.value = relative_pos;
    }
}

impl<'a, D, C> Widget<'a, D, C> for Slider<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        let style = self.style.get_or_insert(context.theme.slider_style);

        Size::new(hint.width, style.track_height.max(style.handle_size.height))
    }

    fn is_interactive(&mut self) -> bool {
        true
    }

    fn max_size(&mut self) -> Size {
        let style = self.style.unwrap();
        Size::new(u32::MAX, style.handle_size.height + 2)
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
    ) -> EventResult {
        let style = self.style.get_or_insert(context.theme.slider_style);

        let handle_style = style.handle_style.style(event_args.event);
        let track_style = style.track_style.style(event_args.event);

        let track_rect = Rectangle::new(
            Point::new(
                rect.top_left.x,
                rect.top_left.y + style.handle_size.height as i32 - (style.handle_size.height / 2) as i32,
            ),
            Size::new(rect.size.width, style.track_height),
        );

        let _ = track_rect
            // пиздец компилер лох. даже ТАКУЮ ПРОСТУЮ ВЕЩЬ как вычислить тип примитива не смог.....
            .into_styled::<PrimitiveStyle<C>>(track_style.into())
            .draw(&mut context.draw_target);

        let handle_position_x = rect.top_left.x + (rect.size.width as f32 * self.value) as i32;
        let _ = Rectangle::new(
            Point::new(
                handle_position_x,
                track_rect.center().y - (style.handle_size.height as i32 / 2),
            ),
            style.handle_size,
        )
        // пиздец компилер лох. даже ТАКУЮ ПРОСТУЮ ВЕЩЬ как вычислить тип примитива не смог.....
        .into_styled::<PrimitiveStyle<C>>(handle_style.into())
        .draw(&mut context.draw_target);

        if event_args.is_focused {
            if let Some(color) = style.handle_style.base().accent_color {
                let _ = Rectangle::new(
                    Point::new(
                        track_rect.top_left.x,
                        track_rect.center().y - style.track_height as i32 - 2,
                    ),
                    Size::new(rect.size.width, style.handle_size.height + 2),
                )
                .into_styled(PrimitiveStyle::with_stroke(color, 1))
                .draw(&mut context.draw_target);
            }

            match event_args.system_event {
                SystemEvent::Increase(step) => {
                    self.value += step;
                    (self.callback)(self.value);
                }

                SystemEvent::Decrease(step) => {
                    self.value -= step;
                    (self.callback)(self.value);
                }

                _ => {}
            }
        }

        match event_args.event {
            Event::Active(Some(position)) => {
                context.focused_element = event_args.id;
                self.pos_to_value(rect, *position);
                (self.callback)(self.value);
                EventResult::Stop
            }

            Event::Drag(position) => {
                context.focused_element = event_args.id;
                self.pos_to_value(rect, *position);
                (self.callback)(self.value);
                EventResult::Stop
            }
            _ => EventResult::Pass,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::hope_diamond::{self};
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565};

    #[test]
    fn slider_size() {
        let display = MockDisplay::<Rgb565>::new();
        let mut ctx = UiContext::new(display, hope_diamond::apply());

        let style = SliderStyle::<Rgb565>::new(
            DynamicStyle::default(),
            DynamicStyle::default(),
            1,
            Size::new(1, 5),
        );
        let slider_size = Slider::new_with_style(style,0.1, Box::new(|_| {})).size(&mut ctx, Size::new(10, 10));

        assert_eq!(slider_size.width, 10);
        assert_eq!(slider_size.height, 5);
    }
}
