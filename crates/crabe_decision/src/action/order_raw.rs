use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

/// The `RawOrder` struct represents an action that sends only one time a raw command to be executed by a robot.
#[derive(Clone)]
pub struct RawOrder {
    state: State,
    command: Command,
}

impl From<&mut RawOrder> for RawOrder {
    fn from(other: &mut RawOrder) -> RawOrder {
        RawOrder {
            state: other.state,
            command: other.command,
        }
    }
}

impl RawOrder {
    /// Creates a new `RawOrder` instance with the specified command.
    pub fn new(command: Command) -> Self {
        Self {
            state: State::Running,
            command,
        }
    }
}

impl Action for RawOrder {
    /// Returns the name of the action.
    fn name(&self) -> String {
        String::from("RawOrder")
    }

    /// Returns the current state of the action.
    fn state(&mut self) -> State {
        self.state
    }

    /// Computes the command to be executed and updates the state of the action to "Done".
    fn compute_order(&mut self, _id: u8, _world: &mut World, _tools: &mut ToolData) -> Command {
        self.state = State::Done;
        self.command
    }
}
