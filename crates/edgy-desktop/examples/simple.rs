use std::time::{Duration, Instant};

use edgy_desktop::{SoftbufferWindow, WindowProperties, fonts::{govno::GOVNO}};
use edgy_ui::{Event, decorators::{self, ViewExt}, graphics::{self, Color, draw::{self, BasicStyle}, framebuffer::FrameBuffer, geometry::{Point, Rectangle, Size}, parse_palette_rgb888, text::{self, LayoutOptions}}, margin, widgets::{Behavior, NullBehavior, View, WidgetObject, label::{self, TypographyLabel}, linear_layout::{LayoutAlignment, LinearLayout}}};


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
    pub hover_color: Color,
}

impl<'a> LinkLabel<'a> {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            base: TypographyLabel::new(&GOVNO, text, LayoutOptions::default(), 2),
            underline: true,
            hover_color: 1,
        }
    }
}

impl<'a> View for LinkLabel<'a> {
    type State = LinkState;

    fn measure(&mut self, hint: Size) -> Size {
        self.base.measure(hint)
    }

    fn layout(&mut self, rect: Rectangle, state: &LinkState) -> Rectangle {
        let color = match state {
            LinkState::Normal => self.base.color,
            LinkState::Hovered => self.hover_color,
        }; 

        self.base.color = color;
        rect
    }

    fn draw(
        &mut self,
        framebuffer: &mut FrameBuffer,
        rect: Rectangle,
        state: &LinkState,
    ) {
        self.base.draw(framebuffer, rect, state);
        // draw::rect(framebuffer, rect, BasicStyle::with_border(4, 1));

        if self.underline {
            let y = rect.top_left.y + rect.size.height as i32;
            
            draw::line(
                framebuffer,
                Point::new(rect.top_left.x, y),
                Point::new(rect.top_left.x + rect.size.width as i32, y),
                1,
                1,
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
        let options = LayoutOptions {
            wrap: text::Wrap::Word,
            horizontal: text::HorizontalAlign::Left,
            ..Default::default()
        };
        let linear_layout = LinearLayout::new_vertical(LayoutAlignment::Start, LayoutAlignment::Start, 5, |ui| {
           ui.add(label::typography_label(&GOVNO, "The str type, also called a ‘string slice’, is the most primitive string type. It is usually seen in its borrowed form, &str. It is also the type of string literals, &'static str.".into(), options, 14));
           ui.add(WidgetObject::new(StateBehavior::new(LinkState::Hovered), LinkLabel::new(String::from("CLICK HERE FOR FREE COOKIES!"))));
           ui.add(label::typography_label(&GOVNO, "Here we have declared a string slice initialized with a string literal. String literals have a static lifetime, which means the string hello_world is guaranteed to be valid for the duration of the entire program. We can explicitly specify hello_world’s lifetime as well:".into(), options, 14));
           ui.add(WidgetObject::new(NullBehavior, label::TypographyLabel::new(&GOVNO, "let hello_world: &'static str = \"Hello, world!\";", options, 15).background(BasicStyle::with_fill(7)).margin(margin!(20))));
        });
        
        ctx.update(WidgetObject::new(NullBehavior, linear_layout.background(BasicStyle::with_fill(8))));
        graphics::draw::text(&mut ctx.framebuffer, Point::new(10, 10), &GOVNO, &format!("FPS: {}", fps), 3);

    }).unwrap();
}