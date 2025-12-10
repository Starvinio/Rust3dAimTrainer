use std::fs;
use std::io::{BufRead, BufReader};

use std::f32::consts::PI;

use crate::engine::camera::{Camera};
use crate::engine::core::{BLUE, CONFIG, Mesh, RESET, Triangle, Vec3d};

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

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-1.5 * (x - 0.5)).exp())
}

pub fn load_from_obj(path: &str) -> Result<Mesh, std::io::Error> {
    let file = fs::File::open(path)?;
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


pub fn ray_intersects_triangle(camera:&Camera, triangle: &Triangle) -> bool {
    // Implementation of the Möller–Trumbore intersection algorithm
    // src: https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm

    // Two edges that define the span of the triangle (Forming a V-shape of two vectors)
    let edge1: Vec3d = triangle.p[0].vec_to(&triangle.p[1]);
    let edge2: Vec3d = triangle.p[0].vec_to(&triangle.p[2]);

    let ray_cross_e2 = camera.look_dir.cross(edge2);
    let det: f32 = ray_cross_e2.dot(edge1);

    /*
    Now we want to check if the ray is parallel to the triangle
    the following could be 'dot_product(triangle.normal, ray_vec)'
    but this is avoided in favor of performance
    */

    if det > -f32::EPSILON && det < f32::EPSILON {
        return false;
    } 

    let inv_det: f32 = 1.0 / det;
    let s: Vec3d = camera.position - triangle.p[0];
    let u: f32 = inv_det * s.dot(ray_cross_e2);

    if (u < 0.0 && u.abs() > f32::EPSILON) || (u > 1.0 && (u - 1.0).abs() > f32::EPSILON) {
        // checking if  0 ≤ u ≤ 1
        return false;
    }

    let s_cross_e1: Vec3d = s.cross(edge1);
    let v: f32 = inv_det * camera.look_dir.dot(s_cross_e1);

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

pub fn dyn_clamp_pos(pos:f32, vel:f32, n1:f32, n2:f32) -> (f32,f32) {
        if n1 < n2 && (pos < n1 || pos > n2) {
            return (pos.clamp(n1, n2), vel * -1.0);
        } else if n2 < n1 && (pos < n2 || pos > n1) {
            return (pos.clamp(n2, n1), vel * -1.0);
        } else if n2 == n1 {
            return (n1, vel);
        }
        (pos, vel)
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
