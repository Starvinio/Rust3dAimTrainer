
use crate::engine::{CONFIG, Mat4x4, Scenario, Triangle, Vec3d, dyn_clamp_pos};
use std::{ops::Sub, time::{Duration, Instant}};

use rand::{Rng, rngs::ThreadRng};

pub struct Target {

    // Triangles for target visuals
    pub tris: Vec<Triangle>,

    // Target static properties
    pub radius: f32, // stored to avoid clipping
    pub hp: i32,

    // Variables concerning target movement
    pub moving: Option<MovingTarget>,
    pub position: Vec3d,
    pub velocity: Vec3d,
}
pub enum TargetType {
    Square,
    Sphere,
    Rectangle,
    Bean
}
impl Target {
    pub fn sphere(position:Vec3d, radius: f32, moving: &mut Option<MovingTarget>, hp: i32, velo:Vec3d) -> Self {
        let mut tris = Vec::new();

        let detail = CONFIG.targets.sphere_detail;

        for i in 0..detail {
            let phi1 = std::f32::consts::PI * (i as f32 / detail as f32 - 0.5);
            let phi2 = std::f32::consts::PI * ((i + 1) as f32 / detail as f32 - 0.5);
            let y1 = radius * phi1.sin();
            let y2 = radius * phi2.sin();
            let r1 = radius * phi1.cos();
            let r2 = radius * phi2.cos();

            for j in 0..detail {
                let theta1 = 2.0 * std::f32::consts::PI * (j as f32 / detail as f32);
                let theta2 = 2.0 * std::f32::consts::PI * ((j + 1) as f32 / detail as f32);

                let p1 = (r1 * theta1.cos(), y1, r1 * theta1.sin());
                let p2 = (r2 * theta1.cos(), y2, r2 * theta1.sin());
                let p3 = (r2 * theta2.cos(), y2, r2 * theta2.sin());
                let p4 = (r1 * theta2.cos(), y1, r1 * theta2.sin());

                tris.push(Triangle::new(p1, p2, p3));
                tris.push(Triangle::new(p1, p3, p4));
            }
        }


        let mut can_move:Option<MovingTarget> = None;
        if let Some(moving_target) = moving {
            moving_target.last_dir_change_time = Instant::sub(Instant::now(), Duration::from_secs_f32(moving_target.frequency));
            can_move = Some(*moving_target)
        } 

        Self {
            tris,
            moving: can_move,
            radius,
            hp,
            position,
            velocity: velo,
        }
    }

    pub fn square(pos:Vec3d, rad: f32, hp: i32, velo:Vec3d) -> Self {
        Self {
            tris: vec![
                Triangle::new((-rad, -rad, -rad), (-rad, rad, -rad), (rad, rad, -rad)),
                Triangle::new((-rad, -rad, -rad), (rad, rad, -rad), (rad, -rad, -rad)),
                Triangle::new((rad, -rad, -rad), (rad, rad, -rad), (rad, rad, rad)),
                Triangle::new((rad, -rad, -rad), (rad, rad, rad), (rad, -rad, rad)),
                Triangle::new((rad, -rad, rad), (rad, rad, rad), (-rad, rad, rad)),
                Triangle::new((rad, -rad, rad), (-rad, rad, rad), (-rad, -rad, rad)),
                Triangle::new((-rad, -rad, rad), (-rad, rad, rad), (-rad, rad, -rad)),
                Triangle::new((-rad, -rad, rad), (-rad, rad, -rad), (-rad, -rad, -rad)),
                Triangle::new((-rad, rad, -rad), (-rad, rad, rad), (rad, rad, rad)),
                Triangle::new((-rad, rad, -rad), (rad, rad, rad), (rad, rad, -rad)),
                Triangle::new((rad, -rad, rad), (-rad, -rad, rad), (-rad, -rad, -rad)),
                Triangle::new((rad, -rad, rad), (-rad, -rad, -rad), (rad, -rad, -rad)),
            ],
            moving: None,
            hp,
            radius:rad,
            position: pos,
            velocity: velo,
        }
    }

