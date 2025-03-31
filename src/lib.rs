#![no_std]
//! edgy - no_std immediate-mode GUI library for microcontrollers. It uses ``embedded_graphics`` for
//! rendering and some types like ``Color`` or ``Rectangle``. Library uses ``alloc`` for widget
//! dynamic dispatch, threfore a allocator is required.
use alloc::{boxed::Box, rc::Rc, string::String};
use core::{
    cell::RefCell,
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering},
    u32,
};
pub use embedded_graphics;
use themes::Theme;

use embedded_graphics::{prelude::*, primitives::Rectangle};
use widgets::{
    alert::Alert,
    root_layout::{Anchor, RootLayout},
    WidgetObject,
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
    C: PixelColor + 'static,
{
    /// ``DrawTarget`` basically is display for drawing
    pub draw_target: D,
    /// Theme for widgets for this comtext
    pub theme: Theme<C>,
    /// Event to pass in the library
    event_queue: heapless::Vec<SystemEvent, 5>,
    /// Enable/disable debug mode - displays red rectangles around widget bounds
    pub debug_mode: bool,
    alert_text: Rc<RefCell<String>>,
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
            alert_text: Rc::new(RefCell::new(String::new())),
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
        let mut borrow = self.alert_text.borrow_mut();
        *borrow = text.into();
    }

    pub fn dismiss_alerts(&mut self) {
        let mut borrow = self.alert_text.borrow_mut();
        *borrow = String::new();
    }

    /// Updates and draws the UI, probably you want run this in main loop
    pub fn update(&mut self, root: WidgetObject<'a, D, C>) {
        self.elements_count = WIDGET_IDS.load(Ordering::Relaxed);
        WIDGET_IDS.store(0, Ordering::Relaxed);
        let bounds = self.draw_target.bounding_box();
        let event = *self.event_queue.last().unwrap_or(&SystemEvent::Idle);

        let alert_shown = !self.alert_text.borrow().is_empty();
        let mut root_layout = RootLayout::new();
        root_layout.add_widget_obj(root, bounds, !alert_shown, Anchor::TopLeft);

        if alert_shown {
            let alert_text = self.alert_text.clone();
            let alert_msg = alert_text.borrow().clone();

            let alert = Alert::new(
                alert_msg,
                self.theme.modal_style,
                Box::new(move || {
                    alert_text.take();
                }),
            );

            root_layout.add_widget_obj(
                WidgetObject::new(Box::new(alert)),
                Rectangle::new(bounds.center(), Size::zero()),
                true,
                Anchor::Center,
            );
        }

        let mut root_layout = root_layout.finish();
        root_layout.size(self, bounds.size);
        root_layout.layout(self, bounds);

        root_layout.draw(self, &event);
        if !event.is_motion_event() {
            self.consume_event(&event);
        }
    }
}
