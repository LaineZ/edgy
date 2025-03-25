use core::marker::PhantomData;
use std::u32;

use alloc::boxed::Box;
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use super::{Widget, WidgetEvent};
use crate::{themes::Style, Event, EventResult, SystemEvent, UiContext};

#[derive(Clone, Copy)]
pub struct SliderStyle<C: PixelColor, TrackStyle: Style<C>, HandleStyle: Style<C>> {
    pub track_style: TrackStyle,
    pub handle_style: HandleStyle,
    pub track_height: u32,
    pub handle_size: Size,
    pub data: PhantomData<C>,
}

impl<C: PixelColor, TrackStyle: Style<C>, HandleStyle: Style<C>>
    SliderStyle<C, TrackStyle, HandleStyle>
{
    pub fn new(
        track_style: TrackStyle,
        handle_style: HandleStyle,
        track_height: u32,
        handle_size: Size,
    ) -> Self {
        Self {
            track_style,
            handle_style,
            track_height,
            handle_size,
            data: PhantomData,
        }
    }
}

/// Slider
pub struct Slider<'a, C: PixelColor, TrackStyle: Style<C>, HandleStyle: Style<C>> {
    value: f32,
    callback: Box<dyn FnMut(f32) + 'a>,
    style: SliderStyle<C, TrackStyle, HandleStyle>,
}

impl<'a, C, TrackStyle, HandleStyle> Slider<'a, C, TrackStyle, HandleStyle>
where
    C: PixelColor + 'a,
    TrackStyle: Style<C>,
    HandleStyle: Style<C>,
{
    pub fn new(
        value: f32,
        callback: Box<dyn FnMut(f32) + 'a>,
        style: SliderStyle<C, TrackStyle, HandleStyle>,
    ) -> Self {
        Self {
            value,
            callback,
            style,
        }
    }

    fn pos_to_value(&mut self, rect: Rectangle, position: Point) {
        let relative_pos = (position.x - rect.top_left.x) as f32 / rect.size.width as f32;
        self.value = relative_pos;
    }
}

impl<'a, D, C, TrackStyle, HandleStyle> Widget<'a, D, C> for Slider<'a, C, TrackStyle, HandleStyle>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
    TrackStyle: Style<C> + 'a,
    HandleStyle: Style<C> + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        Size::new(
            hint.width,
            self.style.track_height.max(self.style.handle_size.height),
        )
    }

    fn is_interactive(&mut self) -> bool {
        true
    }

    fn max_size(&mut self) -> Size {
        Size::new(u32::MAX, self.style.handle_size.height)
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
    ) -> EventResult {
        let handle_style = self.style.handle_style.style(event_args.event);
        let track_style = self.style.track_style.style(event_args.event);

        let track_rect = Rectangle::new(
            Point::new(
                rect.top_left.x,
                rect.top_left.y + self.style.handle_size.height as i32,
            ),
            Size::new(rect.size.width, self.style.track_height),
        );

        let _ = track_rect
            // пиздец компилер лох. даже ТАКУЮ ПРОСТУЮ ВЕЩЬ как вычислить тип примитива не смог.....
            .into_styled::<PrimitiveStyle<C>>(track_style.into())
            .draw(&mut context.draw_target);

        let handle_position_x = rect.top_left.x + (rect.size.width as f32 * self.value) as i32;
        let _ = Rectangle::new(
            Point::new(
                handle_position_x,
                track_rect.center().y - (self.style.handle_size.height as i32 / 2),
            ),
            self.style.handle_size,
        )
        // пиздец компилер лох. даже ТАКУЮ ПРОСТУЮ ВЕЩЬ как вычислить тип примитива не смог.....
        .into_styled::<PrimitiveStyle<C>>(handle_style.into())
        .draw(&mut context.draw_target);

        if event_args.is_focused {
            if let Some(color) = self.style.handle_style.base().accent_color {
                let _ = Rectangle::new(
                    Point::new(track_rect.top_left.x, track_rect.center().y - self.style.track_height as i32 - 2),
                    Size::new(rect.size.width, self.style.handle_size.height + 2),
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
    use crate::themes::hope_diamond::{self, DefaultButtonStyle};
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565};

    #[test]
    fn slider_size() {
        let display = MockDisplay::new();
        let mut ctx = UiContext::new(display, hope_diamond::apply());

        let style = SliderStyle::<Rgb565, DefaultButtonStyle, DefaultButtonStyle>::new(
            DefaultButtonStyle,
            DefaultButtonStyle,
            1,
            Size::new(1, 5),
        );
        let slider = Slider::new(0.1, Box::new(|_| {}), style).size(&mut ctx, Size::new(10, 10));

        assert_eq!(slider.height, 5);
    }
}
