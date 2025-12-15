
use crate::engine::{CONFIG, Mat4x4, Triangle, Vec3d, dyn_clamp_pos};
use rand::{Rng, rngs::ThreadRng};
use rodio::decoder::Settings;

pub struct Target {
    // Triangles for target visuals
    pub tris: Vec<Triangle>,

    // Target properties
    pub radius: f32, // stored to avoid clipping
    pub hp: i32,

    // Variables concerning target movement
    pub movement: Option<MovingTarget>,
    pub position: Vec3d,
    pub velocity: Vec3d,
}
#[derive(Debug, Clone, Copy)]
pub struct MovingTarget {
    pub mv_bounds: (Vec3d, Vec3d), //  Cuboid space of movement, corners marked by vectors
    pub player_dist_r: f32, //  Radius around the player that the target stays at
    pub frequency: f32, //  How often should we generate a direction change
    pub p_change_dir: f64, //  How likely is it to change direction upon a check?
    pub vel_bounds: (Vec3d, Vec3d), //  How slow/fast does it move on each axis
    pub interval_dir_change: f32, //  How long since last direction change
}
#[derive(Copy, Clone)]
pub enum TargetShape {
    Block,
    Sphere,
    Square,
    Bean
}
pub struct TargetVec {
    pub vec: Vec<Target>,
    pub settings: TargetSettings,
    pub old: Option<Vec3d>
}
#[derive(Clone, Copy)]
pub struct TargetSettings {
    pub shape: TargetShape,
    pub spawn: (Vec3d, Vec3d),
    pub count: usize,
    pub rad: f32,
    pub hp: i32,
    pub movement: Option<MovingTarget>,
}
impl Target {
    pub fn block(t_settings: TargetSettings, pos:Vec3d, velo:Vec3d) -> Self {
        let rad = t_settings.rad;
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
            movement: t_settings.movement,
            hp: t_settings.hp,
            radius:rad,
            position: pos,
            velocity: velo,
        }
    }
    pub fn square(t_settings: TargetSettings, pos:Vec3d, velo:Vec3d) -> Self {
        let rad = t_settings.rad;
        let v0 = Vec3d::new(-rad, -rad, -rad);
        let v1 = Vec3d::new(-rad, rad, -rad);
        let v2 = Vec3d::new(rad, rad, -rad);
        let v3 = Vec3d::new(rad, -rad, -rad);

        let look_vec = pos.vec_to(&Vec3d::zero()).normalize();
        let default_normal = Vec3d::new(0.0, 0.0, -1.0);
        let mut axis = default_normal.cross(look_vec);
        let mut angle = -default_normal.dot(look_vec)
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
            movement: t_settings.movement,
            radius:rad,
            hp: t_settings.hp,
            position: pos,
            velocity: velo,
        }
    }
    pub fn sphere(t_settings:TargetSettings, position:Vec3d, velo:Vec3d) -> Self {
        let detail = CONFIG.targets.sphere_detail;
        let rad = t_settings.rad;

        let mut tris = Vec::with_capacity(detail * detail * 2);
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

                let p1 = (r1 * theta1.cos(), y1, r1 * theta1.sin());
                let p2 = (r2 * theta1.cos(), y2, r2 * theta1.sin());
                let p3 = (r2 * theta2.cos(), y2, r2 * theta2.sin());
                let p4 = (r1 * theta2.cos(), y1, r1 * theta2.sin());

                tris.push(Triangle::new(p1, p2, p3));
                tris.push(Triangle::new(p1, p3, p4));
            }
        }

        Self {
            tris,
            movement: t_settings.movement,
            radius: rad,
            hp: t_settings.hp,
            position,
            velocity: velo,
        }
    }
    pub fn bean(t_settings: TargetSettings, pos:Vec3d, velo:Vec3d) -> Self {
        let mut tris = Vec::new();

        let rad = t_settings.rad; let stretch_y = rad * 5.0;

        let detail = CONFIG.targets.sphere_detail;

        for i in 0..detail {
            let phi1 = std::f32::consts::PI * (i as f32 / detail as f32 - 0.5);
            let phi2 = std::f32::consts::PI * ((i + 1) as f32 / detail as f32 - 0.5);
            let y1 = rad * phi1.sin();
            let y2 = rad * phi2.sin();
            let r1 = rad * phi1.cos();
            let r2 = rad * phi2.cos();

            fn non_uniform_scl(c_rad:f32, stretch_y:f32, r: f32, y:f32, theta:f32) -> (f32, f32, f32) {
                (c_rad * r * theta.cos(),
                 stretch_y * y,
                 c_rad * r * theta.sin()
                )
            }

            for j in 0..detail {
                let theta1 = 2.0 * std::f32::consts::PI * (j as f32 / detail as f32);
                let theta2 = 2.0 * std::f32::consts::PI * ((j + 1) as f32 / detail as f32);

                let p1 = non_uniform_scl(rad, stretch_y, r1, y1, theta1);
                let p2 = non_uniform_scl(rad, stretch_y, r2, y2, theta1);
                let p3 = non_uniform_scl(rad, stretch_y, r2, y2, theta2);
                let p4 = non_uniform_scl(rad, stretch_y, r1, y1, theta2);

                tris.push(Triangle::new(p1, p2, p3));
                tris.push(Triangle::new(p1, p3, p4));
            }
        }

        Self {
            tris,
            radius:t_settings.rad,
            hp: t_settings.hp,
            movement: t_settings.movement,
            position:pos,
            velocity: velo,
        }
    }

    pub fn random_movement(&mut self, cam_pos:Vec3d, rng: &mut ThreadRng, delta_time:f32) {

        if let Some(ref mut m) = self.movement {

            let freq = m.frequency * rng.gen_range(0.5..1.5);
            m.interval_dir_change += delta_time;

            if m.interval_dir_change >= freq {

                m.interval_dir_change = 0.0;

                let min = m.vel_bounds.0; let max = m.vel_bounds.1;

                //  Generate a direction vector that can be either positive or negative (per axis)
                self.velocity = Vec3d::new(
                    new_direction(m, self.velocity.x, rng, min.x, max.x),
                    new_direction(m, self.velocity.y, rng, min.y, max.y),
                    new_direction(m, self.velocity.z, rng, min.z, max.z)
                );
            }

            let from_player = self.position - cam_pos;
            let dist = from_player.length();

            if dist < m.player_dist_r {
                // Normalized direction *away* from player
                let away_dir = from_player / dist;

                // Compute how much velocity is pointing *toward or away* from the player
                let toward_component = away_dir * self.velocity.dot(away_dir);

                // If movement *toward* the player, remove that component
                if self.velocity.dot(away_dir) < 0.0 {
                    self.velocity = self.velocity - toward_component;
                }
            }

            // Get new position via random multiplication
            self.position = self.position + (self.velocity * delta_time);


            (self.position.x, self.velocity.x) = dyn_clamp_pos(
                self.position.x,
                self.velocity.x,
                m.mv_bounds.0.x,
                m.mv_bounds.1.x
            );
            (self.position.y, self.velocity.y) = dyn_clamp_pos(
                self.position.y,
                self.velocity.y,
                m.mv_bounds.0.y,
                m.mv_bounds.1.y
            );
            (self.position.z, self.velocity.z) = dyn_clamp_pos(
                self.position.z,
                self.velocity.z,
                m.mv_bounds.0.z,
                m.mv_bounds.1.z
            );
        }
    }
}
impl TargetVec {
    pub fn empty(settings: &TargetSettings) -> Self {
        Self {
            vec: Vec::with_capacity(settings.count),
            settings: settings.clone(),
            old:None,
        }
    }
    pub fn init(settings: &TargetSettings) -> Self {
        let mut empty = TargetVec::empty(settings);
        empty.fill();
        empty
    }
    pub fn add_target(&mut self) {
        let mut rng = rand::thread_rng();
        let start_vel = if let Some(ref mut m) = self.settings.movement {
            let min = m.vel_bounds.0;
            Vec3d::new(
                if rng.gen_bool(0.5) {-min.x} else {min.x},
                if rng.gen_bool(0.5) {-min.y} else {min.y},
                if rng.gen_bool(0.5) {-min.z} else {min.z},
            )
        } else {Vec3d::zero()};

        // cuboid-like spawn room based on 2 Vectors
        let (spawn_a, spawn_b) = self.settings.spawn;
        
        let spawn = if (spawn_b-spawn_a).length() > 0.0 {
            let n_loops:i8 = 0;
            let rad_2x = self.settings.rad * 2.0;
            'outer: loop {
                //create a random spawn location
                let spawn = Vec3d::from_rng_range(spawn_a, spawn_b, &mut rng);

                // Check if spawn loc collides with existing target
                if n_loops < 50 { // prevent infinite looping
                    // check each existing target
                    for existing_target in self.vec.iter() {
                        if (existing_target.position-spawn).length() < rad_2x {
                            // target to close to existing, generate loc again
                            continue 'outer;
                        }
                    }
                    // Optionally check prior target
                    if let Some(old) = self.old {
                        if (old-spawn).length() < rad_2x {
                            // target to close to prior, generate loc again
                            continue 'outer;
                        }
                    }
                }
                break spawn;
            }
        } else {
            spawn_a
        };

        match self.settings.shape {
            TargetShape::Block => self.vec.push(Target::block(self.settings, spawn, start_vel)),

            TargetShape::Square => self.vec.push(Target::square(self.settings, spawn, start_vel)),

            TargetShape::Sphere => self.vec.push(Target::sphere(self.settings, spawn, start_vel)),

            TargetShape::Bean => self.vec.push(Target::bean(self.settings, spawn, start_vel)),
        }
    }
    pub fn fill(&mut self)  {
        for _ in 0..self.settings.count {
            self.add_target();
        }
    }
}


pub fn new_direction(movement_target: &mut MovingTarget, velo:f32, rng:&mut ThreadRng, min:f32, max:f32) -> f32 {
    if velo > 0.0 {
        if rng.gen_bool(movement_target.p_change_dir / 100.0) {
            -min
        } else {
            (velo + velo / 100.0).clamp(-max, max)
        }
    } else {
        if rng.gen_bool(movement_target.p_change_dir / 100.0) {
            min
        } else {
            (velo + velo / 100.0).clamp(-max, max)
        }
    }
}


