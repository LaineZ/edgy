#![no_std]
/// # Edgy
/// no_std immediate-mode GUI library for microcontrollers. It uses ``embedded_graphics`` for
/// rendering and some types like ``Color`` or ``Rectangle``. Library uses ``alloc`` for widget
/// dynamic dispatch, threfore a allocator is required.

pub use embedded_graphics;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::Rectangle};
use widgets::WidgetObj;

pub mod widgets;
extern crate alloc;

/// Event result struct
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventResult {
    /// Event processed
    Stop,
    /// Event passed, trying next widget
    Pass,
}


/// Events that can be inserted into UI context
#[derive(Clone, Copy)]
pub enum Event {
    /// Idle event (None, Null) event
    Idle,
    /// Focus to the next widget
    NextWidgetFocus,
    /// Focus to the previous widget
    PreviousWidgetFocus,
    /// Active press at surface (e.g touch or mouse press)
    Active(Point),
    /// Hover at surface event (e.g mouse moved to element)
    Hover(Point),
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
    pub last_event: Event,
    /// Enable/disable debug mode - displays red rectangles around widget bounds
    pub debug_mode: bool,
}

impl<'a, D, C> UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    /// Creates a new UI context
    pub fn new(draw_target: &'a mut D, bounds: Rectangle, theme: Theme<C>) -> Self {
        Self {
            draw_target,
            bounds,
            theme,
            last_event: Event::Idle,
            debug_mode: false,
        }
    }

    /// Updates and draws the UI, probably you want run this in some loop
    pub fn update(&mut self, root: &mut WidgetObj<'a, D, C>) {
        let event = self.last_event;
        root.size(self, self.bounds.size);
        root.layout(self, self.bounds);
        root.handle_event(self, &event);

        root.draw(self);
        self.last_event = Event::Idle;
    }
}
