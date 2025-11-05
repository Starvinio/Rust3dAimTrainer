use softbuffer::{Context, Surface};
use rand::Rng;
use std::{
    num::NonZeroU32,
    time::{Duration, Instant},
};
use winit::{
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};
use crate::engine::{
    Statistic, Vec3d, camera::Camera, core::{CONFIG, Mat4x4, TriToRaster}, input::InputState, rendering::{draw_crosshair, render_triangles, room_proj_loop, target_proj_loop, tri_clip_xy, window}, scenario::{Gun, Scenario, add_target, create_room, create_target_vec}

};


pub fn run(scenario: &mut Scenario, duration_secs: Duration) {
    let (event_loop, window) = window::event_loop_setup();

    let context = Context::new(&window).unwrap();
    let mut surface = Surface::new(&context, &window).unwrap();

    // Initialize custom structs
    let mut camera = Camera::new(scenario.player_spawn);
    let mut user_input = InputState::new();
    let mut stats = Statistic::new();

    // need to make this save in scenario, no need for it here
    let mut gun = Gun::new(0.2); // also magic num

    // maybe save this in scenario too
    let mut room = create_room(scenario.room_type, scenario.room_rad);

    let (mut target_vec, mut old_target) = create_target_vec(scenario);


    let proj_matrix = Mat4x4::projection();

    
    let test_point = camera.position + camera.look_dir * 10.0;
    let proj = proj_matrix.project_vec(camera.view_matrix * test_point);
    

    let mut rng = rand::thread_rng();


    let mut pixel_buffer: Vec<u32> = vec![0; CONFIG.display.width * CONFIG.display.height];
    let window_size = window.inner_size();

    let _ = event_loop.run(|event, window_target| {
        window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);

        match stats.scenario_starttime.elapsed() {
            Ok(elapsed) if elapsed > duration_secs => {
                window_target.exit();
            }
            _ => {}
        }
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),

                WindowEvent::KeyboardInput {
                    event: key_event, ..
                } => {
                    if let PhysicalKey::Code(keycode) = key_event.physical_key {
                        match key_event.state {
                            ElementState::Pressed => {
                                if keycode == KeyCode::Escape {
                                    window_target.exit();
                                }
                                user_input.pressed_key(keycode);
                            }
                            ElementState::Released => {
                                user_input.released_key(keycode);
                            }
                        }
                    }
                }

                WindowEvent::MouseInput { state, button, .. } => match state {
                    ElementState::Pressed => {
                        user_input.pressed_mouse(button);
                    }
                    ElementState::Released => {
                        user_input.released_mouse(button);
                    }
                },

                WindowEvent::RedrawRequested => {
                    let now = Instant::now();
                    let delta_time = now.duration_since(camera.last_frame_time).as_secs_f32();
                    
                    camera.last_frame_time = now;
                    camera.update_look_dir();


                    // User Keyboard Input
                    let move_speed = CONFIG.input.move_speed * delta_time; 
                    if scenario.allow_movement {user_input.handle_movement(&mut camera, move_speed)};
                    user_input.check_fullscreen(&window);


                    camera.update_view_matrix();


                    let mut tri_vec: Vec<TriToRaster> = Vec::new();
                    let mut target_tri_vec: Vec<TriToRaster> = Vec::new();

                    room_proj_loop(
                        &mut room,
                        &mut tri_vec,
                        &camera,
                        &proj_matrix,
                    );
                /*  
                    Logic required for registering hits in target_proj_loop(), in which we'll pass hit_target.
                    gun_shot should be set to true (depending on scenario config) upon mouse click or hold (only if gun can shoot).
                */  let mut hit_target = false;
                    let mut gun_shot = false;
                    if (!scenario.allow_mouse_hold && user_input.mouse_buttons_just_pressed.contains(&MouseButton::Left)) || 
                    (scenario.allow_mouse_hold && user_input.mouse_buttons_pressed.contains(&MouseButton::Left) && gun.can_shoot()) {
                        gun.shoot();
                        gun_shot = true;
                    }

                /*  
                    This is the main target loop.
                    To avoid redundant looping, we handle both movement, rendering and hit detection in this loop 
                */  for target in &mut target_vec {

                        target.random_movement(camera.position, now, &mut rng, delta_time);

                        let hit = target_proj_loop(target, &mut target_tri_vec, &camera,  gun_shot, &proj_matrix);

                        if hit {
                            hit_target = true;
                            target.hp -= 1;
                            if target.hp < 1 {
                                old_target = Some(target.position);
                            }
                        }
                        
                    }

                /* 
                    Here, we sort the Target'sTriangles from furthest to closest (from camera), so that the triangles that are behind others
                    get drawn first, to avoid translucency.
                */  target_tri_vec.sort_by(|t1, t2| t2.avg_z.partial_cmp(&t1.avg_z).unwrap());

                /* 
                    Room's Triangles are already in this vector, since the room does not have Triangles behind others.
                    Other than that, we always want to render the room first an then the triangles.
                */  tri_vec.append(&mut target_tri_vec);


                    // Clearing the pixels of the last frame
                    pixel_buffer.fill(0);

                /* 
                    We now iterate through the Triangle Vector, which contains the 2d Triangles of both the Targets and the Room.
                    We first clipp the triangles at the screen edges and then use the fill function to draw them onto the screen.
                */  let mut tri_clipped:Vec<TriToRaster> = Vec::with_capacity(4);
                    for tri2d in &mut tri_vec {
                        tri_clipped.clear();
                        tri_clip_xy(tri2d, &mut tri_clipped);
                        render_triangles(&mut pixel_buffer, &tri_clipped);
                    }

                /*
                    Crosshair is drawn based on users settings in config.toml
                */  draw_crosshair(&mut pixel_buffer, CONFIG.crosshair);

                /*
                    Remove Targets with an hp <= 0 and spawn in a new one to keep the target count consistent
                */  target_vec.retain(|x| x.hp > 0);
                    if target_vec.len() < scenario.target_count as usize {
                        add_target(scenario, &mut target_vec, old_target);
                    }
                
                /*
                    Simply updates stats 
                */  if gun_shot && hit_target {
                        stats.add_hit();
                    } else if gun_shot {
                        stats.add_shot();
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
                    
                /* 
                    This should be done every frame to prevent single presses being detected as holding down 
                */  user_input.clear_just_pressed();
                }
                _ => (),
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                let (delta_x, delta_y) = delta;
                camera.update_yaw_pitch(delta_x, delta_y);
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        };
    });

/*  end scenario to be able to display total time played
    print stats at the end 
*/  stats.end_scenario();
    stats.print_stats();
}
