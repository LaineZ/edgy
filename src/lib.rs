#![no_std]

use alloc::{boxed::Box, collections::btree_map::BTreeMap, vec::Vec};
use embedded_graphics::{
    mono_font::{iso_8859_10::FONT_4X6, MonoTextStyle},
    prelude::*,
    primitives::Rectangle,
    text::{renderer::TextRenderer, Text},
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

    fn label(&mut self, text: &'a str, color: C) {
        self.add_widget(Label::new(text, MonoTextStyle::new(&FONT_4X6, color)))
    }

    fn linear_layout(
        &mut self,
        direction: LayoutDirection,
        fill: impl FnOnce(&mut LinearLayoutBuilder<'a, D, C>),
    ) {
        let mut builder = LinearLayoutBuilder {
            direction,
            children: Vec::new(),
        };
        fill(&mut builder);
        self.add_widget_obj(builder.finish());
    }

    // сюда добавлять функции для стройки виджетов, типа button

    fn finish(self) -> WidgetObj<'a, D, C>;
}

pub struct Label<'a, C: PixelColor> {
    text: &'a str,
    style: MonoTextStyle<'a, C>,
    position: Point,
}

impl<'a, C> Label<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(text: &'a str, style: MonoTextStyle<'a, C>) -> Self {
        Self {
            text,
            style,
            position: Point::default(),
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Label<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn layout(&mut self, _hint: Size) -> Size {
        let size_rect = self
            .style
            .measure_string(
                &self.text,
                self.position,
                embedded_graphics::text::Baseline::Middle,
            )
            .bounding_box;
        size_rect.size
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        self.position = rect.top_left;
        let text = Text::new(&self.text, rect.top_left, self.style);
        let _ = text.draw(context.draw_target);
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

pub struct LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub children: Vec<WidgetObj<'a, D, C>>,
    pub direction: LayoutDirection,
}

impl<'a, D, C> Default for LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn default() -> Self {
        Self {
            children: Vec::new(),
            direction: LayoutDirection::Vertical,
        }
    }
}

impl<'a, D, C> UiBuilder<'a, D, C> for LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn add_widget_obj(&mut self, widget: WidgetObj<'a, D, C>) {
        self.children.push(widget);
    }

    fn finish(self) -> WidgetObj<'a, D, C> {
        WidgetObj {
            widget: Box::new(LinearLayout {
                direction: self.direction,
                children: self.children,
            }),
        }
    }
}

pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

pub struct LinearLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    children: Vec<WidgetObj<'a, D, C>>,
    direction: LayoutDirection,
}

impl<'a, D, C> Widget<'a, D, C> for LinearLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn layout(&mut self, hint: Size) -> Size {
        let mut width = 0;
        let mut height = 0;
    
        for child in &mut self.children {
            let child_size = child.layout(hint);
    
            match self.direction {
                LayoutDirection::Horizontal => {
                    width += child_size.width;
                    height = height.max(child_size.height);
                }
                LayoutDirection::Vertical => {
                    height += child_size.height;
                    width = width.max(child_size.width);
                }
            }
        }

        Size::new(width.min(hint.width), height.min(hint.height))
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

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        let mut offset = Point::zero();
        
        for child in &mut self.children {
            let child_size = child.layout(Size::new(rect.size.width, rect.size.height));
            let limited_size = Size::new(
                child_size.width.min(rect.size.width),
                child_size.height.min(rect.size.height),
            );
    
            let child_rect = Rectangle::new(rect.top_left + offset, limited_size);
            child.draw(context, child_rect);
    
            match self.direction {
                LayoutDirection::Horizontal => {
                    offset.x += limited_size.width as i32;
                }
                LayoutDirection::Vertical => {
                    offset.y += limited_size.height as i32;
                }
            }
        }
    }
}
