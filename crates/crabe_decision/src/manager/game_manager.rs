use log::info;
use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::game_state::GameState;
use crabe_framework::data::world::World;
use crate::strategy::goalkeeper::Goalkeeper;
use crate::strategy::mbappe::Mbappe;
use crate::strategy::testing::Square;
use crate::strategy::stand::Stand;

/// The `Manual` struct represents a decision manager that executes strategies manually
/// added to its list.
/// It's used for testing individual strategies only and not meant to be used during an actual game.
///
/// To add a strategy, simply create a new instance of the desired strategy and add it to the
/// `strategies` field in the `new()` method of the `Manual` struct.
#[derive(Default)]
pub struct GameManager { // Karen says what to do lmao
    last_game_state: Option<GameState>,
    strategies: Vec<Box<dyn Strategy>>,
}

const KEEPER_ID: u8 = 0;

impl GameManager {
    /// Creates a new `Manual` instance with the desired strategies to test.
    pub fn new() -> Self {
        Self {
            last_game_state: None,
            strategies: vec![]
        }
    }
}

impl Manager for GameManager {
    /// Executes the list of strategies on the given `World` data, `ToolData`, and `ActionWrapper`.
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) {
        if self.last_game_state.is_none() || self.last_game_state.unwrap() != world.data.state {
            info!("clearing strategy");
            // clear current strategy
            self.strategies.clear();
            for id in world.allies_bot.keys() {
                action_wrapper.clean(*id);
            }

            match world.data.state {
                GameState::Halted(_) => {}
                GameState::Stopped(_) => {}
                GameState::Running(_) => {
                    info!("GameState::Running");
                    self.strategies.push(Box::new(Goalkeeper::new(KEEPER_ID)));

                    let closest_robot_to_ball_id = world.allies_bot
                        .iter()
                        .filter(|(id, _)| **id != KEEPER_ID)
                        .map(|(id, robot)| (id, robot, robot.distance(&world.ball.clone().unwrap_or_default().position.xy())))
                        .min_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2))
                        .map(|(id, _, _)| id);

                    let mut rest: Vec<u8> = world.allies_bot.iter().map(|(id, _)| *id).filter(|id| *id != KEEPER_ID).collect();
                    if let Some(bappe_id) = closest_robot_to_ball_id {
                        self.strategies.push(Box::new(Mbappe::new(*bappe_id)));

                        rest = world.allies_bot.iter().map(|(id, _)| *id).filter(|id| *id != KEEPER_ID && *id != *bappe_id).collect();
                    }

                    for id in rest {
                        self.strategies.push(Box::new(Stand::new(id)));
                    }
                }
            }
        }


        for strategy in &mut self.strategies {
            strategy.step(world, tools_data, action_wrapper);
        }

        self.last_game_state = Some(world.data.state);
    }
}
