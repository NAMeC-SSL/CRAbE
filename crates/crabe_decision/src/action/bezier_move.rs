use std::f64::consts::FRAC_PI_2;
use log::error;
use nalgebra::{distance, Isometry2, Matrix1x4, Matrix2x4, Matrix4, Matrix4x1, Point2, Vector2, Vector3};
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use crate::action::Action;
use crate::action::state::State;

/// In this file, you can search for algorithmic decisions (such as determining how to compute one of the specific
/// control points) by searching for the pattern [POC].
/// This only works as proof and is not made to be used in a competition yet.

// This cannot be allocated here, using the vec![] macro
// I'm not exactly sure why, happy to discuss it one day
// const CUBIC_BEZIER_CHARAC_MATRIX: Matrix4<f64> = Matrix4::from_vec(vec![
//     1., 0., 0., 0.,
//     -3., 3., 0., 0.,
//     3., -6., 3., 0.,
//     -1., 3., -3., 1.]
// );

/// Tolerance to attain the target
const DIST_TARGET_REACHED: f64 = 0.2;

/// Represents a Bézier curve made of 4 control points
#[derive(Clone)]
pub struct CubicBezierCurve {
    /// Vector of the 4 control points defining the cubic Bézier
    control_points: Matrix1x4<Point2<f64>>,
    /// Contains all the points that form the Bézier curve
    points_on_curve: Vec<Point2<f64>>,
}

impl CubicBezierCurve {

    /// Computes the additional control point used to generate
    /// a trajectory that will avoid the given obstacle
    /// A single obstacle is considered here
    fn avoidance_point(start: &Point2<f64>, obstacle: &Point2<f64>, end: &Point2<f64>) -> Point2<f64> {
        // obtain vector from obstacle to end
        let vec_obs_end = end - obstacle;

        // rotate this vector by 90 degrees
        // [POC] this may be clockwise or counter-clockwise, depending on the environment
        // right now it doesn't matter
        let mut nvec = Isometry2::rotation(FRAC_PI_2) * vec_obs_end;
        nvec = nvec.normalize();

        //scale this vector depending on distance from start to obstacle
        nvec = nvec * distance(&start, &obstacle).max(1.5); // very arbitrary number, for now

        // other possible test : bind scale to 1.5
        // which unit is it ? probably meters
        // nvec = nvec * 1.5;

        obstacle + nvec
    }

    fn new(start: Point2<f64>, obstacle: Point2<f64>, end: Point2<f64>) -> Self {
        let avoid_ctrl_pt = CubicBezierCurve::avoidance_point(&start, &obstacle, &end);
        Self {
            control_points: Matrix1x4::new(start, avoid_ctrl_pt, obstacle, end),
            points_on_curve: vec![],
        }
    }

    /// Converts a 1x4 matrix of Point2 into a 2x4 matrix containing the x, y coordinates of each point
    /// Required to perform dot products with matrices
    fn convert_p2_matrix(mat: &Matrix1x4<Point2<f64>>) -> Matrix2x4<f64> {
        let mut store_vec: Vec<f64> = vec![];
        mat.iter().for_each(|p| {
            // store p.x and p.y aside in a vector
            // move them into the result vector
            store_vec.append(&mut vec![p.x, p.y]);
        });

        let res_mat = Matrix2x4::<f64>::from_vec(store_vec);
        dbg!(&res_mat);
        res_mat
    }

    fn compute_points_on_curve(&mut self, num_points: i16) -> &Vec<Point2<f64>> {
        // clear previously computed points
        self.points_on_curve = vec![];

        // this is supposed to be const, see top of file
        let CUBIC_BEZIER_CHARAC_MATRIX: Matrix4<f64> = Matrix4::from_vec(vec![
            1., 0., 0., 0.,
            -3., 3., 0., 0.,
            3., -6., 3., 0.,
            -1., 3., -3., 1.]
        );

        // we need to convert the matrices of control points `Matrix4x1<Point2<f64>>`
        // into a 2 column matrix `Matrix4x2<f64>` to perform dot products
        dbg!(&self.control_points);
        let ctrl_pts_cvt = CubicBezierCurve::convert_p2_matrix(&self.control_points);

        // compute Bézier curve points using matrix form
        // it's the product of the t coefficients matrix, the Bernstein polynomials coefficients and the control points
        for t_int in 1..(num_points + 1) {
            let t = t_int as f64 / num_points as f64;
            let t_mat = Matrix4x1::from_vec([1., t, t.powf(2.), t.powf(3.)].to_vec());
            let influence_coeffs: Matrix4x1<f64> = CUBIC_BEZIER_CHARAC_MATRIX * t_mat;

            // convert matrix result back into a Point2
            let two_mat_point = ctrl_pts_cvt * influence_coeffs;
            let step_point: Point2<f64> = Point2::new(two_mat_point.x, two_mat_point.y);
            self.points_on_curve.push(step_point);
        }

        &self.points_on_curve
    }
}

