use crate::dsl::generation::Generation;
use crate::formulas::stats::{stat_formula, StatFormula};

pub struct Config {
    pub generation: Generation,
    pub stat_formula: Box<dyn StatFormula>,
}

impl Config {
    pub fn from_generation(generation: Generation) -> Self {
        Self {
            generation,
            stat_formula: stat_formula(generation),
        }
    }
}
