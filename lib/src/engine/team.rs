use crate::engine::faction::FactionId;
use crate::engine::fighter::{Fighter, FighterId};
use crate::engine::format::BattleFormat;
use crate::engine::slot::{Slot, SlotId};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TeamId(usize);

impl TeamId {
    pub(crate) fn new(idx: usize) -> Self {
        Self(idx)
    }
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TeamRef {
    pub faction: FactionId,
    pub id: TeamId,
}

/// Equals a player team participating in a battle.
pub struct Team {
    fighters: Vec<Fighter>,
    slots: Vec<Slot>,
}

// Creation
impl Team {
    pub fn new() -> Self {
        Self {
            fighters: Vec::new(),
            slots: Vec::new(),
        }
    }

    pub fn add_fighter(&mut self, fighter: Fighter) -> FighterId {
        let id = FighterId::new(self.fighters.len());
        self.fighters.push(fighter);
        id
    }
}

// Accessors
impl Team {
    pub fn fighter(&self, id: FighterId) -> Option<&Fighter> {
        self.fighters.get(id.index())
    }

    pub fn fighter_mut(&mut self, id: FighterId) -> Option<&mut Fighter> {
        self.fighters.get_mut(id.index())
    }

    pub fn slot(&self, id: SlotId) -> Option<&Slot> {
        self.slots.get(id.index())
    }

    pub(crate) fn slot_mut(&mut self, id: SlotId) -> Option<&mut Slot> {
        self.slots.get_mut(id.index())
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }
}

// Logic
impl Team {
    pub(crate) fn start(&mut self, format: &BattleFormat) {
        self.init_slots(format.active_pokemon_per_team);
    }

    fn init_slots(&mut self, slot_count: usize) {
        self.slots = (0..slot_count)
            .map(|i| {
                if i < self.fighters.len() {
                    Slot::with_occupant(FighterId::new(i))
                } else {
                    Slot::empty()
                }
            })
            .collect();
    }
}
