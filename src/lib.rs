#![no_std]
//! edgy - no_std immediate-mode GUI library for microcontrollers. It uses ``embedded_graphics`` for
//! rendering and some types like ``Color`` or ``Rectangle``. Library uses ``alloc`` for widget
//! dynamic dispatch, threfore a allocator is required.
use alloc::{rc::Rc, string::String};
use core::{
    cell::Cell, marker::PhantomData, sync::atomic::{AtomicUsize, Ordering}, u32
};
pub use embedded_graphics;
use themes::Theme;

use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    prelude::*,
    primitives::Rectangle,
    text::Alignment,
};
use widgets::{
    linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder},
    UiBuilder, WidgetObject,
};

// pub use embedded_graphics::primitives::Rectangle as Rectangle;
// pub use embedded_graphics::geometry::Point as Point;
// pub use embedded_graphics::geometry::Size as Size;

pub mod themes;
pub mod widgets;

extern crate alloc;

pub(crate) static WIDGET_IDS: AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

pub const MAX_SIZE: Size = Size::new(u32::MAX, u32::MAX);
pub const MIN_SIZE: Size = Size::zero();

/// Event result struct
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventResult {
    /// Event processed
    Stop,
    /// Event passed, trying next widget
    Pass,
}

/// Your events that can be inserted into UI context
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SystemEvent {
    /// Idle event (None, Null) event
    Idle,
    /// Focus to specified widget ID
    FocusTo(usize),
    // Active selected specified widget ID,
    ActiveTo(usize),
    /// Active press at surface (e.g touch or mouse press)
    Active(Point),
    /// Movement at surface event (e.g mouse moved to element)
    Move(Point),
    /// Dragging at surface event (e.g mouse press and move)
    Drag(Point),
    /// Increase the value in specified step in range 0.0-1.0, used for sliders
    Increase(f32),
    /// Decreases the value in specified step in range 0.0-1.0, used for sliders
    Decrease(f32),
}

impl SystemEvent {
    fn is_motion_event(&self) -> bool {
        matches!(self, SystemEvent::FocusTo(_) | SystemEvent::Move(_))
    }
}

/// Filtered to specified widget event
#[derive(Clone, Copy, PartialEq)]
pub enum Event {
    /// Idle event (None, Null) event
    Idle,
    /// Focus event. E.g hover from mouse or widget cycler (tab)
    Focus,
    // Active press at surface. E.g touch or mouse click
    Active(Option<Point>),
    Drag(Point),
}

/// Primary UI Context
pub struct UiContext<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    /// ``DrawTarget`` basically is display for drawing
    pub draw_target: D,
    /// Theme for widgets for this comtext
    pub theme: Theme<C>,
    /// Event to pass in the library
    event_queue: heapless::Vec<SystemEvent, 5>,
    /// Enable/disable debug mode - displays red rectangles around widget bounds
    pub debug_mode: bool,
    alert_shown: Rc<Cell<bool>>,
    alert_text: String,
    elements_count: usize,
    pub(crate) focused_element: usize,
    marker: PhantomData<&'a C>,
}

