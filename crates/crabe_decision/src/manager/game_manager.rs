use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::strategy::Strategy;
use crate::strategy::attacker::Shooter;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::game_state::{GameState, RunningState, StoppedState};
use crabe_framework::data::world::World;

/// The `Manual` struct represents a decision manager that executes strategies manually
/// added to its list.
/// It's used for testing individual strategies only and not meant to be used during an actual game.
///
/// To add a strategy, simply create a new instance of the desired strategy and add it to the
/// `strategies` field in the `new()` method of the `Manual` struct.
#[derive(Default)]
pub struct GameManager {
    last_game_state: Option<GameState>,
    strategies: Vec<Box<dyn Strategy>>,
}

impl GameManager {
    /// Creates a new `Manual` instance with the desired strategies to test.
    pub fn new() -> Self {
        Self {
            last_game_state: None,
            strategies: vec![],
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
        //dbg!(world.data.state);
        // info!("{:?}", &world.data.state);
        if self.last_game_state.is_none() || self.last_game_state.unwrap() != world.data.state {
            // info!("clearing strategy");
            // clear current strategy
            self.strategies.clear();
            action_wrapper.clear();

            match world.data.state {
                GameState::Halted(_) => {
                    println!("halted");
                }
                GameState::Stopped(stopped_state) => match stopped_state {
                    StoppedState::Stop => {
                        println!("stop");
                    }
                    StoppedState::PrepareKickoff(team) => {
                        println!("prepare kick off {:?}",team);
                    }
                    StoppedState::PreparePenalty(team) => {
                        println!("prepare penalty {:?}",team);
                    }
                    StoppedState::BallPlacement(team) => {
                        println!("ball placement {:?}",team);
                    }
                },
                GameState::Running(running_state) => match running_state {
                    RunningState::KickOff(team) => {
                        println!("kickoff for {:#?}", team);
                    }
                    RunningState::Penalty(team) => {
                        println!("penalty for {:#?}", team);
                    }
                    RunningState::FreeKick(team) => {
                        println!("free kick for {:#?}", team);
                    }
                    RunningState::Run => {
                        println!("run");
                        self.strategies.push(Box::new(Shooter::new(0)));
                    }
                },
            }
        }

        for strategy in &mut self.strategies {
            strategy.step(world, tools_data, action_wrapper);
        }

        self.last_game_state = Some(world.data.state);
    }
}
