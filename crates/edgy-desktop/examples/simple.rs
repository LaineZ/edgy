use std::time::{Duration, Instant};

use edgy_desktop::{SoftbufferWindow, WindowProperties, fonts::unscii::UNSCII};
use edgy_ui::{Event, edgy_graphics::{draw, framebuffer::FrameBuffer, geometry::Rectangle, parse_palette_rgb888}, widgets::{Behavior, NullBehavior, View, WidgetObject, label::{TypographyLabel, typography_label}}};


const COLORS: [u32; 256] = parse_palette_rgb888::<256>(include_str!("vga13h.hex"));
#[derive(Clone, Copy)]
pub enum LinkState {
    Normal,
    Hovered
}

pub struct StateBehavior<S> {
    state: S,
}

impl<S> StateBehavior<S> {
    pub fn new(state: S) -> Self {
        Self { state }
    }
}

impl<S> Behavior for StateBehavior<S> {
    type State = S;

    fn handle(&mut self, _event: Event) {}

    fn state(&self) -> &S {
        &self.state
    }
}

pub struct LinkLabel<'a> {
    pub base: TypographyLabel<'a, LinkState>,
    pub underline: bool,
    pub hover_color: u8,
}

impl<'a> LinkLabel<'a> {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            base: TypographyLabel::new(&UNSCII, text, 2),
            underline: false,
            hover_color: 1,
        }
    }
}

impl<'a> View for LinkLabel<'a> {
    type State = LinkState;

    fn layout(&mut self, _rect: Rectangle, state: &LinkState) {
        let color = match state {
            LinkState::Normal => self.base.color,
            LinkState::Hovered => self.hover_color,
        }; 

        self.base.color = color;
    }

    fn draw(
        &self,
        framebuffer: &mut FrameBuffer,
        rect: Rectangle,
        state: &LinkState,
    ) {
        self.base.draw(framebuffer, rect, state);

        if self.underline {
            draw::line(
                framebuffer,
                rect.top_left,
                rect.top_right(),
                1,
                1
            );
        }
    }
}

fn main() {
    let mut window = SoftbufferWindow::<i32>::new(WindowProperties::default(), COLORS.to_vec());
    let mut frames = 0;
    let mut fps = 0;
    let mut last = Instant::now();
    
    window.run(move |ctx| {
        frames += 1;

        if last.elapsed() >= Duration::from_secs(1) {
            fps = frames;
            frames = 0;
            last = Instant::now();
        }

        ctx.framebuffer.clear();

        
        let link_label = WidgetObject::new(StateBehavior {
            state: LinkState::Normal
        }, LinkLabel::new(String::from("pizda")));

        let root = link_label;
        ctx.update(root);
    }).unwrap();
}