use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{Isometry2, Point2, Vector2, Vector3, ComplexField};
use std::f64::consts::PI;
use crate::constants::{KEEPER_ID};


/// The `MoveTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.
#[derive(Clone)]
pub struct MoveTo {
    /// The current state of the action.
    state: State,
    /// The target position to move to.
    target: Point2<f64>,
    /// The target orientation of the robot.
    orientation: f64,
    kick: Option<Kick>,
    dribble: f32
}

impl From<&mut MoveTo> for MoveTo {
    fn from(other: &mut MoveTo) -> MoveTo {
        MoveTo {
            state: other.state,
            target: other.target,
            orientation: other.orientation,
            kick: other.kick,
            dribble: other.dribble
        }
    }
}

impl MoveTo {
    /// Creates a new `MoveTo` instance.
    ///
    /// # Arguments
    ///
    /// * `target`: The target position on the field to move the robot to.
    /// * `orientation`: The target orientation of the robot.
    pub fn new(target: Point2<f64>, orientation: f64, dribble: f32, kick: Option<Kick>) -> Self {
        Self {
            state: State::Running,
            target,
            orientation,
            kick,
            dribble
        }
    }
}

fn frame(x: f64, y: f64, orientation: f64) -> Isometry2<f64> {
    Isometry2::new(Vector2::new(x, y), orientation)
}

fn frame_inv(frame: Isometry2<f64>) -> Isometry2<f64> {
    frame.inverse()
}

fn robot_frame(robot: &Robot<AllyInfo>) -> Isometry2<f64> {
    frame(
        robot.pose.position.x,
        robot.pose.position.y,
        robot.pose.orientation,
    )
}

fn angle_wrap(alpha: f64) -> f64 {
    (alpha + PI) % (2.0 * PI) - PI
}

/// The default factor speed for the robot to move towards the target position.
const GOTO_SPEED: f64 = 1.5;
/// The default factor speed for the robot to rotate towards the target orientation.
const GOTO_ROTATION: f64 = 1.5;
/// The error tolerance for arriving at the target position.
const ERR_TOLERANCE: f64 = 0.115;

impl Action for MoveTo {
    /// Returns the name of the action.
    fn name(&self) -> String {
        String::from("MoveTo")
    }

    /// Returns the state of the action.
    fn state(&mut self) -> State {
        self.state
    }

    /// Computes the orders to be sent to the robot and returns a `Command` instance.
    /// If the robot arrives at the target position and orientation, the action is considered done.
    ///
    /// # Arguments
    ///
    /// * `id`: The id of the robot for which the orders are computed.
    /// * `world`: The current state of the world.
    /// * `tools`: A collection of external tools used by the action, such as a viewer.
    fn compute_order(&mut self, id: u8, world: &World, _tools: &mut ToolData) -> Command {
        if let Some(robot) = world.allies_bot.get(&id) {
            let mut target = if &world.data.positive_half == &world.team_color {
                Point2::new(-self.target.x, self.target.y)
            }else{
                self.target
            };
            //prevent going in the goal zone
            if id != KEEPER_ID{
                dbg!(&target.y.abs(),&(&world.geometry.ally_penalty.width / 2.));
                if &target.y.abs() < &(&world.geometry.ally_penalty.width / 2.) && &world.geometry.field.length>&0.{
                    let penalty_y = dbg!(&world.geometry.field.length/2.) - dbg!(&world.geometry.ally_penalty.depth);
                    target.x = target.x.clamp(-penalty_y, penalty_y);
                }
            }
            let ti = frame_inv(robot_frame(robot));
            let target_in_robot = ti * Point2::new(target.x, target.y);

            let error_orientation = angle_wrap(self.orientation - robot.pose.orientation);
            let error_x = target_in_robot[0];
            let error_y = target_in_robot[1];
            let arrived = Vector3::new(error_x, error_y, error_orientation).norm() < ERR_TOLERANCE;
            if arrived {
                self.state = State::Done;
            }
            let order = Vector3::new(
                GOTO_SPEED * error_x,
                GOTO_SPEED * error_y,
                GOTO_ROTATION * error_orientation,
            );

            let dribble = self.dribble.clamp(0., 1.);

            Command {
                forward_velocity: order.x as f32,
                left_velocity: order.y as f32,
                angular_velocity: order.z as f32,
                charge: true,
                kick: self.kick,
                dribbler: dribble,
            }
        } else {
            Command::default()
        }
    }
}
