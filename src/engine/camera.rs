use std::time::Instant;
use std::f32::consts::{PI, FRAC_PI_2};

use crate::engine::{Mat4x4, FPS_DIGIT_WIDTH};
use crate::engine::core::{Vec3d, CONFIG};

pub struct Camera {
    pub position: Vec3d,
    pub look_dir: Vec3d,
    pub view_matrix: Mat4x4,
    pub yaw: f32,
    pub pitch: f32, 
    pub last_frame_time:Instant
}

impl Camera {
    pub fn new(pos:Vec3d) -> Self {
        let look_dir = Vec3d::new(0.0, 0.0, 1.0);
        let up = Vec3d {x: 0.0, y: 1.0, z: 0.0};
        Self {
            position:pos,
            look_dir,
            view_matrix: Mat4x4::point_at(&pos, &(pos + look_dir), &up).quick_inverse(),
            yaw:0.0,
            pitch:0.0,
            last_frame_time:Instant::now()
        }
    }
    pub fn update_yaw_pitch(&mut self, delta_x:f64, delta_y:f64) {
        let sens = CONFIG.input.sensitivity;
        self.yaw += delta_x as f32 * sens * 0.001;
        self.pitch -= delta_y as f32 * sens * 0.001;
    }
    pub fn update_look_dir(&mut self) {
        self.pitch = self.pitch.clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01);
        self.yaw %= 2.0 * PI;
        self.look_dir.x = self.pitch.cos() * self.yaw.sin();
        self.look_dir.y = self.pitch.sin();
        self.look_dir.z = self.pitch.cos() * self.yaw.cos();
        self.look_dir = self.look_dir.normalize();
    }
    pub fn update_view_matrix(&mut self) {
        let up = Vec3d {x: 0.0, y: 1.0, z: 0.0};
        self.view_matrix = Mat4x4::point_at(&self.position, &(self.position + self.look_dir), &up).quick_inverse();
    }
}

pub struct FPS {
    pub total_frame_count: u32,
    pub fps_str: String,
    pub interval_frame_count: u32,
    pub last_fps_display: Instant,
    pub width_px: usize
}
impl FPS {
    pub fn init() -> Self {
        Self {
            total_frame_count: 0,
            fps_str: String::from("0"),
            interval_frame_count: 0,
            last_fps_display: Instant::now(),
            width_px: FPS_DIGIT_WIDTH
        }
    }
    pub fn update_str(&mut self, now: Instant) {
        if now.duration_since(self.last_fps_display).as_secs_f32() >= 1.0 {
            self.fps_str = self.interval_frame_count.to_string();
            self.last_fps_display = Instant::now();
            self.interval_frame_count = 0;
            self.width_px = FPS_DIGIT_WIDTH * self.fps_str.len();
        }
    }
}

