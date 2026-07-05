use std::path::PathBuf;

use edgy_graphics::font::Glyph;
use fontdue::Font;

use crate::providers::{FontRasterizerProvider, LineMetrics};

pub struct FontdueProvider {
    font: fontdue::Font,
    size: u8,
    coverage: u8,
}

impl FontdueProvider {
    pub fn new(path: PathBuf, size: u8, coverage: u8) -> Self {
        let bytes = std::fs::read(path).unwrap();
        let font = Font::from_bytes(bytes, fontdue::FontSettings::default()).unwrap();

        Self {
            font,
            size,
            coverage,
        }
    }
}

impl FontRasterizerProvider for FontdueProvider {
    fn rasterize(&self, character: char) -> Vec<u8> {
        let (metrics, bitmap) = self.font.rasterize(character, self.size as f32);
        let font_metrics = self.font.horizontal_line_metrics(self.size as f32).unwrap();
        
        let ascent = font_metrics.ascent.ceil() as i32;
        let descent = (-font_metrics.descent).ceil() as i32;
        let height = ascent + descent;
        let mut glyph_bitmap = vec![0; height as usize * metrics.width];
        let y_offset = ascent - (metrics.height as i32 + metrics.ymin);
        
        for y in 0..metrics.height {
            let dst_y = y + y_offset as usize;
        
            for x in 0..metrics.width {
                let alpha = bitmap[y * metrics.width + x];
        
                if alpha >= self.coverage {
                    glyph_bitmap[dst_y * metrics.width + x] = 1;
                }
            }
        }
        glyph_bitmap
    }

    fn get_glyph_data(&self, character: char, offset: usize) -> edgy_graphics::font::Glyph {
        let (metrics, _) = self.font.rasterize(character, self.size as f32);
        let font_metrics = self.font.horizontal_line_metrics(self.size as f32).unwrap();

        let ascent = font_metrics.ascent.ceil() as i32;
        let descent = (-font_metrics.descent).ceil() as i32;
        
        Glyph {
            character,
            advance_width: metrics.advance_width.round() as u8,
            height: (ascent + descent) as u8,
            width: metrics.width as u8,
            offset: offset as u16,
            x_offset: metrics.xmin as i8,
            y_offset: metrics.ymin as i8,
        }
    }

    fn get_font_metrics(&self) -> LineMetrics {
        let font_metrics = self.font.horizontal_line_metrics(self.size as f32).unwrap();
        LineMetrics {
            ascent: font_metrics.ascent.round() as u8,
            descent: font_metrics.descent.round() as u8,
            line_height: font_metrics.new_line_size.round() as u8
        }
    }
}
