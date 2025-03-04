//#![no_std]
//! edgy - no_std immediate-mode GUI library for microcontrollers. It uses ``embedded_graphics`` for
//! rendering and some types like ``Color`` or ``Rectangle``. Library uses ``alloc`` for widget
//! dynamic dispatch, threfore a allocator is required.
use alloc::{rc::Rc, string::String};
use core::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
};
pub use embedded_graphics;
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Alignment,
};
use widgets::{
    linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder}, UiBuilder, WidgetObj
};

pub mod widgets;
extern crate alloc;

pub(crate) static WIDGET_IDS: AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

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
}

impl SystemEvent {
    fn is_motion_event(&self) -> bool {
        match self {
            SystemEvent::FocusTo(_) => true,
            SystemEvent::Move(_) => true,
            _ => false,
        }
    }
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

/// Theme struct. You can freely create own themes
#[derive(Clone, Copy)]
pub struct Theme<C: PixelColor> {
    /// Primary background
    pub background: C,
    pub background2: C,
    pub background3: C,
    /// Primary foreground
    pub foreground: C,
    pub foreground2: C,
    pub foreground3: C,
    pub debug_rect: C,
    pub success: C,
    pub warning: C,
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
            success: Rgb888::new(79, 113, 75).into(),
            warning: Rgb888::new(128, 126, 83).into(),
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
            success: Rgb888::WHITE.into(),
            warning: Rgb888::WHITE.into(),
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
    /// Theme for widgets for this comtext
    pub theme: Theme<C>,
    /// Event to pass in the library
    event_queue: heapless::Vec<SystemEvent, 2>,
    /// Enable/disable debug mode - displays red rectangles around widget bounds
    pub debug_mode: bool,
    alert_shown: Rc<Cell<bool>>,
    alert_text: String,
    elements_count: usize,
    focused_element: usize,
}

impl<'a, D, C> UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    /// Creates a new UI context with specified `DrawTaget` and `Theme`
    pub fn new(draw_target: &'a mut D, theme: Theme<C>) -> Self {
        Self {
            elements_count: 0,
            draw_target,
            theme,
            event_queue: heapless::Vec::new(),
            focused_element: 0,
            debug_mode: false,
            alert_text: String::new(),
            alert_shown: Rc::new(Cell::new(false)),
        }
    }

    pub fn push_event(&mut self, event: SystemEvent) {
        if self.event_queue.is_full() || event.is_motion_event() {
            self.event_queue.clear();
        }

        self.event_queue.push(event).unwrap();
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
        let bounds = self.draw_target.bounding_box();
        let size = bounds.size;
        for x in 0..size.width {
            for y in 0..size.height {
                if (x + y) % 2 == 0 {
                    let _ = Pixel(Point::new(x as i32, y as i32), self.theme.background)
                        .draw(self.draw_target);
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
    pub fn update(&mut self, root: &mut WidgetObj<'a, D, C>) {
        let bounds = self.draw_target.bounding_box();
        self.elements_count = WIDGET_IDS.load(Ordering::Relaxed);
        WIDGET_IDS.store(0, Ordering::Relaxed);
        root.size(self, bounds.size);
        root.layout(self, bounds);

        if !self.event_queue.is_empty() && !self.alert_shown.get() {
            let event = self.event_queue[self.event_queue.len() - 1];

            if root.handle_event(self, &event) == EventResult::Stop && !event.is_motion_event() {
                self.consume_event(&event);
            }
        }

        root.draw(self);


        // alerts
        if self.alert_shown.get() {
            self.dim_screen();
            let bounds = self.draw_target.bounding_box();
            let mut layout = LinearLayoutBuilder::default()
                .direction(LayoutDirection::Vertical)
                .vertical_alignment(LayoutAlignment::Stretch)
                .horizontal_alignment(LayoutAlignment::Stretch)
                .style(
                    PrimitiveStyleBuilder::new()
                        .fill_color(self.theme.background)
                        .stroke_color(self.theme.background2)
                        .stroke_width(2)
                        .build(),
                );

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
                    MonoTextStyle::new(&FONT_4X6, self.theme.foreground),
                );
            });

            layout.button("OK", &FONT_4X6, move || {
                alert_shown.set(false);
            });

            let mut obj = layout.finish();

            let size= obj.size(self, bounds.size);
            let modal_size = Size::new(
                size.width.min(bounds.size.width),
                size.height.min(bounds.size.height),
            );

            obj.layout(self, Rectangle::new(bounds.center() - Rectangle::new(Point::zero(), modal_size).center(), modal_size));

            if !self.event_queue.is_empty() {
                let event = self.event_queue[self.event_queue.len() - 1];
                if obj.handle_event(self, &event) == EventResult::Stop && !event.is_motion_event() {
                    self.consume_event(&event);
                }
            }

            obj.draw(self);
        }
    }
}
