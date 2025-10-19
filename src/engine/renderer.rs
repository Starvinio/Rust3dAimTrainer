use std::collections::HashSet;
use winit::event::MouseButton;

use crate::engine::{
    consts::CONFIG,
    helpers::{
        cross_product, dot_product, multiply_matrix_vec, ray_intersects_triangle,
        triangle_clip_against_plane, vec_invert,
    },
    structures::{Mat4x4, Mesh, TriToRaster, Triangle, Vec3d},
    targets::Target,
};

pub fn create_target_proj(
    target: &mut Target,
    tri_vec: &mut Vec<TriToRaster>,
    old_target: &mut Option<(f32, f32)>,
    v_camera: &Vec3d,
    v_light_direction: &Vec3d,
    v_look_dir: &Vec3d,
    mb_pressed: &HashSet<MouseButton>,
    mat_view: &Mat4x4,
    mat_proj: &Mat4x4,
    hits: &mut i32,
) {
    //store target loc in case of hit
    let (old_x, old_y) = (target.tris[0].p[0].x, target.tris[0].p[0].y);

    for tri in &mut target.tris {
        let mut tri_viewed = Triangle::new_empty();

        let line1 = &tri.p[0].to(&tri.p[1]);
        let line2 = &tri.p[0].to(&tri.p[2]);
        let mut normal = cross_product(line1, line2);
        normal.normalize();

        if dot_product(&normal, &v_camera.to(&tri.p[0])) < 0.0 {
            let target_color: [u8; 3] = CONFIG.target.color;

            let dp = dot_product(&normal, &v_light_direction); // How similar is the tri normal to the light direction
            let brightness = dp.clamp(0.2, 0.8);
            let shade = (brightness * 255.0) as u32;
            let shade_redc = (shade as f32 * 0.2) as u32;
            let r = shade_redc * target_color[0] as u32;
            let g = shade_redc * target_color[1] as u32;
            let b = shade_redc * target_color[2] as u32;
            let color = (0xFF << 24) | (r << 16) | (g << 8) | b;

            if ray_intersects_triangle(&v_camera, &v_look_dir, &tri)
                && mb_pressed.contains(&MouseButton::Left)
            {
                target.hp -= 1;
                *hits += 1;
                *old_target = Some((old_x, old_y));
            }

            for i in 0..3 {
                multiply_matrix_vec(tri.p[i], &mut tri_viewed.p[i], &mat_view);
            }

            clip_triangle_z(&mut tri_viewed, color, tri_vec, mat_proj);
        }
    }
}

pub fn create_room_proj(
    room: &mut Mesh,
    tri_vec: &mut Vec<TriToRaster>,
    v_camera: &Vec3d,
    v_light_direction: &Vec3d,
    mat_view: &Mat4x4,
    mat_proj: &Mat4x4,
) {
    for tri in &mut room.tris {
        let mut tri_viewed = Triangle::new_empty();

        let v_light_direction = vec_invert(v_light_direction);

        let line1 = &tri.p[0].to(&tri.p[1]);
        let line2 = &tri.p[0].to(&tri.p[2]);
        let mut normal = cross_product(line1, line2);
        normal.normalize();

        if dot_product(&normal, &v_camera.to(&tri.p[0])) < 0.0 {
            let dp = dot_product(&normal, &v_light_direction); // How similar is the tri normal to the light direction
            let brightness = dp.clamp(0.2, 0.8);
            let shade = (brightness * 255.0) as u32;
            let color = (0xFF << 24) | (shade << 16) | (shade << 8) | shade;

            for i in 0..3 {
                multiply_matrix_vec(tri.p[i], &mut tri_viewed.p[i], &mat_view);
            }

            clip_triangle_z(&mut tri_viewed, color, tri_vec, mat_proj);
        }
    }
}

fn clip_triangle_z(
    tri: &mut Triangle,
    color: u32,
    tri_vec: &mut Vec<TriToRaster>,
    mat_proj: &Mat4x4,
) {
    let mut tri_projected = Triangle::new_empty();
    let mut clipped = [Triangle::new_empty(), Triangle::new_empty()];
    let n_clipped_triangles = triangle_clip_against_plane(
        Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.1,
        }, // Triangles clipped due to being partially behind camera
        &mut Vec3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        tri,
        &mut clipped,
    );

    for n in 0..n_clipped_triangles {
        for i in 0..3 {
            multiply_matrix_vec(clipped[n].p[i], &mut tri_projected.p[i], &mat_proj);
            tri_projected.p[i].x = (1.0 - tri_projected.p[i].x) * 0.5 * CONFIG.display.width as f32;
            tri_projected.p[i].y =
                (1.0 - tri_projected.p[i].y) * 0.5 * CONFIG.display.height as f32;
        }
        let avg_z = tri.avg_z(); // Needed for sorting (painters algorithm)
        tri_vec.push(TriToRaster {
            tri: tri_projected,
            color,
            avg_z,
        });
    }
}

