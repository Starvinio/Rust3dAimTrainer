use crate::engine::{helpers::load_config, structures::Config};
use once_cell::sync::Lazy;

#[cfg(debug_assertions)]
pub static CONFIG_PATH: &str = "src/config.toml";

#[cfg(not(debug_assertions))]
pub static CONFIG_PATH: &str = "config.toml";

// Lazily load config at runtime, only once
pub static CONFIG: Lazy<Config> = Lazy::new(|| load_config(CONFIG_PATH));

// Compute aspect ratio from config
pub fn aspect_ratio() -> f32 {
    let display = &CONFIG.display;
    display.height as f32 / display.width as f32
}

// CLI styling constants
pub const BLUE: &str = "\x1b[94m";
pub const RESET: &str = "\x1b[0m";
