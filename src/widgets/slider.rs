use core::marker::PhantomData;

use alloc::{boxed::Box, string::String};
use embedded_graphics::{
    mono_font::MonoFont,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Alignment,
};

use super::{Widget, WidgetEvent};
use crate::{
    themes::{NoneStyle, Style},
    Event, EventResult, SystemEvent, UiContext,
};

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
    fn size(&mut self, context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        Size::new(
            hint.width,
            self.style.track_height.max(self.style.handle_size.height),
        )
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
        let handle_style = self.style.handle_style.style(event_args.event);
        let track_style = self.style.track_style.style(event_args.event);

        let track_rect = Rectangle::new(
            rect.top_left,
            Size::new(rect.size.width, self.style.track_height),
        );

        let _ = track_rect
            // пиздец компилер лох. даже ТАКУЮ ПРОСТУЮ ВЕЩЬ как вычислить тип примитива не смог.....
            .into_styled::<PrimitiveStyle<C>>(track_style.into())
            .draw(context.draw_target);

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
        .draw(context.draw_target);

        let event_result = match event_args.event {
            Event::Active(Some(position)) => {
                self.pos_to_value(rect, *position);
                (self.callback)(self.value);
                EventResult::Stop
            }

            Event::Increase(step) => {
                self.value += step;
                EventResult::Stop
            }

            Event::Decrease(step) => {
                self.value -= step;
                EventResult::Stop
            }

            Event::Drag(position) => {
                self.pos_to_value(rect, *position);
                (self.callback)(self.value);
                EventResult::Stop
            }
            _ => EventResult::Pass,
        };
        event_result
    }
}

#[cfg(test)]
mod tests {
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565};
    use crate::themes::hope_diamond::{self, DefaultButtonStyle};
    use super::*;

    #[test]
    fn slider_size() {

        let mut display = MockDisplay::new();
        let mut ctx = UiContext::new(&mut display, hope_diamond::apply());

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
