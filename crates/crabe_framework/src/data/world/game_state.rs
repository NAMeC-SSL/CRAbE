use std::time::Instant;
use crate::data::world::TeamColor;
use serde::Serialize;

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GameState {
    Halted(HaltedState),
    Stopped(StoppedState),
    Running(RunningState),
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum HaltedState {
    Halt,
    Timeout,
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StoppedState {
    Stop,
    PrepareKickoff(TeamColor),
    PreparePenalty(TeamColor),
    BallPlacement(TeamColor, #[serde(skip)] Instant),
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RunningState {
    KickOff(TeamColor, #[serde(skip)] Instant),
    Penalty(TeamColor, #[serde(skip)] Instant),
    FreeKick(TeamColor, #[serde(skip)] Instant),
    Run,
}
