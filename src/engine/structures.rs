use serde::Deserialize;

#[derive(Clone, Copy, Debug)]
pub struct Vec3d {
    // Saves coordinates in cartesian space
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) -> Vec3d {
        let l = &self.length();
        self.x /= l;
        self.y /= l;
        self.z /= l;

        *self
    }

    pub fn to(&self, to: &Vec3d) -> Vec3d {
        Vec3d {
            x: to.x - self.x,
            y: to.y - self.y,
            z: to.z - self.z,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    // Groups together 3 Vectors to create a Triangle
    pub p: [Vec3d; 3],
}
impl Triangle {
    pub fn new_empty() -> Self {
        Self {
            p: [Vec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }; 3],
        }
    }

    pub fn avg_z(&self) -> f32 {
        self.p[0].z + self.p[1].z + self.p[2].z / 3.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TriToRaster {
    pub tri: Triangle,
    pub color: u32,
    pub avg_z: f32,
}

pub struct Mesh {
    // Groups together any number of triangles to a mesh
    pub tris: Vec<Triangle>,
}

#[derive(Clone, Copy, Debug)]
pub struct Mat4x4 {
    // Creates a 4x4 Matrix for Projection Calculation
    pub m: [[f32; 4]; 4],
}
impl Mat4x4 {
    pub fn new_empty() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub display: Display,
    pub camera: Camera,
    pub input: Input,
    pub target: Target,
    pub environment: Environment,
}

#[derive(Debug, Deserialize)]
pub struct Display {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Deserialize)]
pub struct Camera {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub sensitivity: f32,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    pub color: [u8; 3],
}

#[derive(Debug, Deserialize)]
pub struct Environment {
    pub sphere_detail: usize,
}
