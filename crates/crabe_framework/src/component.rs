use crate::data::input::InboundData;
use crate::data::output::{CommandMap, FeedbackMap};
use crate::data::tool::{ToolCommands, ToolData};
use crate::data::world::World;

/// The Component trait defines the methods shared for a component that is a part
/// of the robot's AI pipeline.
/// It includes a close method to be used when a component is no longer needed.
pub trait Component: ComponentBoxed {
    fn close(self);
}

pub trait ComponentBoxed {
    fn close_boxed(self: Box<Self>);
}

impl<T> ComponentBoxed for T
where
    T: Component,
{
    fn close_boxed(self: Box<Self>) {
        (*self).close()
    }
}

impl<T> Component for Box<T>
where
    T: ?Sized + Component,
{
    fn close(self) {
        self.close_boxed()
    }
}

/// The `InputComponent` trait defines the methods required for a component that handles
/// the input data for a SSL game (Vision, Game Controller, ...).
/// Reads input data and returns a new `InboundData` struct with the processed data.
/// The feedback parameter is used to provide feedback to the component,
/// such as odometry or infrared data.
pub trait InputComponent: Component {
    fn step(&mut self, feedback: &mut FeedbackMap) -> InboundData;
}
/// The `FilterComponent` trait defines the methods required for a component that applies
/// filters to the input data to remove noise, unwanted or unnecessary information.
/// It processes `InboundData` and updates `World` struct with the desired information
pub trait FilterComponent: Component {
    fn step(&mut self, data: InboundData, world: &mut World);
}
/// The `DecisionComponent` trait defines the methods required for a component that makes decisions
/// for a SSL robot fleet based on the filtered input data.
pub trait DecisionComponent: Component {
    fn step(&mut self, data: &World) -> (CommandMap, ToolData);
}

/// The `ToolComponent` trait defines the methods required for a component that manages and
/// manipulates additional tools used by the project's crates.
/// These tools can include things like a joystick handler or sending and
/// receiving data for tools, such as a viewer or a control center.
pub trait ToolComponent: Component {
    fn step(
        &mut self,
        world_data: &World,
        tools_data: &mut ToolData,
        commands: &mut CommandMap,
    ) -> ToolCommands;
}
/// The `GuardComponent` trait defines the methods required for a component that guards the robot
/// from potentially dangerous or unwanted actions.
pub trait GuardComponent: Component {
    fn step(&mut self, world: &World, command: &mut CommandMap, tools_commands: &mut ToolCommands);
}

/// The `OutputComponent` trait defines the methods required for a component that sends output
/// commands and received feedback from the robot.
pub trait OutputComponent: Component {
    fn step(&mut self, commands: CommandMap, tool_commands: ToolCommands) -> FeedbackMap;
}
