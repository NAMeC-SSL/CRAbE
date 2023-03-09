use crate::config::CommonConfig;
use crate::data::geometry::Geometry;
use clap::builder::Str;
use nalgebra::{Point2, Point3, Vector3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use nalgebra::Vector2;

#[derive(Serialize, Clone, Default, Debug)]
pub struct AllyInfo;
#[derive(Serialize, Clone, Default, Debug)]
pub struct EnemyInfo;

#[derive(Serialize, Default, Debug, Clone)]
pub struct RobotVelocity {
    pub linear: Vector2<f64>,
    pub angular: f64
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Pose {
    pub orientation: f64,
    pub position: Point2<f64>
}

impl Pose {
    pub fn new(position: Point2<f64>, orientation: f64) -> Pose {
        Pose {
            orientation,
            position
        }
    }
}

#[derive(Serialize, Default, Debug)]
pub struct Robot<T> {
    pub id: u32,
    pub has_ball: bool,
    pub robot_info: T,
    pub pose: Pose,
    pub velocity: RobotVelocity,
    pub timestamp: DateTime<Utc>
}

impl<T: Clone> Clone for Robot<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            has_ball: self.has_ball,
            robot_info: self.robot_info.clone(),
            pose: self.pose.clone(),
            velocity: self.velocity.clone(),
            timestamp: self.timestamp
        }
    }
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct Ball {
    pub position: Point3<f64>,
    pub timestamp: DateTime<Utc>,
    pub velocity: Vector3<f64>
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TeamColor {
    Blue,
    Yellow,
}

impl TeamColor {
    pub fn opposite(&self) -> Self {
        match self {
            TeamColor::Blue => TeamColor::Yellow,
            TeamColor::Yellow => TeamColor::Blue,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Team {
    color: TeamColor,
    name: Option<String>,
}

impl Team {
    pub fn with_color(color: TeamColor) -> Self {
        Self { color, name: None }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct GameState {
    pub ally: Team,
    pub enemy: Team,
    pub positive_half: TeamColor,
}

impl GameState {
    pub fn new(team_color: TeamColor) -> Self {
        Self {
            ally: Team::with_color(team_color),
            enemy: Team::with_color(team_color.opposite()),
            positive_half: team_color.opposite(),
        }
    }
}

pub type RobotMap<T> = HashMap<u32, Robot<T>>;

#[derive(Serialize, Clone, Debug)]
pub struct World {
    pub state: GameState,
    pub geometry: Geometry,
    pub allies_bot: RobotMap<AllyInfo>,
    pub enemies_bot: RobotMap<EnemyInfo>,
    pub ball: Option<Ball>,
    pub team_color: TeamColor,
}

impl World {
    pub fn with_config(config: &CommonConfig) -> Self {
        let team_color = if config.yellow {
            TeamColor::Yellow
        } else {
            TeamColor::Blue
        };
        Self {
            state: GameState::new(team_color),
            geometry: Default::default(),
            allies_bot: Default::default(),
            enemies_bot: Default::default(),
            ball: None,
            team_color,
        }
    }
}
