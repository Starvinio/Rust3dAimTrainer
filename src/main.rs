use std::io;
use supports_color::{Stream, ColorLevel};
pub mod engine;
use crate::engine::{dyn_clicking, evasive_switching, load_all_scenarios, precise_tracking, reactive_tracking, speed_switching, static_clicking, EngineError};
use crate::engine::color::Colors;
use crate::engine::print::print_cli_select;

fn main() -> Result<(), EngineError>
{
    let colors = Colors::detect_set_colors();
    engine::rendering::print::print_logo(&colors);

    print_cli_select(0, "List all Scenarios", &colors);

    println!("{}AIMING TYPE CATEGORIES:{}", colors.blue, colors.reset);
    println!("[{}1{}] Static Clicking", colors.blue, colors.reset);
    println!("[{}2{}] Dynamic Clicking", colors.blue, colors.reset);
    println!("[{}3{}] Reactive Tracking", colors.blue, colors.reset);
    println!("[{}4{}] Precise Tracking", colors.blue, colors.reset);
    println!("[{}5{}] Speed Switching", colors.blue, colors.reset);
    println!("[{}6{}] Evasive Switching", colors.blue, colors.reset);
    println!("\nPlease select a Listing Type by entering a number between '{}0{}' and '{}6{}':", colors.blue, colors.reset, colors.blue, colors.reset);

    let list_type = 'listType: loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if num < 7 {break num;}
                else {
                    println!("Please Enter a valid number between '0' and '6'");
                    continue 'listType;
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue 'listType;
            }
        };
    };

    let mut scenarios = match list_type {

        1 => {println!("\n{}STATIC CLICKING{}", colors.blue, colors.reset); static_clicking()},
        2 => {println!("\n{}DYNAMIC CLICKING{}", colors.blue, colors.reset); dyn_clicking()},
        3 => {println!("\n{}REACTIVE TRACKING{}", colors.blue, colors.reset); reactive_tracking()},
        4 => {println!("\n{}PRECISE CLICKING{}", colors.blue, colors.reset); precise_tracking()},
        5 => {println!("\n{}SPEED SWITCHING{}", colors.blue, colors.reset); speed_switching()},
        6 => {println!("\n{}STATIC CLICKING{}", colors.blue, colors.reset); evasive_switching()},

        0 | _ => {println!("\n{}ALL SCENARIOS{}", colors.blue, colors.reset); load_all_scenarios()},
    };


    for (i,scenario) in scenarios.iter().enumerate() {
        println!("[{}{}{}] {}", colors.blue, i, colors.reset, scenario.name);
    }
    println!("\nPlease select a {}scenario{} by entering a number between '0' and '{}':", colors.blue, colors.reset,  scenarios.len()-1);

    let scenario_index = 'scenario: loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if num < scenarios.len() {break num;}
                else { 
                    println!("Please Enter a valid number between '0' and '{}'", scenarios.len()-1);
                    continue 'scenario;
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue 'scenario;
            }
        };
    };

    engine::runtime::run(&mut scenarios[scenario_index])?;
    Ok(())
}
