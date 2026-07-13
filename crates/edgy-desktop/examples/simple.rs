use edgy_desktop::{SoftbufferWindow, WindowProperties, fonts::govno::UNSCII};
use edgy_ui::{NodeId, Prop, UiContext, graphics::{framebuffer::FrameBuffer, geometry::Size, parse_palette_rgb888}, widgets::label::TypographyLabel};

const COLORS: [u32; 256] = parse_palette_rgb888::<256>(include_str!("vga13h.hex"));

struct App {
    ui: UiContext<i32>,

    temperature_label: NodeId,
    status_label: NodeId,

    connected: bool,
    temperature: f32,
}

impl App {
    pub fn new(size: Size) -> Self {
        let mut ui = UiContext::new(size);

        let mut temperature_label = NodeId::default();
        let mut status_label = NodeId::default();

        ui.build(|b| {
            b.column(|b| {
                temperature_label = b.add(
                    TypographyLabel::new(
                        Prop::Value("Temperature: --".into()),
                        Prop::Value(&UNSCII),
                        Prop::Value(4),
                    )
                );

                status_label = b.add(
                    TypographyLabel::new(
                        Prop::Value("Connected".into()),
                        Prop::Value(&UNSCII),
                        Prop::Value(4),
                    )
                );
            });
        });

        Self {
            ui,
            temperature_label,
            status_label,
            connected: true,
            temperature: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.temperature += 0.1;

        self.ui.modify::<TypographyLabel>(self.temperature_label, |label| {
            label.text = Prop::Value(format!("Temperature: {:.1} °C", self.temperature));
        });

        if self.ui.tree.contains_key(self.status_label) && !self.connected {
            self.ui.remove(self.status_label);
        }
    }

    pub fn render(&mut self, framebuffer: &mut FrameBuffer) {
        self.ui.resize(Size::new(
            framebuffer.width() as u32,
            framebuffer.height() as u32,
        ));

        self.ui.layout();
        self.ui.draw(framebuffer);
    }
}

fn main() {
    let mut window =
        SoftbufferWindow::new(WindowProperties::default(), COLORS.to_vec());

    let mut app = App::new(Size::new(800, 600));

    window.run(move |fb| {
        fb.clear();

        app.update();
        app.render(fb);
    });
}