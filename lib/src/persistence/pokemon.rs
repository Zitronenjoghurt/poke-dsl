use crate::config::Config;
use crate::data::dex::species::SpeciesData;
use crate::dsl::nature::Nature;
use crate::dsl::stats::{Stats, EV, IV};
use std::sync::Arc;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StoredPokemon {
    species_id: String,
    level: u8,
    ev: EV,
    iv: IV,
    nature: Nature,
}

pub struct Pokemon {
    pub species: Arc<SpeciesData>,
    pub level: u8,
    pub ev: EV,
    pub iv: IV,
    pub nature: Nature,
}

impl Pokemon {
    pub fn calc_stats(&self, config: &Config) -> Stats {
        config.stat_formula.calc(config, self)
    }
}
