use softbuffer::{Context, Surface};
use std::{
    collections::HashSet,
    f32::consts::{FRAC_PI_2, PI},
    num::NonZeroU32,
    time::{Duration, Instant, SystemTime},
};
use winit::{
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub mod consts;
pub mod helpers;
pub mod renderer;
pub mod scenarios;
pub mod shapes;
pub mod structures;
pub mod targets;
pub mod window;

use consts::*;
use helpers::*;
use renderer::*;
use shapes::*;
use structures::*;

use targets::{add_target, create_target_vec};

use crate::engine::scenarios::{Scenario, UserScore};

pub fn run(scenario: Scenario, duration_secs: Duration) {
    let (event_loop, window) = window::event_loop_setup();

    let context = Context::new(&window).unwrap();
    let mut surface = Surface::new(&context, &window).unwrap();

    let mut last_frame_time = Instant::now();
    let mut keys_pressed: HashSet<KeyCode> = HashSet::new(); // keyboard keys pressed during the current single frame
    let mut keys_just_pressed: HashSet<KeyCode> = HashSet::new(); // Only new key presses
    let mut mouse_buttons_pressed: HashSet<MouseButton> = HashSet::new(); // mouse buttons pressed during current single frame
    let mut mouse_buttons_just_pressed: HashSet<MouseButton> = HashSet::new(); // Only new mouse button presses

    let mut hits = 0;
    let mut shots = 0;

    //Inward facing cube as room
    let mut room = create_room(scenario.room_size);
    for tri in &mut room.tris {
        for i in 0..3 {
            tri.p[i].z += scenario.room_dist;
        }
    }

    //3 Sphere Targets
    let (mut target_vec, mut old_target) = create_target_vec(
        &scenario.target_type,
        scenario.target_count,
        scenario.spawn_d,
        scenario.target_dist,
        scenario.target_hp,
    );

    let f_fov_rad: f32 = get_fov_rad(CONFIG.camera.fov);

    // v_camera + r * v_look_dir = straight line of sight
    let mut v_camera = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; // camera location vector
    let mut v_look_dir = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    }; // camera direction vector

    let v_light_direction = Vec3d::new(-0.5, -0.5, 1.5);

    let mut f_yaw: f32 = 0.0; // horizontal rotation
    let mut f_pitch: f32 = 0.0; // vertical rotation

    let mat_proj = create_proj_mat(f_fov_rad);

    let mut pixel_buffer: Vec<u32> = vec![0; CONFIG.display.width * CONFIG.display.height]; // Vec of size WIDTH * HEIGHT
    let window_size = window.inner_size(); // Just stores WIDTH & HEIGHT

    let start = SystemTime::now();

    let _ = event_loop.run(|event, window_target| {
        window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);

        match start.elapsed() {
            Ok(elapsed) if elapsed > duration_secs => {
                window_target.exit();
            }
            _ => {}
        }
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(), // closing via X

                WindowEvent::KeyboardInput {
                    event: key_event, ..
                } => {
                    if let PhysicalKey::Code(keycode) = key_event.physical_key {
                        match key_event.state {
                            ElementState::Pressed => {
                                if keys_pressed.insert(keycode) {
                                    keys_just_pressed.insert(keycode);
                                } // Adds key to hash if key is being pressed
                                if keycode == KeyCode::Escape {
                                    window_target.exit();
                                    window.set_minimized(true);
                                }
                            }
                            ElementState::Released => {
                                keys_pressed.remove(&keycode); // Removes key from hash if key is being released
                            }
                        }
                    }
                }

                WindowEvent::MouseInput { state, button, .. } => match state {
                    ElementState::Pressed => {
                        if mouse_buttons_pressed.insert(button) {
                            mouse_buttons_just_pressed.insert(button);
                        }
                    }
                    ElementState::Released => {
                        mouse_buttons_pressed.remove(&button);
                    }
                },

                WindowEvent::RedrawRequested => {
                    let now = Instant::now();
                    let delta_time = now.duration_since(last_frame_time).as_secs_f32();
                    last_frame_time = now;

                    let v_up = Vec3d {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    };
                    let move_speed = 20.0 * delta_time;

                    f_pitch = f_pitch.clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01); // Clamping to prevent camera flipping
                    f_yaw %= 2.0 * PI;
                    v_look_dir.x = f_pitch.cos() * f_yaw.sin();
                    v_look_dir.y = f_pitch.sin();
                    v_look_dir.z = f_pitch.cos() * f_yaw.cos();
                    v_look_dir = v_look_dir.normalize();

                    let v_right = cross_product(&v_up, &v_look_dir).normalize();

                    if scenario.allow_movement {
                        if keys_pressed.contains(&KeyCode::Space) {
                            v_camera.y += move_speed;
                        }
                        if keys_pressed.contains(&KeyCode::ShiftLeft) {
                            v_camera.y -= move_speed;
                        }
                        if keys_pressed.contains(&KeyCode::KeyW) {
                            v_camera = vec_add(&v_camera, &vec_multiply(&v_look_dir, move_speed));
                        }
                        if keys_pressed.contains(&KeyCode::KeyS) {
                            v_camera =
                                vec_subtract(&v_camera, &vec_multiply(&v_look_dir, move_speed));
                        }
                        if keys_pressed.contains(&KeyCode::KeyA) {
                            v_camera = vec_subtract(&v_camera, &vec_multiply(&v_right, move_speed));
                        }
                        if keys_pressed.contains(&KeyCode::KeyD) {
                            v_camera = vec_add(&v_camera, &vec_multiply(&v_right, move_speed));
                        }
                    }

                    if keys_just_pressed.contains(&KeyCode::F11) {
                        toggle_fullscreen(&window);
                    }

                    let v_target = vec_add(&v_camera, &v_look_dir);
                    let mat_camera: Mat4x4 = matrix_point_at(&v_camera, &v_target, &v_up);
                    let mat_view = matrix_quickinverse(mat_camera);
                    let mut tri_vec: Vec<TriToRaster> = Vec::new();
                    let mut target_tri_vec: Vec<TriToRaster> = Vec::new();

                    create_room_proj(
                        &mut room,
                        &mut tri_vec,
                        &v_camera,
                        &v_light_direction,
                        &mat_view,
                        &mat_proj,
                    );

                    for target in &mut target_vec {
                        let mb_pressed: &HashSet<MouseButton>;
                        if scenario.allow_mouse_hold {
                            mb_pressed = &mouse_buttons_pressed;
                        } else {
                            mb_pressed = &mouse_buttons_just_pressed;
                        }
                        create_target_proj(
                            target,
                            &mut target_tri_vec,
                            &mut old_target,
                            &v_camera,
                            &v_light_direction,
                            &v_look_dir,
                            mb_pressed,
                            &mat_view,
                            &mat_proj,
                            &mut hits,
                        );
                    }
                    target_tri_vec.sort_by(|t1, t2| t2.avg_z.partial_cmp(&t1.avg_z).unwrap());
                    tri_vec.append(&mut target_tri_vec);

                    pixel_buffer.fill(0); // clearing the screen

                    renderer::tri_clip_and_render(&mut tri_vec, &mut pixel_buffer);
                    renderer::render_crosshair(&mut pixel_buffer, 0xFF0000FF, 5, 3, -1);

                    target_vec.retain(|x| x.hp > 0); // kill targets with 0 hp
                    if target_vec.len() < scenario.target_count as usize {
                        // spawn new target
                        add_target(
                            &scenario.target_type,
                            scenario.spawn_d,
                            &mut target_vec,
                            scenario.target_dist,
                            scenario.target_hp,
                            &old_target,
                        );
                    }

                    if mouse_buttons_just_pressed.contains(&MouseButton::Left) {
                        shots += 1;
                    }

                    surface
                        .resize(
                            NonZeroU32::new(window_size.width).unwrap(),
                            NonZeroU32::new(window_size.height).unwrap(),
                        )
                        .unwrap();
                    let mut buffer = surface.buffer_mut().unwrap();
                    buffer.copy_from_slice(&pixel_buffer);
                    buffer.present().unwrap();
                    keys_just_pressed.clear();
                    mouse_buttons_just_pressed.clear();
                }
                _ => (),
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                let (delta_x, delta_y) = delta;
                let sens = CONFIG.input.sensitivity;
                f_yaw += delta_x as f32 * sens * 0.001;
                f_pitch -= delta_y as f32 * sens * 0.001;
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        };
    });

    let time_played = SystemTime::now().duration_since(start).unwrap().as_secs() as i32;

    let score = UserScore {
        time: time_played,
        hits: hits,
        shots: shots,
        accuracy: { hits as f32 / shots as f32 * 100.0 },
    };

    pub const BLUE: &str = "\x1b[94m";
    pub const RESET: &str = "\x1b[0m";
    println!("\n{}----- RESULTS -----{}", BLUE, RESET);
    println!("Duration: {} seconds", score.time);
    println!("Hits: {}", score.hits);
    println!("Shots: {}", score.shots);
    println!("Accuracy: {:.2}%", score.accuracy);
    println!("{}----- END RESULTS -----{}\n", BLUE, RESET);
}
