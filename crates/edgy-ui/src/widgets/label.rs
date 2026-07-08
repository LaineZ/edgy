use alloc::string::String;
use edgy_graphics::{
    draw,
    font::Font,
    framebuffer::FrameBuffer,
    geometry::{Point, Rectangle, Size},
    text::{self, LayoutOptions},
};

use crate::widgets::{NullBehavior, View, WidgetObject};

pub struct TypographyLabel<'a> {
    pub text: String,
    pub font: &'a Font<'a>,
    pub options: LayoutOptions,
    pub color: u8,
}

impl<'a> TypographyLabel<'a> {
    pub fn new<S: Into<String>>(font: &'a Font<'a>, text: S, color: u8) -> Self {
        Self {
            text: text.into(),
            font,
            color,
            options: LayoutOptions::default(),
        }
    }
}

impl<'a> View for TypographyLabel<'a> {
    type State = ();
    fn measure(&mut self, hint: Size) -> Size {
        let bounds = text::layout_bounds(
            &self.font,
            Rectangle::new(Point::<i32>::zero(), hint),
            &self.text,
            &self.options,
            |_, _| {},
        );

        bounds.bounding_box.size
    }

    fn draw(&self, framebuffer: &mut FrameBuffer, rect: Rectangle, _state: &()) {
        draw::text_advanced(
            framebuffer,
            rect,
            &self.options,
            &self.font,
            &self.text,
            self.color,
        );
    }
}

pub fn typography_label<'a>(
    font: &'a Font<'a>,
    text: String,
    color: u8,
) -> WidgetObject<NullBehavior, TypographyLabel<'a>> {
    WidgetObject::new(NullBehavior, TypographyLabel::new(font, text, color))
}
