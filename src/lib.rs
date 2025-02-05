#![no_std]
use core::sync::atomic::{AtomicUsize, Ordering};

use alloc::{collections::btree_map::BTreeMap, vec::Vec};
/// # Edgy
/// no_std immediate-mode GUI library for microcontrollers. It uses ``embedded_graphics`` for
/// rendering and some types like ``Color`` or ``Rectangle``. Library uses ``alloc`` for widget
/// dynamic dispatch, threfore a allocator is required.

pub use embedded_graphics;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::Rectangle};
use widgets::{Widget, WidgetObj};

pub mod widgets;
extern crate alloc;

pub static WIDGET_IDS: AtomicUsize = core::sync::atomic::AtomicUsize::new(1);

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
    /// Hover at surface event (e.g mouse moved to element)
    Hover(Point),
}


/// Event that can widget recieve
#[derive(Clone, Copy, PartialEq)]
pub enum Event {
    /// Idle event (None, Null) event
    Idle,
    /// Focus event. E.g hover from mouse or widget cycler (tab)
    Focus,
    // Active press at surface. E.g touch or mouse click
    Active,
}

pub(crate) fn contains(rectangle: Rectangle, position: Point) -> bool {
    rectangle.top_left.x < position.x
        && position.x < rectangle.top_left.x + rectangle.size.width as i32
        && rectangle.top_left.y < position.y
        && position.y < rectangle.top_left.y + rectangle.size.height as i32
}

/// Theme struct. You can freely create own themes
#[derive(Clone, Copy)]
pub struct Theme<C: PixelColor> {
    pub background: C,
    pub background2: C,
    pub background3: C,
    pub foreground: C,
    pub foreground2: C,
    pub foreground3: C,
    pub debug_rect: C,
}

impl<C: PixelColor + From<Rgb888>> Theme<C> {
    /// Hope diamond theme - recommended for color screens
    pub fn hope_diamond() -> Self {
        Self {
            background: Rgb888::new(21, 14, 16).into(),
            background2: Rgb888::new(39, 39, 57).into(),
            background3: Rgb888::new(57, 56, 73).into(),
            foreground: Rgb888::new(119, 136, 140).into(),
            foreground2: Rgb888::new(79, 90, 100).into(),
            foreground3: Rgb888::new(59, 65, 82).into(),
            debug_rect: Rgb888::RED.into(),
        }
    }
    
    /// Binary color theme - recommended for 1-bit displays, like OLED SSD1306
    pub fn binary() -> Self {
        Self {
            background: Rgb888::BLACK.into(),
            background2: Rgb888::BLACK.into(),
            background3: Rgb888::BLACK.into(),
            foreground: Rgb888::WHITE.into(),
            foreground2: Rgb888::WHITE.into(),
            foreground3: Rgb888::WHITE.into(),
            debug_rect: Rgb888::WHITE.into(),
        }
    }
}

/// Primary UI Context
pub struct UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    /// ``DrawTarget`` basically is display for drawing
    pub draw_target: &'a mut D,
    /// Rectangular bounds. Probaly you need to pass display dimensions 
    pub bounds: Rectangle,
    /// Theme for widgets for this comtext
    pub theme: Theme<C>,
    /// Event to pass in the library
    pub last_event: SystemEvent,
    /// Enable/disable debug mode - displays red rectangles around widget bounds
    pub debug_mode: bool,
    pub elements_count: usize,
    pub focused_element: usize,
}

impl<'a, D, C> UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    /// Creates a new UI context
    pub fn new(draw_target: &'a mut D, bounds: Rectangle, theme: Theme<C>) -> Self {
        Self {
            elements_count: 0,
            draw_target,
            bounds,
            theme,
            last_event: SystemEvent::Idle,
            focused_element: 0,
            debug_mode: false,
        }
    }

    pub fn next_widget(&mut self) {
        if self.focused_element > self.elements_count - 1 {
            self.focused_element = 1;
        } else {
            self.focused_element += 1;
        }

        self.last_event = SystemEvent::FocusTo(self.focused_element);
    }

    pub fn activate_selected_widget(&mut self) {
        self.last_event = SystemEvent::ActiveTo(self.focused_element);
    }

    /// Updates and draws the UI, probably you want run this in some loop
    pub fn update(&mut self, root: &mut WidgetObj<'a, D, C>) {
        let event = self.last_event;
        self.elements_count = WIDGET_IDS.load(Ordering::Relaxed);
        WIDGET_IDS.store(0, Ordering::Relaxed);
        root.size(self, self.bounds.size);
        root.layout(self, self.bounds);
        root.handle_event(self, &event);

        root.draw(self);
    }
}
