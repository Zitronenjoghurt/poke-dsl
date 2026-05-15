use crate::persistence::pokemon::Pokemon;

pub struct Fighter {
    pokemon: Pokemon,
    team: usize,
}

impl Fighter {
    pub fn new(pokemon: Pokemon) -> Self {
        Self { pokemon, team: 0 }
    }

    pub fn with_team(mut self, team: usize) -> Self {
        self.team = team;
        self
    }
}
