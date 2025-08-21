use crate::codec::raw::icon_bitmap::RawIconBitmap;
use crate::codec::raw::icon_palette::RawIconPalette;
use png::{BitDepth, ColorType, Compression};

#[derive(Debug, Default, Clone)]
pub struct RomIcon {
    pub palette: RawIconPalette,
    pub bitmap: RawIconBitmap,
}

impl RomIcon {
    pub fn render_icon_png_32x(&self, buffer: &mut Vec<u8>) {
        self.render_png(buffer, 32, 32, &self.bitmap.get_indexed_image_data());
    }

    pub fn render_icon_png_64x(&self, buffer: &mut Vec<u8>) {
        self.render_png(buffer, 64, 64, &self.get_icon_image_data(2));
    }

    pub fn render_icon_png_128x(&self, buffer: &mut Vec<u8>) {
        self.render_png(buffer, 128, 128, &self.get_icon_image_data(4));
    }

    pub fn render_icon_png_256x(&self, buffer: &mut Vec<u8>) {
        self.render_png(buffer, 256, 256, &self.get_icon_image_data(8));
    }

    pub fn render_icon_png_512x(&self, buffer: &mut Vec<u8>) {
        self.render_png(buffer, 512, 512, &self.get_icon_image_data(16));
    }

    pub fn render_icon_png_1024x(&self, buffer: &mut Vec<u8>) {
        self.render_png(buffer, 1024, 1024, &self.get_icon_image_data(32));
    }

    pub fn render_icon_png_2048x(&self, buffer: &mut Vec<u8>) {
        self.render_png(buffer, 2048, 2048, &self.get_icon_image_data(64));
    }

    fn render_png(&self, buffer: &mut Vec<u8>, width: u32, height: u32, image_data: &[u8]) {
        let palette_colors = self.palette.get_palette_bytes();
        let palette_transparent = vec![
            0u8, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ];

        let mut encoder = png::Encoder::new(buffer, width, height);
        encoder.set_color(ColorType::Indexed);
        encoder.set_depth(BitDepth::Four);
        encoder.set_palette(palette_colors);
        encoder.set_trns(palette_transparent);
        encoder.set_compression(Compression::Best);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(image_data).unwrap();
    }

    fn get_icon_image_data(&self, scale: u32) -> Vec<u8> {
        let data = self.bitmap.get_indexed_image_data();

        let size = 32 * scale;
        let output_size = ((size * size) / 2) as usize;
        let mut output = vec![0u8; output_size];

        for y in 0..size {
            for x in 0..size {
                let orig_x = x / scale;
                let orig_y = y / scale;

                let orig_byte_index = (orig_x / 2 + 16 * orig_y) as usize;
                let orig_byte = data[orig_byte_index];

                let pixel_value = if orig_x % 2 == 0 {
                    (orig_byte >> 4) & 0x0F
                } else {
                    orig_byte & 0x0F
                };

                let out_byte_index = (x / 2 + (size / 2) * y) as usize;

                if x % 2 == 0 {
                    output[out_byte_index] = (output[out_byte_index] & 0x0F) | (pixel_value << 4);
                } else {
                    output[out_byte_index] = (output[out_byte_index] & 0xF0) | pixel_value;
                }
            }
        }

        output
    }
}
