use crate::engine::targets::TargetType;

pub struct UserScore {
    pub time: i32,
    pub hits: i32,
    pub shots: i32,
    pub accuracy: f32,
}
pub struct Scenario {
    pub room_size: f32,
    pub room_dist: f32,
    pub target_type: TargetType,
    pub target_count: i8,
    pub spawn_d: f32,
    pub target_dist: f32,
    pub target_hp: i32,
    pub allow_mouse_hold: bool,
    pub allow_movement: bool,
}
impl Scenario {
    pub fn jumbo_tf() -> Self {
        Scenario {
            room_size: 4.0,
            room_dist: 3.0,
            target_type: TargetType::Square,
            target_count: 3,
            spawn_d: 3.0,
            target_dist: 4.5,
            target_hp: 1,
            allow_mouse_hold: false,
            allow_movement: false,
        }
    }
    pub fn jumbo_tf_flat() -> Self {
        Scenario {
            room_size: 4.0,
            room_dist: 3.0,
            target_type: TargetType::Rectangle,
            target_count: 3,
            spawn_d: 3.0,
            target_dist: 4.5,
            target_hp: 1,
            allow_mouse_hold: false,
            allow_movement: false,
        }
    }
    pub fn lw6t_te() -> Self {
        Scenario {
            room_size: 10.0,
            room_dist: 9.0,
            target_type: TargetType::Sphere,
            target_count: 6,
            spawn_d: 10.0,
            target_dist: 20.0,
            target_hp: 1,
            allow_mouse_hold: false,
            allow_movement: false,
        }
    }
    pub fn lw6ts() -> Self {
        Scenario {
            room_size: 30.0,
            room_dist: 29.0,
            target_type: TargetType::Sphere,
            target_count: 6,
            spawn_d: 25.0,
            target_dist: 55.0,
            target_hp: 1,
            allow_mouse_hold: false,
            allow_movement: false,
        }
    }
    pub fn lw6tes() -> Self {
        Scenario {
            room_size: 55.0,
            room_dist: 54.0,
            target_type: TargetType::Sphere,
            target_count: 6,
            spawn_d: 45.0,
            target_dist: 105.0,
            target_hp: 1,
            allow_mouse_hold: false,
            allow_movement: false,
        }
    }
}

pub fn map_scenario(n: i8) -> Scenario {
    match n {
        1 => Scenario::jumbo_tf(),
        2 => Scenario::jumbo_tf_flat(),
        3 => Scenario::lw6t_te(),
        4 => Scenario::lw6ts(),
        5 => Scenario::lw6tes(),
        _ => Scenario::lw6t_te(),
    }
}
