use edgy_graphics::font::Glyph;

pub mod bdf;
pub mod fontdue;

pub trait FontRasterizerProvider {
    fn rasterize(&self, character: char) -> Vec<u8>;
    fn get_glyph_data(&self, character: char, offset: usize) -> Glyph;
}
