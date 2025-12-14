use crate::engine::{
    camera::Camera, core::{CONFIG, Mat4x4, Mesh, TriToRaster, Triangle2d, Vec3d, }, rendering::{ray_intersects_triangle, sigmoid, tri_clip_z}, scenario::Target
};


pub fn target_proj_loop(target: &mut Target, tri_vec: &mut Vec<TriToRaster>, camera: &Camera, gun_shot:bool, proj_matrix: &Mat4x4) -> bool {
/*  
    This function performs both rendering (transformation, shading, projection, clipping),
    as well as hit detection on all triangles of a target.
    It returns True only if the player shot their gun and actually hit the target. 
*/
    let mut hit = false;

    //It utilizes the following procedure on each loop iteration:
    for tri in &mut target.tris {

    /*
        1. transform the triangles' position (in case of movement) 
        [we do this because it's faster than separately looping through each triangle again]
    */  let tri_world = *tri + target.position;
        

    /* 
        2. check if the triangle is facing away from the camera, in which case we can skip it entirely.
            [if the angle between normal of the triangle and vector camera to triangle is greater than 90 Degrees]
    */  let normal = tri_world.normal();
        if normal.dot(camera.position.vec_to(&tri_world.p[0])) > 0.0 {
            continue;
        }

    /*
        3. Apply colorized shading utilizing the normal of the triangle
    */  let mut color = color_triangle(normal, CONFIG.targets.color);


    /* 
        4. Convert Triangles' World Space to View Space by multiplying with camera's view matrix.
        [view matrix = inverse of the camera’s position and orientation in world space]
    */  let mut tri_pre_clipping = camera.view_matrix * tri_world;

    /* 
        5. If a triangle passes behind the camera, it has to be clipped into smaller triangles that fit the screen, in order to
        prevent lagging and crashes. 
    */  let (clipped, n_clipped) = tri_clip_z(&mut tri_pre_clipping);


    /*
        6. Converts Triangles from camera space to Projection (Screen) space, meaning we can omit z completely and 
        turn the Vec3d's into Vec2d's (to safe some performance)
    */  let mut tri_projected = Triangle2d::new_origin();
        for n in 0..n_clipped {
            for i in 0..3 {
                tri_projected.p[i] = proj_matrix.project_vec(clipped[n].p[i]);
                tri_projected.p[i].x = (1.0 - tri_projected.p[i].x) * 0.5 * CONFIG.display.width as f32;
                tri_projected.p[i].y = (1.0 - tri_projected.p[i].y) * 0.5 * CONFIG.display.height as f32;

            }
            let avg_z = tri.avg_z(); // Needed for sorting (painters algorithm)
            tri_vec.push(TriToRaster {
                tri: tri_projected,
                color,
                avg_z,
            });
            
        }
    /*
        7. If the player shot the gun while hovering over any of the targets' triangles with their crosshair, they hit the target,
        which is why we then return true.
    */  if gun_shot && !hit && ray_intersects_triangle(&camera, &tri_world) {
            hit = true;
        }
        
    }
    hit
}

pub fn room_proj_loop(room: &mut Mesh, tri_vec: &mut Vec<TriToRaster>, camera: &Camera, proj_matrix: &Mat4x4) {
/*
    This function handles rendering preperation for the room's triangles using the following steps:
*/
    for tri in &mut room.tris {
    /* 
        1. check if the triangle is facing away from the camera, in which case we can skip it entirely.
            [if the angle between normal of the triangle and vector camera to triangle is greater than 90 Degrees]
    */
        let normal = tri.normal();
        if normal.dot(camera.position.vec_to(&tri.p[0])) > 0.0 {
            continue;
        }
    
    /*
        2. Apply colorized shading utilizing the normal of the triangle
    */
        let color = color_triangle(normal, CONFIG.environment.scene_color);
    
    /* 
        3. Convert Triangles' World Space to View Space by multiplying with camera's view matrix.
        [view matrix = inverse of the camera’s position and orientation in world space]
    */
        let mut tri_pre_clipping = camera.view_matrix * *tri;
    
    /* 
        4. If a triangle passes behind the camera, it has to be clipped into smaller triangles that fit the screen, in order to
        prevent lagging and crashes. 
    */
        let (clipped, n_clipped) = tri_clip_z(&mut tri_pre_clipping);

    /*
        5. Converts Triangles from camera space to Projection (Screen) space, meaning we can omit z completely and 
        turn the Vec3d's into Vec2d's (to safe some performance)
    */
        let mut tri_projected = Triangle2d::new_origin();
        for n in 0..n_clipped {
            for i in 0..3 {
                tri_projected.p[i] = proj_matrix.project_vec(clipped[n].p[i]);
                tri_projected.p[i].x = (1.0 - tri_projected.p[i].x) * 0.5 * CONFIG.display.width as f32;
                tri_projected.p[i].y = (1.0 - tri_projected.p[i].y) * 0.5 * CONFIG.display.height as f32;
            }
            let avg_z = tri.avg_z(); // Needed for sorting (painters algorithm)
            tri_vec.push(TriToRaster {
                tri: tri_projected,
                color,
                avg_z,
            });
        }
        
    }
}


pub fn color_triangle(normal:Vec3d, color:[u8;3]) -> u32 {
    let mut brightness = normal.dot(Vec3d::from_tuple(CONFIG.environment.light_direction).normalize());
    brightness = (sigmoid(brightness) + 0.3).clamp(0.0, 1.0);
    brightness = brightness.powf(1.2);

    let r = (color[0] as f32 * brightness) as u32;
    let g = (color[1] as f32 * brightness) as u32;
    let b = (color[2] as f32 * brightness) as u32;

    (0xFF << 24) | (r << 16) | (g << 8) | b
}

