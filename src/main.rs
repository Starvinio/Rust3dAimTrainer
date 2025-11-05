#![warn(unused_extern_crates)]

use std::{io,time::Duration};

pub mod engine;

use crate::engine::core::{BLUE, RESET};
use crate::engine::scenario::map_scenario;

fn main() {
    engine::rendering::draw_logo();

    println!("Welcome to the Rust 3D Aim Trainer Demo Version!\n");

    println!(
        "Each run will last 30 seconds\nPress {}ESC{} to exit\n",
        BLUE, RESET
    );
    println!("Please select a {}scenario{}:", BLUE, RESET);
    println!("[{}1{}] Jumbo Tile Frenzy", BLUE, RESET);
    println!("[{}2{}] Jumbo Tile Frenzy Flat", BLUE, RESET);
    println!("[{}3{}] 1 Wall 6 Targets TE", BLUE, RESET);
    println!("[{}4{}] 1 Wall 6 Targets Small", BLUE, RESET);
    println!("[{}5{}] 1 Wall 6 Targets Extra Small", BLUE, RESET);
    println!("[{}6{}] 1 Wall 5 Targets Pasu", BLUE, RESET);
    println!("[{}7{}] 1 Wall 1 Target Spheretrack", BLUE, RESET);

    let mut scenario = 'scenario: loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i8>() {
            Ok(num) => break map_scenario(num),
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue 'scenario;
            }
        };
    };

    engine::runtime::run(&mut scenario, Duration::from_secs(30));
}
