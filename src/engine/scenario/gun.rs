#![allow(unused)]

use std::{ops::Sub, time::{Duration, Instant}};

pub struct Gun {
    pub automatic:bool,
    pub fire_rate_s:f32,
    pub last_shot:Instant,
}
impl Gun {
    pub fn pistol() -> Self {
        Self {
            automatic: false,
            fire_rate_s: 0.05, 
            last_shot:Instant::now()
        }
    }
    pub fn laser() -> Self {
        Self {
            automatic: true,
            fire_rate_s: 0.01, 
            last_shot:Instant::now()
        }
    }
    pub fn can_shoot(&self) -> bool {
        self.last_shot.elapsed().as_secs_f32() >= self.fire_rate_s
    }
    pub fn shoot(&mut self) {
        self.last_shot = Instant::now();
    }
}
