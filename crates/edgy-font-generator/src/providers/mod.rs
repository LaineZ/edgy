use edgy_graphics::font::Glyph;

pub mod bdf;
pub mod fontdue;

#[derive(Clone, Copy)]
pub struct LineMetrics {
    pub line_height: u8,
    pub ascent: u8,
    pub descent: u8
}

pub trait FontRasterizerProvider {
    fn rasterize(&self, character: char) -> Vec<u8>;
    fn get_glyph_data(&self, character: char, offset: usize) -> Glyph;
    fn get_font_metrics(&self) -> LineMetrics;
}
