use crate::engine::fighter::FighterId;
use crate::engine::slot::SlotRef;

pub enum TurnAction {
    SwitchIn { slot: SlotRef, fighter: FighterId },
}
