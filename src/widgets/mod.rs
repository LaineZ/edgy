use embedded_graphics::primitives::Rectangle;

pub mod label;

pub enum Response {
    None,
    Pressed
}

pub trait Widget {
    fn size(&self, hint: Rectangle) -> Rectangle;
    fn layout(&mut self, hint: Rectangle);
    fn response() -> Response {
        Response::None
    }
}