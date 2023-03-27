use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{Isometry2, Point, Point2, Vector2, Vector3};
use std::f64::consts::PI;
use std::time::{Duration, Instant};

fn delta_angle(a: f64, b: f64) -> f64 {
    let mut a = a % (2.0 * PI);
    if a < 0.0 {
        a += 2.0 * PI;
    }
    let mut b = b % (2.0 * PI);
    if b < 0.0 {
        b += 2.0 * PI;
    }
    let mut r = b - a;
    if r > PI {
        r -= 2.0 * PI;
    } else if r < -PI {
        r += 2.0 * PI;
    }
    r
}

pub enum How {
    Fast,
    Accurate,
    Intersept,
    StopLimits,
    Goal,
}

pub struct MoveTo {
    robot_id: u8,
    through: Point2<f64>,
    has_through: bool,
    dst: Point2<f64>,
    xy_speed: RampSpeed,
    angle_speed: RampSpeed,
    xy_hyst: f64,
    angle_hyst: f64,
    closest_distance: Option<f64>,
    last_closest_distance: Option<Instant>,
    state: State,
}

impl MoveTo {
    pub fn new(robot_id: u8, through: Option<Point2<f64>>, dst: Point2<f64>, how: How) -> MoveTo {
        let mut moveto = MoveTo {
            robot_id,
            dst,
            through: through.unwrap_or(dst),
            xy_speed: RampSpeed::new(0.0, 0.0, 0.0, 0.0),
            angle_speed: RampSpeed::new(0.0, 0.0, 0.0, 0.0),
            xy_hyst: 0.0,
            angle_hyst: 0.0,
            state: State::Running,
            closest_distance: None,
            last_closest_distance: None,
            has_through: through.is_some()
        };

        moveto.update_how(how);
        moveto
    }


    fn update_how(&mut self, how: How) {
        match how {
            How::Fast => {
                self.xy_speed.update(0.2, 4.0, 4.0, 2.0);
                self.angle_speed.update(0.1, 4.0, 4.0, PI);
                self.xy_hyst = 0.1;
                self.angle_hyst = PI / 8.0;
            }
            How::Accurate => {
                self.xy_speed.update(0.01, 3.0, 1.5, 1.5);
                self.angle_speed.update(0.05, 3.0, 3.0, PI);
                self.xy_hyst = 0.01;
                self.angle_hyst = 2.5;
            }
            How::Intersept => {
                self.xy_speed.update(0.25, 5.0, 5.0, 4.0);
                self.angle_speed.update(0.1, 5.0, 5.0, 2.0 * PI);
                self.xy_hyst = 0.0001;
                self.angle_hyst = PI / 16.0;
            }
            How::StopLimits => {
                self.xy_speed.update(0.1, 3.0, 2.0, 1.4);
                self.angle_speed.update(0.1, 3.0, 3.0, PI / 2.0);
                self.xy_hyst = 0.01;
                self.angle_hyst = PI / 16.0;
            }
            How::Goal => {
                self.xy_speed.update(0.4, 8.0, 4.0, 4.0);
                self.angle_speed.update(0.01, 2.0 * PI, 3.0, 2.0 * PI);
                self.xy_hyst = 0.01;
                self.angle_hyst = 0.1;
            }
        }
    }
}

impl Action for MoveTo {
    fn name(&self) -> String {
        "MoveTo".to_string()
    }

    fn state(&mut self) -> State {
        self.state
    }

