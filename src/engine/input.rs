use std::collections::HashSet;
use winit::{keyboard::KeyCode, event::MouseButton, window::{Window, Fullscreen}};

use crate::engine::{Vec3d, camera::Camera};

pub struct InputState {
    pub keys_pressed: HashSet<KeyCode>,
    pub keys_just_pressed: HashSet<KeyCode>,
    pub mouse_buttons_pressed: HashSet<MouseButton>,
    pub mouse_buttons_just_pressed: HashSet<MouseButton>
}

impl InputState {
    pub fn new() -> Self {
        Self { 
            keys_pressed: HashSet::new(), 
            keys_just_pressed: HashSet::new(), 
            mouse_buttons_pressed: HashSet::new(), 
            mouse_buttons_just_pressed: HashSet::new() 
        }
    }
    pub fn pressed_key(&mut self, keycode:KeyCode) {
        if self.keys_pressed.insert(keycode) {
            self.keys_just_pressed.insert(keycode);
        }
    }
    pub fn released_key(&mut self, keycode:KeyCode) {
        self.keys_pressed.remove(&keycode);
    }
    pub fn pressed_mouse(&mut self, mouse_button:MouseButton) {
        if self.mouse_buttons_pressed.insert(mouse_button) {
            self.mouse_buttons_just_pressed.insert(mouse_button);
        }
    }
    pub fn released_mouse(&mut self, mouse_button:MouseButton) {
        self.mouse_buttons_pressed.remove(&mouse_button);
    }
    pub fn clear_just_pressed(&mut self) {
        self.keys_just_pressed.clear();
        self.mouse_buttons_just_pressed.clear();
    }
    pub fn handle_movement(&self, camera:&mut Camera, move_speed:f32) {

        let right = Vec3d::new(0.0, 1.0, 0.0).cross(camera.look_dir).normalize();

        if self.keys_pressed.contains(&KeyCode::Space) {
            camera.position.y += move_speed;
        }
        if self.keys_pressed.contains(&KeyCode::ShiftLeft) {
            camera.position.y -= move_speed;
        }
        if self.keys_pressed.contains(&KeyCode::KeyW) {
            camera.position.x += camera.look_dir.x * move_speed;
            camera.position.z += camera.look_dir.z * move_speed;
        }
        if self.keys_pressed.contains(&KeyCode::KeyS) {
            camera.position.x -= camera.look_dir.x * move_speed;
            camera.position.z -= camera.look_dir.z * move_speed;
        }
        if self.keys_pressed.contains(&KeyCode::KeyA) {
            camera.position = camera.position - right * move_speed;
        }
        if self.keys_pressed.contains(&KeyCode::KeyD) {
            camera.position = camera.position + right * move_speed;
        }
    }
    pub fn check_fullscreen(&self, window: &Window) {
        if self.keys_just_pressed.contains(&KeyCode::F11) {
            toggle_fullscreen(&window);
        }
    }
}

pub fn toggle_fullscreen(window: &Window) {
    match window.fullscreen() {
        None => window.set_fullscreen(Some(Fullscreen::Borderless(None))),
        Some(_) => window.set_fullscreen(None),
    }
}