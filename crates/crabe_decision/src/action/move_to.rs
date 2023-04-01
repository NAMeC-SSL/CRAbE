mod discrete_field;

use std::cmp::{max, Ordering};
use crabe_math::shape::{Circle, Line, Rectangle};
use std::collections::{HashMap, HashSet};
use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{distance, Isometry2, Point, Point2, Vector2, Vector3};
use std::f64::consts::PI;
use std::io::Write;
use std::ops::Index;
use std::time::{Duration, Instant};
use log::{debug, error, info};
use discrete_field::{DiscreteField, CellData};


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
    through: Point2<f64>,
    angle: f64,
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
    pub fn new(through: Option<Point2<f64>>, dst: Point2<f64>, angle: f64, how: How) -> MoveTo {
        let mut moveto = MoveTo {
            dst,
            angle,
            through: through.unwrap_or(dst),
            xy_speed: RampSpeed::new(0.0, 0.0, 0.0, 0.0),
            angle_speed: RampSpeed::new(0.0, 0.0, 0.0, 0.0),
            xy_hyst: 0.0,
            angle_hyst: 0.0,
            state: State::Running,
            closest_distance: None,
            last_closest_distance: None,
            has_through: through.is_some(),
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

    pub fn update_target(&mut self, dst: Point2<f64>) {
        self.state = State::Running;
        self.dst = dst;
        self.closest_distance = None;
        self.last_closest_distance = None;
        self.has_through = false;
        self.through = dst;
    }

    pub fn update_through(&mut self, through: Point2<f64>) {
        self.state = State::Running;
        self.through = through;
        self.has_through = true;
        self.closest_distance = None;
        self.last_closest_distance = None;
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

        let dt = delta_angle(robot.pose.orientation, self.angle);
        if dt.abs() < self.angle_hyst {
            angl_ok = true;
            cmd.angular_velocity = 0.0;
        } else {
            cmd.angular_velocity = (dt.signum() *
                self.angle_speed.new_speed(cmd.angular_velocity.abs() as f64, dt.abs())) as f32;
        }

        if distance_through < self.xy_hyst * 2.0 {
            xy_ok = true;
        }

        if distance < self.xy_hyst {
            cmd.forward_velocity = 0.0;
            cmd.left_velocity = 0.0;
        }

        if !xy_ok {
            let world_speed = (robot.velocity.linear.x.powi(2) + robot.velocity.linear.y.powi(2)).sqrt();
            let ns = self.xy_speed.new_speed(world_speed, distance);
            let target_x = dx / distance_through * ns;
            let target_y = dy / distance_through * ns;
            cmd.forward_velocity = (target_x * robot.pose.orientation.cos() + target_y * robot.pose.orientation.sin()) as f32;
            cmd.left_velocity = (-target_x * robot.pose.orientation.sin() + target_y * robot.pose.orientation.cos()) as f32;
        }

        if angl_ok && xy_ok {
            if self.has_through {
                self.through = self.dst;
                self.closest_distance = Some((self.dst - robot.pose.position).norm());
                self.last_closest_distance = Some(Instant::now());
                self.has_through = false;
            } else {
                if self.state != State::Done {
                    println!("moving {} arrive at {} {}", id, robot.pose.position.x, robot.pose.position.y);
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

        if cmd.forward_velocity.is_nan() || cmd.left_velocity.is_nan() || cmd.angular_velocity.is_nan() {
            error!("nan in command: {:#?}", cmd);
            return Command::default();
        }

        cmd
    }
}


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

    pub fn new_speed(&mut self, mut current_speed: f64, target_distance: f64) -> f64 {
        let mut dt = (Instant::now() - self.last_computation_time_).as_secs_f64();
        self.last_computation_time_ = Instant::now();

        // TODO: fix this
        // if dbg!(dt) > 0.5 {
        //     dt = 0.5;
        // }

        if current_speed < self.min_speed_ {
            current_speed = self.min_speed_;
        }

        let delta_pos = current_speed * (1.0 / self.deceleration_factor_);  // max distance if we slow now
        let mut new_speed = current_speed;
        if (target_distance - delta_pos) > 0.0  // we can accelerate
        {                                       // acceleration
            new_speed = current_speed + current_speed * self.acceleration_factor_ * dt;
        } else {  // decelaration
            new_speed = current_speed - current_speed * self.deceleration_factor_ * dt;
        }
        if new_speed < 0 as f64 {
            new_speed = 0.0;
        }
        if new_speed > self.max_speed_ {
            new_speed = self.max_speed_;
        }
        return new_speed;
    }
}


pub struct MoveToStar {
    subcommand: MoveTo,
    how: How,
    dst: Point2<f64>,
    internal_state: State,
    res: f64,
    field: DiscreteField<CellData>,
}

impl MoveToStar {
    pub fn new(dst: Point2<f64>, how: How, field_length: f64, field_width: f64) -> MoveToStar {
        let res = 0.4;

        Self {
            subcommand: MoveTo::new(None, dst, 0.0, How::Accurate),
            how,
            dst,
            internal_state: State::Running,
            res,
            field: DiscreteField::new(res, 9.0, 6.0),
        }
    }
}

use std::collections::BinaryHeap;
use std::thread::sleep;
use std::usize;

fn reconstruct_path(field: &Vec<Vec<CellData>>, end: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    let mut current = end;

    while let Some(parent) = field[current.0][current.1].parent {
        path.push(current);
        current = parent;
    }

    path.push(current);
    path.reverse();
    path
}

#[derive(PartialEq)]
struct Node {
    position: (usize, usize),
    f_score: f64,
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.partial_cmp(&other.f_score).unwrap_or(Ordering::Equal).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star_search(field: &mut Vec<Vec<CellData>>, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();

    field[start.0][start.1].g_score = field[start.0][start.1].cost;
    open_set.push(Node { position: start, f_score: heuristic(start, end) });

    while let Some(current) = open_set.pop() {
        let current_pos = current.position;

        if current_pos == end {
            return Some(reconstruct_path(field, end));
        }

        field[current_pos.0][current_pos.1].visited = true;

        for neighbor_pos in neighbors(current_pos, field) {
            let neighbor = &field[neighbor_pos.0][neighbor_pos.1];

            if neighbor.visited {
                continue;
            }

            let tentative_g_score = &field[current_pos.0][current_pos.1].g_score + neighbor.cost;

            if tentative_g_score < neighbor.g_score {
                let neighbor = &mut field[neighbor_pos.0][neighbor_pos.1];
                neighbor.parent = Some(current_pos);
                neighbor.g_score = tentative_g_score;
                open_set.push(Node { position: neighbor_pos, f_score: tentative_g_score + heuristic(neighbor_pos, end) });
            }
        }
    }

    None
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> f64 {
    let dx = (a.0 as isize - b.0 as isize).abs();
    let dy = (a.1 as isize - b.1 as isize).abs();
    (dx + dy) as f64
}

fn neighbors(pos: (usize, usize), field: &Vec<Vec<CellData>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dx, dy) in directions.iter() {
        let new_x = (pos.0 as isize + dx) as usize;
        let new_y = (pos.1 as isize + dy) as usize;

        if new_x < field.len() && new_y < field[0].len() {
            result.push((new_x, new_y));
        }
    }

    result
}


#[cfg(test)]
mod test {
    use crate::action::move_to::a_star_search;
    use crate::action::move_to::discrete_field::CellData;

    #[test]
    fn test_astar() {
        let mut field = vec![
            vec![CellData::default(), CellData::default(), CellData::default()],
            vec![CellData::default(), CellData::default(), CellData::default()],
            vec![CellData::default(), CellData::default(), CellData::default()],
        ];
        let start = (0, 0);
        let end = (2, 2);

        let path = a_star_search(&mut field, start, end);

        assert!(path.is_some());
        dbg!(path);
    }
}

impl Action for MoveToStar {
    fn name(&self) -> String {
        "MoveToStar".to_string()
    }

    fn state(&mut self) -> State {
        self.internal_state
    }

    fn compute_order(&mut self, id: u8, world: &World, tools: &mut ToolData) -> Command {
        sleep(Duration::from_millis(1));
        if self.internal_state != State::Running {
            return Command::default();
        }

        let robot = match world.allies_bot.get(&id) {
            None => {
                return Command::default();
            }
            Some(r) => r
        };

        if robot.distance(&self.dst) < self.res * 2.0 {
            self.subcommand.update_how(How::Accurate);
            // self.subcommand.update_target(self.dst);
            if self.subcommand.state() == State::Done {
                self.internal_state = State::Done;
            }
            if self.subcommand.state() == State::Failed {
                self.internal_state = State::Failed;
            }
            return self.subcommand.compute_order(id, world, tools);
        }

        // reset field values to default
        self.field.apply(|c| {
            c.g_score = 0.0;
            c.cost = 0.0;
            c.visited = false;
        });

        let mut cell_positions = Vec::new();

        for (row_nb, row) in self.field.data.iter().enumerate() {
            for (col_nb, _cell) in row.iter().enumerate() {
                let cell_pos = self.field.idxs_to_coords(row_nb as i32, col_nb as i32);
                cell_positions.push((row_nb, col_nb, cell_pos));
            }
        }

        for (row_nb, col_nb, cell_pos) in cell_positions {
            // Add the cages as a zone with high weight
            if cell_pos.y >= -0.5 && cell_pos.y <= 0.5 && cell_pos.x <= -4.5 && cell_pos.x >= -4.7 {
                let cell = &mut self.field.data[row_nb][col_nb];
                cell.cost = cell.cost.max(10.0);
            }
            if cell_pos.y >= -0.5 && cell_pos.y <= 0.5 && cell_pos.x >= 4.5 && cell_pos.x <= 4.7 {
                let cell = &mut self.field.data[row_nb][col_nb];
                cell.cost = cell.cost.max(10.0);
            }

            for (_, r) in world.allies_bot.iter().filter(|(_id, _)| **_id != id) {
                if r.velocity.linear.norm() > 0.5 {
                    // let d1 = r.distance(&cell_pos);
                    // let mut time = Duration::from_nanos((d1 * 10.0f64.powi(9)) as u64); // sumimasen wat the fuck
                    // if time > Duration::from_secs(1) {
                    //     time = Duration::from_secs(1);
                    // }
                    // let f2: Point2<f64> = r.position_in(time);
                    // let d2 = distance(&f2, &cell_pos);
                    // let e = r.distance(&f2);
                    // let v = (d1 + d2) * (d1 + d2);
                    // let mut t = 10.0;
                    // if d1 > self.res {
                    //     t = 10.0 / (0.9 * v); // 0.9 is totally arbitrary (TODO base the
                    //     // coefficient on non-empirical data)
                    //     // It is remarkable that the weight of the enemies is higher than
                    //     // the weight of the allies. ie a robot is more inclined to pass
                    //     // close to its allies (at the risk of touching it) than to pass
                    //     // close to the enemies (because the collision can cause a foul)
                    // }
                    // let cell = &mut self.field.data[row_nb][col_nb];
                    // cell.weight = cell.weight.max(t);
                } else {
                    let d = r.distance(&cell_pos);
                    let mut t = 10.0;
                    if d > self.res {
                        t = 3.0 / (d / self.res);
                    }
                    let cell = &mut self.field.data[row_nb][col_nb];
                    cell.cost = cell.cost.max(t);
                }
            }

            // for (_, r) in world.enemies_bot.iter() {
            //     if r.velocity.linear.norm() > 0.5 {
            //         let d1 = r.distance(&cell_pos);
            //         let mut time = Duration::from_nanos((d1 * 10.0f64.powi(9)) as u64); // sumimasen wat the fuck
            //         if time > Duration::from_secs(1) {
            //             time = Duration::from_secs(1);
            //         }
            //         let f2: Point2<f64> = r.position_in(time);
            //         let d2 = distance(&f2, &cell_pos);
            //         let e = r.distance(&f2);
            //         let v = (d1 + d2) * (d1 + d2);
            //         let mut t = 10.0;
            //         if d1 > self.res {
            //             t = 10.0 / (0.5 * v); // 0.6 is totaly arbitrary (TODO base the coefficient on
            //             // non-empirical data)
            //         }
            //         let cell = &mut self.field.data[row_nb][col_nb];
            //         cell.weight = cell.weight.max(t);
            //     } else {
            //         let d = r.distance(&cell_pos);
            //         let mut t = 10.0;
            //         if d > self.res {
            //             t = 10.0 / (d / self.res);
            //         }
            //         let cell = &mut self.field.data[row_nb][col_nb];
            //         cell.weight = cell.weight.max(t);
            //     }
            // }
        }

        // self.field.print();


        let src = self.field.coords_to_idxs(&robot.pose.position);
        let dst = self.field.coords_to_idxs(&self.dst);

        let path = a_star_search(&mut self.field.data, src, dst);

        match path {
            Some(p) => {
                self.field.print_with_path(&p[1..2].to_vec());
                let path: Vec<Point2<f64>> = p.into_iter().map(|(x, y)| self.field.idxs_to_coords(x as i32, y as i32)).collect();
                println!("Path found: {:?}", path);
                println!("adding point");
                tools.annotations.remove("next");
                // tools.annotations.add_point("next".to_string(), dbg!(path[1]));
                tools.annotations.add_circle("next".to_string(), Circle {
                    center: path[1],
                    radius: 0.5
                });

                return Command::default();
                self.subcommand.update_through(dbg!(path[1]));
                return dbg!(self.subcommand.compute_order(id, world, tools));
            }
            None => {
                println!("No path found");
            }
        }

        self.internal_state = State::Failed;

        Command::default()
    }
}

fn steal_min_from_vec<T: Ord>(v: &mut Vec<T>) -> T {
    let mut min_i = 0;
    for i in 1..v.len() {
        if v[i] < v[min_i] {
            min_i = i;
        }
    }
    v.remove(min_i)
}

fn steal_min_from_vec_by<T>(v: &mut Vec<T>, f: fn(&T, &T) -> Ordering) -> T {
    let mut min_i = 0;
    for i in 1..v.len() {
        if f(&v[i], &v[min_i]) == Ordering::Less {
            min_i = i;
        }
    }
    v.remove(min_i)
}
