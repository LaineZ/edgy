use embedded_graphics::{mono_font::MonoTextStyle, prelude::*, primitives::Rectangle, text::Text};

pub struct Label<'a, C: PixelColor> {
    text: &'a str,
    size: Rectangle,
    style: MonoTextStyle<'a, C>,
}

impl<'a, C: PixelColor> Label<'a, C> {
    pub fn new(text: &'a str, style: MonoTextStyle<'a, C>) -> Self {
        Self {
            text,
            style,
            size: Rectangle::zero(),
        }
    }
}

impl<C> Drawable for Label<'_, C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Text::new(self.text, self.size.top_left, self.style).draw(target)?;
        Ok(())
    }
}
