use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{Isometry2, Point2, Vector2, Vector3};
use std::f64::consts::PI;
use std::ops::Deref;
use std::sync::Arc;
use crate::action::move_to::MoveTo;

/// The `MoveTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.
// #[derive(Clone)]
pub struct MoveToWithParams {
    pub move_to: MoveTo,
    pub should_kick: Option<Kick>,
    pub should_dribble: f32
}

impl MoveToWithParams {
    pub fn new(target: Point2<f64>, orientation: f64, should_kick:  Option<Kick>, should_dribble: f32) -> Self
    {
        Self {
            move_to: MoveTo::new(target, orientation),
            should_kick,
            should_dribble,
        }
    }
}

impl Action for MoveToWithParams {
    /// Returns the name of the action.
    fn name(&self) -> String {
        String::from("MoveToWithParams")
    }

    /// Returns the state of the action.
    fn state(&mut self) -> State {
        self.move_to.state()
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
        let mut cmd= self.move_to.compute_order(id, world, _tools);
        cmd.kick = self.should_kick;
        cmd.dribbler = self.should_dribble;
        cmd
    }
}
