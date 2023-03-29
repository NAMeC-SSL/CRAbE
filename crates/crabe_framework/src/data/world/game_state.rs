use serde::{Serialize};

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum GameState {
    Halted(HaltedState),
    Stopped(StoppedState),
    Running(RunningState)
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum HaltedState {
    Halt,
    Timeout
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum StoppedState {
    Stop,
    PrepareKickoff,
    PreparePenalty,
    BallPlacement
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum RunningState {
    KickOff,
    Penalty,
    FreeKick,
    Run
}
