use crate::engine::{Mesh, Triangle};

#[derive(Clone, Copy, Debug)]
pub enum RoomType {
    Cube,
    Wall,
    Cone,
    Octagon
}

pub fn create_room(room_type:RoomType, radius: f32) -> Mesh {
/*
    Center of a room will always be the origin (0,0,0).
    Therefore, radius spins around the origin.
    If offset is required, we will move the player's spawn instead.
*/

    match room_type {
        RoomType::Cube => { cube_room(radius) },
        RoomType::Wall => { wall_room(radius) },
        RoomType::Cone => { cone_room(radius) },
        RoomType::Octagon => { octagon_room(radius) }
    }
}

fn cube_room(rad: f32) -> Mesh {

    Mesh {
        tris: vec![
            // SOUTH (z = -rad)
            Triangle::new((-rad, -rad, -rad), (rad, rad, -rad), (-rad, rad, -rad)),
            Triangle::new((-rad, -rad, -rad), (rad, -rad, -rad), (rad, rad, -rad)),

            // NORTH (z = +rad)
            Triangle::new((rad, -rad, rad), (-rad, rad, rad), (rad, rad, rad)),
            Triangle::new((rad, -rad, rad), (-rad, -rad, rad), (-rad, rad, rad)),

            // EAST (x = +rad)
            Triangle::new((rad, -rad, -rad), (rad, rad, rad), (rad, rad, -rad)),
            Triangle::new((rad, -rad, -rad), (rad, -rad, rad), (rad, rad, rad)),

            // WEST (x = -rad)
            Triangle::new((-rad, -rad, rad), (-rad, rad, -rad), (-rad, rad, rad)),
            Triangle::new((-rad, -rad, rad), (-rad, -rad, -rad), (-rad, rad, -rad)),

            // TOP (y = +rad)
            Triangle::new((-rad, rad, -rad), (rad, rad, rad), (-rad, rad, rad)),
            Triangle::new((-rad, rad, -rad), (rad, rad, -rad), (rad, rad, rad)),

            // BOTTOM (y = -rad)
            Triangle::new((rad, -rad, rad), (-rad, -rad, -rad), (-rad, -rad, rad)),
            Triangle::new((rad, -rad, rad), (rad, -rad, -rad), (-rad, -rad, -rad)),
        ],
    }
}

fn wall_room(rad: f32) -> Mesh {

    Mesh {
        tris: vec![
        Triangle::new((rad, -rad, rad), (-rad, rad, rad), (rad, rad, rad)),
        Triangle::new((rad, -rad, rad), (-rad, -rad, rad), (-rad, rad, rad)),
        ]
    }       
}

fn cone_room(rad:f32) -> Mesh {
    Mesh {
        tris: vec![
            // NORTH (z = +rad)
            Triangle::new((rad, -rad, rad), (-rad, rad, rad), (rad, rad, rad)),
            Triangle::new((rad, -rad, rad), (-rad, -rad, rad), (-rad, rad, rad)),

            // BOTTOM (y = -rad)
            Triangle::new((rad, -rad, rad), (0.0, -rad, -rad), (-rad, -rad, rad)),

            // TOP (y = +rad)
            Triangle::new((0.0, rad, -rad), (rad, rad, rad), (-rad, rad, rad)),

            // EAST (x = +rad)
            Triangle::new((0.0, -rad, -rad), (rad, rad, rad), (0.0, rad, -rad)),
            Triangle::new((0.0, -rad, -rad), (rad, -rad, rad), (rad, rad, rad)),

            // WEST (x = -rad)
            Triangle::new((-rad, -rad, rad), (0.0, rad, -rad), (-rad, rad, rad)),
            Triangle::new((-rad, -rad, rad), (0.0, -rad, -rad), (0.0, rad, -rad)),
        ]
    }
}

fn octagon_room(rad: f32) -> Mesh {


//  For sake of simplicity during creation, height will equal the radius
    let height = rad;

    let mut tris = Vec::new();
    let angle_step = std::f32::consts::PI / 4.0; // 8 sides

    // Generate 8 base vertices (bottom and top)
    let mut bottom = Vec::new();
    let mut top = Vec::new();

    for i in 0..8 {
        let theta = i as f32 * angle_step;
        let x = rad * theta.cos();
        let z = rad * theta.sin();
        bottom.push((x, -height / 2.0, z));
        top.push((x, height / 2.0, z));
    }

    // Side faces
    for i in 0..8 {
        let next = (i + 1) % 8;
        let b1 = bottom[i];
        let b2 = bottom[next];
        let t1 = top[i];
        let t2 = top[next];

        // Each rectangular side = 2 triangles
        tris.push(Triangle::new(b1, b2, t1));
        tris.push(Triangle::new(t1, b2, t2));
    }

    // Top face (fan from center)
    let top_center = (0.0, height / 2.0, 0.0);
    for i in 0..8 {
        let next = (i + 1) % 8;
        tris.push(Triangle::new(top[i], top[next], top_center));
    }

    // Bottom face (fan from center)
    let bottom_center = (0.0, -height / 2.0, 0.0);
    for i in 0..8 {
        let next = (i + 1) % 8;
        // reverse order to keep normal facing outward
        tris.push(Triangle::new(bottom[next], bottom[i], bottom_center));
    }

    Mesh { tris }
}