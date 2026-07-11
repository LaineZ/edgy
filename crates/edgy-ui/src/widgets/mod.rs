
use alloc::{boxed::Box, vec::Vec};
use edgy_graphics::{
    draw::{self, BasicStyle},
    framebuffer::FrameBuffer,
    geometry::{Rectangle, Size},
};

use crate::{Event, EventDispatcher, EventResult, IdGenerator, StateStorage, SystemEvent, WidgetId};

pub mod label;
pub mod linear_layout;
pub mod root_layout;

pub trait Behavior {
    type State: Default + 'static;
    fn handle(&mut self, state: &mut Self::State, event: Event) -> EventResult {
        EventResult::Pass
    }
}

pub trait View {
    type State;

    /// Returns the size the widget wants. use for auto-calculate in layouts. Default implementation occupies all available space
    fn measure(&mut self, hint: Size) -> Size {
        hint
    }

    /// Returns a minimum size of widget
    fn min_size(&mut self) -> Size {
        Size::zero()
    }

    /// Returs a maximum size of widget
    fn max_size(&mut self) -> Size {
        Size::new(u32::MAX, u32::MAX)
    }

    /// Calls at layout pass. Gives a try for layout computation in Layouts (Containers)
    fn layout(&mut self, _rect: Rectangle, _state: &Self::State) {}

    fn draw(&mut self, framebuffer: &mut FrameBuffer, rect: Rectangle, state: &Self::State);
}

pub trait Widget {
    fn init(&mut self, ids: &mut IdGenerator, storage: &mut StateStorage) -> WidgetId;
    fn measure(&mut self, hint: Size) -> Size;
    fn layout(&mut self, rect: Rectangle, storage: &mut StateStorage);
    fn draw(&mut self, fb: &mut FrameBuffer, storage: &mut StateStorage);
    fn handle_system_event(&mut self, storage: &mut StateStorage, dispatcher: &mut EventDispatcher) -> EventResult;
    fn rect(&self) -> Rectangle;
    fn set_rect(&mut self, rect: Rectangle);
}

pub struct WidgetObject<B, V>
where
    B: Behavior,
    V: View<State = B::State>,
{
    behavior: B,
    view: V,
    computed_rect: Rectangle,
    id: WidgetId,
}

impl<B, V> WidgetObject<B, V>
where
    B: Behavior,
    V: View<State = B::State>,
{
    pub fn new(behavior: B, view: V) -> Self {
        Self {
            behavior,
            view,
            id: WidgetId::default(),
            computed_rect: Rectangle::zero(),
        }
    }
}

impl<B, V> Widget for WidgetObject<B, V>
where
    B: Behavior,
    V: View<State = B::State>,
{

    fn init(&mut self, id: &mut IdGenerator, _storage: &mut crate::StateStorage) -> WidgetId {
        self.id = id.next();
        self.id
    }
    
    fn measure(&mut self, hint: Size) -> Size {
        self.view.measure(hint)
    }

    fn layout(&mut self, rect: Rectangle, storage: &mut crate::StateStorage) {
        self.view.layout(rect, storage.get_or_insert::<B::State>(self.id));
        self.set_rect(rect);
    }


    fn handle_system_event(
        &mut self,
        storage: &mut StateStorage,
        dispatcher: &mut EventDispatcher,
    ) -> EventResult {
        let state = storage.get_or_insert::<B::State>(self.id);

        match dispatcher.current() {
            SystemEvent::PointerMove(point) if !dispatcher.pointer_down => {
                let contains = self.rect().contains(point);
                let was_hovered = dispatcher.old_hovered_widget == Some(self.id);
                let already_claimed = dispatcher.hovered_widget.is_some();

                if contains && !was_hovered && !already_claimed {
                    dispatcher.hovered_widget = Some(self.id);
                    self.behavior.handle(state, Event::HoverEnter);
                    EventResult::Pass
                } else if contains && was_hovered {
                    dispatcher.hovered_widget = Some(self.id);
                    EventResult::Pass
                } else if !contains && was_hovered {
                    self.behavior.handle(state, Event::HoverLeave);
                    EventResult::Pass
                } else {
                    EventResult::Pass
                }
            }

            SystemEvent::PointerDown(point) => {
                if self.rect().contains(point) {
                    self.behavior.handle(state, Event::Press);
                    EventResult::Stop
                } else {
                    EventResult::Pass
                }
            }

            SystemEvent::PointerUp(point) => {
                if self.rect().contains(point) {
                    self.behavior.handle(state, Event::Release);
                    EventResult::Stop
                } else {
                    EventResult::Pass
                }
            }

            _ => EventResult::Pass,
        }
    }

    fn draw(&mut self, fb: &mut FrameBuffer, storage: &mut crate::StateStorage) {
        let state = storage.get_or_insert::<B::State>(self.id);
        self.view.draw(fb, self.rect(), &state);

        draw::rect(fb, self.computed_rect, BasicStyle::with_border(40, 1));
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.computed_rect = rect;
    }

    fn rect(&self) -> Rectangle {
        self.computed_rect
    }
}

impl<T: Widget + 'static> From<T> for Box<dyn Widget> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

/// No behavior for widget
pub struct NullBehavior;

impl Behavior for NullBehavior {
    type State = ();
}


/// No view for widget, useful for containers
pub struct NullView;

impl View for NullView {
    type State = ();
    
    fn draw(&mut self, _: &mut FrameBuffer, _: Rectangle, _: &Self::State) {}
}

pub struct Ui<'a> {
    children: &'a mut Vec<Box<dyn Widget>>,
}

impl<'a> Ui<'a> {
    pub fn add<W: Into<Box<dyn Widget>>>(&mut self, widget: W) {
        self.children.push(widget.into());
    }

    // TODO: Add more widgets here
}
