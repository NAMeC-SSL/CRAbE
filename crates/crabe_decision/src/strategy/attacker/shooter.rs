use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::{Point2, Point3};
use std::f64::consts::PI;
use std::ops::Sub;

#[derive(Default)]
pub struct Shooter {
    /// The id of the robot to move.
    id: u8,
    internal_state: ShooterState
}

#[derive(Debug, Default)]
enum ShooterState {
    #[default]
    GoingBehindBall,
    GoingShoot
}
impl Shooter {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, internal_state: ShooterState::GoingBehindBall }
    }
}

impl Strategy for Shooter {
    /// Executes the Square strategy.
    ///
    /// This strategy commands the robot with the specified ID to move in a square shape in a
    /// counter-clockwise direction.
    ///
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
        if let Some(ball) = &world.ball {
            let target = Point3::new(-world.geometry.field.length/2.,0.,0.);
            let mut dir = ball.position.sub(target);
            dir = dir.normalize();
            dir = dir * 0.2;
            match &self.internal_state {
                ShooterState::GoingBehindBall => {
                    action_wrapper.push(self.id, MoveTo::new(Point2::new(ball.position.x + dir.x, ball.position.y + dir.y), PI / 4.0));
                    self.internal_state = ShooterState::GoingShoot;
                },
                ShooterState::GoingShoot => {
                    action_wrapper.push(self.id, MoveTo::new(Point2::new(ball.position.x, ball.position.y), PI / 4.0));
                },
            }
            
        }
        false
    }
}
