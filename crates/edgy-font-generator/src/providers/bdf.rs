use std::{fs::File, io::BufReader, path::PathBuf};

use crate::providers::{FontRasterizerProvider, LineMetrics};
use bdf_reader::{Font, Value::Integer};
use edgy_graphics::font::Glyph;

pub struct BdfProvider {
    font: bdf_reader::Font,
}

impl BdfProvider {
    pub fn new(path: PathBuf) -> Self {
        let reader = BufReader::new(File::open(path).unwrap());
        let font = Font::read(reader).expect("Failed to parse font");

        Self { font }
    }
}

impl FontRasterizerProvider for BdfProvider {
    fn rasterize(&self, character: char) -> Vec<u8> {
        let glyph = self.font.glyph(character).unwrap();
        let bitmap = glyph.bitmap();

        let mut result = Vec::new();
        result.resize(bitmap.width() * bitmap.height(), 0);

        for x in 0..bitmap.width() {
            for y in 0..bitmap.height() {
                result[y * bitmap.width() + x] = bitmap.get(x, y).unwrap_or(false) as u8;
            }
        }

        result
    }

    fn get_glyph_data(&self, character: char, offset: usize) -> edgy_graphics::font::Glyph {
        let glyph = self.font.glyph(character).unwrap();
        let bb = glyph.bounding_box();
        let dwidth = glyph.dwidth().unwrap_or_default();

        Glyph {
            character,
            offset: offset as u16,
            width: bb.width as u8,
            height: bb.height as u8,
            x_offset: bb.offset_x as i8,
            y_offset: bb.offset_y as i8,
            advance_width: dwidth.0 as u8,
        }
    }

    fn get_font_metrics(&self) -> LineMetrics {
        let ascent = match self.font.property("FONT_ASCENT") {
            Some(Integer(n)) => *n,
            _ => 0
        };

        let descent = match self.font.property("FONT_DESCENT") {
            Some(Integer(n)) => *n,
            _ => 0
        };
        
        LineMetrics {
            ascent: ascent as u8,
            descent: descent as u8,
            line_height: (ascent + descent) as u8
        }
    }
}
