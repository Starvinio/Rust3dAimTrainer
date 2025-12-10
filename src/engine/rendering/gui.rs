use std::io::BufReader;
use serde::Deserialize;
use crate::engine::rasterizer::put_pixel;

pub struct GUI {
    pub logo: Texture,
    pub digits_timer: [Texture; 10],
    pub digits_fps: [Texture; 10],
    pub colon: Texture,
}
impl GUI {
    pub fn load_gui(path: &str) -> Self {
        let texture = Texture::load_from_png(path).unwrap();
        let digits_timer = [
            texture.extract_region(0, 75, 33, 130), // 0
            texture.extract_region(0, 150, 33, 205), // 1
            texture.extract_region(0, 225, 33, 280), // 2
            texture.extract_region(0, 300, 33, 355), // 3
            texture.extract_region(0, 375, 38, 430), // 4
            texture.extract_region(0, 450, 33, 505), // 5
            texture.extract_region(0, 525, 33, 580), // 6
            texture.extract_region(0, 600, 33, 655), // 7
            texture.extract_region(0, 675, 33, 730), // 8
            texture.extract_region(0, 750, 33, 805), // 9
        ];
        let digits_fps = [
            texture.extract_region(0, 45, 17, 70),
            texture.extract_region(17, 45, 34, 70),
            texture.extract_region(34, 45, 51, 70),
            texture.extract_region(51,  45, 68, 70),
            texture.extract_region(68,  45, 85, 70),
            texture.extract_region(85,  45, 102, 70),
            texture.extract_region(102, 45, 119, 70),
            texture.extract_region(119, 45, 136, 70),
            texture.extract_region(136, 45, 153, 70),
            texture.extract_region(153, 45, 170, 70),
        ];
        GUI {
            logo: texture.extract_region(0, 0, 222, 50),
            digits_timer,
            digits_fps,
            colon: texture.extract_region(0, 815, 18, 865)
        }
    }
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>, // ARGB format
}
impl Texture {
    pub fn load_from_png(path: &str) -> Result<Self, String> {

        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let buf_reader = BufReader::new(file);
        let decoder = png::Decoder::new(buf_reader);
        let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
        let mut buf = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut buf).map_err(|e| e.to_string())?;

        let width = info.width;
        let height = info.height;

        let mut data = Vec::with_capacity((width * height) as usize);

        match info.color_type {
            png::ColorType::Rgba => {
                for chunk in buf[..info.buffer_size()].chunks(4) {
                    let r = chunk[0] as u32;
                    let g = chunk[1] as u32;
                    let b = chunk[2] as u32;
                    let a = chunk[3] as u32;
                    data.push((a << 24) | (r << 16) | (g << 8) | b);
                }
            }
            png::ColorType::Rgb => {
                for chunk in buf[..info.buffer_size()].chunks(3) {
                    let r = chunk[0] as u32;
                    let g = chunk[1] as u32;
                    let b = chunk[2] as u32;
                    data.push(0xFF000000 | (r << 16) | (g << 8) | b);
                }
            }
            _ => return Err("Unsupported PNG format".to_string()),
        }

        Ok(Self { width, height, data })
    }

    pub fn extract_region(&self, start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> Self {
        let width  = end_x - start_x;
        let height = end_y - start_y;

        let mut data = Vec::with_capacity((width * height) as usize);

        for dy in 0..height {
            for dx in 0..width {
                let src_x = start_x + dx;
                let src_y = start_y + dy;

                let pixel = if src_x < self.width && src_y < self.height {
                    let idx = (src_y * self.width + src_x) as usize;
                    self.data.get(idx).copied().unwrap_or(0)
                } else {
                    0
                };

                data.push(pixel);
            }
        }

        Self { width, height, data }
    }

    // Create a texture from raw ARGB data
    pub fn from_argb(width: u32, height: u32, data: Vec<u32>) -> Self {
        Self { width, height, data }
    }
}

/// Draw a texture at position (x, y)
pub fn draw_texture_optimized(
    buffer: &mut [u32],
    buffer_width: usize,
    buffer_height: usize,
    texture: &Texture,
    x: i32,
    y: i32,
) {
    let tex_width = texture.width as i32;
    let tex_height = texture.height as i32;

    // Calculate visible region (clipping)
    let start_x = x.max(0);
    let start_y = y.max(0);
    let end_x = (x + tex_width).min(buffer_width as i32);
    let end_y = (y + tex_height).min(buffer_height as i32);

    if start_x >= end_x || start_y >= end_y {
        return; // Fully clipped
    }

    // Process row by row for cache efficiency
    for dy in start_y..end_y {
        let tex_y = (dy - y) as usize;
        let tex_row_start = tex_y * texture.width as usize;
        let dst_row_start = dy as usize * buffer_width;

        for dx in start_x..end_x {
            let tex_x = (dx - x) as usize;
            let tex_idx = tex_row_start + tex_x;

            if tex_idx < texture.data.len() {
                let src_pixel = texture.data[tex_idx];
                let alpha = (src_pixel >> 24) & 0xFF;

                if alpha > 0 {
                    let dst_idx = dst_row_start + dx as usize;
                    if dst_idx < buffer.len() {
                        if alpha == 255 {
                            // Fully opaque - direct copy (fastest path)
                            buffer[dst_idx] = src_pixel;
                        } else {
                            // Alpha blend
                            buffer[dst_idx] = blend_pixel_fast(buffer[dst_idx], src_pixel, alpha);
                        }
                    }
                }
            }
        }
    }
}

/// Fast alpha blending optimized for your use case
#[inline]
pub fn blend_pixel_fast(dst: u32, src: u32, alpha: u32) -> u32 {
    if alpha == 255 {
        return src;
    }
    if alpha == 0 {
        return dst;
    }

    // Fast approximate blend using bit shifts
    let inv_alpha = 255 - alpha;

    let src_rb = src & 0x00FF00FF;
    let src_g = src & 0x0000FF00;
    let dst_rb = dst & 0x00FF00FF;
    let dst_g = dst & 0x0000FF00;

    let rb = ((src_rb * alpha + dst_rb * inv_alpha) >> 8) & 0x00FF00FF;
    let g = ((src_g * alpha + dst_g * inv_alpha) >> 8) & 0x0000FF00;

    0xFF000000 | rb | g
}



#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Crosshair {
    pub color: u32,
    pub line_length: i32,
    pub line_thickness: i32,
    pub gap: i32,
}

pub fn draw_crosshair(
    buf: &mut [u32],
    crosshair: Crosshair,
    width: usize,
    height: usize,
) {
    let center_x = width as i32 / 2;
    let center_y = height as i32 / 2;

    // Horizontal
    for dx in (crosshair.gap + 1)..=(crosshair.gap + crosshair.line_length) {
        for dy in -(crosshair.line_thickness / 2)..=(crosshair.line_thickness / 2) {
            put_pixel(buf, center_x + dx, center_y + dy, crosshair.color, width, height);
            put_pixel(buf, center_x - dx, center_y + dy, crosshair.color, width, height);
        }
    }

    // Vertical
    for dy in (crosshair.gap + 1)..=(crosshair.gap + crosshair.line_length) {
        for dx in -(crosshair.line_thickness / 2)..=(crosshair.line_thickness / 2) {
            put_pixel(buf, center_x + dx, center_y + dy, crosshair.color, width, height);
            put_pixel(buf, center_x + dx, center_y - dy, crosshair.color, width, height);
        }
    }
}