//! This module contains all widgets available in edgy.
//! ## Common concept definitions:
//! `Widget` - Any UI-object both interactive and static, including `Layout`
//!
//! `Layout` - A container(-like) widget that holds another widgets
use alloc::{boxed::Box, format, string::String, vec::Vec};
use button::Button;
use eg_seven_segment::SevenSegmentStyle;
use embedded_graphics::{
    mono_font::{iso_8859_16::FONT_4X6, MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
};
use filler::{FillStrategy, Filler};
use gauge::{Gauge, GaugeStyle};
use grid_layout::GridLayoutBuilder;
use image::Image;
use label::{Label, SevenSegmentWidget};
use linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder};
use margin_layout::{Margin, MarginLayout};
use plot::Plot;
use primitive::Primitive;
use slider::{Slider, SliderStyle};
use toggle_button::ToggleButton;

use crate::{themes::Style, Event, EventResult, SystemEvent, UiContext};

pub mod button;
pub mod filler;
pub mod gauge;
pub mod grid_layout;
pub mod image;
pub mod label;
pub mod linear_layout;
pub mod margin_layout;
pub mod plot;
pub mod primitive;
pub mod slider;
pub mod toggle_button;

#[derive(Clone, Copy)]
pub struct WidgetEvent<'a> {
    pub system_event: &'a SystemEvent,
    pub is_focused: bool,
    pub id: usize,
    pub event: &'a Event,
}

/// Trait for any widgets including containers
/// Can also used as object
#[allow(unused_variables)]
pub trait Widget<'a, D, C>: 'a
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    /// Defines is interactivity of widget. (Currently only implement for cycling between widgets, like Tab key behaviour on PC)
    fn is_interactive(&mut self) -> bool {
        false
    }

    /// Returns the size the widget wants. use for auto-calculate in layouts. Default implementation occupies all available space
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        hint
    }

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

    /// Widget drawing logic
    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
    ) -> EventResult {
        EventResult::Pass
    }
}

/// Any-widget struct
pub struct WidgetObj<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub(crate) widget: Box<dyn Widget<'a, D, C>>,
    pub(crate) computed_rect: Rectangle,
    requested_size: Size,
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
            requested_size: Size::default(),
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
        if self.requested_size == Size::zero() {
            self.requested_size = self.widget.size(context, hint);
        }

        self.requested_size
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

    fn handle_event(&mut self, system_event: &SystemEvent) -> Event {
        // TODO: Reconsider a better solution

        match *system_event {
            SystemEvent::FocusTo(id) => {
                if self.id == id {
                    return Event::Focus;
                }
            }
            SystemEvent::ActiveTo(id) => {
                if self.id == id {
                    return Event::Active(None);
                }
            }
            SystemEvent::Active(point) => {
                if self.computed_rect.contains(point) {
                    // compute local coords
                    let x = point.x - self.computed_rect.top_left.x;
                    let y = point.y - self.computed_rect.top_left.y;
                    return Event::Active(Some(Point::new(x, y)));
                }
            }
            SystemEvent::Move(point) => {
                if self.computed_rect.contains(point) {
                    return Event::Focus;
                }
            }
            SystemEvent::Idle => {
                return Event::Idle;
            }
            SystemEvent::Drag(point) => {
                if self.computed_rect.contains(point) {
                    // compute local coords
                    let x = point.x - self.computed_rect.top_left.x;
                    let y = point.y - self.computed_rect.top_left.y;
                    return Event::Drag(Point::new(x, y));
                }
            }
            // do nothing, since is only system event
            SystemEvent::Decrease(_) => {}
            SystemEvent::Increase(_) => {}
        }

        Event::Idle
    }

    /// Actual draw and event handling function for widget.
    pub fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        system_event: &SystemEvent,
    ) -> EventResult {
        let event = self.handle_event(system_event);
        let event_args = WidgetEvent {
            system_event,
            is_focused: context.focused_element == self.id,
            id: self.id,
            event: &event,
        };

        let event_result = self.widget.draw(context, self.rect(), event_args);

        if context.debug_mode {
            let text = MonoTextStyle::new(&FONT_4X6, context.theme.debug_rect);
            if self.id > 0 {
                let _ = Text::new(
                    &format!("id: {}", self.id),
                    Point::new(
                        self.computed_rect.top_left.x,
                        self.computed_rect.top_left.y + 6,
                    ),
                    text,
                )
                .draw(&mut context.draw_target);
            } else {
                let _ = Text::new(
                    &format!(
                        "{}x{}",
                        self.computed_rect.size.width, self.computed_rect.size.height
                    ),
                    Point::new(
                        self.computed_rect.top_left.x,
                        self.computed_rect.top_left.y + 6,
                    ),
                    text,
                )
                .draw(&mut context.draw_target);
            }

            if event_args.is_focused {
                let _ = embedded_graphics::prelude::Primitive::into_styled(
                    self.rect(),
                    PrimitiveStyleBuilder::new()
                        .stroke_color(context.theme.debug_rect_active)
                        .stroke_width(1)
                        .build(),
                )
                .draw(&mut context.draw_target);
            } else {
                let _ = embedded_graphics::prelude::Primitive::into_styled(
                    self.rect(),
                    PrimitiveStyleBuilder::new()
                        .stroke_color(context.theme.debug_rect)
                        .stroke_width(1)
                        .build(),
                )
                .draw(&mut context.draw_target);
            }
        }

        event_result
    }
}

