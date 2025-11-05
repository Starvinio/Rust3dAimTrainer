use crate::engine::{Vec3d, Triangle};

pub struct Mesh {
    // Groups together any number of triangles to a mesh
    pub tris: Vec<Triangle>,
}
impl Mesh {
    pub fn translate(&mut self, vec:Vec3d) {
        for triangle in &mut self.tris {
            triangle.translate(vec);
        }
    }
}