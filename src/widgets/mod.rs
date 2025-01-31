use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    prelude::*,
    primitives::Rectangle,
};
use label::Label;
use linear_layout::{LayoutDirection, LinearLayoutBuilder};
use margin::{Margin, MarginLayout, MarginLayoutBuilder};

use crate::{Event, EventResult, UiContext};

pub mod label;
pub mod linear_layout;
pub mod margin;

/// Trait for any widgets including containers
/// Can also used as object
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

/// Any-widget struct
pub struct WidgetObj<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub(crate) widget: Box<dyn Widget<'a, D, C>>,
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

/// Ui-builder traits for containers
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

    fn margin_layout(
        &mut self,
        margin: Margin,
        fill: impl FnOnce(&mut MarginLayout<'a, D, C>),
    ) {
        let mut builder = MarginLayout {
            margin,
            child: None,
        };
        fill(&mut builder);
        self.add_widget_obj(builder.finish());
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
