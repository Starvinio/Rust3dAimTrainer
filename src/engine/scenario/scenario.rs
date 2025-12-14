use std::time::Duration;

use crate::engine::{Gun, Mesh, RoomType, Vec3d, create_room, scenario::{MovingTarget, TargetType}};
use crate::engine::color::Colors;

pub struct Scenario {

    //scenario info
    pub name: String,
    pub aiming_type: AimingType,

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
pub enum AimingType {
    StaticClicking,
    DynamicClicking,
    ReactiveTracking,
    PreciseTracking,
    SpeedSwitching,
    EvasiveSwitching
}
impl Scenario {
    fn jumbo_tf() -> Self {
        Scenario {
            name: String::from("Jumbo Tile Frenzy"),
            aiming_type: AimingType::StaticClicking,
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
            aiming_type: AimingType::StaticClicking,
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

    fn mini_tf() -> Self {
        Scenario {
            name: String::from("Mini Tile Frenzy"),
            aiming_type: AimingType::StaticClicking,
            duration_secs: Duration::from_secs(30),
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room: create_room(RoomType::Cube, 5.0),
            target_type: TargetType::Rectangle,
            target_spawn: (
                Vec3d::new(0.4, 0.4, 4.0),
                Vec3d::new(-0.4, -0.4, 4.0)
            ),
            target_count: 3,
            target_rad: 0.1,
            target_hp: 1,
            moving_target: None,
            gun: Gun::pistol(),
            allow_movement: false,
        }
    }
    fn w_6t_te() -> Self {
        Scenario {
            name: String::from("1 Wall 6 Targets TE"),
            aiming_type: AimingType::StaticClicking,
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
            aiming_type: AimingType::StaticClicking,
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
            aiming_type: AimingType::StaticClicking,
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
    fn ww_6t_small() -> Self {
        Scenario {
            name: String::from("Wide Wall 6 Targets small"),
            aiming_type: AimingType::StaticClicking,
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, 1.0, 0.0),
            room: create_room(RoomType::Cone, 20.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(8.0, 3.0, 9.0),
                Vec3d::new(-8.0, -3.0, 9.0)
            ),
            target_count: 6,
            target_rad: 0.2,
            target_hp: 1,
            moving_target: None,
            gun: Gun::pistol(),
            allow_movement: false,
        }
    }
    fn ww_6t_small_ts() -> Self {
        Scenario {
            name: String::from("Wide Wall 6 Targets small TS"),
            aiming_type: AimingType::SpeedSwitching,
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, 1.0, -2.0),
            room: create_room(RoomType::Cone, 20.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(8.0, 3.0, 9.0),
                Vec3d::new(-8.0, -3.0, 9.0)
            ),
            target_count: 6,
            target_rad: 0.2,
            target_hp: 5,
            moving_target: None,
            gun: Gun::laser(),
            allow_movement: false,
        }
    }
    fn control_ts() -> Self {
        Scenario {
            name: String::from("Control TS"),
            duration_secs: Duration::from_secs(60),
            aiming_type: AimingType::EvasiveSwitching,
            player_spawn: Vec3d::new(0.0, -5.0, -2.0),
            room: create_room(RoomType::Cone, 10.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(6.0, -3.0, 9.0),
                Vec3d::new(-6.0, -5.0, 7.0)
            ),
            target_count: 3,
            target_rad: 0.3,
            target_hp: 40,
            moving_target: Some(
                MovingTarget {
                    moving_bounds: (
                        Vec3d::new(6.0, -3.0, 9.0),
                        Vec3d::new(-6.0, -5.0, 7.0)
                    ),
                    player_dist_r: 1.0,
                    frequency: 0.3,
                    p_change_dir: 10.0,
                    vel_bounds: (
                        Vec3d::new(2.0, 1.0, 0.2),
                        Vec3d::new(2.0, 1.0, 0.5)
                    ),
                    interval_dir_change: 0.0,
            }),
            gun: Gun::laser(),
            allow_movement: false,
        }
    }
    pub fn w_5t_pasu() -> Self {
        Scenario {
            name: String::from("1 Wall 5 Targets Pasu"),
            aiming_type: AimingType::DynamicClicking,
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
            aiming_type: AimingType::DynamicClicking,
            duration_secs: Duration::from_secs(90),
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
    pub fn floating_heads() -> Self {
        Scenario {
            name: String::from("Floating Heads"),
            aiming_type: AimingType::DynamicClicking,
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, 1.0, 0.0),
            room: create_room(RoomType::Cube, 7.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(6.0, 1.0, 6.0),
                Vec3d::new(-6.0, -1.0, 4.0)
            ),
            target_count: 5,
            target_rad: 0.05,
            target_hp: 1,
            moving_target: Some(
                MovingTarget {
                    moving_bounds: (
                        Vec3d::new(4.0, 1.0, 4.0),
                        Vec3d::new(-4.0, -1.0, 3.0)
                    ),
                    player_dist_r: 1.0,
                    frequency: 0.1,
                    p_change_dir: 0.5,
                    vel_bounds: (
                        Vec3d::new(0.8, 0.2, 0.1),
                        Vec3d::new(1.0, 0.2, 0.1)
                    ),
                    interval_dir_change: 0.0,
                }),
            gun: Gun::pistol(),
            allow_movement: false,
        }
    }
    pub fn floating_heads_small() -> Self {
        Scenario {
            name: String::from("Floating Heads Small"),
            aiming_type: AimingType::DynamicClicking,
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, 1.0, 0.0),
            room: create_room(RoomType::Cube, 7.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(6.0, 1.0, 6.0),
                Vec3d::new(-6.0, -1.0, 4.0)
            ),
            target_count: 5,
            target_rad: 0.03,
            target_hp: 1,
            moving_target: Some(
                MovingTarget {
                    moving_bounds: (
                        Vec3d::new(4.0, 1.0, 4.0),
                        Vec3d::new(-4.0, -1.0, 3.0)
                    ),
                    player_dist_r: 1.0,
                    frequency: 0.1,
                    p_change_dir: 0.5,
                    vel_bounds: (
                        Vec3d::new(0.6, 0.1, 0.0),
                        Vec3d::new(0.8, 0.1, 0.0)
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
            aiming_type: AimingType::ReactiveTracking,
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
            target_hp: 99999,
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

    pub fn close_strafes_invincible() -> Self {
        Scenario {
            name: String::from("Close Strafes Invincible"),
            aiming_type: AimingType::ReactiveTracking,
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, -1.5, 0.0),
            room: create_room(RoomType::Octagon, 4.0),
            target_type: TargetType::Bean,
            target_spawn: (
                Vec3d::new(0.0, 0.0, 5.0),
                Vec3d::new(0.0, 0.0, 5.0)
            ),
            target_count: 1,
            target_rad: 0.8,
            target_hp: 99999,
            moving_target: Some(
                MovingTarget {
                    moving_bounds: (
                        Vec3d::new(3.0, -2.0, 3.0),
                        Vec3d::new(-3.0, -2.0, -3.0)
                    ),
                    player_dist_r: 1.0,
                    frequency: 0.2,
                    p_change_dir: 20.0,
                    vel_bounds: (
                        Vec3d::new(3.0, 0.0, 3.0),
                        Vec3d::new(5.0, 0.0, 5.0)
                    ),
                    interval_dir_change: 0.0,
                }),
            gun: Gun::laser(),
            allow_movement: true,
        }
    }

    pub fn smooth_strafes_invincible() -> Self {
        Scenario {
            name: String::from("Smooth Strafes 90 Invincible"),
            aiming_type: AimingType::PreciseTracking,
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, -3.0, 0.0),
            room: create_room(RoomType::Cube, 10.0),
            target_type: TargetType::Bean,
            target_spawn: (
                Vec3d::new(0.0, 0.0, 9.0),
                Vec3d::new(0.0, 0.0, 9.0)
            ),
            target_count: 1,
            target_rad: 0.4,
            target_hp: 99999,
            moving_target: Some(
                MovingTarget {
                    moving_bounds: (
                        Vec3d::new(9.0, -4.0, 9.0),
                        Vec3d::new(-9.0, -4.0, 9.0)
                    ),
                    player_dist_r: 7.0,
                    frequency: 0.02,
                    p_change_dir: 2.0,
                    vel_bounds: (
                        Vec3d::new(4.0, 0.0, 4.0),
                        Vec3d::new(6.0, 0.0, 6.0)
                    ),
                    interval_dir_change: 0.0,
                }),
            gun: Gun::laser(),
            allow_movement: true,
        }
    }

    pub fn raw_control_invincible() -> Self {
        Scenario {
            name: String::from("Raw Control Invincible"),
            aiming_type: AimingType::ReactiveTracking,
            duration_secs: Duration::from_secs(60),
            player_spawn: Vec3d::new(0.0, 0.0, -3.0),
            room: create_room(RoomType::Cube, 5.0),
            target_type: TargetType::Sphere,
            target_spawn: (
                Vec3d::new(0.0, 0.0, 5.0),
                Vec3d::new(0.0, 0.0, 5.0)
            ),
            target_count: 1,
            target_rad: 0.3,
            target_hp: 99999,
            moving_target: Some(
                MovingTarget {
                    moving_bounds: (
                        Vec3d::new(4.0, 3.0, 4.0),
                        Vec3d::new(-4.0, -3.0, 0.0)
                    ),
                    player_dist_r: 3.0,
                    frequency: 0.5,
                    p_change_dir: 20.0,
                    vel_bounds: (
                        Vec3d::new(2.0, 2.0, 2.0),
                        Vec3d::new(3.0, 3.0, 3.0)
                    ),
                    interval_dir_change: 0.0,
                }),
            gun: Gun::laser(),
            allow_movement: false,
        }
    }

    fn pure_horizontal_click() -> Self {
        Scenario {
            name: String::from("Pure Horizontal Click"),
            aiming_type: AimingType::StaticClicking,
            duration_secs: Duration::from_secs(999),
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room: create_room(RoomType::Cube, 5.0),
            target_type: TargetType::Square,
            target_spawn: (
                Vec3d::new(5.0, 0.0, 4.0),
                Vec3d::new(-5.0, 0.0, 4.0)
            ),
            target_count: 3,
            target_rad: 0.5,
            target_hp: 1,
            moving_target: None,
            gun: Gun::pistol(),
            allow_movement: true,
        }
    }

    fn pure_vertical_click() -> Self {
        Scenario {
            name: String::from("Pure Vertical Click"),
            aiming_type: AimingType::StaticClicking,
            duration_secs: Duration::from_secs(999),
            player_spawn: Vec3d::new(0.0, 1.0, -4.0),
            room: create_room(RoomType::Cube, 5.0),
            target_type: TargetType::Square,
            target_spawn: (
                Vec3d::new(0.0, 5.0, 4.0),
                Vec3d::new(0.0, -5.0, 4.0)
            ),
            target_count: 3,
            target_rad: 0.5,
            target_hp: 1,
            moving_target: None,
            gun: Gun::pistol(),
            allow_movement: true,
        }
    }
    
}

pub fn load_all_scenarios() -> Vec<Scenario> {
    let mut all_scenarios: Vec<Scenario> = Vec::new();
    all_scenarios.append(&mut static_clicking());
    all_scenarios.append(&mut dyn_clicking());
    all_scenarios.append(&mut reactive_tracking());
    all_scenarios.append(&mut precise_tracking());
    all_scenarios.append(&mut speed_switching());
    all_scenarios.append(&mut evasive_switching());
    all_scenarios
}

pub fn static_clicking() -> Vec<Scenario> {
    vec![
        Scenario::jumbo_tf(),
        Scenario::jumbo_flat(),
        Scenario::mini_tf(),
        Scenario::w_6t_te(),
        Scenario::w_6t_small(),
        Scenario::w_6t_extrasmall(),
        Scenario::ww_6t_small(),
        Scenario::pure_horizontal_click(),
        Scenario::pure_vertical_click(),
    ]
}

pub fn dyn_clicking() -> Vec<Scenario> {
    vec![
        Scenario::w_5t_pasu(),
        Scenario::w_5t_pasu_small(),
        Scenario::floating_heads(),
        Scenario::floating_heads_small(),
    ]
}

pub fn reactive_tracking() -> Vec<Scenario> {
    vec![
        Scenario::air_invincible(),
        Scenario::close_strafes_invincible(),
        Scenario::raw_control_invincible(),
    ]
}

pub fn precise_tracking() -> Vec<Scenario> {
    vec![
        Scenario::smooth_strafes_invincible(),
    ]
}

pub fn speed_switching() -> Vec<Scenario> {
    vec![
        Scenario::ww_6t_small_ts(),
    ]
}

pub fn evasive_switching() -> Vec<Scenario> {
    vec![
        Scenario::control_ts(),
    ]
}

pub fn get_scenarios(i: usize, colors: &Colors) -> Vec<Scenario> {
    match i {
        1 => {
            println!("\n{}STATIC CLICKING{}", colors.blue, colors.reset);
            static_clicking()
        },
        2 => {
            println!("\n{}DYNAMIC CLICKING{}", colors.blue, colors.reset);
            dyn_clicking()
        },
        3 => {
            println!("\n{}REACTIVE TRACKING{}", colors.blue, colors.reset);
            reactive_tracking()
        },
        4 => {
            println!("\n{}PRECISE CLICKING{}", colors.blue, colors.reset);
            precise_tracking()
        },
        5 => {
            println!("\n{}SPEED SWITCHING{}", colors.blue, colors.reset);
            speed_switching()
        },
        6 => {
            println!("\n{}STATIC CLICKING{}", colors.blue, colors.reset);
            evasive_switching()
        },
        0 | _ => {
            println!("\n{}ALL SCENARIOS{}", colors.blue, colors.reset);
            load_all_scenarios()},
    }
}


