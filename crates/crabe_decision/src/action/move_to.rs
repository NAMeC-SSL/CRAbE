use std::f64::consts::{FRAC_PI_6, PI};
use crate::action::state::State;
use crate::action::Action;
use crate::manager::game_manager::GameManager;
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{World, AllyInfo, Robot};
use nalgebra::{distance, Point2, Rotation2, Vector2, Isometry2, Vector3};
use std::ops::{Div};

const OBSTACLE_RADIUS: f64 = 0.7;
const K_ATTRACTION: f64 = 1.0;
const K_REPULSION: f64 = 1.0;
const DIST_CHECK_FINISHED: f64 = 0.02;
const MAX_ANGLE_ERROR: f64 = FRAC_PI_6;
//Speed
const NORM_MULTIPLIER: f64 = 1.0;
/// The default factor speed for the robot to move towards the target position.
const GOTO_SPEED: f64 = 1.5;
/// The default factor speed for the robot to rotate towards the target orientation.
const GOTO_ROTATION: f64 = 3.15;
/// The error tolerance for arriving at the target position.
const ERR_TOLERANCE: f64 = 0.115;
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
    /// Dribble strength
    dribble: f32,
    /// Whether to kick or not. None if no kick required
    kick: Option<Kick>,
    /// Avoid the ball
    avoid_ball: bool,
    /// Set to true if we should charge the capacitors for kicking when being near target
    chg_near_arrival: bool,
}

impl From<&mut MoveTo> for MoveTo {
    fn from(other: &mut MoveTo) -> MoveTo {
        MoveTo {
            state: other.state,
            target: other.target,
            orientation: other.orientation,
            dribble: other.dribble,
            kick: other.kick,
            avoid_ball: other.avoid_ball,
            chg_near_arrival: other.chg_near_arrival,
        }
    }
}

impl MoveTo {
    /// Creates a new `MoveTo` instance, avoiding any obstacles in the way.
    /// Speed is limited by normalizing the movement vector
    /// Implementation of this paper : https://www.researchgate.net/publication/313389747_Potential_field_methods_and_their_inherent_approaches_for_path_planning
    ///
    /// # Arguments
    ///
    /// * `target` : The target position on the field to move the robot to.
    /// * `orientation` : The target orientation of the robot.
    /// * `avoid_ball` : Set to true to make the MoveTo avoid the ball as well as the other robots
    /// * `charge_when_near_target` : Set to true to charge the kickers when we're near the target (about 0.3 meter)
    pub fn new(target: Point2<f64>, orientation: f64, dribble: f32, kick: Option<Kick>, avoid_ball: bool, charge_when_near_target: bool) -> Self {
        Self {
            state: State::Running,
            target,
            orientation,
            avoid_ball,
            dribble,
            kick,
            chg_near_arrival: charge_when_near_target,
        }
    }

    /// Computes the attractive force of the goal target to attain
    /// using the formula from the paper.
    ///
    /// # Arguments
    ///
    /// * `q` : The robot's vector position (or coordinates)
    /// * `q_d` : The target's vector position (or coordinates)
    fn attractive_force(&self, q: &Point2<f64>, q_d: &Point2<f64>) -> Vector2<f64> {
        -K_ATTRACTION * (q - q_d)
    }

    /// Computes the repulsive force generated by an obstacle,
    /// using the formula from the paper.
    ///
    /// # Arguments
    ///
    /// * `d_0` : Constant, radius of the obstacle
    /// * `d_q` : Euclidean distance between the robot and the obstacle
    /// * `q`   : The robot's vector position (or coordinates)
    /// * `q_c` : The obstacle's vector position (or coordinates)
    fn repulsive_force(&self, d_0: &f64, d_q: &f64, q: &Point2<f64>, q_c: &Point2<f64>) -> Vector2<f64> {
        K_REPULSION *
        (1.0.div(d_q) - 1.0.div(d_0)) *
        (1.0.div(d_q.powi(2))) *
        ((q-q_c).div(distance(&q, q_c)))
    }

    /// Computes the angular speed required to adjust the robot's orientation to the required orientation
    ///
    /// # Arguments
    ///
    /// * `robot_theta` : The current orientation of the robot
    fn angular_speed(&self, robot_theta: &f64) -> f32 {
        let wanted_orientation = self.orientation.rem_euclid(2. * PI);
        let curent_orientation = robot_theta.rem_euclid(2. * PI);
        let mut error_orientation = wanted_orientation - curent_orientation;
        if error_orientation.abs() > PI{
            error_orientation = -error_orientation;
        }
        (GOTO_ROTATION * error_orientation) as f32
    }