    pub fn rectangle(pos:Vec3d, rad: f32, hp: i32, player_spawn:Vec3d, velo:Vec3d) -> Self {
        let v0 = Vec3d::new(-rad, -rad, -rad);
        let v1 = Vec3d::new(-rad, rad, -rad);
        let v2 = Vec3d::new(rad, rad, -rad);
        let v3 = Vec3d::new(rad, -rad, -rad);

        let look_vec = pos.vec_to(&player_spawn).normalize();
        let default_normal = Vec3d::new(0.0, 0.0, -1.0);
        let mut axis = default_normal.cross(look_vec);
        let mut angle = -(default_normal.dot(look_vec))
            .clamp(-1.0, 1.0)
            .acos();

        if axis.length() < 1e-6 {
            if default_normal.dot(look_vec) > 0.0 {
                axis = Vec3d::new(1.0, 0.0, 0.0);
                angle = 0.0;
            } else {
                axis = Vec3d::new(1.0, 0.0, 0.0);
                angle = std::f32::consts::PI;
            }
        } else {
            axis = axis.normalize();
        }

        let mat_rot = Mat4x4::general_rotation(axis, angle);

        let v0r = mat_rot * v0;
        let v1r = mat_rot * v1;
        let v2r = mat_rot * v2;
        let v3r = mat_rot * v3;

        Target {
            tris: vec![
                Triangle { p: [v0r, v1r, v2r] },
                Triangle { p: [v0r, v2r, v3r] },
            ],
            moving: None,
            radius:rad,
            hp,
            position: pos,
            velocity: velo,
        }
    }

    pub fn bean(pos:Vec3d, rad: f32, hp: i32, stretch_xz:f32, stretch_y:f32, moving: Option<MovingTarget>, velo:Vec3d) -> Self {
        let mut tris = Vec::new();

        let detail = CONFIG.targets.sphere_detail;

        for i in 0..detail {
            let phi1 = std::f32::consts::PI * (i as f32 / detail as f32 - 0.5);
            let phi2 = std::f32::consts::PI * ((i + 1) as f32 / detail as f32 - 0.5);
            let y1 = rad * phi1.sin();
            let y2 = rad * phi2.sin();
            let r1 = rad * phi1.cos();
            let r2 = rad * phi2.cos();

            for j in 0..detail {
                let theta1 = 2.0 * std::f32::consts::PI * (j as f32 / detail as f32);
                let theta2 = 2.0 * std::f32::consts::PI * ((j + 1) as f32 / detail as f32);

                // Apply the non-uniform scaling here
                let p1 = (
                    stretch_xz * r1 * theta1.cos(),
                    stretch_y * y1,
                    stretch_xz * r1 * theta1.sin(),
                );
                let p2 = (
                    stretch_xz * r2 * theta1.cos(),
                    stretch_y * y2,
                    stretch_xz * r2 * theta1.sin(),
                );
                let p3 = (
                    stretch_xz * r2 * theta2.cos(),
                    stretch_y * y2,
                    stretch_xz * r2 * theta2.sin(),
                );
                let p4 = (
                    stretch_xz * r1 * theta2.cos(),
                    stretch_y * y1,
                    stretch_xz * r1 * theta2.sin(),
                );

                tris.push(Triangle::new(p1, p2, p3));
                tris.push(Triangle::new(p1, p3, p4));
            }
        }

        Self {
            tris,
            radius:rad,
            hp,
            moving,
            position:pos,
            velocity: velo,
        }
    }

    pub fn random_movement(&mut self, cam_pos:Vec3d, now:Instant, rng: &mut ThreadRng, delta_time:f32) {

        if let Some(ref mut moving_target) = self.moving {
            if now
                .duration_since(moving_target.last_dir_change_time)
                .as_secs_f32()
                >= moving_target.frequency * now
                .duration_since(moving_target.last_dir_change_time)
                .as_secs_f32()
            {
            
            //  Generate a direction vector that can be either positive or negative (per axis)
                let dir_vec = Vec3d::new(
                    new_direction(moving_target, self.velocity.x, rng),
                    new_direction(moving_target, self.velocity.y, rng),
                    new_direction(moving_target, self.velocity.z, rng)
                );

                self.velocity = dir_vec;

                moving_target.last_dir_change_time = now;
            }

            let from_player = self.position - cam_pos;
            let dist = from_player.length();

            if dist < moving_target.player_dist_r {
                // Normalized direction *away* from player
                let away_dir = from_player / dist;

                // Compute how much velocity is pointing *toward or away* from the player
                let toward_component = away_dir * self.velocity.dot(away_dir);

                // If moving *toward* the player, remove that component
                if self.velocity.dot(away_dir) < 0.0 {
                    self.velocity = self.velocity - toward_component;
                }
            }

            // Get new position via random multiplication
            self.position = self.position + (self.velocity * delta_time);

        
            (self.position.x, self.velocity.x) = dyn_clamp_pos(
                self.position.x, 
                self.velocity.x,
                moving_target.moving_room.0.x, 
                moving_target.moving_room.1.x
            );
            (self.position.y, self.velocity.y) = dyn_clamp_pos(
                self.position.y, 
                self.velocity.y,
                moving_target.moving_room.0.y, 
                moving_target.moving_room.1.y
            );
            (self.position.z, self.velocity.z) = dyn_clamp_pos(
                self.position.z, 
                self.velocity.z,
                moving_target.moving_room.0.z, 
                moving_target.moving_room.1.z
            );
        }
    }
}

