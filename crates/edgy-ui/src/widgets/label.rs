use core::marker::PhantomData;

use alloc::{boxed::Box, string::String};
use edgy_graphics::{
    draw,
    font::Font,
    framebuffer::FrameBuffer,
    geometry::{Point, Rectangle, Size},
    text::{self, LayoutOptions},
};

use crate::widgets::{NullBehavior, View, WidgetObject};

pub struct TypographyLabel<'a, S> {
    pub text: String,
    pub font: &'a Font<'a>,
    pub options: LayoutOptions,
    pub color: u8,
    _state: PhantomData<S>
}

impl<'a, S> TypographyLabel<'a, S> {
    pub fn new<ST: Into<String>>(font: &'a Font<'a>, text: ST, options: LayoutOptions, color: u8) -> Self {
        Self {
            text: text.into(),
            font,
            color,
            options,
            _state: PhantomData::<S>::default(),
        }
    }
}

impl<'a, S> View for TypographyLabel<'a, S> {
    type State = S;
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

    fn draw(&mut self, framebuffer: &mut FrameBuffer, rect: Rectangle, _state: &Self::State) {
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
    options: LayoutOptions,
    color: u8,
) -> WidgetObject<NullBehavior, TypographyLabel<'a, ()>> {
    WidgetObject::new(NullBehavior, TypographyLabel::new(font, text, options, color))
}
