#![allow(unused_imports)]

use core::f32::consts::PI;
use micromath::F32Ext;

use super::{Widget, WidgetEvent};
use crate::{EventResult, UiContext};
use alloc::{string::ToString, vec::Vec};
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    prelude::*,
    primitives::{
        Arc, Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable,
    },
    text::{Alignment, Text},
};

#[derive(Copy, Clone)]
pub struct GaugeDetent<C: PixelColor> {
    pub range: [f32; 2],
    pub color: C,
}

impl<C: PixelColor> GaugeDetent<C> {
    pub fn new(range: [f32; 2], color: C) -> Self {
        Self { range, color }
    }
}

#[derive(Copy, Clone)]
pub struct GaugeStyle {
    divisions: u32,
    display_values: bool,
    min_angle: f32,
    max_angle: f32,
}

impl GaugeStyle {
    pub fn divisions(mut self, divisions: u32) -> Self {
        self.divisions = divisions.clamp(2, u32::MAX);
        self
    }

    pub fn min_angle(mut self, min_angle: f32) -> Self {
        self.min_angle = min_angle;
        self
    }

    pub fn max_angle(mut self, max_angle: f32) -> Self {
        self.max_angle = max_angle;
        self
    }
}

impl Default for GaugeStyle {
    fn default() -> Self {
        Self {
            divisions: 5,
            min_angle: 40.0,
            max_angle: 320.0,
            display_values: false,
        }
    }
}

/// Gauge widget
pub struct Gauge<'a, C: PixelColor> {
    pub value: f32,
    detents: Vec<GaugeDetent<C>>,
    gauge_style: GaugeStyle,
    text: &'a str,
}

impl<'a, C: PixelColor> Gauge<'a, C> {
    pub fn new(value: f32, text: &'a str, gauge_style: GaugeStyle) -> Self {
        Self {
            value,
            gauge_style,
            detents: Vec::new(),
            text,
        }
    }

    pub fn add_detent(&mut self, detent: GaugeDetent<C>) {
        self.detents.push(detent);
    }
}

impl<'a, D, C> Widget<'a, D, C> for Gauge<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        Size::new(hint.height, hint.height)
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        _event_args: WidgetEvent,
    ) -> EventResult {
        let style = context.theme.gauge_style;
        let foreground_color = style
            .foreground_color
            .expect("Gauge must have a foreground color to draw");
        let stroke_color = style.stroke_color.unwrap_or(foreground_color);
        let accent_color = style.accent_color.unwrap_or(foreground_color);

        let gauge_stroke_width = style.stroke_width.clamp(2, u32::MAX);

        let circle = Circle::with_center(
            Point::new(rect.center().x, rect.center().y),
            rect.size.width - gauge_stroke_width,
        )
        .into_styled(style.into());

        let circle_size = circle.primitive.diameter;
        let center = circle.primitive.center();
        let _ = circle.draw(&mut context.draw_target);

        // draw detents
        for detent in self.detents.iter() {
            let angle_start = self.gauge_style.min_angle
                + (self.gauge_style.max_angle - self.gauge_style.min_angle) * detent.range[0];
            let angle_end = self.gauge_style.min_angle
                + (self.gauge_style.max_angle - self.gauge_style.min_angle) * detent.range[1];
            let angle_sweep = angle_end - angle_start;
            let arc = Arc::from_circle(
                circle.primitive,
                Angle::from_degrees(angle_start + 90.0),
                Angle::from_degrees(angle_sweep),
            )
            .into_styled(PrimitiveStyle::with_stroke(
                detent.color,
                gauge_stroke_width / 2,
            ));

            let _ = arc.draw(&mut context.draw_target);
        }

        // draw a dashes
        let total_angle = self.gauge_style.max_angle - self.gauge_style.min_angle;
        let angle_step = total_angle / (self.gauge_style.divisions - 1) as f32;

        let tick_length = circle_size as f32 * 0.1;
        let line_width = gauge_stroke_width as f32 / 2.0;

        for i in 0..self.gauge_style.divisions {
            let angle = (self.gauge_style.min_angle + i as f32 * angle_step) + 90.0;
            let angle_rad = angle.to_radians();

            let start_x =
                center.x as f32 + (circle_size as f32 / 2.0 - line_width / 2.0) * angle_rad.cos();
            let start_y =
                center.y as f32 + (circle_size as f32 / 2.0 - line_width / 2.0) * angle_rad.sin();

            let end_x =
                center.x as f32 + (circle_size as f32 / 2.0 - tick_length) * angle_rad.cos();
            let end_y =
                center.y as f32 + (circle_size as f32 / 2.0 - tick_length) * angle_rad.sin();

            if self.gauge_style.display_values {
                let tex_end_x =
                    center.x as f32 + (circle_size as f32 / 2.5 - tick_length) * angle_rad.cos();
                let tex_end_y =
                    center.x as f32 + (circle_size as f32 / 2.5 - tick_length) * angle_rad.sin();

                let _ = Text::new(
                    "0",
                    Point::new(tex_end_x as i32, tex_end_y as i32),
                    MonoTextStyle::new(&FONT_4X6, stroke_color),
                )
                .draw(&mut context.draw_target);
            }

            let _ = Line::new(
                Point::new((start_x + 0.5) as i32, (start_y + 0.5) as i32),
                Point::new((end_x + 0.5) as i32, (end_y + 0.5) as i32),
            )
            .draw_styled(
                &PrimitiveStyle::with_stroke(stroke_color, gauge_stroke_width / 2),
                &mut context.draw_target,
            );
        }

        // draw a center circle (for needle)
        let _ = Circle::with_center(
            circle.primitive.center(),
            (circle.primitive.diameter / 10).clamp(2, 4),
        )
        .into_styled(PrimitiveStyle::with_fill(foreground_color))
        .draw(&mut context.draw_target);

        // needle
        let needle_width = (circle.primitive.diameter / 10).clamp(1, 2) as f32;

        let arrow_angle: f32 = (self.gauge_style.min_angle
            + (self.gauge_style.max_angle - self.gauge_style.min_angle) * self.value)
            .clamp(0.0, self.gauge_style.max_angle);
        //println!("{} -> {}", self.value, arrow_angle);
        let arrow_angle_rad = arrow_angle.to_radians() + (PI / 2.0);
        let end_x = center.x as f32
            + (circle_size as f32 / 2.0 + needle_width / 2.0) * arrow_angle_rad.cos();

        let end_y = center.y as f32
            + (circle_size as f32 / 2.0 + needle_width / 2.0) * arrow_angle_rad.sin();

        let _ = Line::new(center, Point::new(end_x as i32, end_y as i32))
            .into_styled(PrimitiveStyle::with_stroke(
                accent_color,
                needle_width as u32,
            ))
            .draw(&mut context.draw_target);

        // text
        let _ = Text::with_alignment(
            self.text,
            Point::new(center.x, center.y + 10),
            MonoTextStyle::new(&FONT_4X6, accent_color),
            Alignment::Center,
        )
        .draw(&mut context.draw_target);

        EventResult::Pass
    }
}
