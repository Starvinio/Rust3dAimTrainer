use std::ops::{Add, Sub};

use crate::engine::{Vec3d, core::Vec2d};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub p: [Vec3d; 3],
}
impl Add<Vec3d> for Triangle {
    type Output = Triangle;
    fn add(self, vec: Vec3d) -> Self::Output {
        Triangle {
            p: [
                self.p[0] + vec,
                self.p[1] + vec,
                self.p[2] + vec,
            ]
        }
    }
}
impl Sub<Vec3d> for Triangle {
    type Output = Triangle;
    fn sub(self, vec: Vec3d) -> Self::Output {
        Triangle {
            p: [
                self.p[0] - vec,
                self.p[1] - vec,
                self.p[2] - vec,
            ]
        }
    }
}
impl Triangle {
    pub fn new_origin() -> Self {
        Self { p: [Vec3d {x: 0.0, y: 0.0, z: 0.0}; 3] }
    }
    pub fn new(a: (f32, f32, f32), b: (f32, f32, f32), c: (f32, f32, f32)) -> Triangle {
        let p_a = Vec3d::new(a.0, a.1, a.2);
        let p_b = Vec3d::new(b.0, b.1, b.2);
        let p_c = Vec3d::new(c.0, c.1, c.2);
        Self { p: [p_a, p_b, p_c] }
    }
    pub fn avg_z(&self) -> f32 {
        (self.p[0].z + self.p[1].z + self.p[2].z) / 3.0
    }
    pub fn translate(&mut self, xyz: Vec3d) {
        self.p[0] = self.p[0] + xyz; self.p[1] = self.p[1] + xyz; self.p[2] = self.p[2] + xyz;
    }
    pub fn normal(&self) -> Vec3d {
        let line1 = self.p[0].vec_to(&self.p[1]);
        let line2 = self.p[0].vec_to(&self.p[2]);
        line1.cross(line2).normalize()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle2d {
    pub p: [Vec2d;3]
}
impl Triangle2d {
    pub fn new_origin() -> Self {
        Self { p: [Vec2d {x: 0.0, y: 0.0}; 3] }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TriToRaster {
    pub tri: Triangle2d, pub color: u32, pub avg_z: f32,
}