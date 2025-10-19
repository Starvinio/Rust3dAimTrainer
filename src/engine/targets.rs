use crate::engine::consts::CONFIG;
use crate::engine::shapes::create_tri;
use crate::engine::structures::Triangle;

use rand::Rng;

pub struct Target {
    pub tris: Vec<Triangle>,
    pub hp: i32,
}
pub enum TargetType {
    Square,
    Sphere,
    Rectangle,
}

impl Target {
    pub fn sphere(
        x: f32,
        y: f32,
        z: f32,
        target_d: f32,
        slices: usize,
        stacks: usize,
        hp: i32,
    ) -> Self {
        let mut tris = Vec::new();

        let target_r = target_d / 2.0;

        // Loop over stacks (latitude)
        for i in 0..stacks {
            let phi1 = std::f32::consts::PI * (i as f32 / stacks as f32 - 0.5);
            let phi2 = std::f32::consts::PI * ((i + 1) as f32 / stacks as f32 - 0.5);

            let y1 = target_r * phi1.sin();
            let y2 = target_r * phi2.sin();

            let r1 = target_r * phi1.cos();
            let r2 = target_r * phi2.cos();

            // Loop over slices (longitude)
            for j in 0..slices {
                let theta1 = 2.0 * std::f32::consts::PI * (j as f32 / slices as f32);
                let theta2 = 2.0 * std::f32::consts::PI * ((j + 1) as f32 / slices as f32);

                let p1 = (x + r1 * theta1.cos(), y + y1, z + r1 * theta1.sin());
                let p2 = (x + r2 * theta1.cos(), y + y2, z + r2 * theta1.sin());
                let p3 = (x + r2 * theta2.cos(), y + y2, z + r2 * theta2.sin());
                let p4 = (x + r1 * theta2.cos(), y + y1, z + r1 * theta2.sin());

                // Create two triangles per quad
                tris.push(create_tri(p1, p2, p3));
                tris.push(create_tri(p1, p3, p4));
            }
        }

        Self { tris, hp }
    }

    pub fn square(x: f32, y: f32, z: f32, hp: i32) -> Self {
        Self {
            tris: vec![
                // SOUTH
                create_tri(
                    (0.0 + x, 0.0 + y, 0.0 + z),
                    (0.0 + x, 1.0 + y, 0.0 + z),
                    (1.0 + x, 1.0 + y, 0.0 + z),
                ),
                create_tri(
                    (0.0 + x, 0.0 + y, 0.0 + z),
                    (1.0 + x, 1.0 + y, 0.0 + z),
                    (1.0 + x, 0.0 + y, 0.0 + z),
                ),
                // EAST
                create_tri(
                    (1.0 + x, 0.0 + y, 0.0 + z),
                    (1.0 + x, 1.0 + y, 0.0 + z),
                    (1.0 + x, 1.0 + y, 1.0 + z),
                ),
                create_tri(
                    (1.0 + x, 0.0 + y, 0.0 + z),
                    (1.0 + x, 1.0 + y, 1.0 + z),
                    (1.0 + x, 0.0 + y, 1.0 + z),
                ),
                // NORTH
                create_tri(
                    (1.0 + x, 0.0 + y, 1.0 + z),
                    (1.0 + x, 1.0 + y, 1.0 + z),
                    (0.0 + x, 1.0 + y, 1.0 + z),
                ),
                create_tri(
                    (1.0 + x, 0.0 + y, 1.0 + z),
                    (0.0 + x, 1.0 + y, 1.0 + z),
                    (0.0 + x, 0.0 + y, 1.0 + z),
                ),
                // WEST
                create_tri(
                    (0.0 + x, 0.0 + y, 1.0 + z),
                    (0.0 + x, 1.0 + y, 1.0 + z),
                    (0.0 + x, 1.0 + y, 0.0 + z),
                ),
                create_tri(
                    (0.0 + x, 0.0 + y, 1.0 + z),
                    (0.0 + x, 1.0 + y, 0.0 + z),
                    (0.0 + x, 0.0 + y, 0.0 + z),
                ),
                // TOP
                create_tri(
                    (0.0 + x, 1.0 + y, 0.0 + z),
                    (0.0 + x, 1.0 + y, 1.0 + z),
                    (1.0 + x, 1.0 + y, 1.0 + z),
                ),
                create_tri(
                    (0.0 + x, 1.0 + y, 0.0 + z),
                    (1.0 + x, 1.0 + y, 1.0 + z),
                    (1.0 + x, 1.0 + y, 0.0 + z),
                ),
                // BOTTOM
                create_tri(
                    (1.0 + x, 0.0 + y, 1.0 + z),
                    (0.0 + x, 0.0 + y, 1.0 + z),
                    (0.0 + x, 0.0 + y, 0.0 + z),
                ),
                create_tri(
                    (1.0 + x, 0.0 + y, 1.0 + z),
                    (0.0 + x, 0.0 + y, 0.0 + z),
                    (1.0 + x, 0.0 + y, 0.0 + z),
                ),
            ],
            hp: hp,
        }
    }

