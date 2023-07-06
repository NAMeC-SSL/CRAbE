//KickOffAlly set ups the kick of when in favor of the kick off
mod prepare_kick_off_ally;
pub use self::prepare_kick_off_ally::PrepareKickOffAlly;

//KickOffEnemy set ups the kick of when not in favor of the kick off
mod prepare_kick_off_enemy;
pub use self::prepare_kick_off_enemy::PrepareKickOffEnemy;

//FreeKickEnemy set ups the kick of when not in favor of the free kick
mod prepare_freekick_enemy;
pub use self::prepare_freekick_enemy::PrepareFreeKickEnemy;
