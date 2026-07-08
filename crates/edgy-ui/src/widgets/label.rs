use alloc::string::String;
use edgy_graphics::{draw, font::Font, geometry::{Point, Rectangle, Size}, text::{self, LayoutOptions}};

use crate::widgets::Widget;

pub struct TypographyLabel<'a> {
    text: String,
    font: Font<'a>,
    pub options: LayoutOptions,
    pub color: u8
}

impl<'a> TypographyLabel<'a> {
    pub fn new<S: Into<String>>(font: Font<'a>, text: S, color: u8) -> Self {
        Self {
            text: text.into(),
            font,
            color,
            options: LayoutOptions::default()
        }
    }
}

impl<'a, M> Widget<'a, M> for TypographyLabel<'a> {
    fn measure(&mut self, hint: Size) -> Size {
        let bb = text::layout_bounds(&self.font, Rectangle::new(Point::<i32>::zero(), hint), &self.text, &self.options, |_, _| {});
        bb.bounding_box.size
    }
    
    fn draw(&mut self, ui: &mut crate::UiContext<M>, rect: Rectangle) {
        draw::text_advanced(&mut ui.framebuffer, rect, &self.options, &self.font, &self.text, self.color);
    }
}