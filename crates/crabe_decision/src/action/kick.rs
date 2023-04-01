use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::{Command, Kick as KickType};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{Isometry2, Point2, Vector2, Vector3};



/// The `MoveTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.
// #[derive(Clone)]
pub struct Kick {
    kick_type: KickType,
    state: State
}

impl Kick {
    pub fn new(kick_type: KickType) -> Self
    {
        Self {
            state: State::Running,
            kick_type
        }
    }
}

impl Action for Kick {
    /// Returns the name of the action.
    fn name(&self) -> String {
        String::from("MoveToWithKick")
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
        self.state = State::Done;

        let mut cmd = Command::default();
        cmd.kick = Some(self.kick_type);

        cmd
    }
}
