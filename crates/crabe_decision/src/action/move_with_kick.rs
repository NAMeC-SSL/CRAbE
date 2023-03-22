use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use crabe_framework::data::output::Kick;
use nalgebra::{Isometry2, Point2, Vector2, Vector3};
use std::f64::consts::PI;
use crate::action::move_to::MoveTo;

/// The `MoveTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.
#[derive(Clone)]
pub struct MoveToWithKick<SK: Fn(&World, u8, &ToolData) -> Option<Kick>> {
    /// The current state of the action.
    pub state: State,
    pub should_kick: SK,
    pub move_to: MoveTo
}

impl<SK: Fn(&World, u8, &ToolData) -> Option<Kick> + Clone> From<&mut MoveToWithKick<SK>> for MoveToWithKick<SK> {
    fn from(other: &mut MoveToWithKick<SK>) -> MoveToWithKick<SK> {
        MoveToWithKick {
            state: other.state.clone(),
            should_kick: other.should_kick.clone(),
            move_to: other.move_to.clone()
        }
    }
}

impl<SK: Fn(&World, u8, &ToolData) -> Option<Kick>> MoveToWithKick<SK> {
    pub fn new(target: Point2<f64>, orientation: f64, should_kick: SK) -> Self {
        Self {
            state: State::Running,
            move_to: MoveTo::new(target, orientation),
            should_kick
        }
    }
}

impl<SK: Fn(&World, u8, &ToolData) -> Option<Kick>> Action for MoveToWithKick<SK> {
    fn name(&self) -> String {
        String::from("MoveToWithKick")
    }

    fn state(&mut self) -> State {
        self.state
    }

    fn compute_order(&mut self, id: u8, world: &World, tools: &mut ToolData) -> Command {
        let mut command = self.move_to.compute_order(id, world, tools);
        command.kick = (self.should_kick)(world, id, tools);
        command
    }
}