    fn compute_order(&mut self, id: u8, world: &World, tools: &mut ToolData) -> Command {
        let robot = match world.allies_bot.get(&id) {
            None => {
                // self.state = State::Failed;
                return Command::default();
            }
            Some(robot) => {
                robot
            }
        };

        if self.state == State::Failed || self.state == State::Done {
            return Command::default();
        }

        let mut cmd = Command::default();

        let mut angl_ok = false;
        let mut xy_ok = false;

        let dx = self.through.x - robot.pose.position.x;
        let dy = self.through.y - robot.pose.position.y;
        let distance = (self.dst - robot.pose.position).norm();
        let distance_through = (self.through - robot.pose.position).norm();

        match &mut self.closest_distance {
            None => {
                self.closest_distance = Some(distance_through);
                self.last_closest_distance = Some(Instant::now());
            }
            Some(closest_distance) => {
                if distance_through < *closest_distance {
                    *closest_distance = distance;
                    self.last_closest_distance = Some(Instant::now());
                }
            }
        }

        let dt = delta_angle(robot.pose.orientation, 0.0);
        if dt.abs() < self.angle_hyst {
            angl_ok = true;
            cmd.angular_velocity = 0.0;
        } else {
            cmd.angular_velocity = (dt.signum() *
                self.angle_speed.new_speed(cmd.angular_velocity.abs() as f64, dt.abs())) as f32;

            dbg!(cmd.angular_velocity);
        }

        if distance_through < self.xy_hyst * 2.0 {
            xy_ok = true;
        }

        if distance < self.xy_hyst {
            cmd.forward_velocity = 0.0;
            cmd.left_velocity= 0.0;
        }

        if !xy_ok {
            let x_cur = cmd.forward_velocity;
            let y_cur = cmd.left_velocity;
            let ns = self.xy_speed.new_speed((x_cur.powi(2) + y_cur.powi(2)).sqrt() as f64, distance);
            let target_x = dx / distance_through * ns;
            let target_y = dy / distance_through * ns;
            cmd.forward_velocity = (target_x * robot.pose.orientation.cos() + target_y * robot.pose.orientation.sin()) as f32;
            cmd.left_velocity = (-target_x * robot.pose.orientation.sin() + target_y * robot.pose.orientation.cos()) as f32;


            dbg!(cmd.forward_velocity);
            dbg!(cmd.left_velocity);
            dbg!(cmd.angular_velocity);
        }

        if angl_ok && xy_ok {
            if self.has_through {
                self.through = self.dst;
                self.closest_distance = Some((self.dst - robot.pose.position).norm());
                self.last_closest_distance = Some(Instant::now());
                self.has_through = false;
            } else {
                if self.state != State::Done {
                    println!("moving {} arrive at {} {}", self.robot_id, robot.pose.position.x, robot.pose.position.y);
                }
                cmd.forward_velocity = 0.0;
                cmd.left_velocity = 0.0;
                cmd.angular_velocity = 0.0;
                self.state = State::Done;
            }
        }

        if self.last_closest_distance.unwrap().elapsed() > Duration::from_secs(2) && !xy_ok {
            println!("MoveTo: failed to get closer to destination point:{} => {} {}?{} arrived: {}",
                     robot.id, self.dst, self.dst - robot.pose.position, self.xy_hyst, xy_ok);
            // println!("last distance was {} at {}s", );
            println!("time elapsed is: {}s",
                     self.last_closest_distance.unwrap().elapsed().as_secs_f64());
            cmd = Command::default();
            self.state = State::Failed;
        }

        cmd
    }
}

