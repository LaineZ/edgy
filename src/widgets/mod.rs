use std::u32;

use alloc::{boxed::Box, vec::Vec};
use button::Button;
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use label::Label;
use linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder};
use margin::{Margin, MarginLayout};

use crate::{Event, EventResult, Theme, UiContext};

pub mod button;
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
    /// Gets a size for widget (for layout compulation)
    fn size(&mut self, hint: Size) -> Size;

    fn min_size(&mut self) -> Size {
        Size::zero()
    }

    fn max_size(&mut self) -> Size {
        Size::new(u32::MAX, u32::MAX)
    }

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
    /// Gets a size for widget (for layout compulation)
    pub fn size(&mut self, hint: Size) -> Size {
        self.widget.size(hint)
    }

    pub fn handle_event(&mut self, event: &Event) -> EventResult {
        self.widget.handle_event(event)
    }

    /// Actual draw function for widget. `rect` parameter contains actual allocated space for widget in container
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

    fn label(&mut self, text: &'a str, style: MonoTextStyle<'a, C>) {
        self.add_widget(Label::new(text, style))
    }

    fn button(
        &mut self,
        text: &'a str,
        theme: Theme<C>,
        font: &'a MonoFont,
        callback: impl FnMut() + 'a,
    ) {
        self.add_widget(Button::new(
            text,
            font,
            theme,
            Box::new(callback),
        ));
    }

    fn margin_layout(&mut self, margin: Margin, fill: impl FnOnce(&mut MarginLayout<'a, D, C>)) {
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
        alignment: LayoutAlignment,
        fill: impl FnOnce(&mut LinearLayoutBuilder<'a, D, C>),
    ) {
        let mut builder = LinearLayoutBuilder {
            direction,
            children: Vec::new(),
            alignment,
            ..Default::default()
        };
        fill(&mut builder);
        self.add_widget_obj(builder.finish());
    }

    // сюда добавлять функции для стройки виджетов, типа button

    fn finish(self) -> WidgetObj<'a, D, C>;
}
