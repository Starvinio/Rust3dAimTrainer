use crate::engine::structures::{Mesh, Triangle, Vec3d};

pub fn create_tri(a: (f32, f32, f32), b: (f32, f32, f32), c: (f32, f32, f32)) -> Triangle {
    Triangle {
        p: [
            Vec3d::new(a.0, a.1, a.2),
            Vec3d::new(b.0, b.1, b.2),
            Vec3d::new(c.0, c.1, c.2),
        ],
    }
}

pub fn create_room(size: f32) -> Mesh {
    Mesh {
        tris: vec![
            // SOUTH (z = -size)
            create_tri(
                (-size, -size, -size),
                (size, size, -size),
                (-size, size, -size),
            ),
            create_tri(
                (-size, -size, -size),
                (size, -size, -size),
                (size, size, -size),
            ),
            // NORTH (z = +size)
            create_tri((size, -size, size), (-size, size, size), (size, size, size)),
            create_tri(
                (size, -size, size),
                (-size, -size, size),
                (-size, size, size),
            ),
            // EAST (x = +size)
            create_tri(
                (size, -size, -size),
                (size, size, size),
                (size, size, -size),
            ),
            create_tri(
                (size, -size, -size),
                (size, -size, size),
                (size, size, size),
            ),
            // WEST (x = -size)
            create_tri(
                (-size, -size, size),
                (-size, size, -size),
                (-size, size, size),
            ),
            create_tri(
                (-size, -size, size),
                (-size, -size, -size),
                (-size, size, -size),
            ),
            // TOP (y = +size)
            create_tri(
                (-size, size, -size),
                (size, size, size),
                (-size, size, size),
            ),
            create_tri(
                (-size, size, -size),
                (size, size, -size),
                (size, size, size),
            ),
            // BOTTOM (y = -size)
            create_tri(
                (size, -size, size),
                (-size, -size, -size),
                (-size, -size, size),
            ),
            create_tri(
                (size, -size, size),
                (size, -size, -size),
                (-size, -size, -size),
            ),
        ],
    }
}
