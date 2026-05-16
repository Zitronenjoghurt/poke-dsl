use crate::engine::fighter::FighterId;
use crate::engine::team::TeamRef;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SlotId(usize);

impl SlotId {
    pub(crate) fn new(idx: usize) -> Self {
        Self(idx)
    }
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SlotRef {
    pub team: TeamRef,
    pub id: SlotId,
}

pub struct Slot {
    occupant: Option<FighterId>,
}

// Creation
impl Slot {
    pub(crate) fn empty() -> Self {
        Self { occupant: None }
    }

    pub(crate) fn with_occupant(fighter: FighterId) -> Self {
        Self {
            occupant: Some(fighter),
        }
    }
}

// Accessors
impl Slot {
    pub fn occupant(&self) -> Option<FighterId> {
        self.occupant
    }

    pub(crate) fn set_occupant(&mut self, fighter: Option<FighterId>) -> Option<FighterId> {
        std::mem::replace(&mut self.occupant, fighter)
    }
}
