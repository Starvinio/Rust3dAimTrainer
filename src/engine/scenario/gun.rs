#![allow(unused)]

use std::{ops::Sub, time::{Duration, Instant}};

pub struct Gun {
    pub fire_rate_s:f32,
    pub last_shot:Instant,
}
impl Gun {
    pub fn new(fire_rate:f32) -> Self {
        Self {
            fire_rate_s: fire_rate, 
            last_shot:Instant::sub(Instant::now(), Duration::from_secs_f32(fire_rate))
        }
    }
    pub fn can_shoot(&self) -> bool {
        self.last_shot.elapsed().as_secs_f32() >= self.fire_rate_s
    }
    pub fn shoot(&mut self) {
        self.last_shot = Instant::now();
    }
}

struct GunWithMag {
    pub fire_rate_s:f32,
    pub magazine:i8,
    pub reload_time_s:f32,
    pub last_shot:Instant,
}
impl GunWithMag {
    pub fn new(fire_rate:f32, mag_capacity:i8, reload_time:f32) -> Self {
        Self {
            fire_rate_s: fire_rate, 
            magazine: mag_capacity, 
            reload_time_s: reload_time,
            last_shot:Instant::sub(Instant::now(), Duration::from_secs_f32(fire_rate))
        }
    }
    pub fn can_shoot(&self) -> bool {
        self.magazine > 0 && self.last_shot.elapsed().as_secs_f32() >= self.fire_rate_s
    }
    pub fn shoot(&mut self) {
        if self.can_shoot() {
            self.last_shot = Instant::now();
            self.magazine -= 1;
        }
    }
}