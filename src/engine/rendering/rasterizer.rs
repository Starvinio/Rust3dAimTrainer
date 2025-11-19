use crate::engine::{CONFIG, Triangle2d, TriToRaster};


pub fn draw_triangle(buf: &mut [u32], tri: &Triangle2d, width: usize, height: usize) {
    let color = 0xFF000000; // black
    draw_line(
        buf,
        tri.p[0].x as i32,
        tri.p[0].y as i32,
        tri.p[1].x as i32,
        tri.p[1].y as i32,
        color,
        width,
        height,
    );
    draw_line(
        buf,
        tri.p[1].x as i32,
        tri.p[1].y as i32,
        tri.p[2].x as i32,
        tri.p[2].y as i32,
        color,
        width,
        height,
    );
    draw_line(
        buf,
        tri.p[2].x as i32,
        tri.p[2].y as i32,
        tri.p[0].x as i32,
        tri.p[0].y as i32,
        color,
        width,
        height,
    );
}

fn draw_line(buf: &mut [u32], x0: i32, y0: i32, x1: i32, y1: i32, color: u32, width: usize, height: usize) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        put_pixel(buf, x0, y0, color, width, height);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn fill_triangle_optimized(
    buf: &mut [u32],
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    color: u32,
    width: usize,
    height: usize,
) {
    // Sort vertices by y-coordinate
    if y0 > y1 {
        std::mem::swap(&mut y0, &mut y1);
        std::mem::swap(&mut x0, &mut x1);
    }
    if y0 > y2 {
        std::mem::swap(&mut y0, &mut y2);
        std::mem::swap(&mut x0, &mut x2);
    }
    if y1 > y2 {
        std::mem::swap(&mut y1, &mut y2);
        std::mem::swap(&mut x1, &mut x2);
    }

    let total_height = y2 - y0;
    if total_height == 0 {
        return;
    }

    for i in 0..=total_height {
        let y = y0 + i;
        if y < 0 {
            continue;
        }
        if y >= height as i32 {
            break;
        }

        let second_half = i > (y1 - y0) || y1 == y0;
        let segment_height = if second_half { y2 - y1 } else { y1 - y0 };
        if segment_height == 0 {
            continue;
        }

        let alpha = i as f32 / total_height as f32;
        let beta = (i - if second_half { y1 - y0 } else { 0 }) as f32 / segment_height as f32;

        let ax = x0 as f32 + (x2 - x0) as f32 * alpha;
        let bx = if second_half {
            x1 as f32 + (x2 - x1) as f32 * beta
        } else {
            x0 as f32 + (x1 - x0) as f32 * beta
        };

        let mut minx = if ax < bx { ax as i32 } else { bx as i32 };
        let mut maxx = if ax < bx { bx as i32 } else { ax as i32 };

        // Clip to screen bounds
        minx = minx.max(0);
        maxx = maxx.min(width as i32 - 1);

        if minx > maxx {
            continue;
        }

        // Use memset-like operation for horizontal spans
        let row_start = (y as usize) * width + (minx as usize);
        let span_length = (maxx - minx + 1) as usize;
        buf[row_start..row_start + span_length].fill(color);
    }
}

pub fn put_pixel(buf: &mut [u32], x: i32, y: i32, color: u32, width: usize, height: usize) {
    if x >= 0
        && y >= 0
        && (x as usize) < width
        && (y as usize) < height
    {
        buf[(y as usize) * width + (x as usize)] = color;
    }
}

pub fn render_triangles(buffer: &mut [u32], list_triangles: &Vec<TriToRaster>, width: usize, height: usize) {
    for tri in list_triangles.iter() {
        fill_triangle_optimized(
            buffer,
            tri.tri.p[0].x as i32,
            tri.tri.p[0].y as i32,
            tri.tri.p[1].x as i32,
            tri.tri.p[1].y as i32,
            tri.tri.p[2].x as i32,
            tri.tri.p[2].y as i32,
            tri.color,
            width,
            height,
        );
    }
}