use crate::engine::{Vec3d, Vec2d, Triangle, Triangle2d, TriToRaster, CONFIG};

pub fn vector_intersect_plane(
    plane_p: &Vec3d,
    plane_n: &mut Vec3d,
    line_start: &Vec3d,
    line_end: &Vec3d,
) -> Vec3d {
    let plane_n = plane_n.normalize();
    let plane_d = -(plane_n.dot(*plane_p));
    let ad = line_start.dot(plane_n);
    let bd = line_end.dot(plane_n);
    let t = (-plane_d - ad) / (bd - ad);
    let line_start_to_end = line_start.vec_to(line_end);
    let line_to_intersect = line_start_to_end * t;

    *line_start + line_to_intersect
}

pub fn tri2d_clip_against_line(
    line_p: Vec2d,
    line_n: &mut Vec2d,
    in_tri: &Triangle2d,
    out_tri_array: &mut [Triangle2d; 2],
) -> usize {
    // Ensure the normal is normalized
    let line_n = line_n.normalize();

    // Compute signed distance from point to the clipping line
    fn dist(p: Vec2d, line_p: Vec2d, line_n: Vec2d) -> f32 {
        let d = -(line_n.dot(line_p));
        line_n.x * p.x + line_n.y * p.y + d
    }

    // Temporary storage for inside/outside points
    let mut inside_points: [Vec2d; 3] = [Vec2d::zero(); 3];
    let mut n_inside_point = 0;
    let mut outside_points: [Vec2d; 3] = [Vec2d::zero(); 3];
    let mut n_outside_point = 0;

    // Get signed distance of each point in triangle to line
    let d0 = dist(in_tri.p[0], line_p, line_n);
    let d1 = dist(in_tri.p[1], line_p, line_n);
    let d2 = dist(in_tri.p[2], line_p, line_n);

    // Classify points
    if d0 >= 0.0 {
        inside_points[n_inside_point] = in_tri.p[0];
        n_inside_point += 1;
    } else {
        outside_points[n_outside_point] = in_tri.p[0];
        n_outside_point += 1;
    }

    if d1 >= 0.0 {
        inside_points[n_inside_point] = in_tri.p[1];
        n_inside_point += 1;
    } else {
        outside_points[n_outside_point] = in_tri.p[1];
        n_outside_point += 1;
    }

    if d2 >= 0.0 {
        inside_points[n_inside_point] = in_tri.p[2];
        n_inside_point += 1;
    } else {
        outside_points[n_outside_point] = in_tri.p[2];
        n_outside_point += 1;
    }

    // Now classify and construct output triangles

    if n_inside_point == 0 {
        // No points inside
        return 0;
    }

    if n_inside_point == 3 {
        // All points inside
        out_tri_array[0].p = in_tri.p;
        return 1;
    }

    // Helper: line intersection between two 2D points and a clipping line
    fn line_intersect(
        line_p: &Vec2d,
        line_n: &Vec2d,
        p_start: &Vec2d,
        p_end: &Vec2d,
    ) -> Vec2d {
        let line_dir = *p_end - *p_start;
        let t = (line_n.dot(*line_p - *p_start)) / line_n.dot(line_dir);
        *p_start + line_dir * t
    }

    if n_inside_point == 1 && n_outside_point == 2 {
        // One inside point, form one new triangle
        out_tri_array[0].p[0] = inside_points[0];
        out_tri_array[0].p[1] = line_intersect(&line_p, &line_n, &inside_points[0], &outside_points[0]);
        out_tri_array[0].p[2] = line_intersect(&line_p, &line_n, &inside_points[0], &outside_points[1]);
        return 1;
    }

    if n_inside_point == 2 && n_outside_point == 1 {
        // Two inside points, form two new triangles (quad split)
        out_tri_array[0].p[0] = inside_points[0];
        out_tri_array[0].p[1] = inside_points[1];
        out_tri_array[0].p[2] = line_intersect(&line_p, &line_n, &inside_points[0], &outside_points[0]);

        out_tri_array[1].p[0] = inside_points[1];
        out_tri_array[1].p[1] = out_tri_array[0].p[2];
        out_tri_array[1].p[2] = line_intersect(&line_p, &line_n, &inside_points[1], &outside_points[0]);

        return 2;
    }

    0
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
        let d = -(plane_n.dot(plane_p));
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

pub fn tri_clip_xy(tri_to_raster:&TriToRaster, list_triangles:&mut Vec<TriToRaster>) {
    list_triangles.push(tri_to_raster.clone());
    let mut n_new_triangles = 1;
    for p in 0..4 {
        if list_triangles.is_empty() {
            break;
        }
        let mut n_tris_to_add = 0;
        let mut clipped = [Triangle2d::new_origin(), Triangle2d::new_origin()];
        while n_new_triangles > 0 {
            let mut test = list_triangles.pop().unwrap();
            n_new_triangles -= 1;
            match p {
                0 => {
                    n_tris_to_add = tri2d_clip_against_line(
                        Vec2d {
                            x: 0.0,
                            y: 0.0
                        },
                        &mut Vec2d {
                            x: 0.0,
                            y: 1.0
                        },
                        &mut test.tri,
                        &mut clipped,
                    );
                }
                1 => {
                    n_tris_to_add = tri2d_clip_against_line(
                        Vec2d {
                            x: 0.0,
                            y: CONFIG.display.height as f32 - 1.0,
                        },
                        &mut Vec2d {
                            x: 0.0,
                            y: -1.0,
                        },
                        &mut test.tri,
                        &mut clipped,
                    );
                }
                2 => {
                    n_tris_to_add = tri2d_clip_against_line(
                        Vec2d {
                            x: 0.0,
                            y: 0.0
                        },
                        &mut Vec2d {
                            x: 1.0,
                            y: 0.0
                        },
                        &mut test.tri,
                        &mut clipped,
                    );
                }
                3 => {
                    n_tris_to_add = tri2d_clip_against_line(
                        Vec2d {
                            x: CONFIG.display.width as f32 - 1.0,
                            y: 0.0
                        },
                        &mut Vec2d {
                            x: -1.0,
                            y: 0.0
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
}

pub fn tri_clip_z(tri_pre_clipping: &mut Triangle) -> ([Triangle;2], usize) {
/*
    This function splits up triangles that are partially behind the camera by utilizing z_clipping.
    It returns a tuple of the new triangles as well as the amount of them (useful for later looping).
*/
    let mut clipped = [Triangle::new_origin(), Triangle::new_origin()];
    let n_clipped_triangles = triangle_clip_against_plane(
        // Triangles clipped due to being partially behind camera
        Vec3d {x: 0.0,y: 0.0,z: 0.1,},
        &mut Vec3d {x: 0.0,y: 0.0,z: 1.0},
        tri_pre_clipping,
        &mut clipped,
    );
    (clipped, n_clipped_triangles)
}
