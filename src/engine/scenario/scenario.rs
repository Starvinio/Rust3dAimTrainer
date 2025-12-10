use std::time::Duration;

use crate::engine::{Gun, Mesh, RoomType, Vec3d, create_room, scenario::{MovingTarget, TargetType}};

pub struct Scenario {

    pub name: String,

    //duration of the scenario run
    pub duration_secs: Duration,

    //player 
    pub player_spawn:Vec3d,
    pub gun: Gun,
    pub allow_movement: bool,

    //room (centered around origin) 
    pub room: Mesh,

    //target properties
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
            name: String::from("Jumbo Tile Frenzy"),
            duration_secs: Duration::from_secs(30),
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room: create_room(RoomType::Cube, 5.0),
            target_type: TargetType::Square,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 3,
            target_rad: 0.9, //0.9
            target_hp: 1,
            moving_target: None,
            gun: Gun::pistol(),
            allow_movement: false,
        }
    }
    fn jumbo_flat() -> Self {
        Scenario {
            name: String::from("Jumbo Tile Frenzy Flat"),
            duration_secs: Duration::from_secs(30),
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room: create_room(RoomType::Cube, 5.0),
            target_type: TargetType::Rectangle,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 3,
            target_rad: 0.9,
            target_hp: 1,
            moving_target: None,
            gun: Gun::pistol(),
            allow_movement: false,
        }
    }
    fn w_6t_te() -> Self {
        Scenario {
            name: String::from("1 Wall 6 Targets TE"),
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room: create_room(RoomType::Cube, 5.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 6,
            target_rad: 0.25,
            target_hp: 1,
            moving_target: None,
            gun: Gun::pistol(),
            allow_movement: false,
        }
    }
    fn w_6t_small() -> Self {
        Scenario {
            name: String::from("1 Wall 6 Targets small"),
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room: create_room(RoomType::Cube, 5.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 6,
            target_rad: 0.1,
            target_hp: 1,
            moving_target: None,
            gun: Gun::pistol(),
            allow_movement: false,
        }
    }
    fn w_6t_extrasmall() -> Self {
        Scenario {
            name: String::from("1 Wall 6 Targets extra small"),
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room: create_room(RoomType::Cube, 5.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 6,
            target_rad: 0.05,
            target_hp: 1,
            moving_target: None,
            gun: Gun::pistol(),
            allow_movement: false,
        }
    }
    pub fn w_5t_pasu() -> Self {
        Scenario {
            name: String::from("1 Wall 5 Targets Pasu"),
            duration_secs: Duration::from_secs(90),
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room: create_room(RoomType::Cube, 5.0),
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
                    moving_bounds: (
                        Vec3d::new(4.0, 4.0, 4.0),
                        Vec3d::new(-4.0, -4.0, 4.0)
                    ), 
                    player_dist_r: 1.0, 
                    frequency: 0.3, 
                    p_change_dir: 10.0, 
                    vel_bounds: (
                        Vec3d::new(2.0, 2.0, 0.0), 
                        Vec3d::new(3.0, 3.0, 0.0)
                    ), 
                    interval_dir_change: 0.0,
                }),
            gun: Gun::pistol(),
            allow_movement: false,
        }
    }
    pub fn w_5t_pasu_small() -> Self {
        Scenario {
            name: String::from("1 Wall 5 Targets Pasu Small"),
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room: create_room(RoomType::Cube, 5.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(4.0, 4.0, 4.0),
                Vec3d::new(-4.0, -4.0, 4.0)
            ),
            target_count: 5,
            target_rad: 0.1,
            target_hp: 1,
            moving_target: Some(
                MovingTarget { 
                    moving_bounds: (
                        Vec3d::new(4.0, 4.0, 4.0),
                        Vec3d::new(-4.0, -4.0, 4.0)
                    ), 
                    player_dist_r: 1.0, 
                    frequency: 0.3, 
                    p_change_dir: 10.0, 
                    vel_bounds: (
                        Vec3d::new(2.0, 2.0, 0.0), 
                        Vec3d::new(3.0, 3.0, 0.0)
                    ), 
                    interval_dir_change: 0.0,
                }),
            gun: Gun::pistol(),
            allow_movement: false,
        }
    }
    pub fn air_invincible() -> Self {
        Scenario {
            name: String::from("Air Invincible"),
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, -4.0, 0.0),
            room: create_room(RoomType::Octagon, 5.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(0.0, 0.0, 5.0),
                Vec3d::new(0.0, 0.0, 5.0)
            ),
            target_count: 1,
            target_rad: 0.8,
            target_hp: 9999,
            moving_target: Some(
                MovingTarget { 
                    moving_bounds: (
                        Vec3d::new(4.0, 0.0, 4.0),
                        Vec3d::new(-4.0, -4.0, -4.0)
                    ), 
                    player_dist_r: 1.0, 
                    frequency: 0.2, 
                    p_change_dir: 12.0, 
                    vel_bounds: (
                        Vec3d::new(3.0, 3.0, 3.0), 
                        Vec3d::new(5.0, 5.0, 5.0)
                    ), 
                    interval_dir_change: 0.0,
                }),
            gun: Gun::laser(),
            allow_movement: false,
        }
    }
    
}

pub fn load_scenarios() -> Vec<Scenario> {
    vec![
        Scenario::jumbo_tf(),
        Scenario::jumbo_flat(),
        Scenario::w_6t_te(),
        Scenario::w_6t_small(),
        Scenario::w_6t_extrasmall(),
        Scenario::w_5t_pasu(),
        Scenario::w_5t_pasu_small(),
        Scenario::air_invincible(),
    ]
}
