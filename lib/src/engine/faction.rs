use crate::engine::format::BattleFormat;
use crate::engine::slot::{Slot, SlotRef};
use crate::engine::team::{Team, TeamId};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FactionId(usize);

impl FactionId {
    pub(crate) fn new(idx: usize) -> Self {
        Self(idx)
    }
    pub fn index(self) -> usize {
        self.0
    }
}

/// Equals a side of the battle, might have multiple teams.
pub struct Faction {
    teams: Vec<Team>,
}

// Creation
impl Faction {
    pub fn new() -> Self {
        Self { teams: Vec::new() }
    }

    pub fn add_team(&mut self, team: Team) -> TeamId {
        let id = TeamId::new(self.teams.len());
        self.teams.push(team);
        id
    }
}

// Accessors
impl Faction {
    pub fn team(&self, id: TeamId) -> Option<&Team> {
        self.teams.get(id.index())
    }

    pub fn team_mut(&mut self, id: TeamId) -> Option<&mut Team> {
        self.teams.get_mut(id.index())
    }

    pub fn slot(&self, r: SlotRef) -> Option<&Slot> {
        self.team(r.team.id)?.slot(r.id)
    }

    pub fn slot_mut(&mut self, r: SlotRef) -> Option<&mut Slot> {
        self.team_mut(r.team.id)?.slot_mut(r.id)
    }
}

// Logic
impl Faction {
    pub(crate) fn start(&mut self, format: &BattleFormat) {
        for team in self.teams.iter_mut() {
            team.start(format);
        }
    }
}
