use core::marker::PhantomData;
use core::u32;

use alloc::string::String;
use edgy_graphics::{draw::{self, BasicStyle}, font::Font, geometry::{Point, Rectangle, Size}, text::{self, LayoutOptions}};

use crate::{Prop, geometry::Constraint, widgets::Widget};

pub struct TypographyLabel<'a> {
    pub text: Prop<String>,
    pub font: Prop<&'a Font<'a>>,
    pub options: Prop<LayoutOptions>,
    pub color: Prop<u8>,
}

impl<'a> TypographyLabel<'a> {
    pub fn new(text: impl Into<Prop<String>>, font: impl Into<Prop<&'a Font<'a>>>, color: impl Into<Prop<u8>>) -> Self {
        Self {
            text: text.into(),
            color: color.into(),
            font: font.into(),
            options: Prop::Value(LayoutOptions::default()),
        }
    }

    pub fn options(mut self, options: impl Into<Prop<LayoutOptions>>) -> Self {
        self.options = options.into();
        self
    }
}

impl<'a> Widget for TypographyLabel<'static> {
    fn measure(&mut self, _context: &mut crate::context::SizeContext<'_>, constraint: Constraint) -> Size {

        //println!("constraint: {:?}", constraint);

        
        let bounds = text::layout_bounds(
            &self.font.get(),
            Rectangle::new(Point::<i32>::zero(), constraint.max_size),
            &self.text.get(),
            &self.options.get(),
            |_, _| {},
        );

        //println!("my size is: {:?}", bounds.bounding_box.size);
        
        bounds.bounding_box.size
    }

    fn draw(&mut self, context: &mut crate::context::DrawContext<'_>) {
        let rect = context.rect();
        draw::rect(context.framebuffer, rect, BasicStyle::with_border(4, 1));
        draw::text_advanced(
            context.framebuffer,
            rect,
            &self.options.get(),
            &self.font.get(),
            &self.text.get(),
            self.color.get(),
        );
    }
}