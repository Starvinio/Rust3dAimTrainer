use std::fs;
use winit::window::{Fullscreen, Window};

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::engine::consts::*;
use crate::engine::structures::*;
use std::f32::consts::PI;

use crate::engine::consts::CONFIG;

/*  This turns the view cone specified as FOV into a scaling factor for
projection, which will make sure that, as the FOV increases, the
size of objects decreases and vice versa. */
pub fn get_fov_rad(fov: f32) -> f32 {
    1.0 / ((fov * 0.5 * (PI / 180.0)).tan())
}

pub fn ow_to_engine_fov(fov_h_deg: f32) -> f32 {
    let fov_h_rad: f32 = fov_h_deg.to_radians();
    let aspect = CONFIG.display.width as f32 / CONFIG.display.height as f32;
    let fov_v_rad = 2.0 * ((fov_h_rad / 2.0).tan() / aspect).atan();
    fov_v_rad.to_degrees()
}

pub fn mat_rotation_x(theta: f32) -> Mat4x4 {
    let mut m = [[0.0; 4]; 4];
    m[0][0] = 1.0;
    m[1][1] = theta.cos();
    m[1][2] = theta.sin();
    m[2][1] = -theta.sin();
    m[2][2] = theta.cos();
    m[3][3] = 1.0;
    Mat4x4 { m }
}

pub fn mat_rotation_y(theta: f32) -> Mat4x4 {
    let mut m = [[0.0; 4]; 4];
    m[0][0] = theta.cos();
    m[0][2] = theta.sin();
    m[2][0] = -theta.sin();
    m[1][1] = 1.0;
    m[2][2] = theta.cos();
    m[3][3] = 1.0;
    Mat4x4 { m }
}

pub fn mat_rotation_z(theta: f32) -> Mat4x4 {
    let mut m = [[0.0; 4]; 4];
    m[0][0] = theta.cos();
    m[0][1] = theta.sin();
    m[1][0] = -theta.sin();
    m[1][1] = theta.cos();
    m[2][2] = 1.0;
    m[3][3] = 1.0;
    Mat4x4 { m }
}

pub fn matrix_make_translation(x: f32, y: f32, z: f32) -> Mat4x4 {
    let mut matrix = Mat4x4 { m: [[0.0; 4]; 4] };
    matrix.m[0][0] = 1.0;
    matrix.m[1][1] = 1.0;
    matrix.m[2][2] = 1.0;
    matrix.m[3][3] = 1.0;
    matrix.m[3][0] = x;
    matrix.m[3][1] = y;
    matrix.m[3][2] = z;
    return matrix;
}

pub fn multiply_matrix_vec(i: Vec3d, o: &mut Vec3d, m: &Mat4x4) {
    o.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0];
    o.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1];
    o.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];
    let w: f32 = i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + m.m[3][3];

    if w != 0.0 {
        *o = vec_divide(o, w);
    }
}

pub fn multiply_matrix_matrix(m1: Mat4x4, m2: Mat4x4) -> Mat4x4 {
    let mut matrix = Mat4x4::new_empty();
    for c in 0..4 {
        for r in 0..4 {
            matrix.m[r][c] = m1.m[r][0] * m2.m[0][c]
                + m1.m[r][1] * m2.m[1][c]
                + m1.m[r][2] * m2.m[2][c]
                + m1.m[r][3] * m2.m[3][c];
        }
    }
    matrix
}

pub fn matrix_point_at(pos: &Vec3d, target: &Vec3d, up: &Vec3d) -> Mat4x4 {
    //Calculate new forward direction
    let new_forward: Vec3d = vec_subtract(target, pos).normalize();

    //Calculate new up direction
    let a: Vec3d = vec_multiply(&new_forward, dot_product(&up, &new_forward));
    let new_up: Vec3d = vec_subtract(&up, &a).normalize();

    //Calculate new right direction
    let new_right: Vec3d = cross_product(&new_forward, &new_up);

    let mut matrix: Mat4x4 = Mat4x4::new_empty();

    matrix.m[0][0] = new_right.x;
    matrix.m[0][1] = new_right.y;
    matrix.m[0][2] = new_right.z;
    matrix.m[0][3] = 0.0;
    matrix.m[1][0] = new_up.x;
    matrix.m[1][1] = new_up.y;
    matrix.m[1][2] = new_up.z;
    matrix.m[1][3] = 0.0;
    matrix.m[2][0] = new_forward.x;
    matrix.m[2][1] = new_forward.y;
    matrix.m[2][2] = new_forward.z;
    matrix.m[2][3] = 0.0;
    matrix.m[3][0] = pos.x;
    matrix.m[3][1] = pos.y;
    matrix.m[3][2] = pos.z;
    matrix.m[3][3] = 1.0;

    matrix
}

