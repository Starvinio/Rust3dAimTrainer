use serde::Deserialize;
use std::fs;
use once_cell::sync::Lazy;

use crate::engine::Crosshair;

#[cfg(debug_assertions)]
pub static HIT_TARGET: &str = "/src/engine/assets/sounds/hit_target.wav";

#[cfg(not(debug_assertions))]
pub static HIT_TARGET: &str = "hit_target.wav";

#[cfg(debug_assertions)]
pub static CONFIG_PATH: &str = "src/dev_config.toml";

#[cfg(not(debug_assertions))]
pub static CONFIG_PATH: &str = "config.toml";

// Lazily load config at runtime, only once
pub static CONFIG: Lazy<Config> = Lazy::new(|| load_config(CONFIG_PATH));

#[derive(Debug, Deserialize)]
pub struct Config {
    pub display: Display,
    pub camera: CameraSettings,
    pub input: Input,
    pub targets: Targets,
    pub environment: Environment,
    pub crosshair:Crosshair
}

#[derive(Debug, Deserialize)]
pub struct Display {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Deserialize)]
pub struct CameraSettings {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub sensitivity: f32,
    pub move_speed:f32,
}

#[derive(Debug, Deserialize)]
pub struct Targets {
    pub color: [u8; 3],
    pub sphere_detail: usize,
}

#[derive(Debug, Deserialize)]
pub struct Environment {
    pub scene_color: [u8;3],
    pub light_direction: (f32, f32, f32)
}

// CLI styling constants
pub const BLUE: &str = "\x1b[94m";
pub const RESET: &str = "\x1b[0m";

pub fn load_config(path: &str) -> Config {
    let config_str = fs::read_to_string(path).expect("Failed to read config.toml");

    toml::from_str::<Config>(&config_str).expect("Failed to parse config.toml")
}
