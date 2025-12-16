use std::io::BufReader;
use serde::Deserialize;
use crate::engine::camera::FPS;
use crate::engine::{EngineError, FPS_DIGIT_WIDTH, TIMER_DIGIT_WIDTH};
use crate::engine::rasterizer::put_pixel;

pub struct GUI {
    pub logo: Texture,
    pub digits_timer: [Texture; 10],
    pub digits_fps: [Texture; 10],
    pub colon: Texture,
}
impl GUI {
    pub fn load_gui(path: &str) -> Result<Self, EngineError> {
        let texture = Texture::load_from_png(path)?;
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
        Ok(GUI {
            logo: texture.extract_region(0, 0, 222, 50),
            digits_timer,
            digits_fps,
            colon: texture.extract_region(0, 815, 18, 865)
        })
    }
}

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
}
impl Texture {
    pub fn load_from_png(path: &str) -> Result<Self, EngineError> {

        let file = std::fs::File::open(path).map_err(|_| EngineError::GUILoadErr)?;
        let buf_reader = BufReader::new(file);
        let decoder = png::Decoder::new(buf_reader);
        let mut reader = decoder.read_info()?;

        let mut buf = vec![0; reader.output_buffer_size().ok_or(EngineError::GUILoadErr)?];
        let info = reader.next_frame(&mut buf).map_err(|_| EngineError::GUILoadErr)?;

        let width = info.width as usize;
        let height = info.height as usize;

        let mut data = Vec::with_capacity(width * height);

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
            _ => return Err(EngineError::GUILoadErr),
        }

        Ok(Self { width, height, data })
    }

    pub fn extract_region(&self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Self {
        let width  = end_x - start_x;
        let height = end_y - start_y;

        let mut data = Vec::with_capacity(width * height);

        for dy in 0..height {
            for dx in 0..width {
                let src_x = start_x + dx;
                let src_y = start_y + dy;

                let pixel = if src_x < self.width && src_y < self.height {
                    let idx = src_y * self.width + src_x;
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
    pub fn from_argb(width: usize, height: usize, data: Vec<u32>) -> Self {
        Self { width, height, data }
    }
}

pub struct Timer {
    pub minutes: u64,
    pub seconds: u64
}
impl Timer {
    pub fn new() -> Self {
        Self {minutes:0, seconds:0}
    }
    pub fn update_mins(&mut self) {
        self.minutes = self.seconds / 60;
        self.seconds -= self.minutes * 60;
    }
    pub fn draw_timer(&self, pixel_buffer: &mut [u32], width:usize, height: usize, gui: &GUI) {

        let center_width = width / 2;
        draw_texture_optimized(pixel_buffer, width, height, &gui.colon, center_width - 9, 0);
        
        let minutes_str = self.minutes.to_string();
        for (i, byte) in minutes_str.bytes().enumerate() {
            let digit = (byte - b'0') as usize;

            let x = center_width - 9 - (minutes_str.len() * TIMER_DIGIT_WIDTH) + (i * TIMER_DIGIT_WIDTH);

            draw_texture_optimized(
                pixel_buffer, 
                width, 
                height, 
                &gui.digits_timer[digit], 
                x, 
                0
            );
        }

        let seconds_last_digit = (self.seconds % 10) as usize;
        let seconds_front_digit = (self.seconds as usize - seconds_last_digit) / 10;
        draw_texture_optimized(pixel_buffer, width, height, &gui.digits_timer[seconds_front_digit], center_width + 9, 0);
        draw_texture_optimized(pixel_buffer, width, height, &gui.digits_timer[seconds_last_digit], center_width + 9 + 30, 0);
    }
    /*
    pub fn draw_fps(fps: &FPS, pixel_buffer: &mut [u32], width: usize, height: usize, texture: &[Texture]) {
    for (i, byte) in fps.fps_str.bytes().enumerate() {
        let digit = (byte - b'0') as usize;

        let x = width - fps.width_px  + (i * FPS_DIGIT_WIDTH);

        draw_texture_optimized(
            pixel_buffer,
            width,
            height,
            &texture[digit],
            x,
            0,
        );

    }
}
    */
}

/// Draw a texture at position (x, y)
pub fn draw_texture_optimized(
    buffer: &mut [u32],
    buffer_width: usize,
    buffer_height: usize,
    texture: &Texture,
    x: usize,
    y: usize,
) {
    let tex_width = texture.width;
    let tex_height = texture.height;

    // Calculate visible region (clipping)
    let start_x = x.max(0);
    let start_y = y.max(0);
    let end_x = (x + tex_width).min(buffer_width);
    let end_y = (y + tex_height).min(buffer_height);

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

pub fn draw_fps(fps: &FPS, pixel_buffer: &mut [u32], width: usize, height: usize, texture: &[Texture]) {
    for (i, byte) in fps.fps_str.bytes().enumerate() {
        let digit = (byte - b'0') as usize;

        let x = width - fps.width_px  + (i * FPS_DIGIT_WIDTH);

        draw_texture_optimized(
            pixel_buffer,
            width,
            height,
            &texture[digit],
            x,
            0,
        );

    }
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