//
// /// The `MoveTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.
// #[derive(Clone)]
// pub struct MoveTo {
//     /// The current state of the action.
//     state: State,
//     /// The target position to move to.
//     target: Point2<f64>,
//     /// The target orientation of the robot.
//     orientation: f64,
// }
//
// impl From<&mut MoveTo> for MoveTo {
//     fn from(other: &mut MoveTo) -> MoveTo {
//         MoveTo {
//             state: other.state,
//             target: other.target,
//             orientation: other.orientation,
//         }
//     }
// }
//
// impl MoveTo {
//     /// Creates a new `MoveTo` instance.
//     ///
//     /// # Arguments
//     ///
//     /// * `target`: The target position on the field to move the robot to.
//     /// * `orientation`: The target orientation of the robot.
//     pub fn new(target: Point2<f64>, orientation: f64) -> Self {
//         Self {
//             state: State::Running,
//             target,
//             orientation,
//         }
//     }
// }
//
// fn frame(x: f64, y: f64, orientation: f64) -> Isometry2<f64> {
//     Isometry2::new(Vector2::new(x, y), orientation)
// }
//
// fn frame_inv(frame: Isometry2<f64>) -> Isometry2<f64> {
//     frame.inverse()
// }
//
// fn robot_frame(robot: &Robot<AllyInfo>) -> Isometry2<f64> {
//     frame(
//         robot.pose.position.x,
//         robot.pose.position.y,
//         robot.pose.orientation,
//     )
// }
//
// fn angle_wrap(alpha: f64) -> f64 {
//     (alpha + PI) % (2.0 * PI) - PI
// }
//
// /// The default factor speed for the robot to move towards the target position.
// const GOTO_SPEED: f64 = 1.5;
// /// The default factor speed for the robot to rotate towards the target orientation.
// const GOTO_ROTATION: f64 = 1.5;
// /// The error tolerance for arriving at the target position.
// const ERR_TOLERANCE: f64 = 0.115;
//
// impl Action for MoveTo {
//     /// Returns the name of the action.
//     fn name(&self) -> String {
//         String::from("MoveTo")
//     }
//
//     /// Returns the state of the action.
//     fn state(&mut self) -> State {
//         self.state
//     }
//
//     /// Computes the orders to be sent to the robot and returns a `Command` instance.
//     /// If the robot arrives at the target position and orientation, the action is considered done.
//     ///
//     /// # Arguments
//     ///
//     /// * `id`: The id of the robot for which the orders are computed.
//     /// * `world`: The current state of the world.
//     /// * `tools`: A collection of external tools used by the action, such as a viewer.
//     fn compute_order(&mut self, id: u8, world: &World, _tools: &mut ToolData) -> Command {
//         if let Some(robot) = world.allies_bot.get(&id) {
//             let ti = frame_inv(robot_frame(robot));
//             let target_in_robot = ti * Point2::new(self.target.x, self.target.y);
//
//             let error_orientation = angle_wrap(self.orientation - robot.pose.orientation);
//             let error_x = target_in_robot[0];
//             let error_y = target_in_robot[1];
//             let arrived = Vector3::new(error_x, error_y, error_orientation).norm() < ERR_TOLERANCE;
//             if arrived {
//                 self.state = State::Done;
//             }
//
//             let order = Vector3::new(
//                 GOTO_SPEED * error_x,
//                 GOTO_SPEED * error_y,
//                 GOTO_ROTATION * error_orientation,
//             );
//
//             Command {
//                 forward_velocity: order.x as f32,
//                 left_velocity: order.y as f32,
//                 angular_velocity: order.z as f32,
//                 charge: false,
//                 kick: None,
//                 dribbler: 0.0,
//             }
//         } else {
//             Command::default()
//         }
//     }
// }

pub struct RampSpeed {
    min_speed_: f64,
    acceleration_factor_: f64,
    deceleration_factor_: f64,
    max_speed_: f64,
    last_computation_time_: Instant,
}

impl RampSpeed {
    pub fn new(min_speed: f64, acceleration_factor: f64, deceleration_factor: f64, max_speed: f64) -> RampSpeed {
        RampSpeed {
            min_speed_: min_speed,
            acceleration_factor_: acceleration_factor,
            deceleration_factor_: deceleration_factor,
            max_speed_: max_speed,
            last_computation_time_: Instant::now(),
        }
    }

    pub fn update(&mut self, min_speed: f64, acceleration_factor: f64, deceleration_factor: f64, max_speed: f64) {
        self.min_speed_ = min_speed;
        self.acceleration_factor_ = acceleration_factor;
        self.deceleration_factor_ = deceleration_factor;
        self.max_speed_ = max_speed;
    }

    pub fn new_speed(&mut self, current_speed: f64, target_distance: f64) -> f64 {
        let time_elapsed = (Instant::now() - self.last_computation_time_).as_nanos() as f64;
        self.last_computation_time_ = Instant::now();
        let acceleration = current_speed * self.acceleration_factor_ * time_elapsed;
        let deceleration = current_speed * self.deceleration_factor_ * time_elapsed;
        let target_speed = 2.0 * target_distance;
        let mut new_speed = current_speed;
        if target_speed < new_speed {
            new_speed -= deceleration;
            if target_speed > new_speed {
                new_speed = target_speed;
            }
        } else {
            new_speed += acceleration;
            if target_speed < new_speed {
                new_speed = target_speed;
            }
        }
        new_speed = new_speed.max(self.min_speed_).min(self.max_speed_);
        return new_speed * 100.0;
    }
}
