use std::{num::NonZeroU32, rc::Rc};

use edgy_ui::{
    SystemEvent, UiContext, graphics::{PixelFormat, framebuffer::FrameBuffer, geometry::{Point, Size}},
};
use softbuffer::Surface;
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, error::EventLoopError, event::{ElementState, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::{Window, WindowId},
};

pub mod fonts;

/// Contains a few potential properties to set for a SoftbufferWindow when it is created.
pub struct WindowProperties {
    pub size: PhysicalSize<u32>,
    pub title: Box<str>,
}

impl Default for WindowProperties {
    fn default() -> WindowProperties {
        WindowProperties {
            size: PhysicalSize::new(800, 600),
            title: "a bit edgy ui".into(),
        }
    }
}

impl WindowProperties {
    pub fn new(width: u32, height: u32, title: &str) -> WindowProperties {
        WindowProperties {
            size: PhysicalSize::new(width, height),
            title: title.into(),
        }
    }
}

struct State<M> {
    window: Rc<Window>,
    surface: Surface<Rc<Window>, Rc<Window>>,
    ui_context: UiContext<M>,
}

impl<M> State<M> {
    fn resize(&mut self, new_size: Size) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface
                .resize(
                    NonZeroU32::new(new_size.width).unwrap(),
                    NonZeroU32::new(new_size.height).unwrap(),
                )
                .unwrap();
            self.ui_context
                .framebuffer
                .resize(Size::new(new_size.width, new_size.height));
        }
    }
}

/// Wrapper for Softbuffer and a Winit window
pub struct SoftbufferWindow<M> {
    state: Option<State<M>>,
    update_fn: Box<dyn FnMut(&mut UiContext<M>) -> ()>,
    palette: Vec<u32>,
    properties: WindowProperties,
    cursor_pos: Point,
}

impl<'a, M> ApplicationHandler for SoftbufferWindow<M> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let window = event_loop.create_window(
                Window::default_attributes()
                    .with_title(self.properties.title.clone())
                    .with_inner_size(self.properties.size),
            );
            Rc::new(window.unwrap())
        };
        let size = window.inner_size();
        let context =
            softbuffer::Context::new(window.clone()).expect("Failed to create softbuffer context");
        let framebuffer =
            FrameBuffer::new(size.width as u16, size.height as u16, PixelFormat::Bpp8);

        self.state = Some(State {
            window: window.clone(),
            surface: Surface::new(&context, window.clone()).expect("Failed to create surface"),
            ui_context: UiContext::new(framebuffer),
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        //println!("{:?}", event);
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                let x = position.x as i32;
                let y = position.y as i32;
                let state = self.state.as_mut().unwrap();
                self.cursor_pos = Point::new(x, y);
                state.ui_context.push_event(SystemEvent::PointerMove(self.cursor_pos));
            }

            WindowEvent::MouseInput { device_id, state, button } => {
                let window_state = self.state.as_mut().unwrap();
                if state == ElementState::Pressed {
                    window_state.ui_context.push_event(SystemEvent::PointerDown(self.cursor_pos));
                } else {
                    window_state.ui_context.push_event(SystemEvent::PointerUp(self.cursor_pos));
                }

            }

            WindowEvent::Resized(new_size) => {
                let state = self.state.as_mut().unwrap();
                state.resize(Size::new(new_size.width, new_size.height));
                self.properties.size = new_size;
            }

            WindowEvent::RedrawRequested => {
                self.render();
            }

            _ => {}
        }
    }
}

impl<'a, M> SoftbufferWindow<M> {
    pub fn new(properties: WindowProperties, palette: Vec<u32>) -> SoftbufferWindow<M> {
        SoftbufferWindow {
            state: None,
            palette,
            properties,
            update_fn: Box::new(|_| {}),
            cursor_pos: Point::default()
        }
    }

    /// Runs a SoftbufferWindow event loop.
    pub fn run(
        &mut self,
        update_fn: impl FnMut(&mut UiContext<M>) + 'static,
    ) -> Result<(), EventLoopError> {
        self.update_fn = Box::new(update_fn);
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)
    }

    fn render(&mut self) {
        let state = self.state.as_mut().unwrap();
        (self.update_fn)(&mut state.ui_context);
        let mut buffer = state.surface.buffer_mut().unwrap();

        for (dst, &src) in buffer
            .iter_mut()
            .zip(state.ui_context.framebuffer.data.iter())
        {
            *dst = self.palette[src as usize];
        }

        buffer.present().unwrap();
        state.window.request_redraw();
    }
}
