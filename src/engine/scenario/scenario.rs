
use std::time::Instant;
use crate::engine::{RoomType, Vec3d, scenario::{MovingTarget, TargetType}};

pub struct Scenario {

    //player
    pub player_spawn:Vec3d,
    pub allow_mouse_hold: bool,
    pub allow_movement: bool,

    //room (centered around origin)
    pub room_type: RoomType,
    pub room_rad: f32,

    //target
    pub target_type: TargetType,
    pub target_spawn: (Vec3d, Vec3d),
    pub target_count: i8,
    pub target_rad: f32,
    pub target_hp: i32,
    pub moving_target: Option<MovingTarget>,
}
impl Scenario {
    fn jumbo_tf() -> Self {
        Scenario {
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room_type: RoomType::Cube,
            room_rad: 5.0,
            target_type: TargetType::Square,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 3,
            target_rad: 0.9, //0.9
            target_hp: 1,
            moving_target: None,
            allow_mouse_hold: false,
            allow_movement: false,
        }
    }
    fn jumbo_flat() -> Self {
        Scenario {
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room_type: RoomType::Cube,
            room_rad: 5.0,
            target_type: TargetType::Rectangle,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 3,
            target_rad: 0.9,
            target_hp: 1,
            moving_target: None,
            allow_mouse_hold: false,
            allow_movement: false,
        }
    }
    fn w_6t_te() -> Self {
        Scenario {
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room_type: RoomType::Cube,
            room_rad: 5.0,
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 6,
            target_rad: 0.25,
            target_hp: 1,
            moving_target: None,
            allow_mouse_hold: false,
            allow_movement: true,
        }
    }
    fn w_6t_small() -> Self {
        Scenario {
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room_type: RoomType::Cube,
            room_rad: 5.0,
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 6,
            target_rad: 0.1,
            target_hp: 1,
            moving_target: None,
            allow_mouse_hold: false,
            allow_movement: false,
        }
    }
    fn w_6t_extrasmall() -> Self {
        Scenario {
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room_type: RoomType::Cube,
            room_rad: 5.0,
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 6,
            target_rad: 0.05,
            target_hp: 1,
            moving_target: None,
            allow_mouse_hold: false,
            allow_movement: false,
        }
    }
    pub fn w_5t_pasu() -> Self {
        Scenario {
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room_type: RoomType::Cube,
            room_rad: 5.0,
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 5,
            target_rad: 0.25,
            target_hp: 1,
            moving_target: Some(
                MovingTarget { 
                    moving_room: (
                        Vec3d::new(4.0, 4.0, 4.0),
                        Vec3d::new(-4.0, -4.0, 4.0)
                    ), 
                    player_dist_r: 1.0, 
                    frequency: 0.3, 
                    p_change_dir: 2.0, 
                    vel_min_max: (2.0, 3.0), 
                    last_dir_change_time: Instant::now()
                }),
            allow_mouse_hold: false,
            allow_movement: false,
        }
    }
    pub fn random_sphere() -> Self {
        Scenario {
            player_spawn: Vec3d::new(0.0, -4.0, 0.0),
            room_type: RoomType::Cube,
            room_rad: 10.0,
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(0.0, 0.0, 9.0),
                Vec3d::new(0.0, 0.0, 9.0)
            ),
            target_count: 1,
            target_rad: 1.0,
            target_hp: 1000,
            moving_target: Some(
                MovingTarget { 
                    moving_room: (
                        Vec3d::new(9.0, 0.0, 9.0),
                        Vec3d::new(-9.0, -8.0, -9.0)
                    ), 
                    player_dist_r: 0.5, 
                    frequency: 0.3, 
                    p_change_dir: 0.5, 
                    vel_min_max: (2.0, 8.0), 
                    last_dir_change_time: Instant::now()
                }),
            allow_mouse_hold: true,
            allow_movement: false,
        }
    }
}

pub fn map_scenario(n: i8) -> Scenario {
    match n {
        1 => Scenario::jumbo_tf(),
        2 => Scenario::jumbo_flat(),
        3 => Scenario::w_6t_te(),
        4 => Scenario::w_6t_small(),
        5 => Scenario::w_6t_extrasmall(),
        6 => Scenario::w_5t_pasu(),
        99 => Scenario::random_sphere(),
        _ => Scenario::w_6t_te()
    }
}
