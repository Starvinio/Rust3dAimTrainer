use softbuffer::{Context, Surface};
use std::{
    num::NonZeroU32,
    time::{Instant},
    io::BufReader,
    fs::File
};
use winit::{
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Fullscreen,
};
use rodio::{Decoder, Source};

use crate::engine::{Statistic, draw_texture_optimized, GUI, GUI_TXT_PATH, EngineError, camera::Camera, core::{CONFIG, HIT_TARGET, Mat4x4, TriToRaster}, input::InputState, rendering::{draw_crosshair, render_triangles, room_proj_loop, target_proj_loop, tri_clip_xy, window}, scenario::{Scenario}, cli::play_again, TargetVec, draw_fps, Timer};
use crate::engine::camera::FPS;

pub fn run(scenario: &mut Scenario) -> Result<(), EngineError>{
    let (event_loop, window) = window::event_loop_setup()?;
    let mut window_size = window.inner_size();

    // Initialize softbuffer
    let context = Context::new(&window)?;
    let mut surface = Surface::new(&context, &window)?;

    // Initialize custom structs
    let mut camera = Camera::new(scenario.player_spawn);
    let mut user_input = InputState::new();
    let mut stats = Statistic::new();
    let mut fps = FPS::init();
    let mut timer = Timer::new();

    let gui = match GUI::load_gui(GUI_TXT_PATH.to_str().unwrap()) {
        Ok(g) => {
            Some(g)
        }
        Err(e) => {
            eprintln!("Warning: GUI failed to load: {}", e);
            None
        }
    };

    // Initialize target vector
    let mut target_vec = TargetVec::init(&scenario.t_settings);

    // Pre-allocate Vectors
    let mut tri_vec: Vec<TriToRaster> = Vec::with_capacity(1024);
    let mut target_tri_vec: Vec<TriToRaster> = Vec::with_capacity(256);
    let mut tri_clipped: Vec<TriToRaster> = Vec::with_capacity(4);

    let mut proj_matrix = Mat4x4::projection(window_size.width as f32, window_size.height as f32);

    let mut rng = rand::thread_rng();

    // SFX setup
    let mut stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    rodio::OutputStream::log_on_drop(&mut stream_handle, false);
    let file_hit_target = File::open(&*HIT_TARGET).map_err(|_| EngineError::SFXErr)?;
    let src_hit_target = Decoder::new(BufReader::new(file_hit_target)).unwrap().buffered();
    

    let mut pixel_buffer: Vec<u32> = vec![0; window_size.width as usize * window_size.height as usize];

    let _ = event_loop.run(|event, window_target| {
        window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);

        match stats.scenario_starttime.elapsed() {
            Ok(elapsed) => {
                if elapsed > scenario.duration_secs && !stats.scenario_ended {
                    // Hide window and prompt for replay
                    window.set_fullscreen(None);
                    window.set_minimized(true);
                    stats.end_scenario();
                    stats.print_stats(&scenario.name, fps.total_frame_count / stats.scenario_playtime());

                    if play_again() {
                        stats = Statistic::new();
                        camera = Camera::new(scenario.player_spawn);
                        target_vec = TargetVec::init(&scenario.t_settings);
                        fps.total_frame_count = 0;
                        window.set_minimized(false);
                        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                        window.focus_window();
                    } else {
                        window_target.exit();
                        return;
                    }
                } else if !stats.scenario_ended {
                    // This is used for timer display
                    timer.seconds = (scenario.duration_secs - elapsed).as_secs() + 1;
                    timer.update_mins();
                }
                
            }
            _ => {}
        }
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),

                WindowEvent::Resized(new_size) => {
                    window_size = new_size;
                    proj_matrix = Mat4x4::projection(window_size.width as f32, window_size.height as f32);
                    pixel_buffer.resize(window_size.width as usize * window_size.height as usize, 0);
                    surface
                        .resize(
                            NonZeroU32::new(window_size.width).unwrap(),
                            NonZeroU32::new(window_size.height).unwrap(),
                        )
                        .unwrap();
                }

                WindowEvent::KeyboardInput {
                    event: key_event, ..
                } => {
                    if let PhysicalKey::Code(keycode) = key_event.physical_key {
                        match key_event.state {
                            ElementState::Pressed => {
                                if keycode == KeyCode::Escape {
                                    window.set_fullscreen(None);
                                    window.set_minimized(true);
                                    /*
                                    end scenario to be able to display total time played
                                    print stats at the end
                                */  stats.end_scenario();
                                    stats.print_stats(&scenario.name,
                                                      if stats.scenario_playtime() < 1 {0} else {fps.total_frame_count / stats.scenario_playtime()}
                                    );

                                    if play_again() {
                                        stats = Statistic::new();
                                        camera = Camera::new(scenario.player_spawn);
                                        target_vec = TargetVec::init(&scenario.t_settings);
                                        fps.total_frame_count = 00;
                                        window.set_minimized(false);
                                        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                                        window.focus_window();
                                    } else {
                                        window_target.exit();
                                        return;
                                    }
                                }
                                if keycode == KeyCode::KeyR {
                                    stats = Statistic::new();
                                    camera = Camera::new(scenario.player_spawn);
                                    target_vec = TargetVec::init(&scenario.t_settings);
                                    fps.total_frame_count = 0;

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
                    fps.total_frame_count+=1; fps.interval_frame_count+=1;
                    let now = Instant::now();
                    let delta_time = now.duration_since(camera.last_frame_time).as_secs_f32();
                    
                    camera.last_frame_time = now;
                    camera.update_look_dir();


                    // User Keyboard Input
                    let move_speed = CONFIG.input.move_speed * delta_time; 
                    if scenario.allow_movement {user_input.handle_movement(&mut camera, move_speed)};
                    user_input.check_fullscreen(&window);


                    camera.update_view_matrix();


                    tri_vec.clear();
                    target_tri_vec.clear();

                    room_proj_loop(
                        &mut scenario.room,
                        &mut tri_vec,
                        &camera,
                        &proj_matrix,
                    );
                /*  
                    Logic required for registering hits in target_proj_loop(), in which we'll pass hit_target.
                    gun_shot should be set to true (depending on scenario config) upon mouse click or hold (only if gun can shoot).
                */  let mut hit_target = false;
                    let mut gun_shot = false;
                    if (!scenario.gun.automatic && user_input.mouse_buttons_just_pressed.contains(&MouseButton::Left)) || 
                    (scenario.gun.automatic && user_input.mouse_buttons_pressed.contains(&MouseButton::Left) && scenario.gun.can_shoot()) {
                        scenario.gun.shoot();
                        gun_shot = true;
                    }

                /*  
                    This is the main target loop.
                    To avoid redundant looping, we handle both movement, rendering and hit detection in this loop
                */  for target in &mut target_vec.vec {
                        target.random_movement(camera.position, &mut rng, delta_time);
                        let hit = target_proj_loop(target, &mut target_tri_vec, &camera,  gun_shot, &proj_matrix);
                        if hit {
                            stream_handle.mixer().add(src_hit_target.clone());
                            hit_target = true;
                            target.hp -= 1;
                            if target.hp < 1 {
                                target_vec.old = Some(target.position);
                            }
                        }
                        
                    }

                /* 
                    Here, we sort the Target'sTriangles from furthest to closest (from camera), so that the triangles that are behind others
                    get drawn first, to avoid translucency.
                */  target_tri_vec.sort_unstable_by(|t1, t2| {
                        t2.avg_z.partial_cmp(&t1.avg_z).unwrap_or(std::cmp::Ordering::Equal)
                    });

                /* 
                    Room's Triangles are already in this vector, since the room does not have Triangles behind others.
                    Other than that, we always want to render the room first and then the triangles.
                */  tri_vec.append(&mut target_tri_vec);


                    // Clearing the pixels of the last frame
                    pixel_buffer.fill(0);

                    let width = window_size.width as usize;
                    let height = window_size.height as usize;
                /* 
                    We now iterate through the Triangle Vector, which contains the 2d Triangles of both the Targets and the Room.
                    We first clipp the triangles at the screen edges and then use the fill function to draw them onto the screen.
                */
                    for tri2d in &mut tri_vec {
                        tri_clipped.clear();
                        tri_clip_xy(tri2d, &mut tri_clipped);
                        render_triangles(&mut pixel_buffer, &tri_clipped, width, height);
                    }
                    
                    // Crosshair is drawn based on users settings in config.toml
                    draw_crosshair(&mut pixel_buffer, CONFIG.crosshair, width, height);

                    if let Some(gui) = &gui {
                        draw_texture_optimized(&mut pixel_buffer, width, height, &gui.logo, 0, 0);

                        timer.draw_timer(&mut pixel_buffer, width, height, &gui);

                        fps.update_str(now);
                        draw_fps(&fps, &mut pixel_buffer, width, height, &gui.digits_fps);
                    }
                /*
                    Remove Targets with hp <= 0 and spawn in a new one to keep the target count consistent
                */  target_vec.vec.retain(|x| x.hp > 0);
                    if target_vec.vec.len() < scenario.t_settings.count {
                        target_vec.add_target();
                    }
                
                /*
                    Simply updates stats 
                */  if gun_shot && hit_target {
                        stats.add_hit();
                    } else if gun_shot {
                        stats.add_shot();
                    }

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


    Ok(())
}
