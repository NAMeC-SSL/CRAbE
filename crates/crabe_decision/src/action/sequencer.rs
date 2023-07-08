use crate::action::state::State;
use crate::action::{Action, Actions};
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

/// The Sequencer struct represents a sequence of actions to be executed by a robot.
/// It implements methods to add and remove actions from the sequence, compute the orders
/// to be sent to the robot and cancel the sequence.
pub struct Sequencer {
    state: State,
    pub actions: Vec<Actions>,
}

impl Default for Sequencer {
    fn default() -> Self {
        Self {
            state: State::Running,
            actions: vec![],
        }
    }
}

impl Sequencer {
    /// Creates a new Sequencer instance with a single action.
    pub fn new(action: Actions) -> Self {
        Self {
            state: State::Running,
            actions: vec![action],
        }
    }

    /// Adds an action to the end of the sequence.
    pub fn push(&mut self, action: Actions) {
        self.actions.push(action);
        self.state = match self.state {
            State::Running | State::Done => State::Running,
            State::Failed => State::Failed,
        };
    }

    /// Removes all the actions from the sequence and cancels their execution.
    pub fn clear(&mut self) {
        self.actions.iter_mut().for_each(|a| a.cancel());
        self.actions.clear();
    }

    /// Returns the name of the sequence, which is a concatenation of the names of its actions.
    pub fn name(&self) -> String {
        self.actions
            .iter()
            .map(|action| action.name())
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Computes the orders to be sent to the robot and returns a `Command` instance.
    /// If an action is done, it will automatically remove it from the sequence.
    ///
    /// # Arguments
    ///
    /// * `id`: The id of the robot for which the orders are computed.
    /// * `world`: The current state of the world.
    /// * `tools`: A collection of external tools used by the action, such as a viewer.
    pub fn compute_order(&mut self, id: u8, world: &mut World, tools: &mut ToolData) -> Command {
        if self.state == State::Failed || self.actions.is_empty() {
            return Command::default();
        }

        let mut iter = self.actions.iter_mut();

        if let Some(action) = iter.next() {
            match action.state() {
                State::Failed => {
                    self.state = State::Failed;
                    return Command::default();
                }
                State::Done => {
                    self.actions.remove(0);
                }
                _ => {}
            }
        }

        if let Some(action) = self.actions.iter_mut().next() {
            action.compute_order(id, world, tools)
        } else {
            self.state = State::Done;
            Command::default()
        }
    }
}