    pub fn dumb_moveto(&mut self, robot: &Robot<AllyInfo>, _world: &World, target: Point2<f64>) -> Command {
        let ti = frame_inv(robot_frame(robot));
        let target_in_robot = ti * Point2::new(target.x, target.y);
        let wanted_orientation = self.orientation.rem_euclid(2. * PI);
        let curent_orientation = robot.pose.orientation.rem_euclid(2. * PI);
        let mut error_orientation = wanted_orientation - curent_orientation;
        if error_orientation.abs() > PI{
            error_orientation = -error_orientation;
        }
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
    }
    pub fn smart_moveto(&mut self, robot: &Robot<AllyInfo>, world: &World, target: Point2<f64>) -> Command {
        
        let dist_to_target = distance(&robot.pose.position, &target);
        if dist_to_target <= DIST_CHECK_FINISHED {
            self.state = State::Done;
            return Command::default();
        }

        // Resulting movement vector
        let mut f = Vector2::new(0.0, 0.0);

        // -- Attractive field
        f += self.attractive_force(&robot.pose.position, &target);

        // -- Repulsive field
        let mut dist_to_obst = 0.;
        // Don't compute any repulsion if robot is already near target
        if dist_to_target >= 0.15 {
            let mut repulsive_strength_sum = Vector2::new(0.0, 0.0);
            world.allies_bot.iter()
                // Our robot id is not an obstacle
                .filter(|(id, _)| **id != robot.id)
                .for_each(|(_, ally)| {

                    dist_to_obst = distance(&robot.pose.position, &ally.pose.position);

                    if dist_to_obst < OBSTACLE_RADIUS {
                        repulsive_strength_sum += self.repulsive_force(&OBSTACLE_RADIUS, &dist_to_obst, &robot.pose.position, &ally.pose.position);
                    }
                }
            );

            world.enemies_bot.iter()
                .for_each(|(_, enemy)| {
                    // Distance from our robot and the ally obstacle
                    dist_to_obst = distance(&robot.pose.position, &enemy.pose.position);

                    if dist_to_obst < OBSTACLE_RADIUS {
                        repulsive_strength_sum += self.repulsive_force(&OBSTACLE_RADIUS, &dist_to_obst, &robot.pose.position, &enemy.pose.position);
                    }
                }
            );

            // avoid ball if tasked
            if self.avoid_ball {
                if let Some(ball) = &world.ball {
                    let ball_position = &ball.position.xy();
                    if distance(ball_position, &robot.pose.position) <= OBSTACLE_RADIUS {
                        let d_q = distance(&robot.pose.position, ball_position);
                        repulsive_strength_sum += self.repulsive_force(&OBSTACLE_RADIUS, &d_q, &robot.pose.position, ball_position);
                    }
                }
            }

            f += repulsive_strength_sum;
        }

        // -- Normalizing the strength vector to avoid super Sonic speed
        //    but only if not close to target, otherwise leads to oscillation
        if dist_to_target > 1.0 {
            f = f.normalize() * NORM_MULTIPLIER;
        }

        // -- Compute angle of the resulting vector
        let angular_velocity = self.angular_speed(&robot.pose.orientation);

        // -- Change the basis of the resulting vector to the basis of the robot
        //    I'm not exactly sure why it's `-robot_theta` and not `robot_theta`
        let rob_rotation_basis = Rotation2::new(-&robot.pose.orientation);
        // println!("Before transformation : {}", &f);
        f = rob_rotation_basis * f;
        // println!("After transformation : {}", &f);

        // -- Determine whether we need to charge
        let charge = self.chg_near_arrival && distance(&robot.pose.position, &self.target) <= 0.3;

        Command {
            forward_velocity: f.x as f32,
            left_velocity: f.y as f32,
            angular_velocity,
            charge,
            kick: self.kick,
            dribbler: self.dribble,
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
            let mut target = self.target.clone();
            //prevent going in the goal zone
            if id != KEEPER_ID{
                if &target.y.abs() < &(&world.geometry.ally_penalty.width / 2.) && &world.geometry.field.length>&0.{
                    let penalty_y = &world.geometry.field.length/2. - &world.geometry.ally_penalty.depth;
                    target.x = target.x.clamp(-penalty_y, penalty_y);
                }
            }
            if GameManager::bot_in_trajectory(world, id, target){
                self.smart_moveto(robot, world, target)
            }else{
                self.dumb_moveto(robot, world, target)
            }
        }else {
            Command::default()
        }        
    }
}
