#![no_std]

use alloc::{boxed::Box, vec::Vec};
use core::marker::PhantomData;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::{Primitive, Rectangle},
    text::Text,
    Drawable,
};

extern crate alloc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventResult {
    /// эвент обработан
    Stop,
    /// эвент не обработан, пробуем следующий виджет
    Pass,
}

/// затычка
pub struct Event;

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

/// трейт для любых виджетов, в том числе контейнеров
/// можно использовать в виде объекта
pub trait Widget<'a, D, C>: 'a
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn layout(&mut self, hint: Size) -> Size;

    fn handle_event(&mut self, event: &Event) -> EventResult {
        let _ = event;
        EventResult::Pass
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle);
}

pub trait UiBuilder<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn add_widget_obj(&mut self, widget: WidgetObj<'a, D, C>);

    fn add_widget<W: Widget<'a, D, C>>(&mut self, widget: W) {
        self.add_widget_obj(WidgetObj {
            widget: Box::new(widget),
        });
    }

    fn label(&mut self, text: &'a str) {
        self.add_widget(Label { text })
    }

    fn linear_layout(&mut self, fill: impl FnOnce(&mut LinearLayoutBuilder<'a, D, C>)) {
        let mut builder = LinearLayoutBuilder {
            children: Vec::new(),
        };
        fill(&mut builder);
        builder.finish();
    }

    // сюда добавлять функции для стройки виджетов, типа button

    fn finish(self) -> WidgetObj<'a, D, C>;
}

pub struct Label<'a> {
    text: &'a str,
}

impl<'a, D, C> Widget<'a, D, C> for Label<'a>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn layout(&mut self, hint: Size) -> Size {
        todo!()
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        todo!()
    }
}

pub struct WidgetObj<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    widget: Box<dyn Widget<'a, D, C>>,
}

impl<'a, D, C> WidgetObj<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    pub fn layout(&mut self, hint: Size) -> Size {
        self.widget.layout(hint)
    }

    pub fn handle_event(&mut self, event: &Event) -> EventResult {
        self.widget.handle_event(event)
    }

    pub fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        self.widget.draw(context, rect)
    }
}

#[derive(Default)]
pub struct LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub children: Vec<WidgetObj<'a, D, C>>,
}

impl<'a, D, C> UiBuilder<'a, D, C> for LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn add_widget_obj(&mut self, widget: WidgetObj<'a, D, C>) {
        self.children.push(widget);
    }

    fn label(&mut self, text: &'a str) {
        self.add_widget(Label { text })
    }

    fn finish(self) -> WidgetObj<'a, D, C> {
        WidgetObj {
            widget: Box::new(LinearLayout {
                children: self.children,
            }),
        }
    }
}

pub struct LinearLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    children: Vec<WidgetObj<'a, D, C>>,
}

impl<'a, D, C> Widget<'a, D, C> for LinearLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn layout(&mut self, _hint: Size) -> Size {
        todo!("вычислить размер лаяута с учетом всех childrenов")
    }

    fn handle_event(&mut self, event: &Event) -> EventResult {
        let mut result = EventResult::Pass;

        for child in &mut self.children {
            result = child.handle_event(event);
            if result == EventResult::Stop {
                break;
            }
        }

        result
    }

    fn draw(&mut self, _encoder: &mut UiContext<'a, D, C>, _rect: Rectangle) {
        todo!("отрисовать child виджеты с правильными координатами")
    }
}
