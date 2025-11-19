use serde::Deserialize;
use crate::engine::rasterizer::put_pixel;

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