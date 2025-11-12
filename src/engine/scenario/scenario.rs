use crate::engine::{Gun, Mesh, RoomType, Vec3d, create_room, scenario::{MovingTarget, TargetType}};

pub struct Scenario {

    //player
    pub player_spawn:Vec3d,
    pub gun: Gun,
    pub allow_movement: bool,

    //room (centered around origin)
    pub room: Mesh,

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
            allow_movement: true,
        }
    }
    fn w_6t_small() -> Self {
        Scenario {
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

pub fn map_scenario(n: i8) -> Scenario {
    match n {
        1 => Scenario::jumbo_tf(),
        2 => Scenario::jumbo_flat(),
        3 => Scenario::w_6t_te(),
        4 => Scenario::w_6t_small(),
        5 => Scenario::w_6t_extrasmall(),
        6 => Scenario::w_5t_pasu(),
        7 => Scenario::w_5t_pasu_small(),
        8 => Scenario::air_invincible(),
        _ => Scenario::w_6t_te()
    }
}