pub fn matrix_quickinverse(m: Mat4x4) -> Mat4x4 {
    // Only use for rotation/translation matrices
    let mut matrix: Mat4x4 = Mat4x4::new_empty();

    matrix.m[0][0] = m.m[0][0];
    matrix.m[0][1] = m.m[1][0];
    matrix.m[0][2] = m.m[2][0];
    matrix.m[0][3] = 0.0;
    matrix.m[1][0] = m.m[0][1];
    matrix.m[1][1] = m.m[1][1];
    matrix.m[1][2] = m.m[2][1];
    matrix.m[1][3] = 0.0;
    matrix.m[2][0] = m.m[0][2];
    matrix.m[2][1] = m.m[1][2];
    matrix.m[2][2] = m.m[2][2];
    matrix.m[2][3] = 0.0;
    matrix.m[3][0] =
        -(m.m[3][0] * matrix.m[0][0] + m.m[3][1] * matrix.m[1][0] + m.m[3][2] * matrix.m[2][0]);
    matrix.m[3][1] =
        -(m.m[3][0] * matrix.m[0][1] + m.m[3][1] * matrix.m[1][1] + m.m[3][2] * matrix.m[2][1]);
    matrix.m[3][2] =
        -(m.m[3][0] * matrix.m[0][2] + m.m[3][1] * matrix.m[1][2] + m.m[3][2] * matrix.m[2][2]);
    matrix.m[3][3] = 1.0;

    matrix
}

pub fn create_proj_mat(fov_rad: f32) -> Mat4x4 {
    let far = CONFIG.camera.far;
    let near = CONFIG.camera.near;
    let aspect_ratio = aspect_ratio();
    Mat4x4 {
        m: ([
            [fov_rad * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, fov_rad, 0.0, 0.0],
            [0.0, 0.0, far / (far - near), 1.0],
            [0.0, (-far * near) / (far - near), 0.0, 0.0],
        ]),
    }
}

pub fn vec_add(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
    Vec3d::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z)
}

pub fn vec_subtract(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
    Vec3d::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z)
}

pub fn vec_multiply(v1: &Vec3d, k: f32) -> Vec3d {
    Vec3d::new(v1.x * k, v1.y * k, v1.z * k)
}

fn vec_divide(v1: &Vec3d, k: f32) -> Vec3d {
    Vec3d::new(v1.x / k, v1.y / k, v1.z / k)
}

pub fn vec_invert(vec: &Vec3d) -> Vec3d {
    Vec3d::new(-vec.x, -vec.y, -vec.z)
}

pub fn cross_product(line1: &Vec3d, line2: &Vec3d) -> Vec3d {
    Vec3d::new(
        line1.y * line2.z - line1.z * line2.y,
        line1.z * line2.x - line1.x * line2.z,
        line1.x * line2.y - line1.y * line2.x,
    )
}

pub fn dot_product(line1: &Vec3d, line2: &Vec3d) -> f32 {
    line1.x * line2.x + line1.y * line2.y + line1.z * line2.z
}

pub fn vector_intersect_plane(
    plane_p: &Vec3d,
    plane_n: &mut Vec3d,
    line_start: &Vec3d,
    line_end: &Vec3d,
) -> Vec3d {
    let plane_n = plane_n.normalize();
    let plane_d = -dot_product(&plane_n, plane_p);
    let ad = dot_product(&line_start, &plane_n);
    let bd = dot_product(&line_end, &plane_n);
    let t = (-plane_d - ad) / (bd - ad);
    let line_start_to_end = vec_subtract(line_end, line_start);
    let line_to_intersect = vec_multiply(&line_start_to_end, t);

    vec_add(line_start, &line_to_intersect)
}

