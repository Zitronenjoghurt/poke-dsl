pub struct BattleFormat {
    pub active_pokemon_per_team: usize,
}

impl BattleFormat {
    pub const SINGLES: Self = Self {
        active_pokemon_per_team: 1,
    };

    pub const DOUBLES: Self = Self {
        active_pokemon_per_team: 2,
    };
}
