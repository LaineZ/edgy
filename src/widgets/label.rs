use embedded_graphics::{mono_font::MonoTextStyle, prelude::*, text::Text};

use crate::UiContext;

use super::Widget;

pub struct Label<'a, C: PixelColor> {
    text_object: Text<'a, MonoTextStyle<'a, C>>,
}

impl<'a, C: PixelColor> Label<'a, C> {
    pub fn new<D>(text_style: MonoTextStyle<'a, C>, text: &'a str) -> Self
    where
        D: DrawTarget<Color = C>,
        C: PixelColor,
    {
        let text_object = Text::new(text, Point::zero(), text_style);
        Self { text_object }
    }
}

impl<'a, D, C> Widget<D, C> for Label<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn draw(&mut self, ui: &mut UiContext<D, C>) {
        self.text_object.translate_mut(ui.bounds.top_left);
        let _ = self.text_object.draw(ui.draw_target);
    }

    fn size(&self) -> Size {
        dbg!(self.text_object.bounding_box());
        self.text_object.bounding_box().size
    }
}
