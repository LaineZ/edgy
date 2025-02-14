#![allow(unused_imports)]

use core::f32::consts::PI;
use micromath::F32Ext;

use super::Widget;
use crate::UiContext;
use alloc::vec::Vec;
use embedded_graphics::{
    prelude::*,
    primitives::{
        Arc, Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable
    },
};

const GAUGE_STROKE_WIDTH: u32 = 2;


#[derive(Copy, Clone)]
pub struct GaugeDetent<C: PixelColor> {
    pub range: [f32; 2],
    pub color: C
}

/// Gauge widget
pub struct Gauge<C: PixelColor> {
    pub value: f32,
    detents: Vec<GaugeDetent<C>>
}

impl<C: PixelColor> Gauge<C> {
    pub fn new(value: f32) -> Self {
        Self { value, detents: Vec::new() }
    }

    pub fn add_detent(&mut self, detent: GaugeDetent<C>) {
        self.detents.push(detent);
    }
}

impl<'a, D, C> Widget<'a, D, C> for Gauge<C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        Size::new(hint.height, hint.height)
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        let circle = Circle::with_center(Point::new(rect.center().x, rect.center().y), rect.size.width - GAUGE_STROKE_WIDTH).into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(context.theme.foreground)
                .stroke_width(GAUGE_STROKE_WIDTH)
                .build(),
        );

        let circle_size = circle.primitive.diameter;
        let center = circle.primitive.center();
        let min_angle = 40.0;
        let max_angle = 320.0;

        // draw detents
        for detent in self.detents.iter() {
            let angle_start = min_angle + (max_angle - min_angle) * detent.range[0];
            let angle_end = min_angle + (max_angle - min_angle) * detent.range[1];
            let angle_sweep = angle_end - angle_start;
            let arc = Arc::from_circle(circle.primitive, Angle::from_degrees(angle_start + 90.0), Angle::from_degrees(angle_sweep))
                .into_styled(PrimitiveStyle::with_stroke(detent.color, GAUGE_STROKE_WIDTH + 1));
        
            let _ = arc.draw(context.draw_target);
        }

        // draw a dashes
        let divisions = 20;
        let total_angle = max_angle - min_angle;
        let angle_step = total_angle / (divisions - 1) as f32;

        let tick_length = circle_size as f32 * 0.1;
        let line_width = GAUGE_STROKE_WIDTH as f32 / 2.0;

        for i in 0..divisions {
            let angle = (min_angle + i as f32 * angle_step) + 90.0;
            let angle_rad = angle.to_radians();

            let start_x =
                center.x as f32 + (circle_size as f32 / 2.0 - line_width / 2.0) * angle_rad.cos();
            let start_y =
                center.y as f32 + (circle_size as f32 / 2.0 - line_width / 2.0) * angle_rad.sin();

            let end_x =
                center.x as f32 + (circle_size as f32 / 2.0 - tick_length) * angle_rad.cos();
            let end_y =
                center.y as f32 + (circle_size as f32 / 2.0 - tick_length) * angle_rad.sin();

            let _ = Line::new(
                Point::new((start_x + 0.5) as i32, (start_y + 0.5) as i32),
                Point::new((end_x + 0.5) as i32, (end_y + 0.5) as i32),
            )
            .draw_styled(
                &PrimitiveStyle::with_stroke(context.theme.foreground, GAUGE_STROKE_WIDTH / 2),
                context.draw_target,
            );
        }

        // draw a center circle (for needle)
        let _ = Circle::with_center(
            circle.primitive.center(),
            (circle.primitive.diameter / 10).clamp(2, 4),
        )
        .into_styled(PrimitiveStyle::with_fill(context.theme.foreground))
        .draw(context.draw_target);

        // needle
        let needle_width = (circle.primitive.diameter / 10).clamp(1, 2) as f32;

        let arrow_angle: f32 =
            (min_angle + (max_angle - min_angle) * self.value).clamp(0.0, max_angle);
        //println!("{} -> {}", self.value, arrow_angle);
        let arrow_angle_rad = arrow_angle.to_radians() + (PI / 2.0);
        let end_x = center.x as f32
            + (circle_size as f32 / 2.0 + needle_width / 2.0) * arrow_angle_rad.cos();

        let end_y = center.y as f32
            + (circle_size as f32 / 2.0 + needle_width / 2.0) * arrow_angle_rad.sin();

        let _ = Line::new(center, Point::new(end_x as i32, end_y as i32))
            .into_styled(PrimitiveStyle::with_stroke(
                context.theme.foreground2,
                needle_width as u32,
            ))
            .draw(context.draw_target);

        let _ = circle.draw(context.draw_target);
    }
}
