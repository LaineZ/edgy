use crate::{EventResult, UiContext};

use super::{Widget, WidgetEvent};
use alloc::vec::Vec;
use embedded_graphics::{
    prelude::*,
    primitives::{Line, Polyline, PrimitiveStyle, Rectangle},
};

/// Simple plotter X/Y widget
pub struct Plot {
    pub points: Vec<Point>,
    pub y_scale: f32,
    pub offset: Point,
}

impl Plot {
    pub fn new(y_scale: f32, offset: Point) -> Self {
        Plot {
            points: Vec::new(),
            offset,
            y_scale: y_scale.clamp(0.1, f32::MAX),
        }
    }

    fn scale_graph(&mut self, rect: Rectangle) -> (Size, Size) {
        let (min_x, max_x, min_y, max_y) =
            self.points
                .iter()
                .fold((0, 0, 0, 0), |(min_x, max_x, min_y, max_y), &point| {
                    (
                        min_x.min(point.x),
                        max_x.max(point.x),
                        min_y.min(point.y),
                        max_y.max(point.y),
                    )
                });

        let scale_x = (rect.size.width as f32) / (max_x - min_x) as f32;
        let scale_y = (rect.size.height as f32) / (max_y - min_y) as f32 * self.y_scale;

        for point in &mut self.points {
            let scaled_x = ((point.x - min_x) as f32 * scale_x) as i32 + rect.top_left.x;
            let scaled_y = ((point.y - min_y) as f32 * scale_y) as i32 + rect.top_left.y;
            point.x = scaled_x;
            point.y = scaled_y;
        }

        (
            Size::new(min_x as u32, max_y as u32),
            Size::new(max_x as u32, max_y as u32),
        )
    }
}

impl<'a, D, C> Widget<'a, D, C> for Plot
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, hint: Size, resolved_style: &Style<'a, C>) -> Size {
        hint
    }

       fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        _event_args: WidgetEvent, resolved_style: &Style<'a, C>,
    ) -> EventResult {
        if self.points.is_empty() {
            return EventResult::Pass
        }
        let style = context.theme.plot_style;
        let grid_style = PrimitiveStyle::with_stroke(
            style
                .background_color
                .expect("Plot widghet must have a background color for a drawing"),
            1,
        );
        let axis_style = PrimitiveStyle::with_stroke(
            style
                .foreground_color
                .expect("Plot widghet must have a foreground color for a drawing"),
            2,
        );

        // draw lines
        let bottom_right = rect.bottom_right().unwrap_or_default();

        let _ = Line::new(
            Point::new(rect.top_left.x, rect.center().y),
            Point::new(bottom_right.x, rect.center().y),
        )
        .into_styled(axis_style)
        .draw(&mut context.draw_target);

        let _ = Line::new(
            Point::new(rect.center().x, rect.top_left.y),
            Point::new(rect.center().x, bottom_right.y),
        )
        .into_styled(axis_style)
        .draw(&mut context.draw_target);

        let (min_size, max_size) = self.scale_graph(rect);

        let start_x = (min_size.width / 10) * 10;
        let scale_x = (rect.size.width as f32) / (max_size.width - min_size.width) as f32;

        // draw grid
        if self.y_scale > 0.2 {
            for x in (start_x..)
                .step_by(10_usize)
                .take_while(|&x| x <= min_size.width + rect.size.width)
            {
                let scaled_x = ((x - min_size.width) as f32 * scale_x) as i32 + rect.top_left.x;
                if scaled_x > bottom_right.x {
                    break;
                }
                let _ = Line::new(
                    Point::new(scaled_x, rect.top_left.y),
                    Point::new(scaled_x, bottom_right.y),
                )
                .into_styled(grid_style)
                .draw(&mut context.draw_target);
            }

            let step_y = (10.0 * self.y_scale) as i32;
            for y in (rect.top_left.y..rect.bottom_right().unwrap_or_default().y)
                .step_by(step_y as usize)
                .take_while(|&y| y <= min_size.height as i32 + rect.size.height as i32)
            {
                let _ = Line::new(
                    Point::new(rect.top_left.x, y),
                    Point::new(rect.bottom_right().unwrap_or_default().x, y),
                )
                .into_styled(grid_style)
                .draw(&mut context.draw_target);
            }
        }

        let _ = Polyline::new(&self.points)
            .into_styled(PrimitiveStyle::with_stroke(
                style
                    .accent_color
                    .expect("Plot widghet must have a accent color for a drawing"),
                1,
            ))
            .translate(self.offset)
            .draw(&mut context.draw_target);

        EventResult::Pass
    }
}
