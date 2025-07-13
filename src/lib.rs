#![no_std]
//! edgy - no_std immediate-mode GUI library for microcontrollers. It uses ``embedded_graphics`` for
//! rendering and some types like ``Color`` or ``Rectangle``. Library uses ``alloc`` for widget
//! dynamic dispatch, threfore a allocator is required.
use alloc::{boxed::Box, rc::Rc, string::String, vec::{self, Vec}};
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

use crate::style::{Selector, Style, StyleRule, StyleSheet, Tag};

// pub use embedded_graphics::primitives::Rectangle as Rectangle;
// pub use embedded_graphics::geometry::Point as Point;
// pub use embedded_graphics::geometry::Size as Size;

pub mod prelude;
pub mod style;
pub mod styles;
pub mod themes;
pub mod widgets;

extern crate alloc;

pub(crate) static WIDGET_IDS: AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
pub const MAX_SIZE: Size = Size::new(u32::MAX, u32::MAX);
pub const MIN_SIZE: Size = Size::zero();

pub struct DebugOptions {
    pub enabled: bool,
    pub widget_rects: bool,
    pub widget_rect_active: bool,
    pub widget_sizes: bool,
    pub widget_ids: bool,
}

impl Default for DebugOptions {
    fn default() -> Self {
        DebugOptions {
            enabled: false,
            widget_rects: true,
            widget_rect_active: true,
            widget_sizes: false,
            widget_ids: false,
        }
    }
}

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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    pub stylesheet: StyleSheet<'a, C>,
    /// Event to pass in the library
    motion_event: SystemEvent,
    interaction_event: SystemEvent,
    debug_options: Rc<RefCell<DebugOptions>>,
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
            stylesheet: Vec::new(),
            motion_event: SystemEvent::Idle,
            interaction_event: SystemEvent::Idle,
            focused_element: 0,
            debug_options: Rc::new(RefCell::new(DebugOptions::default())),
            alert_text: Rc::new(RefCell::new(String::new())),
            marker: PhantomData,
        }
    }

    pub fn push_event(&mut self, event: SystemEvent) {
        if event.is_motion_event() {
            self.motion_event = event;
        } else {
            self.interaction_event = event;
        }
    }

    pub fn get_focused_widget_id(&self) -> usize {
        self.focused_element
    }

    /// Cycles to next widget (like Tab key on PC)
    pub fn next_widget(&mut self) {
        if self.focused_element >= self.elements_count - 1 {
            self.focused_element = 1;
        } else {
            self.focused_element += 1;
        }
        self.push_event(SystemEvent::FocusTo(self.focused_element));
    }

    /// Cycles to previous widget (like Shift+Tab key on PC)
    pub fn previous_widget(&mut self) {
        if self.focused_element <= 1 {
            self.focused_element = self.elements_count - 1;
        } else {
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

    pub fn toggle_debug_mode(&mut self) {
        let mut debug_options = self.debug_options.borrow_mut();

        debug_options.enabled = !debug_options.enabled;
    }

    pub fn is_debug_enaled(&self) -> bool {
        self.debug_options.borrow().enabled
    }

    /// Updates and draws the UI, probably you want run this in main loop
    pub fn update(&mut self, root: WidgetObject<'a, D, C>) {
        self.elements_count = WIDGET_IDS.load(Ordering::Relaxed);
        WIDGET_IDS.store(1, Ordering::Relaxed);
        let bounds = self.draw_target.bounding_box();

        let alert_shown = !self.alert_text.borrow().is_empty();
        //let debug_options_enaled = self.debug_options.borrow().enabled;

        let mut root_layout = RootLayout::new();
        root_layout.add_widget_obj(root, bounds, !alert_shown, Anchor::TopLeft);

        // if debug_options_enaled {
        //     let debug_options = self.debug_options.clone();
        //     let debug_pos = Point::new(self.draw_target.bounding_box().size.width as i32 - 100, 2);
        //     root_layout.add_widget_obj(debug_options_ui(debug_options, self.focused_element), Rectangle::new(debug_pos, Size::zero()), true, Anchor::TopLeft);
        // }

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

        if self.interaction_event == SystemEvent::Idle {
            root_layout.draw(self, &self.motion_event.clone());
        } else {
            root_layout.draw(self, &self.interaction_event.clone());
            self.interaction_event = SystemEvent::Idle;
        }
    }
}