/// Handles moving sequentially to multiple targets
#[derive(Clone)]
struct SteppedMovement {
    points: Vec<Point2<f64>>,
    size: usize,
    current_point_index: usize,
    state: State,
}

impl SteppedMovement {
    pub fn new(points: Vec<Point2<f64>>) -> Self {
        if points.len() == 0 { error!("Points to attain vector is empty !"); }
        Self {
            size: points.len(),
            points,
            current_point_index: 0,
            state: State::Running,
        }
    }

    /// Get the current target to go to
    fn current_point(&self) -> Option<&Point2<f64>> {
        if self.state == State::Done { return None; }
        else { return self.points.get(self.current_point_index); }
    }


    /// If one target has been attained, force robot to move to the next one
    /// When all targets have been attained, change state to State::Finished
    fn check_change_to_next_point(&mut self, robot: &Robot<AllyInfo>) {
        if let Some(current_target) = self.current_point() {
            if distance(&robot.pose.position, current_target) <= DIST_TARGET_REACHED {
                self.current_point_index += 1;
                if self.current_point_index >= self.size {
                    self.state = State::Done;
                }
            }
        }
    }

    /// Returns the movement command to move to the current target,
    /// in the vector of targets to attain sequentially
    fn movement_cmd(&mut self, robot: &Robot<AllyInfo>) -> Command {
        self.check_change_to_next_point(robot);
        let current_target_opt = self.current_point();
        match current_target_opt {
            None => { return Command::default() }  // then state is now State::Finished
            Some(current_target) => {
                let mov_vec = (current_target - robot.pose.position) * 2.;

                Command {
                    forward_velocity: mov_vec.x as f32,
                    left_velocity: mov_vec.y as f32,
                    angular_velocity: 0.,
                    ..Default::default()
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct BezierMove {
    state: State,
    target: Point2<f64>,
    orientation: f64,
    // true if initialized at least once
    initialized: bool,
    curve: Option<CubicBezierCurve>,
    move_handler: Option<SteppedMovement>,
    // [POC] hardcoded id of ally to avoid. temporary, just to test it
    hardcoded_avoid_ally_id: u8,
}

impl BezierMove {
    pub fn new(target: Point2<f64>, orientation: f64, hardcoded_avoid_ally_id: u8) -> Self {
        Self {
            state: State::Running,
            target,
            orientation,
            initialized: false,
            curve: None,
            move_handler: None,
            hardcoded_avoid_ally_id,
        }
    }

    /// Initializes the avoidance curve to follow when avoiding a certain obstacle
    /// on the field (i.e. a robot here).
    /// [POC] This is only initialized once and does not adapt to a dynamic environment.
    /// It will be adapted if the POC is considered valid
    pub fn init_curve(&mut self, robot: &Robot<AllyInfo>, world: &World) {
        let mut bcurve = CubicBezierCurve::new(
            robot.pose.position,
            // [POC] hardcoded id for POC (proof of concept)
            world.allies_bot.get(&self.hardcoded_avoid_ally_id).unwrap().pose.position,
            self.target
        );
        self.initialized = true;
        let points = bcurve.compute_points_on_curve(100);
        self.move_handler = Some(SteppedMovement::new(points.clone()));
        self.curve = Some(bcurve);
    }
}


impl Action for BezierMove {
    fn name(&self) -> String { String::from("BezierMove") }

    fn state(&mut self) -> State { self.state }

    fn compute_order(&mut self, id: u8, world: &World, tools: &mut ToolData) -> Command {
        if let Some(robot) = world.allies_bot.get(&id) {
            // [POC] this stays static for the moment, and doesn't take in account change in environment
            if !self.initialized { self.init_curve(robot, world) }
            if let Some(move_handler) = &mut self.move_handler {
                // update this action's state
                self.state = move_handler.state;
                return move_handler.movement_cmd(robot);
            }
        }

        Command::default()
    }
}

impl From<&mut BezierMove> for BezierMove {
    fn from(value: &mut BezierMove) -> BezierMove {
        BezierMove {
            state: value.state,
            target: value.target,
            orientation: value.orientation,
            initialized: value.initialized,
            curve: value.curve.clone(),
            move_handler: value.move_handler.clone(),
            hardcoded_avoid_ally_id: value.hardcoded_avoid_ally_id
        }
    }
}