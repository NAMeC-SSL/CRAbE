use crate::action::ActionWrapper;
use crate::constants::{ATTACKER1_ID};
use crate::manager::game_manager::GameManager;
use crate::strategy::Strategy;
use crate::strategy::attacker::{Shooter, Passer};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{World};
use nalgebra::Point2;
/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
pub struct Attacker {
    /// The id of the robot to move.
    id: u8,
    strategy: Box<dyn Strategy>
}
impl Default for Attacker {
    fn default() -> Self {
        Self { id: ATTACKER1_ID, strategy: Box::new(Shooter::new(ATTACKER1_ID))}
    }
}
impl Attacker {
    /// Creates a new Attacker instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { 
            id, 
            strategy: Box::new(Shooter::new(id))
        }
    }
}

impl Strategy for Attacker {
    /// # Arguments
    ///
    /// * world: The current state of the game world.
    /// * tools_data: A collection of external tools used by the strategy, such as a viewer.    
    /// * action_wrapper: An `ActionWrapper` instance used to issue actions to the robot.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the strategy is finished or not.
    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.clean(self.id);
        if let Some(bappe) = GameManager::closest_ally_shooter_to_ball(world) {
            if self.id != bappe.id {
                if self.strategy.name() != "Passer" {
                    self.strategy = Box::new(Passer::new(self.id));
                }
            }else{
                let target = Point2::new(world.geometry.field.length/2.5, 0.0);
                if GameManager::bot_in_trajectory(world, self.id, target){//change to the size of the ball
                    if self.strategy.name() != "Shooter" {
                        self.strategy = Box::new(Shooter::new(self.id));
                    }
                }else{
                    if self.strategy.name() != "Shooter" {
                        self.strategy = Box::new(Shooter::new(self.id));
                    }
                }
            }
        }
        self.strategy.step(world, tools_data, action_wrapper)
    }
    fn name(&self) -> &'static str {
        return "Attacker"
    }
}
