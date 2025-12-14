use crate::engine::{BLUE, RESET};
use crate::engine::color::Colors;

pub fn print_logo(colors: &Colors) {
    let logo = r#"
  _____           _              _             _______        _
 |  __ \         | |       /\   (_)           |__   __|      (_)
 | |__) |   _ ___| |_     /  \   _ _ __ ___      | |_ __ __ _ _ _ __   ___ _ __
 |  _  / | | / __| __|   / /\ \ | | '_ ` _ \     | | '__/ _` | | '_ \ / _ \ '__|
 | | \ \ |_| \__ \ |_   / ____ \| | | | | | |    | | | (_| | | | | |  __/ |
 |_|  \_\__,_|___/\__| /_/    \_\_|_| |_| |_|    |_|_|  \__,_|_|_| |_|\___|_|


Welcome to the Rust 3D Aim Trainer!

"#;
    println!("{}{}{}", colors.blue, logo, colors.reset);
}

pub fn print_cli_select(n:usize, message:&str, colors: &Colors) {
    println!("[{}{}{}] {}", colors.blue, n, colors.reset, message);
}