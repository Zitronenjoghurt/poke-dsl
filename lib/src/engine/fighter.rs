use crate::data::poke_move::MoveData;
use crate::data::species::SpeciesData;
use std::sync::Arc;

pub struct Fighter {
    species: Arc<SpeciesData>,
    team: usize,
    moves: Vec<Arc<MoveData>>,
}

impl Fighter {
    pub fn new(species: Arc<SpeciesData>) -> Self {
        Self {
            species,
            team: 0,
            moves: Vec::new(),
        }
    }

    pub fn with_team(mut self, team: usize) -> Self {
        self.team = team;
        self
    }

    pub fn with_moves(mut self, moves: Vec<Arc<MoveData>>) -> Self {
        self.moves = moves;
        self
    }
}
