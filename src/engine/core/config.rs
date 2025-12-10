use serde::Deserialize;
use std::fs;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::env;

use crate::engine::Crosshair;

fn asset_path(relative_path: &str) -> PathBuf {
    let base = if cfg!(debug_assertions) {
        // In debug builds, assets are relative to the project root.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    } else {
        // In release builds, assets are relative to the executable.
        let mut path = env::current_exe().expect("Failed to get current executable path.");
        path.pop(); // Get directory of executable
        path
    };
    base.join(relative_path)
}

const HIT_TARGET_REL: &str = if cfg!(debug_assertions) { "src/assets/sounds/hit_target.wav" } else { "assets/sounds/hit_target.wav" };
const POP_REL: &str = if cfg!(debug_assertions) { "src/assets/sounds/pop_sound.mp3" } else { "assets/sounds/pop_sound.mp3" };
const CONFIG_PATH_REL: &str = if cfg!(debug_assertions) { "src/config.toml" } else { "config.toml" };
const GUI_TXT_PATH_REL: &str = if cfg!(debug_assertions) { "src/assets/gui/gui.png" } else { "assets/gui/gui.png" };

pub static HIT_TARGET: Lazy<PathBuf> = Lazy::new(|| asset_path(HIT_TARGET_REL));
pub static POP: Lazy<PathBuf> = Lazy::new(|| asset_path(POP_REL));
pub static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| asset_path(CONFIG_PATH_REL));
pub static GUI_TXT_PATH: Lazy<PathBuf> = Lazy::new(|| asset_path(GUI_TXT_PATH_REL));


// Lazily load config at runtime, only once
pub static CONFIG: Lazy<Config> = Lazy::new(|| load_config(&CONFIG_PATH));

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

pub fn load_config(path: &Path) -> Config {
    let config_str = fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read config file at {:?}: {}", path, e));

    toml::from_str::<Config>(&config_str).expect("Failed to parse config.toml")
}