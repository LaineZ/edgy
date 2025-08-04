use alloc::boxed::Box;
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use super::{Widget, WidgetEvent};
use crate::{
    Event, EventResult, SystemEvent, UiContext,
    style::{Part, SelectorKind},
};

#[derive(Clone, Copy)]
pub struct SliderDimensions {
    pub track_height: u32,
    pub handle_size: Size,
}

impl Default for SliderDimensions {
    fn default() -> Self {
        Self {
            handle_size: Size::new(4, 8),
            track_height: 4,
        }
    }
}

impl SliderDimensions {
    pub fn new(track_height: u32, handle_size: Size) -> Self {
        Self {
            track_height,
            handle_size,
        }
    }
}

/// Slider
pub struct Slider<'a> {
    value: f32,
    callback: Box<dyn FnMut(f32) + 'a>,
    slider_dimensions: SliderDimensions,
}

impl<'a> Slider<'a> {
    pub fn new(
        value: f32,
        slider_dimensions: SliderDimensions,
        callback: Box<dyn FnMut(f32) + 'a>,
    ) -> Self {
        Self {
            value,
            callback,
            slider_dimensions,
        }
    }

    fn pos_to_value(&mut self, rect: Rectangle, position: Point) {
        let relative_pos = (position.x - rect.top_left.x) as f32 / rect.size.width as f32;
        self.value = relative_pos;
    }
}

impl<'a, D, C> Widget<'a, D, C> for Slider<'a>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(
        &mut self,
        _context: &mut UiContext<'a, D, C>,
        hint: Size,
        _selectors: &[SelectorKind<'a>],
    ) -> Size {
        Size::new(
            hint.width,
            self.slider_dimensions
                .track_height
                .max(self.slider_dimensions.handle_size.height)
                + 2,
        )
    }

    fn is_interactive(&mut self) -> bool {
        true
    }

    fn max_size(&mut self) -> Size {
        Size::new(u32::MAX, self.slider_dimensions.handle_size.height + 2)
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
        selectors: &[SelectorKind<'a>],
    ) -> EventResult {
        let handle_style =
            context.resolve_style(selectors, event_args.get_modifier(), Part::SliderHandle);
        let track_style =
            context.resolve_style(selectors, event_args.get_modifier(), Part::SliderTrack);

        let track_rect = Rectangle::new(
            Point::new(
                rect.top_left.x,
                rect.top_left.y + self.slider_dimensions.handle_size.height as i32
                    - (self.slider_dimensions.handle_size.height / 2) as i32,
            ),
            Size::new(rect.size.width, self.slider_dimensions.track_height),
        );

        let _ = track_rect
            .into_styled(track_style.primitive_style())
            .draw(&mut context.draw_target);

        let handle_position_x = rect.top_left.x
            + ((rect.size.width - self.slider_dimensions.handle_size.width) as f32 * self.value) as i32;
        let _ = Rectangle::new(
            Point::new(
                handle_position_x,
                track_rect.center().y - (self.slider_dimensions.handle_size.height as i32 / 2),
            ),
            self.slider_dimensions.handle_size,
        )
        .into_styled::<PrimitiveStyle<C>>(handle_style.primitive_style())
        .draw(&mut context.draw_target);

        if event_args.is_focused {
            if let Some(color) = handle_style.accent_color {
                let _ = Rectangle::new(
                    Point::new(
                        track_rect.top_left.x,
                        track_rect.center().y - self.slider_dimensions.track_height as i32 - 2,
                    ),
                    Size::new(rect.size.width, self.slider_dimensions.handle_size.height + 2),
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
    // use super::*;
    // use crate::themes::hope_diamond::{self};
    // use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565};

    // #[test]
    // fn slider_size() {
    //     let display = MockDisplay::<Rgb565>::new();
    //     let mut ctx = UiContext::new(display, HOPE_DIAMOND.to_vec());

    //     let style = SliderDimensions::<Rgb565>::new(
    //         DynamicStyle::default(),
    //         DynamicStyle::default(),
    //         1,
    //         Size::new(1, 5),
    //     );
    //     let slider_size =
    //         Slider::new_with_style(style, 0.1, Box::new(|_| {})).size(&mut ctx, Size::new(10, 10));

    //     assert_eq!(slider_size.width, 10);
    //     // because of 2 pixel padding for selection box
    //     assert_eq!(slider_size.height, 5 + 2);
    // }
}
