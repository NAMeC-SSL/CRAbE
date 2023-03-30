mod discrete_field;

use std::cmp::{max, Ordering};
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
use log::{error, info};
use discrete_field::{DiscreteField, Cursor, CellData};


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

        let dt = delta_angle(robot.pose.orientation, 0.0);
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
            cmd.left_velocity= 0.0;
        }

        if !xy_ok {
            let world_speed = (robot.velocity.linear.x.powi(2) + robot.velocity.linear.y.powi(2)).sqrt();
            dbg!(world_speed);
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

        if cmd.forward_velocity.is_nan() || cmd.left_velocity.is_nan() || cmd.angular_velocity.is_nan() {
            error!("nan in command");
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
        }
        else
        {  // decelaration
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
    robot_id: u8,
    dst: Point2<f64>,
    internal_state: State,
    res: f64,
    field: DiscreteField<CellData>
}

impl MoveToStar {
    pub fn new(robot_id: u8, dst: Point2<f64>, how: How, field_length: f64, field_width: f64) -> MoveToStar {
        let res = 0.2;

        Self {
            subcommand: MoveTo::new(robot_id, None, dst, How::Accurate),
            how,
            robot_id,
            dst,
            internal_state: State::Running,
            res,
            field: DiscreteField::new(res, 9.0, 6.0),
        }
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
        if self.internal_state != State::Running {
            return Command::default()
        }

        let robot = match world.allies_bot.get(&id) {
            None => {
                return Command::default()
            }
            Some(r) => r
        };

        if robot.distance(&self.dst) < self.res * 2.0 {
            self.subcommand.update_how(How::Accurate);
            self.subcommand.update_target(self.dst);
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
            c.weight = 0.0;
            c.visited = false;
        });

        let mut c = self.field.start();
        while let Some(ref mut cd) = c.next() {
            // Add the cages as a zone with high weight
            if c.pos.y >= -0.5 && c.pos.y <= 0.5 && c.pos.x <= -4.5 && c.pos.x >= -4.7
            {
                cd.weight = cd.weight.max(10.0);
            }
            if c.pos.y >= -0.5 && c.pos.y <= 0.5 && c.pos.x >= 4.5 && c.pos.x <= 4.7
            {
                cd.weight = cd.weight.max( 10.0);
            }

            for (_, r) in world.allies_bot.iter().filter(|(id, _)| **id != self.robot_id) {
                if r.velocity.linear.norm() > 0.5 {
                    let d1 = r.distance(&c.pos);
                    let mut time = Duration::from_nanos((d1 * 10.0f64.powi(9)) as u64); // sumimasen wat the fuck
                    if time > Duration::from_secs(1) {
                        time = Duration::from_secs(1);
                    }
                    let f2: Point2<f64> = r.position_in(time);
                    let d2 = distance(&f2, &c.pos);
                    let e = r.distance(&f2);
                    let v = (d1 + d2) * (d1 + d2);
                    let mut t = 10.0;
                    if d1 > self.res {
                        t = 10.0 / (0.9 * v); // 0.9 is totally arbitrary (TODO base the
                        // coefficient on non-empirical data)
                        // It is remarkable that the weight of the enemies is higher than
                        // the weight of the allies. ie a robot is more inclined to pass
                        // close to its allies (at the risk of touching it) than to pass
                        // close to the enemies (because the collision can cause a foul)
                    }
                    cd.weight = cd.weight.max(t);
                } else {
                    let d = r.distance(&c.pos);
                    let mut t = 10.0;
                    if d > self.res {
                        t = 10.0 / (d / self.res);
                    }
                    cd.weight = cd.weight.max(t);

                    dbg!(cd);
                }
            }

            for (_, r) in world.enemies_bot.iter().filter(|(id, _)| **id != self.robot_id) {
                if r.velocity.linear.norm() > 0.5 {
                    let d1 = r.distance(&c.pos);
                    let mut time = Duration::from_nanos((d1 * 10.0f64.powi(9)) as u64); // sumimasen wat the fuck
                    if time > Duration::from_secs(1) {
                        time = Duration::from_secs(1);
                    }
                    let f2: Point2<f64> = r.position_in(time);
                    let d2 = distance(&f2, &c.pos);
                    let e = r.distance(&f2);
                    let v = (d1 + d2) * (d1 + d2);
                    let mut t = 10.0;
                    if d1 > self.res {
                        t = 10.0 / (0.5 * v); // 0.6 is totaly arbitrary (TODO base the coefficient on
                        // non-empirical data)
                    }
                    cd.weight = cd.weight.max(t);
                } else {
                    let d = r.distance(&c.pos);
                    let mut t = 10.0;
                    if d > self.res {
                        t = 10.0 / (d / self.res);
                    }
                    cd.weight = cd.weight.max(t);
                }
            }

            // debug A* weight, eog -f test_astar.ppm to see it
            if self.robot_id == 0 {
                self.field.print();
            }

            let mut open_set: Vec<(f64, Cursor)> = Vec::new();
            let dir = self.field.start().around();
            let mut start = self.field.start_from(robot.pose.position.x, robot.pose.position.y);
            start.get().g_score = 0.0;
            open_set.push((0.0, start));

            let mut from: HashMap<Cursor, Cursor> = HashMap::new();
            let alpha = 0.8;
            let beta = 1.0 - alpha;
            while open_set.len() > 0 {
                let i = steal_min_from_vec_by(&mut open_set, |(cost1, _), (cost2, _)| cost1.total_cmp(cost2));
                let mut current = &i.1;
                if distance(&self.dst, &current.pos) < self.res {
                    // we arrive
                    // compute path with backtracking
                    // std::cout << "compute backtrace" << std::endl;
                    let mut path = Vec::new();
                    while from.iter().last().is_none() || *from.get(&current).unwrap() != *from.iter().last().unwrap().1 {
                        path.push(current.clone());
                        current = from.get(&current).unwrap();
                    }

                    if path.len() == 0 {
                        error!("path has length 0");
                        return Command::default();
                    } else {
                        info!("it's working hehe");
                    }

                    let mut current_dir = path.pop().unwrap().pos;
                    if path.len() > 0 {
                        let next = path.last().unwrap().pos;
                        current_dir = current_dir + (next - current_dir) / 2.0;
                    }

                    self.subcommand.update_through(current_dir);

                    return self.subcommand.compute_order(id, world, tools);
                }

                for x in current.around() {
                    let g = current.get().g_score + distance(&current.pos, &x.pos);
                    if g < x.get().g_score {
                        from.insert(x.clone(), current.clone());
                        x.get().g_score = g;
                        let f = g + distance(&self.dst, &x.pos) * alpha + x.get().weight * beta;
                        open_set.push((f, x));
                    }
                }
            }
        }
        self.internal_state = State::Failed;

        Command::default()
    }
}

fn steal_min_from_vec<T:Ord>(v: &mut Vec<T>) -> T {
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
