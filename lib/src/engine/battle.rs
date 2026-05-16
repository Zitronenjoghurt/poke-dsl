use crate::config::Config;
use crate::dsl::generation::Generation;
use crate::dsl::trigger::Trigger;
use crate::engine::faction::{Faction, FactionId};
use crate::engine::fighter::{Fighter, FighterRef};
use crate::engine::format::BattleFormat;
use crate::engine::team::{Team, TeamRef};
use rand::Rng;

pub struct Battle {
    format: BattleFormat,
    config: Config,
    factions: Vec<Faction>,
    rng: Box<dyn Rng>,
}

// Creation
impl Battle {
    pub fn new(format: BattleFormat, config: Config) -> Self {
        Self {
            format,
            config,
            factions: Vec::new(),
            rng: Box::new(rand::rng()),
        }
    }

    pub fn from_generation(format: BattleFormat, generation: Generation) -> Self {
        Self::new(format, Config::from_generation(generation))
    }

    pub fn add_faction(&mut self, faction: Faction) -> FactionId {
        let id = FactionId::new(self.factions.len());
        self.factions.push(faction);
        id
    }
}

// Accessors
impl Battle {
    pub fn faction(&self, id: FactionId) -> Option<&Faction> {
        self.factions.get(id.index())
    }

    pub fn faction_mut(&mut self, id: FactionId) -> Option<&mut Faction> {
        self.factions.get_mut(id.index())
    }

    pub fn team(&self, r: TeamRef) -> Option<&Team> {
        self.faction(r.faction)?.team(r.id)
    }

    pub fn team_mut(&mut self, r: TeamRef) -> Option<&mut Team> {
        self.faction_mut(r.faction)?.team_mut(r.id)
    }

    pub fn fighter(&self, r: FighterRef) -> Option<&Fighter> {
        self.team(r.team)?.fighter(r.id)
    }

    pub fn fighter_mut(&mut self, r: FighterRef) -> Option<&mut Fighter> {
        self.team_mut(r.team)?.fighter_mut(r.id)
    }

    pub fn rng(&mut self) -> &mut dyn Rng {
        &mut self.rng
    }
}

// Logic
impl Battle {
    pub fn start(&mut self) {
        for faction in self.factions.iter_mut() {
            faction.start(&self.format);
        }
    }

    pub fn fire(&mut self, trigger: Trigger) {}
}