pub fn triangle_clip_against_plane(
    plane_p: Vec3d,
    plane_n: &mut Vec3d,
    in_tri: &mut Triangle,
    out_tri_array: &mut [Triangle; 2],
) -> usize {
    // Make sure plane normal is indeed normal
    let mut plane_n = plane_n.normalize();

    // Return signed shortest distance from point to plane, plane normal must be normalised
    fn dist(p: Vec3d, plane_p: Vec3d, plane_n: Vec3d) -> f32 {
        // ax + by + cz + d = 0
        // a = plane_n.x, b = plane_n.y, c = plane_n.z
        // d = - (plane_n dot plane_p)
        let d = -dot_product(&plane_n, &plane_p);
        return plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z + d;
    }

    // Create two temporary storage arrays to classify points either side of plane
    let mut inside_points: [&Vec3d; 3] = [&Vec3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; 3];
    let mut n_inside_point = 0;
    let mut outside_points: [&Vec3d; 3] = [&Vec3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; 3];
    let mut n_outside_point = 0;

    // Get signed distance of each point in triangle to plane
    let d0 = dist(in_tri.p[0], plane_p, plane_n);
    let d1 = dist(in_tri.p[1], plane_p, plane_n);
    let d2 = dist(in_tri.p[2], plane_p, plane_n);

    if d0 >= 0.0 {
        inside_points[n_inside_point] = &in_tri.p[0];
        n_inside_point += 1;
    } else {
        outside_points[n_outside_point] = &in_tri.p[0];
        n_outside_point += 1;
    }

    if d1 >= 0.0 {
        inside_points[n_inside_point] = &in_tri.p[1];
        n_inside_point += 1;
    } else {
        outside_points[n_outside_point] = &in_tri.p[1];
        n_outside_point += 1;
    }

    if d2 >= 0.0 {
        inside_points[n_inside_point] = &in_tri.p[2];
        n_inside_point += 1;
    } else {
        outside_points[n_outside_point] = &in_tri.p[2];
        n_outside_point += 1;
    }

    if n_inside_point == 0 {
        //all points are outside the plane
        return 0;
    }

    if n_inside_point == 3 {
        // all points are inside the plane
        out_tri_array[0].p = in_tri.p;

        return 1; // No clipping needed
    }

    if n_inside_point == 1 && n_outside_point == 2 {
        // Triangle should be clipped. As two points lie outside the plane, the triangle
        // simply becomes a smaller triangle.

        // Copy appearance info to new triangle
        out_tri_array[0].p = [Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; 3];

        // The inside point is valid, so keep that...
        out_tri_array[0].p[0] = *inside_points[0];

        // but the two new points are at the locations where the original sides of the triangle (lines)
        // intersect with the plane. We find these points by using a simple line-plane intersection
        // formula (see vector_intersect_plane).

        out_tri_array[0].p[1] = vector_intersect_plane(
            &plane_p,
            &mut plane_n.clone(),
            inside_points[0],
            outside_points[0],
        );
        out_tri_array[0].p[2] = vector_intersect_plane(
            &plane_p,
            &mut plane_n.clone(),
            inside_points[0],
            outside_points[1],
        );

        return 1; // Return the newly formed single triangle
    }

    if n_inside_point == 2 && n_outside_point == 1 {
        // Triangle should be clipped. As two points lie inside the plane,
        // the clipped triangle becomes a "quad". Fortunately, we can
        // represent a quad with two new triangles

        // Copy appearance info to new triangles
        // Copy appearance info to new triangle
        out_tri_array[0].p = [Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; 3];
        out_tri_array[1].p = [Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; 3];

        // The inside point is valid, so keep that...
        out_tri_array[0].p[0] = *inside_points[0];
        out_tri_array[0].p[1] = *inside_points[1];
        out_tri_array[0].p[2] =
            vector_intersect_plane(&plane_p, &mut plane_n, inside_points[0], outside_points[0]);

        // The second triangle is composed of one of he inside points, a
        // new point determined by the intersection of the other side of the
        // triangle and the plane, and the newly created point above
        out_tri_array[1].p[0] = *inside_points[1];
        out_tri_array[1].p[1] = out_tri_array[0].p[2];
        out_tri_array[1].p[2] =
            vector_intersect_plane(&plane_p, &mut plane_n, inside_points[1], outside_points[0]);

        return 2; // Return two newly formed triangles which form a quad
    }

    0
}

pub fn load_from_obj_str(s: &str) -> Result<Mesh, std::io::Error> {
    let mut verts: Vec<Vec3d> = Vec::new();
    let mut tris: Vec<Triangle> = Vec::new();

    for line in s.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                let x: f32 = parts[1].parse().unwrap();
                let y: f32 = parts[2].parse().unwrap();
                let z: f32 = parts[3].parse().unwrap();
                verts.push(Vec3d::new(x, y, z));
            }
            "f" => {
                let idx1: usize = parts[1].split('/').next().unwrap().parse().unwrap();
                let idx2: usize = parts[2].split('/').next().unwrap().parse().unwrap();
                let idx3: usize = parts[3].split('/').next().unwrap().parse().unwrap();

                tris.push(Triangle {
                    p: [
                        verts[idx1 - 1], // .obj indices start at 1
                        verts[idx2 - 1],
                        verts[idx3 - 1],
                    ],
                });
            }
            _ => {} // ignore other lines
        }
    }

    Ok(Mesh { tris })
}

