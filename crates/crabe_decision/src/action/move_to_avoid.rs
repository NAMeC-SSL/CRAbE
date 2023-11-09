use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{Isometry2, Point2, Vector2, Vector3};
use std::f64::consts::PI;

/// The `MoveTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.

struct Grid<T> {
    dimensions: Point2<i32>,
    cells: Vec<T>
}

impl Grid<bool> {

    pub fn new(width: i32, height: i32) -> Self {
        Self {
            dimensions: Point2::new(width, height),
            cells: vec![false; (width * height) as usize],
        }
    }
    pub fn get(&mut self, x: i32, y: i32) -> bool {
        return self.cells[(x + (y * self.dimensions.x)) as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, value: bool) {
        self.cells[(x + (y * self.dimensions.x)) as usize] = value
    }

    pub fn print_grid(&mut self){
        print!("{}", self.dimensions.x);
        for i in 0..self.dimensions.x {
            for j in 0..self.dimensions.y {
                print!("{:?}. ", self.get(i, j))
            }
            print!("\n")
        }
    }
}

struct PathGrid {
    resolution_per_meter: f64,
    world: World,
    occupancy: Grid<bool>,
    // weights: Grid<f64>,
}

impl PathGrid {
    /// Resolution_per_meter is the amount of cell per meter.
    /// The more cells, the more accuracy of movement, the more intense to compute
    /// 5 or so should be minimal to get good results
    pub fn new(resolution_per_meter: f64, world: World) -> Self {
        let height: i32 = (world.geometry.field.length * resolution_per_meter) as i32;
        let width: i32 = (world.geometry.field.width * resolution_per_meter) as i32;
        Self {
            resolution_per_meter,
            occupancy: Grid::new(height, width),
            // weights: Grid::new(height, width),
            world,
        }
    }

    fn position_to_grid(&mut self, position: Point2<f64>) -> Point2<i32>{
        Point2::new((self.resolution_per_meter * position.x) as i32,
                    (self.resolution_per_meter * position.y) as i32)
    }

    // size is in meter
    fn occupy_cells(&mut self, position: &Point2<f64>, size_in_meter: Point2<f64>){
        let grid_pos = self.position_to_grid(*position);
        let grid_size = self.position_to_grid(size_in_meter);
        for x in grid_pos.x..grid_pos.x + grid_size.x {
            for y in grid_pos.y .. grid_pos.y + grid_size.y {
                self.occupancy.set(x , y, true);
            }
        }
    }

    fn fill_occupancy(&mut self) {
        // Approximative robot size (0.4, 0.4)
        let robot_size = Point2::new(0.4f64, 0.4f64);
        let enemies_bot_copy = self.world.enemies_bot.clone(); // extrait la référence
        for robot in enemies_bot_copy.iter() {
            self.occupy_cells(&robot.1.pose.position, robot_size);
        }
    }
}
// impl From<&mut MoveToAvoid> for MoveToAvoid {
//     fn from(other: &mut MoveToAvoid) -> MoveToAvoid {
//         MoveToAvoid {
//             state: other.state,
//             target: other.target,
//             orientation: other.orientation,
//             occupancy: vec![],
//
//         }
//     }
// }

pub struct MoveToAvoid {
    /// The current state of the action.
    state: State,
    /// The target position to move to.
    target: Point2<f64>,
    /// The target orientation of the robot.
    orientation: f64,

    path_grid: PathGrid,

}

impl MoveToAvoid {
    /// Creates a new `MoveTo` instance.
    ///
    /// # Arguments
    ///
    /// * `target`: The target position on the field to move the robot to.
    /// * `orientation`: The target orientation of the robot.
    pub fn new(target: Point2<f64>, orientation: f64, world: World) -> Self {
        Self {
            state: State::Running,
            target,
            orientation,
            path_grid: PathGrid::new(10f64, world)
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

impl Action for MoveToAvoid {
    /// Returns the name of the action.
    fn name(&self) -> String {
        String::from("MoveToAvoid")
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
        // print!("BEFORE FILLING\n");
        // self.path_grid.occupancy.print_grid();
        // self.path_grid.fill_occupancy();
        // print!("AFTER FILLING\n");
        // self.path_grid.occupancy.print_grid();

        if let Some(robot) = world.allies_bot.get(&id) {

            let ti = frame_inv(robot_frame(robot));
            let target_in_robot = ti * Point2::new(self.target.x, self.target.y);

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

            Command {
                forward_velocity: order.x as f32,
                left_velocity: order.y as f32,
                angular_velocity: order.z as f32,
                charge: false,
                kick: None,
                dribbler: 0.0,
            }
        } else {
            Command::default()
        }
    }
}
