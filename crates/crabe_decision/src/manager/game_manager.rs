use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::strategy::Strategy;
use crate::strategy::attacker::Shooter;
use crate::strategy::defender::Stand;
use crate::strategy::keeper::{Keep, PenaltyPrepKeeper};
use crate::strategy::formations::{PrepareKickOffAlly, PrepareKickOffEnemy};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::game_state::{GameState, RunningState, StoppedState};
<<<<<<< HEAD
use crabe_framework::data::world::World;
use crate::strategy::testing::GoToCenter;
=======
use crabe_framework::data::world::{World, Robot, AllyInfo, EnemyInfo};
>>>>>>> strategies

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

const KEEPER_ID: u8 = 0;

impl GameManager {
    /// Creates a new `Manual` instance with the desired strategies to test.
    pub fn new() -> Self {
        Self {
            last_game_state: None,
            strategies: vec![],
        }
    }

    pub fn closest_ally_to_ball(world: &World) -> Option<&Robot<AllyInfo>>{
        world.allies_bot
            .iter()
            .filter(|(id, _)| **id != KEEPER_ID)
            .map(|(id, robot)| (id, robot, robot.distance(&world.ball.clone().unwrap_or_default().position.xy())))
            .min_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2))
            .map(|(_, bot, _)| bot)
    }

    pub fn closest_enemy_to_ball(world: &World) -> Option<&Robot<EnemyInfo>>{
        world.enemies_bot
            .iter()
            .map(|(id, robot)| (id, robot, robot.distance(&world.ball.clone().unwrap_or_default().position.xy())))
            .min_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2))
            .map(|(_, bot, _)| bot)
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
                        self.strategies.push(Box::new(Keep::new(KEEPER_ID)));
                    }
                    StoppedState::PrepareKickoff(team) => {
                        if team == world.team_color {
                            self.strategies.push(Box::new(PrepareKickOffAlly::new()));
                        }else{
                            self.strategies.push(Box::new(PrepareKickOffEnemy::new()));
                        }
                        println!("prepare kick off {:?}",team);
                    }
                    StoppedState::PreparePenalty(team) => {
                        println!("prepare penalty {:?}",team);
                        if team != world.team_color {
                            self.strategies.push(Box::new(PenaltyPrepKeeper::new(KEEPER_ID)));
                        }
                    }
                    StoppedState::BallPlacement(team) => {
                        println!("ball placement {:?}",team);
                    }
                },
                GameState::Running(running_state) => match running_state {
                    RunningState::KickOff(team) => {
                        println!("kickoff for {:#?}", team);
                        if team != world.team_color {
                            return;
                        }
                        self.strategies.push(Box::new(Keep::new(KEEPER_ID)));
                        let rest: Vec<u8> = world.allies_bot.iter().map(|(id, _)| *id).filter(|id| *id != KEEPER_ID).collect();
                        for id in rest {
                            self.strategies.push(Box::new(Shooter::new(id)));
                        }
                    }
                    RunningState::Penalty(team) => {
                        println!("penalty for {:#?}", team);
                        if team == world.team_color {
                            self.strategies.push(Box::new(Keep::new(KEEPER_ID)));
                            let rest: Vec<u8> = world.allies_bot.iter().map(|(id, _)| *id).filter(|id| *id != KEEPER_ID).collect();
                            for id in rest {
                                self.strategies.push(Box::new(Shooter::new(id)));
                            }
                        }else{
                            self.strategies.push(Box::new(Keep::new(KEEPER_ID)));
                        }
                    }
                    RunningState::FreeKick(team) => {
                        println!("free kick for {:#?}", team);
                        self.strategies.push(Box::new(Keep::new(KEEPER_ID)));
                        let rest: Vec<u8> = world.allies_bot.iter().map(|(id, _)| *id).filter(|id| *id != KEEPER_ID).collect();
                        for id in rest {
                            self.strategies.push(Box::new(Shooter::new(id)));
                        }
                    }
                    RunningState::Run => {
                        println!("run");
                        self.strategies.push(Box::new(Keep::new(KEEPER_ID)));

                        let rest: Vec<u8> = world.allies_bot.iter().map(|(id, _)| *id).filter(|id| *id != KEEPER_ID).collect();
                        // if let Some(bappe) = GameManager::closest_ally_to_ball(world) {
                        //     self.strategies.push(Box::new(Shooter::new(bappe.id)));
                        //     rest = world.allies_bot.iter().map(|(id, _)| *id).filter(|id| *id != KEEPER_ID && *id != bappe.id).collect();
                        // }

                        for id in rest {
                            self.strategies.push(Box::new(Shooter::new(id)));
                        }
                    }
                },
            }
        }
        self.strategies.push(Box::new(GoToCenter::new(0)));

        for strategy in &mut self.strategies {
            strategy.step(world, tools_data, action_wrapper);
        }

        self.last_game_state = Some(world.data.state);
    }
}