pub fn load_from_obj(path: &str) -> Result<Mesh, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut verts: Vec<Vec3d> = Vec::new();
    let mut tris: Vec<Triangle> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                // vertex
                let x: f32 = parts[1].parse().unwrap();
                let y: f32 = parts[2].parse().unwrap();
                let z: f32 = parts[3].parse().unwrap();
                verts.push(Vec3d::new(x, y, z));
            }
            "f" => {
                // face (triangles only for now)
                let idx1: usize = parts[1].split('/').next().unwrap().parse().unwrap();
                let idx2: usize = parts[2].split('/').next().unwrap().parse().unwrap();
                let idx3: usize = parts[3].split('/').next().unwrap().parse().unwrap();

                tris.push(Triangle {
                    p: [
                        verts[idx1 - 1], // .obj indices start at 1
                        verts[idx2 - 1],
                        verts[idx3 - 1],
                    ],
                });
            }
            _ => {} // ignore other lines
        }
    }

    Ok(Mesh { tris })
}

pub fn load_config(path: &str) -> Config {
    let config_str = fs::read_to_string(path).expect("Failed to read config.toml");

    toml::from_str::<Config>(&config_str).expect("Failed to parse config.toml")
}

pub fn ray_intersects_triangle(ray_origin: &Vec3d, ray_vec: &Vec3d, triangle: &Triangle) -> bool {
    // Möller–Trumbore intersection algorithm
    // src: https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm

    // Two edges that define the span of the triangle (Forming a V-shape of two vectors)
    let edge1: Vec3d = vec_subtract(&triangle.p[1], &triangle.p[0]);
    let edge2: Vec3d = vec_subtract(&triangle.p[2], &triangle.p[0]);

    let ray_cross_e2 = cross_product(ray_vec, &edge2);
    let det: f32 = dot_product(&ray_cross_e2, &edge1);

    /*
    Now we want to check if the ray is parallel to the triangle
    the following could be 'dot_product(triangle.normal, ray_vec)'
    but this is avoided in favor of performance
    */

    /*
    if det > -f32::EPSILON && det < f32::EPSILON {
        return false;
    } We dont need this part, as we never use this function before checking if triangle
    will even be visible. However, we will need 'det'.*/

    let inv_det: f32 = 1.0 / det;
    let s: Vec3d = vec_subtract(ray_origin, &triangle.p[0]);
    let u: f32 = inv_det * dot_product(&s, &ray_cross_e2);

    if (u < 0.0 && u.abs() > f32::EPSILON) || (u > 1.0 && (u - 1.0).abs() > f32::EPSILON) {
        // checking if  0 ≤ u ≤ 1
        return false;
    }

    let s_cross_e1: Vec3d = cross_product(&s, &edge1);
    let v: f32 = inv_det * dot_product(ray_vec, &s_cross_e1);

    if (v < 0.0 && v.abs() > f32::EPSILON) || (u + v > 1.0 && (u + v - 1.0).abs() > f32::EPSILON) {
        return false;
    }

    /*
    At this point we know that the ray points at the triangle
    The following will calculate the exact point that the ray intersects the triangle
    We only care about the boolean though

    let t = inv_det * dot_product(&e2, &s_cross_e1);

    if t > f32::EPSILON { // ray intersection
        let intersection_point:Vec3d = vec_add(ray_origin, &vec_multiply(ray_vec, t));
        return Some(intersection_point);
    }
    else { // This means that there is a line intersection but not a ray intersection.
        return None;
    }*/

    true
}

pub fn toggle_fullscreen(window: &Window) {
    match window.fullscreen() {
        None => window.set_fullscreen(Some(Fullscreen::Borderless(None))),
        Some(_) => window.set_fullscreen(None),
    }
}

pub fn draw_logo() {
    let logo = r#"
  _____           _              _             _______        _                 
 |  __ \         | |       /\   (_)           |__   __|      (_)                
 | |__) |   _ ___| |_     /  \   _ _ __ ___      | |_ __ __ _ _ _ __   ___ _ __ 
 |  _  / | | / __| __|   / /\ \ | | '_ ` _ \     | | '__/ _` | | '_ \ / _ \ '__|
 | | \ \ |_| \__ \ |_   / ____ \| | | | | | |    | | | (_| | | | | |  __/ |   
 |_|  \_\__,_|___/\__| /_/    \_\_|_| |_| |_|    |_|_|  \__,_|_|_| |_|\___|_|   
                                                                                
                                                                                
"#;
    println!("{}{}{}", BLUE, logo, RESET);
}