impl<'a, D, C> UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    /// Creates a new UI context with specified `DrawTaget` and `Theme`
    pub fn new(draw_target: D, theme: Theme<C>) -> Self {
        Self {
            elements_count: 0,
            draw_target,
            theme,
            event_queue: heapless::Vec::new(),
            focused_element: 0,
            debug_mode: false,
            alert_text: String::new(),
            alert_shown: Rc::new(Cell::new(false)),
            marker: PhantomData,
        }
    }

    pub fn push_event(&mut self, event: SystemEvent) {
        if self.event_queue.is_full() || !event.is_motion_event() {
            self.event_queue.clear();
        }

        if let Some(last_event) = self.event_queue.last() {
            if last_event == &event {
                return;
            }
        }

        self.event_queue.push(event).unwrap();
    }

    pub fn get_focused_widget_id(&self) -> usize {
        self.focused_element
    }

    pub fn consume_event(&mut self, event: &SystemEvent) {
        self.event_queue.retain(|f| f != event);
    }

    /// Cycles to next widget (like Tab key on PC)
    pub fn next_widget(&mut self) {
        if self.focused_element > self.elements_count - 1 {
            self.focused_element = 1;
        } else {
            self.focused_element += 1;
        }

        self.push_event(SystemEvent::FocusTo(self.focused_element));
    }

    /// Cycles to previous widget (like Shift+Tab key on PC)
    pub fn previous_widget(&mut self) {
        if self.focused_element != 0 {
            self.focused_element -= 1;
        }

        self.push_event(SystemEvent::FocusTo(self.focused_element));
    }

    /// Activates selected widget (like Enter key on PC)
    pub fn activate_selected_widget(&mut self) {
        self.push_event(SystemEvent::ActiveTo(self.focused_element));
    }

    pub fn dim_screen(&mut self) {
        let modal_style = self.theme.modal_style;

        let modal_background = modal_style
            .background_color
            .expect("Modal must have a background color for drawing");

        let bounds = self.draw_target.bounding_box();
        let size = bounds.size;
        for x in 0..size.width {
            for y in 0..size.height {
                if (x + y) % 2 == 0 {
                    let _ = Pixel(Point::new(x as i32, y as i32), modal_background)
                        .draw(&mut self.draw_target);
                }
            }
        }
    }

    pub fn alert<S: Into<String>>(&mut self, text: S) {
        self.alert_shown.set(true);
        self.alert_text = text.into();
    }

    pub fn dismiss_alerts(&mut self) {
        self.alert_shown.set(false);
    }

    /// Updates and draws the UI, probably you want run this in main loop
    pub fn update(&mut self, root: &mut WidgetObject<'a, D, C>) {
        let bounds = self.draw_target.bounding_box();
        self.elements_count = WIDGET_IDS.load(Ordering::Relaxed);
        WIDGET_IDS.store(0, Ordering::Relaxed);
        root.size(self, bounds.size);
        root.layout(self, bounds);

        if !self.alert_shown.get() {
            let event = *self.event_queue.last().unwrap_or(&SystemEvent::Idle);

            root.draw(self, &event);
            if !event.is_motion_event() {
                self.consume_event(&event);
            }
        } else {
            root.draw(self, &SystemEvent::Idle);
        }

        // alerts
        if self.alert_shown.get() {
            self.dim_screen();
            let bounds = self.draw_target.bounding_box();
            let mut layout = LinearLayoutBuilder::default()
                .direction(LayoutDirection::Vertical)
                .vertical_alignment(LayoutAlignment::Stretch)
                .horizontal_alignment(LayoutAlignment::Stretch)
                .style(self.theme.modal_style);

            let alert_shown = self.alert_shown.clone();

            // layout.margin_layout(margin!(5), |ui| {
            //     ui.grid_layout([100].into(), [25, 75].into(), |ui| {
            //         ui.add_widget(WarningTriangle::new_both_sizes(Size::new(16, 16), Size::new(32, 32)));
            //         ui.margin_layout(margin!(5), |ui| {
            //             ui.label(
            //                 &self.alert_text,
            //                 Alignment::Left,
            //                 MonoTextStyle::new(&FONT_4X6, self.theme.foreground),
            //             );
            //         });
            //     });
            // });

            layout.margin_layout(margin!(5), |ui| {
                ui.label(
                    &self.alert_text,
                    Alignment::Left,
                    MonoTextStyle::new(
                        &FONT_4X6,
                        self.theme
                            .modal_style
                            .foreground_color
                            .expect("Modal style must have a foreground color for drawing"),
                    ),
                );
            });

            layout.button("OK", &FONT_4X6, move || {
                alert_shown.set(false);
            });

            let mut obj = layout.finish();

            let size = obj.size(self, bounds.size);
            let modal_size = Size::new(
                size.width.min(bounds.size.width),
                size.height.min(bounds.size.height),
            );

            obj.layout(
                self,
                Rectangle::new(
                    bounds.center() - Rectangle::new(Point::zero(), modal_size).center(),
                    modal_size,
                ),
            );

            // TODO: Event handling for alert
            let event = *self.event_queue.last().unwrap_or(&SystemEvent::Idle);
            obj.draw(self, &event);
            if !event.is_motion_event() {
                self.consume_event(&event);
            }
        }
    }
}
