use edgy_graphics::geometry::Rectangle;

use crate::{Event, EventDispatcher, EventResult, SystemEvent, widgets::Behavior};

pub(crate) fn handle_system_event<B: Behavior>(
    behavior: &mut B,
    rect: Rectangle,
    dispatcher: &EventDispatcher,
) -> EventResult {
    match dispatcher.current() {
        SystemEvent::PointerMove(point) if rect.contains(point) => {
            behavior.handle(Event::HoverEnter)
        }

        SystemEvent::PointerDown(point) if rect.contains(point) => {
            behavior.handle(Event::Press)
        }

        SystemEvent::PointerUp(point) if rect.contains(point) => {
            behavior.handle(Event::Release)
        }

        _ => EventResult::Pass,
    }
}