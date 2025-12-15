use std::ops::{Add, Sub, Mul, Div};
use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3d {
    pub x: f32, pub y: f32, pub z: f32,
}
impl Add for Vec3d {
    type Output = Vec3d;

    fn add(self, v2: Vec3d) -> Vec3d {
        Vec3d { x: self.x + v2.x, y: self.y + v2.y, z: self.z + v2.z }
    }
}
impl Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, v2: Vec3d) -> Vec3d {
        Vec3d { x: self.x - v2.x, y: self.y - v2.y, z: self.z - v2.z }
    }
}
impl Mul<f32> for Vec3d {
    type Output = Vec3d;

    fn mul(self, k: f32) -> Self::Output {
        Vec3d { x: self.x * k, y: self.y * k, z: self.z * k}
    }
}
impl Div<f32> for Vec3d {
    type Output = Vec3d;

    fn div(self, k: f32) -> Self::Output {
        Vec3d { x: self.x / k, y: self.y / k, z: self.z / k}
    }
}
impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {Self {x:0.0, y:0.0, z:0.0}}
    pub fn from_tuple(tuple:(f32,f32,f32)) -> Self {
        Self { x: tuple.0, y: tuple.1, z: tuple.2 }
    }
    pub fn from_rng_range(a:Vec3d, b:Vec3d, rng:&mut ThreadRng) -> Self {
        Self {
            x: rng.gen_range(a.x.min(b.x)..=a.x.max(b.x)),
            y: rng.gen_range(a.y.min(b.y)..=a.y.max(b.y)),
            z: rng.gen_range(a.z.min(b.z)..=a.z.max(b.z)),
        }
    }
    pub fn to_f32(&self) -> (f32,f32,f32) {
        (self.x, self.y, self.z)
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Vec3d {
        let l = self.length();
        Vec3d::new(self.x / l as f32, self.y / l as f32, self.z / l as f32)
    }
    pub fn vec_to(&self, to: &Vec3d) -> Vec3d {
        Vec3d { x: to.x - self.x, y: to.y - self.y, z: to.z - self.z }
    }
    pub fn invert(&self) -> Vec3d {
        Vec3d::new(-self.x, -self.y, -self.z)
    }
    pub fn dot(&self, v:Vec3d) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    pub fn cross(&self, v:Vec3d) -> Vec3d {
        Vec3d::new(
        self.y * v.z - self.z * v.y,
        self.z * v.x - self.x * v.z,
        self.x * v.y - self.y * v.x,
        )
    }
    pub fn hadamard(&self, v: Vec3d) -> Vec3d {
        Vec3d::new(
            self.x * v.x,
            self.y * v.y,
            self.z * v.z
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2d {
/* 
    This Vector is used for projection as well as texturing.
    Since screens are 2d, we dont have to store the z coordinate after projection to screen space.
*/
    pub x:f32, pub y:f32
}
impl Add for Vec2d {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Sub for Vec2d {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl Mul<f32> for Vec2d {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Vec2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y}
    }
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn dot(self, rhs: Vec2d) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn normalize(self) -> Self {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        Self { x: self.x / len, y: self.y / len }
    }
}