    pub fn rectangle(x: f32, y: f32, z: f32, hp: i32) -> Self {
        Target {
            tris: vec![
                create_tri(
                    (0.0 + x, 0.0 + y, 0.0 + z),
                    (0.0 + x, 1.0 + y, 0.0 + z),
                    (1.0 + x, 1.0 + y, 0.0 + z),
                ),
                create_tri(
                    (0.0 + x, 0.0 + y, 0.0 + z),
                    (1.0 + x, 1.0 + y, 0.0 + z),
                    (1.0 + x, 0.0 + y, 0.0 + z),
                ),
            ],
            hp: hp,
        }
    }

    //Coming Soon
    //pub fn bean
    //pub fn humanoid
}

pub fn add_target(
    target_type: &TargetType,
    spawn_d: f32,
    target_vec: &mut Vec<Target>,
    target_dist: f32,
    hp: i32,
    old_target: &Option<(f32, f32)>,
) {
    let mut rng = rand::thread_rng();

    let spawn_r = spawn_d / 2.0;
    'outer: loop {
        let x = rng.gen_range(-spawn_r..spawn_r);
        let y = rng.gen_range(-spawn_r..spawn_r);

        let new_x = -0.5 + x * 1.5;
        let new_y = -0.5 + y * 1.5;

        for target in target_vec.iter() {
            let existing_x = target.tris[0].p[0].x;
            let existing_y = target.tris[0].p[0].y;

            if (new_x - existing_x).abs() < 1.0 && (new_y - existing_y).abs() < 1.0 {
                continue 'outer;
            }
            if let Some(old) = old_target {
                if (new_x - old.0).abs() < 1.0 && (new_y - old.1).abs() < 1.0 {
                    continue 'outer;
                }
            }
        }
        match target_type {
            TargetType::Square => {
                target_vec.push(Target::square(new_x, new_y, target_dist, hp));
            }
            TargetType::Sphere => target_vec.push(Target::sphere(
                new_x,
                new_y,
                target_dist,
                1.0,
                CONFIG.environment.sphere_detail,
                CONFIG.environment.sphere_detail,
                hp,
            )),
            TargetType::Rectangle => {
                target_vec.push(Target::rectangle(new_x, new_y, target_dist, hp))
            }
        }
        break;
    }
}

pub fn create_target_vec(
    target_type: &TargetType,
    count: i8,
    spawn_d: f32,
    target_dist: f32,
    hp: i32,
) -> (Vec<Target>, Option<(f32, f32)>) {
    let mut target_vec: Vec<Target> = Vec::new();
    for _ in 0..count {
        add_target(
            &target_type,
            spawn_d,
            &mut target_vec,
            target_dist,
            hp,
            &None,
        );
    }
    (target_vec, Option::None)
}
