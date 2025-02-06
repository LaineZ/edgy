use alloc::{boxed::Box, format, vec::Vec};
use button::Button;
use embedded_graphics::{
    mono_font::{iso_8859_16::FONT_4X6, MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use grid_layout::GridLayoutBuilder;
use label::Label;
use linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder};
use margin::{Margin, MarginLayout};

use crate::{Event, EventResult, SystemEvent, UiContext};

pub mod button;
pub mod grid_layout;
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
    /// Defines is interactivity of widget. (Currently only implement for cycling between widgets, like Tab key behaviour on PC)
    fn is_interactive(&mut self) -> bool {
        false
    }

    /// Returns the size the widget wants. use for auto-calculate in layouts
    fn size(&mut self, context: &mut UiContext<'a, D, C>, hint: Size) -> Size;

    /// Calls at layout pass. Gives a try for layout computation in Layouts (Containers)
    fn layout(&mut self, _context: &mut UiContext<'a, D, C>, _rect: Rectangle) {}

    /// Returns a minimum size of widget
    fn min_size(&mut self) -> Size {
        Size::zero()
    }

    /// Returs a maximum size of widget
    fn max_size(&mut self) -> Size {
        Size::new(u32::MAX, u32::MAX)
    }

    /// Event processing in widget
    fn handle_event(
        &mut self,
        _context: &mut UiContext<'a, D, C>,
        _system_event: &SystemEvent,
        event: &Event,
    ) -> EventResult {
        let _ = event;
        EventResult::Pass
    }

    /// Widget drawing logic
    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle);
}

/// Any-widget struct
pub struct WidgetObj<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub(crate) widget: Box<dyn Widget<'a, D, C>>,
    pub(crate) computed_rect: Rectangle,
    pub(crate) id: usize,
}

impl<'a, D, C> WidgetObj<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub fn new(widget: Box<dyn Widget<'a, D, C>>) -> Self {
        Self {
            computed_rect: Rectangle::default(),
            widget,
            id: 0,
        }
    }
}

impl<'a, D, C> WidgetObj<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    /// Gets a size for widget (for layout compulation)
    pub fn size(&mut self, context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        self.widget.size(context, hint)
    }

    fn assign_id(&mut self) {
        if self.widget.is_interactive() {
            let id = crate::WIDGET_IDS.load(core::sync::atomic::Ordering::Relaxed) + 1;
            crate::WIDGET_IDS.store(id, core::sync::atomic::Ordering::Relaxed);
            self.id = id;
        }
    }

    /// Returns a minimum size of widget
    pub fn min_size(&mut self) -> Size {
        self.widget.min_size()
    }

    /// Returns a maximum size of widget
    pub fn max_size(&mut self) -> Size {
        self.widget.max_size()
    }

    /// Returns a actually computed rectangle for widget
    pub fn rect(&self) -> Rectangle {
        self.computed_rect
    }

    /// Calls at layout pass. Gives a try for layout computation in Layouts (Containers)
    pub fn layout(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        self.computed_rect = rect;
        self.widget.layout(context, rect);
    }

    /// Calculate sizes clamping to minimum and maximum sizes
    pub fn calculate_bound_sizes(&mut self, size: Size) -> Size {
        Size::new(
            size.width
                .clamp(self.min_size().width, self.max_size().width),
            size.height
                .clamp(self.min_size().height, self.max_size().height),
        )
    }

    /// Event processing in widget. You can also use like update callback
    pub fn handle_event(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        system_event: &SystemEvent,
    ) -> EventResult {
        match *system_event {
            SystemEvent::FocusTo(id) => {
                if self.id == id {
                    self.widget
                        .handle_event(context, system_event, &Event::Focus)
                } else {
                    self.widget
                        .handle_event(context, system_event, &Event::Idle)
                }
            }
            SystemEvent::ActiveTo(id) => {
                if self.id == id {
                    self.widget
                        .handle_event(context, system_event, &Event::Active)
                } else {
                    self.widget
                        .handle_event(context, system_event, &Event::Idle)
                }
            }
            SystemEvent::Active(point) => {
                if crate::contains(self.computed_rect, point) {
                    self.widget
                        .handle_event(context, system_event, &Event::Active)
                } else {
                    self.widget
                        .handle_event(context, system_event, &Event::Idle)
                }
            }
            SystemEvent::Move(point) => {
                if crate::contains(self.computed_rect, point) {
                    self.widget
                        .handle_event(context, system_event, &Event::Focus)
                } else {
                    self.widget
                        .handle_event(context, system_event, &Event::Idle)
                }
            }
            _ => self.widget.handle_event(context, system_event, &Event::Idle),
        }
    }

    /// Actual draw function for widget.
    pub fn draw(&mut self, context: &mut UiContext<'a, D, C>) {
        self.widget.draw(context, self.rect());

        if context.debug_mode {
            let text = MonoTextStyle::new(&FONT_4X6, context.theme.foreground2);
            let _ = Text::new(
                &format!("id: {}", self.id),
                Point::new(
                    self.computed_rect.top_left.x,
                    self.computed_rect.top_left.y + 6,
                ),
                text,
            )
            .draw(context.draw_target);
            let _ = self
                .rect()
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_color(context.theme.debug_rect)
                        .stroke_width(1)
                        .build(),
                )
                .draw(context.draw_target);
        }
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
        let mut object = WidgetObj::new(Box::new(widget));
        object.assign_id();
        self.add_widget_obj(object);
    }

    fn label(&mut self, text: &'a str, style: MonoTextStyle<'a, C>) {
        self.add_widget(Label::new(text, style))
    }

    fn button(&mut self, text: &'a str, font: &'a MonoFont, callback: impl FnMut() + 'a) {
        self.add_widget(Button::new(text, font, Box::new(callback)));
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

    fn grid_layout(
        &mut self,
        rows: Vec<u32>,
        colums: Vec<u32>,
        fill: impl FnOnce(&mut GridLayoutBuilder<'a, D, C>),
    ) {
        let mut builder = GridLayoutBuilder {
            children: Vec::new(),
            col_fracs: colums,
            row_fracs: rows,
        };
        fill(&mut builder);
        self.add_widget_obj(builder.finish());
    }

    // сюда добавлять функции для стройки виджетов, типа button

    fn finish(self) -> WidgetObj<'a, D, C>;
}
