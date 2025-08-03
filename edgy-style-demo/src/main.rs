use edgy_style_derive::css;
use edgy;

fn main() {
    let rules = css! {
        button {
            background_color: Rgb888::BLUE;
        }

        .danger {
            background_color: Rgb888::RED;
        }

        #main:focus {
            font: &FONT_4X6;
        }
    };
}