/// Ui-builder traits for containers
pub trait UiBuilder<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    // here add function for building widgets like button

    /// Method for adding widget in Layouts
    fn add_widget_obj(&mut self, widget: WidgetObj<'a, D, C>);

    /// Adds a widget in current layout
    fn add_widget<W: Widget<'a, D, C>>(&mut self, widget: W) {
        let mut object = WidgetObj::new(Box::new(widget));
        object.assign_id();
        self.add_widget_obj(object);
    }

    /// Creates a [Label] widget
    fn label<S: Into<String>>(
        &mut self,
        text: S,
        text_alignment: Alignment,
        style: MonoTextStyle<'a, C>,
    ) {
        self.add_widget(Label::new(text.into(), text_alignment, style))
    }

    /// Creates a [SevenSegmentWidget] widget
    fn seven_segment<S: Into<String>>(&mut self, text: S, style: SevenSegmentStyle<C>) {
        self.add_widget(SevenSegmentWidget::new(text.into(), style));
    }

    /// Creates a [Gauge] widget
    fn gauge(&mut self, value: f32) {
        self.add_widget(Gauge::new(value, "text", GaugeStyle::default()));
    }

    /// Shorthand construct for [Button] widget
    fn button<S: Into<String>>(
        &mut self,
        text: S,
        font: &'a MonoFont,
        callback: impl FnMut() + 'a,
    ) {
        self.add_widget(Button::new(text.into(), font, Box::new(callback)));
    }

    /// Shorthand construct for [Image] widget
    fn image<I: ImageDrawable<Color = C>>(&mut self, image: &'a I) {
        self.add_widget(Image::<'a, I>::new(image));
    }

    /// Shorthand construct for [ToggleButton] widget
    fn toggle_button<S: Into<String>>(
        &mut self,
        text: S,
        font: &'a MonoFont,
        state: bool,
        callback: impl FnMut(bool) + 'a,
    ) {
        self.add_widget(ToggleButton::new(
            text.into(),
            font,
            state,
            Box::new(callback),
        ));
    }

    /// Construct a [MarginLayout] widget
    fn margin_layout(&mut self, margin: Margin, fill: impl FnOnce(&mut MarginLayout<'a, D, C>)) {
        let mut builder = MarginLayout {
            margin,
            child: None,
            style: PrimitiveStyle::default(),
        };
        fill(&mut builder);
        self.add_widget_obj(builder.finish());
    }

    /// Construct a styled [MarginLayout] widget
    fn margin_layout_styled(
        &mut self,
        margin: Margin,
        style: PrimitiveStyle<C>,
        fill: impl FnOnce(&mut MarginLayout<'a, D, C>),
    ) {
        let mut builder = MarginLayout {
            margin,
            child: None,
            style,
        };
        fill(&mut builder);
        self.add_widget_obj(builder.finish());
    }

    /// Shorthand construct for [LinearLayout] widget. Creates a linear layout with in vertical direction
    fn vertical_linear_layout(
        &mut self,
        alignment: LayoutAlignment,
        fill: impl FnOnce(&mut LinearLayoutBuilder<'a, D, C>),
    ) {
        let mut builder = LinearLayoutBuilder {
            direction: LayoutDirection::Vertical,
            children: Vec::new(),
            ..Default::default()
        }
        .alignment(alignment);
        fill(&mut builder);
        self.add_widget_obj(builder.finish());
    }

    /// Shorthand construct for [LinearLayout] widget. Creates a linear layout with in horizontal direction
    fn horizontal_linear_layout(
        &mut self,
        alignment: LayoutAlignment,
        fill: impl FnOnce(&mut LinearLayoutBuilder<'a, D, C>),
    ) {
        let mut builder = LinearLayoutBuilder {
            direction: LayoutDirection::Horizontal,
            children: Vec::new(),
            ..Default::default()
        }
        .alignment(alignment);
        fill(&mut builder);
        self.add_widget_obj(builder.finish());
    }

    /// Shorthand construct for [GridLayout] widget.
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

    fn plot<V: Into<Vec<Point>>>(&mut self, points: V, scale: f32, offset: Point) {
        let mut plot = Plot::new(scale, offset);
        plot.points = points.into();
        self.add_widget(plot);
    }

    fn filler(&mut self, fill: FillStrategy) {
        self.add_widget(Filler::new(fill));
    }

    /// Any embedded-graphics drawable (primitive)
    fn primitive<P: Drawable<Color = C> + Dimensions + Transform + 'a>(&mut self, primitive: P) {
        self.add_widget(Primitive::new(primitive));
    }

    fn slider<TrackStyle: Style<C> + 'a, HandleStyle: Style<C> + 'a>(
        &mut self,
        value: f32,
        callback: impl FnMut(f32) + 'a,
        style: SliderStyle<C, TrackStyle, HandleStyle>,
    ) {
        self.add_widget(Slider::new(value, Box::new(callback), style));
    }

    fn finish(self) -> WidgetObj<'a, D, C>;
}
