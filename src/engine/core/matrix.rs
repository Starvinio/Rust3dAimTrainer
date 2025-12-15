use std::ops::Mul;
use std::f32::consts::PI;
use crate::engine::Triangle;
use crate::engine::{Vec3d, Vec2d};
use crate::engine::CONFIG;

#[derive(Clone, Copy, Debug)]
pub struct Mat4x4 {
    // Creates a 4x4 Matrix for Vector Transformation
    pub m: [[f32; 4]; 4],
}
impl Mul<Vec3d> for Mat4x4 {
    type Output = Vec3d;
    fn mul(self, v: Vec3d) -> Vec3d {
        self.transform_vec(v)
    }
}
impl Mul<Triangle> for Mat4x4 {
    type Output = Triangle;
    fn mul(self, tri: Triangle) -> Triangle {
        Triangle {
            p: [
                self.transform_vec(tri.p[0]),
                self.transform_vec(tri.p[1]),
                self.transform_vec(tri.p[2]),
            ]
        }
    }
}
impl Mul for Mat4x4 {
    type Output = Mat4x4;
    fn mul(self, m: Mat4x4) -> Mat4x4 {
        let mut matrix = Mat4x4::zero();
        for c in 0..4 {
            for r in 0..4 {
                matrix.m[r][c] = 
                    self.m[r][0] * m.m[0][c] + 
                    self.m[r][1] * m.m[1][c] + 
                    self.m[r][2] * m.m[2][c] + 
                    self.m[r][3] * m.m[3][c];
            }
        }
        matrix
    }
}
impl Mat4x4 {
    pub fn zero() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }
    pub fn projection(width: f32, height:f32) -> Self {
        // Dynamically creates a projection matrix based on FOV, aspect ratio and far/near settings
        let fov_scale = 1.0 / (CONFIG.camera.fov * 0.5 * (PI / 180.0)).tan();
        let aspect = height as f32 / width as f32;
        let near = CONFIG.camera.near; let far = CONFIG.camera.far;
        Self {
            m: [
                [fov_scale * aspect,    0.0,                            0.0,                  0.0],

                [0.0,                   fov_scale,                      0.0,                  0.0],

                [0.0,                   0.010001,                       far / (far - near),   1.0],
                
                [0.0,                   (-far * near) / (far - near),   0.0,                  0.0],
            ],
        }
    }
    pub fn x_rotation(theta:f32) -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = theta.cos();
        m[0][2] = theta.sin();
        m[2][0] = -theta.sin();
        m[1][1] = 1.0;
        m[2][2] = theta.cos();
        m[3][3] = 1.0;
        Self { m }
    }
    pub fn y_rotation(theta:f32) -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = theta.cos();
        m[0][2] = theta.sin();
        m[2][0] = -theta.sin();
        m[1][1] = 1.0;
        m[2][2] = theta.cos();
        m[3][3] = 1.0;
        Self { m }
    }
    pub fn z_rotation(theta:f32) -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = theta.cos();
        m[0][1] = theta.sin();
        m[1][0] = -theta.sin();
        m[1][1] = theta.cos();
        m[2][2] = 1.0;
        m[3][3] = 1.0;
        Self { m }
    }
    pub fn general_rotation(axis: Vec3d, theta:f32) -> Self {
        let axis = axis.normalize();
        let cos = theta.cos();
        let sin = theta.sin();
        let one_minus = 1.0 - cos;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        let mut m = Mat4x4::zero();

        m.m[0][0] = cos + x * x * one_minus;
        m.m[0][1] = x * y * one_minus - z * sin;
        m.m[0][2] = x * z * one_minus + y * sin;
        m.m[0][3] = 0.0;

        m.m[1][0] = y * x * one_minus + z * sin;
        m.m[1][1] = cos + y * y * one_minus;
        m.m[1][2] = y * z * one_minus - x * sin;
        m.m[1][3] = 0.0;

        m.m[2][0] = z * x * one_minus - y * sin;
        m.m[2][1] = z * y * one_minus + x * sin;
        m.m[2][2] = cos + z * z * one_minus;
        m.m[2][3] = 0.0;

        m.m[3][0] = 0.0;
        m.m[3][1] = 0.0;
        m.m[3][2] = 0.0;
        m.m[3][3] = 1.0;

        m
    }
    pub fn point_at(pos: &Vec3d, target: &Vec3d, up: &Vec3d) -> Self {
        
        //Calculate new forward direction
        let new_forward: Vec3d = (*target - *pos).normalize();

        //Calculate new up direction
        let a: Vec3d = new_forward * up.dot(new_forward);
        let new_up: Vec3d = (*up - a).normalize();

        //Calculate new right direction
        let new_right: Vec3d = new_forward.cross(new_up).normalize();

        Mat4x4 { m: 
         [
            [ new_right.x,      new_right.y,    new_right.z,    0.0 ],
            [ new_up.x,         new_up.y,       new_up.z,       0.0 ],
            [ new_forward.x,    new_forward.y,  new_forward.z,  0.0 ],
            [ pos.x,            pos.y,          pos.z,          1.0 ],
        ],
      }
    }
    pub fn transform_vec(&self, v: Vec3d) -> Vec3d {
        let x = v.x * self.m[0][0] + v.y * self.m[1][0] + v.z * self.m[2][0] + self.m[3][0];
        let y = v.x * self.m[0][1] + v.y * self.m[1][1] + v.z * self.m[2][1] + self.m[3][1];
        let z = v.x * self.m[0][2] + v.y * self.m[1][2] + v.z * self.m[2][2] + self.m[3][2];
        let w = v.x * self.m[0][3] + v.y * self.m[1][3] + v.z * self.m[2][3] + self.m[3][3];

        if w != 0.0 {
            Vec3d::new(x / w, y / w, z / w)
        } else {
            Vec3d::new(x, y, z)
        }
    }
    pub fn project_vec(&self, v: Vec3d) -> Vec2d {
        let x = v.x * self.m[00][0] + v.y * self.m[1][0] + v.z * self.m[2][0] + self.m[3][0];
        let y = v.x * self.m[00][1] + v.y * self.m[1][1] + v.z * self.m[2][1] + self.m[3][1];
        let w = v.x * self.m[00][3] + v.y * self.m[1][3] + v.z * self.m[2][3] + self.m[3][3];

        if w != 0.0 {
            Vec2d::new(x / w, y / w)
        } else {
            Vec2d::new(x, y)
        }
    }
    pub fn quick_inverse(&self) -> Mat4x4 {
        // !!Only use for rotation/translation matrices!!
        let mut matrix: Mat4x4 = Mat4x4::zero();

        matrix.m[0][0] = self.m[0][0];
        matrix.m[0][1] = self.m[1][0];
        matrix.m[0][2] = self.m[2][0];
        matrix.m[0][3] = 0.0;
        matrix.m[1][0] = self.m[0][1];
        matrix.m[1][1] = self.m[1][1];
        matrix.m[1][2] = self.m[2][1];
        matrix.m[1][3] = 0.0;
        matrix.m[2][0] = self.m[0][2];
        matrix.m[2][1] = self.m[1][2];
        matrix.m[2][2] = self.m[2][2];
        matrix.m[2][3] = 0.0;
        matrix.m[3][0] =
            -(self.m[3][0] * matrix.m[0][0] + self.m[3][1] * matrix.m[1][0] + self.m[3][2] * matrix.m[2][0]);
        matrix.m[3][1] =
            -(self.m[3][0] * matrix.m[0][1] + self.m[3][1] * matrix.m[1][1] + self.m[3][2] * matrix.m[2][1]);
        matrix.m[3][2] =
            -(self.m[3][0] * matrix.m[0][2] + self.m[3][1] * matrix.m[1][2] + self.m[3][2] * matrix.m[2][2]);
        matrix.m[3][3] = 1.0;

        matrix
    }
}