pub fn add_target(scenario:&mut Scenario, target_vec: &mut Vec<Target>, old_target: Option<Vec3d>) {
    let mut rng = rand::thread_rng();

    let mut target_start_vel = Vec3d::new(0.0, 0.0, 0.0);

    if let Some(ref mut m) = scenario.moving_target {
        m.last_dir_change_time = Instant::now();
        let min = m.vel_min_max.0; let max = m.vel_min_max.1;
        target_start_vel = Vec3d::new(
            if rng.gen_bool(0.5) {-min} else {max},
            if rng.gen_bool(0.5) {-min} else {max},
            if rng.gen_bool(0.5) {-min} else {max},
        );
    }

    // cuboid-like spawn room based on 2 Vectors
    let spawn_from = scenario.target_spawn.0;
    let spawn_to = scenario.target_spawn.1;

    let mut spawn_loc:Vec3d;

    if (spawn_to-spawn_from).length() > 0.0 {
        let n_loops:i8 = 0; 

        'outer: loop {
            //create a random spawn location
            spawn_loc = Vec3d::new(
                gen_from_range(spawn_from.x, spawn_to.x, &mut rng),
                gen_from_range(spawn_from.y, spawn_to.y, &mut rng),
                gen_from_range(spawn_from.z, spawn_to.z, &mut rng)
            );
            
            // Check if spawn loc collides with existing target
            if n_loops < 50 { // prevent infinite looping 

                // check each existing target
                for existing_target in target_vec.iter() {
                    if (existing_target.position-spawn_loc).length() < scenario.target_rad * 2.0 {
                        // target to close to existing, generate loc again
                        continue 'outer;
                    }
                }
                // Optionally check prior target
                if let Some(old) = old_target {
                    if (old-spawn_loc).length() < scenario.target_rad*2.0 {
                        // target to close to prior, generate loc again
                        continue 'outer;
                    }
                }
            }
            break;
        }
    } else {
        spawn_loc = spawn_from;
    }
    
    match scenario.target_type {
        TargetType::Square => target_vec.push(Target::square(spawn_loc, scenario.target_rad, scenario.target_hp, target_start_vel)),

        TargetType::Rectangle => target_vec.push(Target::rectangle(spawn_loc, scenario.target_rad, scenario.target_hp, scenario.player_spawn, target_start_vel)),
        
        TargetType::Sphere => target_vec.push(Target::sphere(
            spawn_loc,
            scenario.target_rad,
            &mut scenario.moving_target,
            scenario.target_hp,
            target_start_vel
        )),

        TargetType::Bean => target_vec.push(Target::bean(
            spawn_loc, 
            scenario.target_rad,
            scenario.target_hp, 
            0.6, 
            1.5,
            scenario.moving_target,
            target_start_vel
        )),
    }
}

fn gen_from_range(a:f32, b:f32, rng:&mut ThreadRng) -> f32{
    if a == b { a } else { rng.gen_range(a.min(b)..a.max(b)) }
}

pub fn create_target_vec(scenario:&mut Scenario) -> (Vec<Target>, Option<Vec3d>) {
    let mut target_vec: Vec<Target> = Vec::new();
    let target_count = scenario.target_count;
    for _ in 0..target_count {
        add_target(
            scenario,
            &mut target_vec,
            None,
        );
    }
    (target_vec, Option::None)
}

pub fn new_direction(moving_target: &mut MovingTarget, velo:f32, rng:&mut ThreadRng) -> f32 {
    if velo > 0.0 {
        if rng.gen_bool(moving_target.p_change_dir / 100.0) {
            -moving_target.vel_min_max.0
        } else {
            (velo + velo / 100.0).clamp(-moving_target.vel_min_max.1, moving_target.vel_min_max.1)
        }
    } else {
        if rng.gen_bool(moving_target.p_change_dir / 100.0) {
            moving_target.vel_min_max.0
        } else {
            (velo + velo / 100.0).clamp(-moving_target.vel_min_max.1, moving_target.vel_min_max.1)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MovingTarget {
    pub moving_room: (Vec3d, Vec3d),
    pub player_dist_r: f32,
    pub frequency: f32,
    pub p_change_dir: f64,
    pub vel_min_max: (f32,f32),
    pub last_dir_change_time: Instant,
}


