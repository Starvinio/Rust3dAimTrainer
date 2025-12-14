use std::io;
use std::io::Write;
use crate::engine::color::Colors;

pub fn print_logo(colors: &Colors) {
    let logo = r#"
  _____           _              _             _______        _
 |  __ \         | |       /\   (_)           |__   __|      (_)
 | |__) |   _ ___| |_     /  \   _ _ __ ___      | |_ __ __ _ _ _ __   ___ _ __
 |  _  / | | / __| __|   / /\ \ | | '_ ` _ \     | | '__/ _` | | '_ \ / _ \ '__|
 | | \ \ |_| \__ \ |_   / ____ \| | | | | | |    | | | (_| | | | | |  __/ |
 |_|  \_\__,_|___/\__| /_/    \_\_|_| |_| |_|    |_|_|  \__,_|_|_| |_|\___|_|


"#;
    println!("{}{}{}Welcome to the Rust 3D Aim Trainer!{}", colors.red, logo, colors.blue, colors.reset);
}

pub fn print_cli_select(n:usize, message:&str, colors: &Colors) {
    println!("[{}{}{}] {}", colors.blue, n, colors.reset, message);
}

pub fn print_categories(colors: &Colors) {
    print_cli_select(0, "List all Scenarios", &colors);
    println!("{}AIMING TYPE CATEGORIES:{}", colors.blue, colors.reset);
    print_cli_select(1, "Static Clicking", &colors);
    print_cli_select(2, "Dynamic Clicking", &colors);
    print_cli_select(3, "Reactive Tracking", &colors);
    print_cli_select(4, "Precise Tracking", &colors);
    print_cli_select(5, "Speed Switching", &colors);
    print_cli_select(6, "Evasive Switching", &colors);
    print!("\nSelect category by entering a number between '{}0{}' and '{}6{}': ",
             colors.blue, colors.reset, colors.blue, colors.reset
    );
    match io::stdout().flush() {
        Err(_) => {
            println!("\nSelect category by entering a number between '{}0{}' and '{}6{}': ",
                   colors.blue, colors.reset, colors.blue, colors.reset
            );
        },
        _ => {}
    }
}

pub fn get_list_type() -> usize {
    'listType: loop {
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
    }
}

pub fn get_scenario_index(len: usize, colors: &Colors) -> usize {
    print!("\nPlease select a {}scenario{} by entering a number between '{}0{}' and '{}{}{}': ",
             colors.blue, colors.reset, colors.blue, colors.reset, colors.blue, len-1, colors.reset);
    match io::stdout().flush() {
        Err(_) => {
            println!("\nPlease select a {}scenario{} by entering a number between '{}0{}' and '{}{}{}': ",
                     colors.blue, colors.reset, colors.blue, colors.reset, colors.blue, len-1, colors.reset
            );
        },
        _ => {}
    }

    'scenario: loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if num < len {return num;}
                else {
                    println!("Please Enter a valid number between '0' and '{}'", len-1);
                    continue 'scenario;
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue 'scenario;
            }
        };
    };
}

pub fn play_again() -> bool {
    print!("Play again? (y/N): ");
    match io::stdout().flush() {
        Err(_) => {
            println!("Play again? (y/N): ");
        },
        _ => {}
    }

    let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

    match input.trim().parse::<char>() {
            Ok(char) => {
                match char {
                    'y' | 'Y' => {true}
                    _ => {false},
                }
            },
            Err(_) => {false}
    }

}