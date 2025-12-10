use std::io;
pub mod engine;
use std::env;
use crate::engine::core::{BLUE, RESET};
use crate::engine::{load_scenarios};
fn main() -> Result<(),engine::core::error::SetupError>
{

    engine::rendering::draw_logo();

    env::set_var("RUST_BACKTRACE", "1");

    let mut scenarios = load_scenarios();

    println!("Welcome to the Rust 3D Aim Trainer!\n");

    for (i,scenario) in scenarios.iter().enumerate() {
        println!("[{}{}{}] {}", BLUE, i, RESET, scenario.name);
    }
    println!("Please select a {}scenario{} by entering a number:", BLUE, RESET);

    let scenario_index = 'scenario: loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue 'scenario;
            }
        };
    };

    engine::runtime::run(&mut scenarios[scenario_index])
}
