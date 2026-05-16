use crate::engine::team::TeamRef;
use crate::persistence::pokemon::Pokemon;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FighterId(usize);

impl FighterId {
    pub(crate) fn new(idx: usize) -> Self {
        Self(idx)
    }
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FighterRef {
    pub team: TeamRef,
    pub id: FighterId,
}

pub struct Fighter {
    pokemon: Pokemon,
}

impl Fighter {
    pub fn new(pokemon: Pokemon) -> Self {
        Self { pokemon }
    }
}
