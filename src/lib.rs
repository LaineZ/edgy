use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Primitive, Rectangle},
    text::Text,
    Drawable,
};

pub struct UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub draw_target: &'a mut D,
    pub bounds: Rectangle,
}

impl<'a, D, C> UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub fn new(draw_target: &'a mut D, bounds: Rectangle) -> Self {
        Self {
            draw_target,
            bounds,
        }
    }
}

pub trait Widget<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn draw(&mut self, ui: &mut UiContext<D, C>);
    fn size(&self) -> Size;
}

pub struct Label<'a, C: PixelColor> {
    size: Size,
    text_object: Text<'a, MonoTextStyle<'a, C>>,
}

impl<'a, C: PixelColor> Label<'a, C> {
    pub fn new<D>(text_style: MonoTextStyle<'a, C>, text: &'a str) -> Self
    where
        D: DrawTarget<Color = C>,
        C: PixelColor,
    {
        let text_object = Text::new(text, Point::zero(), text_style);
        Self {
            text_object,
            size: Size::zero(),
        }
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

pub struct StackLayout;

impl StackLayout {
    pub fn new<D, C, F>(ui: &mut UiContext<D, C>, direction: StackLayoutDirection, mut f: F)
    where
        D: DrawTarget<Color = C>,
        C: PixelColor,
        F: FnMut(&mut dyn FnMut(&mut dyn Widget<D, C>)),
    {
        let mut current_position = ui.bounds.top_left;
        let mut remaining_size = ui.bounds.size;

        // Создаем дочерние элементы разметки
        f(&mut |widget: &mut dyn Widget<D, C>| {
            // Вызываем size() для вычисления размеров виджета
            let widget_size = widget.size();

            // Проверяем, что виджет помещается в оставшееся пространство
            if (direction == StackLayoutDirection::Vertical
                && widget_size.height > remaining_size.height)
                || (direction == StackLayoutDirection::Horizontal
                    && widget_size.width > remaining_size.width)
            {
                return; // Если не помещается — пропускаем
            }

            // Рисуем виджет
            let mut child_ui = UiContext {
                draw_target: ui.draw_target,
                bounds: Rectangle::new(current_position, widget_size),
            };

            widget.draw(&mut child_ui);

            // Обновляем текущую позицию и оставшееся пространство
            match direction {
                StackLayoutDirection::Vertical => {
                    current_position.y += widget_size.height as i32;
                    remaining_size.height -= widget_size.height;
                }
                StackLayoutDirection::Horizontal => {
                    current_position.x += widget_size.width as i32;
                    remaining_size.width -= widget_size.width;
                }
            }
        });
    }
}
#[derive(PartialEq)]
pub enum StackLayoutDirection {
    Vertical,
    Horizontal,
}
