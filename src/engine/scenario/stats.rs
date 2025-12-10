use std::time::SystemTime;

use crate::engine::core::{BLUE, RESET};

pub struct Statistic {
    pub hits:i32,
    pub shots:i32,
    pub accuracy:f32,
    pub scenario_starttime:SystemTime,
    pub scenario_endtime:SystemTime
}
impl Statistic {
    pub fn new() -> Self {
        Self {
            hits: 0,
            shots:0,
            accuracy:0.0,
            scenario_starttime:SystemTime::now(),
            scenario_endtime:SystemTime::now(),
        }
    }
    pub fn add_hit(&mut self) {
        self.hits += 1; self.shots += 1;
    }
    pub fn add_shot(&mut self) {
        self.shots += 1;
    }
    pub fn get_accuracy_p(&mut self) -> f32{
        // Accuracy in % only if shots > 0
        if self.shots > 0 {
            self.accuracy = self.hits as f32 / self.shots as f32 * 100.0;
        }
        self.accuracy
    }
    pub fn end_scenario(&mut self) {
        self.scenario_endtime = SystemTime::now()
    }
    pub fn scenario_playtime(&self) -> i32 {
        self.scenario_endtime.duration_since(self.scenario_starttime).unwrap().as_secs() as i32
    }
    pub fn print_stats(&mut self, scenario_name:&String) {
        println!("\n{}--- RESULTS --- {}", BLUE, RESET);
        println!("\n{}{}{}", BLUE, scenario_name.to_uppercase(), RESET);
        println!("Duration: {} seconds", self.scenario_playtime());
        println!("Hits: {}", self.hits);
        println!("Shots: {}", self.shots);
        println!("Accuracy: {:.2}%", self.get_accuracy_p());
        println!("\n{}--- END RESULTS --- {}\n", BLUE, RESET);
        println!("Rerun executable to play again.\n");
    }
}