pub fn tri_clip_and_render(tri_vec: &mut Vec<TriToRaster>, buf: &mut [u32]) {
    for tri_to_raster in tri_vec {
        let mut list_triangles: Vec<TriToRaster> = Vec::new();
        list_triangles.push(tri_to_raster.clone());
        let mut n_new_triangles = 1;
        for p in 0..4 {
            if list_triangles.is_empty() {
                break;
            }
            let mut n_tris_to_add = 0;
            let mut clipped = [Triangle::new_empty(), Triangle::new_empty()];
            while n_new_triangles > 0 {
                let mut test = list_triangles.pop().unwrap();
                n_new_triangles -= 1;
                match p {
                    0 => {
                        n_tris_to_add = triangle_clip_against_plane(
                            Vec3d {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            &mut Vec3d {
                                x: 0.0,
                                y: 1.0,
                                z: 0.0,
                            },
                            &mut test.tri,
                            &mut clipped,
                        );
                    }
                    1 => {
                        n_tris_to_add = triangle_clip_against_plane(
                            Vec3d {
                                x: 0.0,
                                y: CONFIG.display.height as f32 - 1.0,
                                z: 0.0,
                            },
                            &mut Vec3d {
                                x: 0.0,
                                y: -1.0,
                                z: 0.0,
                            },
                            &mut test.tri,
                            &mut clipped,
                        );
                    }
                    2 => {
                        n_tris_to_add = triangle_clip_against_plane(
                            Vec3d {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            &mut Vec3d {
                                x: 1.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            &mut test.tri,
                            &mut clipped,
                        );
                    }
                    3 => {
                        n_tris_to_add = triangle_clip_against_plane(
                            Vec3d {
                                x: CONFIG.display.width as f32 - 1.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            &mut Vec3d {
                                x: -1.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            &mut test.tri,
                            &mut clipped,
                        );
                    }
                    _ => {}
                }
                for w in 0..n_tris_to_add {
                    list_triangles.push(TriToRaster {
                        tri: clipped[w],
                        color: tri_to_raster.color,
                        avg_z: tri_to_raster.avg_z,
                    });
                }
            }
            n_new_triangles = list_triangles.len();
        }
        for tri in list_triangles.iter() {
            fill_triangle_optimized(
                buf,
                tri.tri.p[0].x as i32,
                tri.tri.p[0].y as i32,
                tri.tri.p[1].x as i32,
                tri.tri.p[1].y as i32,
                tri.tri.p[2].x as i32,
                tri.tri.p[2].y as i32,
                tri.color,
            );
        }
    }
}

pub fn render_crosshair(
    buf: &mut [u32],
    color: u32,
    line_length: i32,
    line_thickness: i32,
    gap: i32,
) {
    let center_x = CONFIG.display.width as i32 / 2;
    let center_y = CONFIG.display.height as i32 / 2;

    // Horizontal
    for dx in (gap + 1)..=(gap + line_length) {
        for dy in -(line_thickness / 2)..=(line_thickness / 2) {
            put_pixel(buf, center_x + dx, center_y + dy, color);
            put_pixel(buf, center_x - dx, center_y + dy, color);
        }
    }

    // vertical
    for dy in (gap + 1)..=(gap + line_length) {
        for dx in -(line_thickness / 2)..=(line_thickness / 2) {
            put_pixel(buf, center_x + dx, center_y + dy, color);
            put_pixel(buf, center_x + dx, center_y - dy, color);
        }
    }
}

pub fn draw_triangle(buf: &mut [u32], tri: &Triangle) {
    let color = 0xFF00FF00; // black
    draw_line(
        buf,
        tri.p[0].x as i32,
        tri.p[0].y as i32,
        tri.p[1].x as i32,
        tri.p[1].y as i32,
        color,
    );
    draw_line(
        buf,
        tri.p[1].x as i32,
        tri.p[1].y as i32,
        tri.p[2].x as i32,
        tri.p[2].y as i32,
        color,
    );
    draw_line(
        buf,
        tri.p[2].x as i32,
        tri.p[2].y as i32,
        tri.p[0].x as i32,
        tri.p[0].y as i32,
        color,
    );
}

fn draw_line(buf: &mut [u32], x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        put_pixel(buf, x0, y0, color);
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

pub fn fill_triangle(
    buf: &mut [u32],
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    color: u32,
) {
    // Sort vertices by y-coordinate (y0 ≤ y1 ≤ y2)
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

        let (minx, maxx) = if ax < bx {
            (ax as i32, bx as i32)
        } else {
            (bx as i32, ax as i32)
        };

        let y = y0 + i;
        for x in minx..=maxx {
            put_pixel(buf, x, y, color);
        }
    }
}

pub fn fill_triangle_optimized(
    buf: &mut [u32],
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    color: u32,
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
        if y >= CONFIG.display.height as i32 {
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
        maxx = maxx.min(CONFIG.display.width as i32 - 1);

        if minx > maxx {
            continue;
        }

        // Use memset-like operation for horizontal spans
        let row_start = (y as usize) * CONFIG.display.width + (minx as usize);
        let span_length = (maxx - minx + 1) as usize;
        buf[row_start..row_start + span_length].fill(color);
    }
}

pub fn put_pixel(buf: &mut [u32], x: i32, y: i32, color: u32) {
    if x >= 0
        && y >= 0
        && (x as usize) < CONFIG.display.width
        && (y as usize) < CONFIG.display.height
    {
        buf[(y as usize) * CONFIG.display.width + (x as usize)] = color;
    }
}
