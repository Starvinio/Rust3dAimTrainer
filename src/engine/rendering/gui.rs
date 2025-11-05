use serde::Deserialize;
use crate::engine::{CONFIG};
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
) {
    let center_x = CONFIG.display.width as i32 / 2;
    let center_y = CONFIG.display.height as i32 / 2;

    // Horizontal
    for dx in (crosshair.gap + 1)..=(crosshair.gap + crosshair.line_length) {
        for dy in -(crosshair.line_thickness / 2)..=(crosshair.line_thickness / 2) {
            put_pixel(buf, center_x + dx, center_y + dy, crosshair.color);
            put_pixel(buf, center_x - dx, center_y + dy, crosshair.color);
        }
    }

    // vertical
    for dy in (crosshair.gap + 1)..=(crosshair.gap + crosshair.line_length) {
        for dx in -(crosshair.line_thickness / 2)..=(crosshair.line_thickness / 2) {
            put_pixel(buf, center_x + dx, center_y + dy, crosshair.color);
            put_pixel(buf, center_x + dx, center_y - dy, crosshair.color);
        }
    }
}