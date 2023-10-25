/// The `move_to` module contains the `MoveTo` action which moves a robot to a specific location on the field and a target orientation.
pub mod move_to;
/// The `order_raw` module contains the `RawOrder` action which sends a raw command to the robot.
pub mod order_raw;

/// The sequencer module contains the `Sequencer` struct which sequences a collection of actions to be executed.
pub mod sequencer;

/// The state module contains the State enum which represents the current state of an action.
pub mod state;

use crate::action::move_to::MoveTo;
use crate::action::order_raw::RawOrder;
use crate::action::sequencer::Sequencer;
use crabe_framework::data::output::{Command, CommandMap};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use enum_dispatch::enum_dispatch;
use state::State;
use std::collections::HashMap;

/// The Action trait represents an action that can be performed by a robot, such as moving to a certain point.
#[enum_dispatch(Actions)]
pub trait Action {
    /// Returns the name of the action.
    fn name(&self) -> String;
    /// Returns the current state of the action.
    fn state(&mut self) -> State;
    /// Computes the next command to be executed by the robot.
    fn compute_order(&mut self, id: u8, world: &World, tools: &mut ToolData) -> Command;
    /// Cancel the action.
    fn cancel(&mut self) {}
}

/// The Actions enum is used to define the various actions that can be taken by a robot and implement the Action
#[enum_dispatch]
pub enum Actions {
    MoveTo(MoveTo),
    RawOrder(RawOrder),
}

/// The `ActionWrapper` struct represents a wrapper for a sequence of actions to be executed for each robot.
/// It provides methods to add an action to the sequence, clear the sequence for a specific robot, and compute
/// the sequence of actions to be executed for each robot and return a `CommandMap` containing the commands
/// to be sent to each robot.
#[derive(Default)]
pub struct ActionWrapper {
    pub actions: HashMap<u8, Sequencer>,
}

impl ActionWrapper {
    /// Adds an action to the sequence of actions to be executed for a given robot.
    ///
    /// # Arguments
    ///
    /// * id: The id of the robot to which the action will be applied.
    /// * action: The action to be added to the sequence.
    pub fn push<T: Action + Into<Actions>>(&mut self, id: u8, action: T) {
        if let Some(sequencer) = self.actions.get_mut(&id) {
            sequencer.push(action.into());
        } else {
            self.actions.insert(id, Sequencer::new(action.into()));
        }
    }

    /// Clears the sequence of actions to be executed for a given robot.
    ///
    /// # Arguments
    ///
    /// * `id`: The id of the robot whose sequence of actions will be cleared.
    pub fn clean(&mut self, id: u8) {
        if let Some(sequencer) = self.actions.get_mut(&id) {
            sequencer.clear();
        }
    }

    /// Clears the sequence of actions to be executed of all robot.
    pub fn clear(&mut self) {
        self.actions.iter_mut().for_each(|(_, sequencer)| {
            sequencer.clear();
        })
    }

    /// Computes the sequence of actions to be executed for each robot and returns a
    /// `CommandMap` containing the commands to be sent to each robot.
    ///
    /// # Arguments
    ///
    /// * `world`: The current state of the world.
    /// * `tools`: A collection of external tools used by the action, such as a viewer.    .
    pub fn compute(&mut self, world: &World, tools: &mut ToolData) -> CommandMap {
        let mut command_map = CommandMap::default();
        self.actions.iter_mut().for_each(|(id, action)| {
            command_map.insert(*id, action.compute_order(*id, world, tools));
        });
        command_map
    }
}
