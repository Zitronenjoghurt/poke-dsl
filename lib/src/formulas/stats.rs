use crate::config::Config;
use crate::dsl::generation::Generation;
use crate::dsl::nature::Nature;
use crate::dsl::stats::{Stat, Stats};
use crate::persistence::pokemon::Pokemon;

pub fn stat_formula(generation: Generation) -> Box<dyn StatFormula> {
    if generation.get() <= 2 {
        Box::new(Gen12StatFormula)
    } else {
        Box::new(Gen3OnwardsStatFormula)
    }
}

pub trait StatFormula {
    fn calc(&self, config: &Config, pokemon: &Pokemon) -> Stats;
}

pub struct Gen12StatFormula;

impl Gen12StatFormula {
    fn calc_inner(level: u8, base: u16, ev: u16, iv: u16) -> u16 {
        ((((base + iv) as usize * 2 + ((ev as usize).isqrt() / 4)) * level as usize) / 100) as u16
    }

    fn calc_hp(level: u8, base: u16, ev: u16, iv: u16) -> u16 {
        Self::calc_inner(level, base, ev, iv) + level as u16 + 10
    }

    fn calc_stat(level: u8, base: u16, ev: u16, iv: u16) -> u16 {
        Self::calc_inner(level, base, ev, iv) + 5
    }
}

impl StatFormula for Gen12StatFormula {
    fn calc(&self, config: &Config, pokemon: &Pokemon) -> Stats {
        let base = pokemon.species.base.get(config.generation);
        let level = pokemon.level;
        let ev = &pokemon.ev;
        let iv = &pokemon.iv;

        let special = Self::calc_stat(level, base.spa, ev.spa, iv.spa);

        Stats {
            hp: Self::calc_hp(level, base.hp, ev.hp, iv.hp),
            atk: Self::calc_stat(level, base.atk, ev.atk, iv.atk),
            def: Self::calc_stat(level, base.def, ev.def, iv.def),
            spa: special,
            spd: special,
            spe: Self::calc_stat(level, base.spe, ev.spe, iv.spe),
        }
    }
}

pub struct Gen3OnwardsStatFormula;

impl Gen3OnwardsStatFormula {
    fn calc_inner(level: u8, base: u16, ev: u16, iv: u16) -> u16 {
        (((2 * base as usize + iv as usize + (ev as usize) / 4) * level as usize) / 100) as u16
    }

    fn calc_hp(level: u8, base: u16, ev: u16, iv: u16) -> u16 {
        Self::calc_inner(level, base, ev, iv) + level as u16 + 10
    }

    fn calc_stat(level: u8, base: u16, ev: u16, iv: u16, stat: Stat, nature: Nature) -> u16 {
        ((Self::calc_inner(level, base, ev, iv) + 5) as f64 * Self::nature_factor(stat, nature))
            .floor() as u16
    }

    fn nature_factor(stat: Stat, nature: Nature) -> f64 {
        let increase = nature.increased_stat() == stat;
        let decrease = nature.decreased_stat() == stat;
        if increase && decrease {
            1.0
        } else if increase {
            1.1
        } else if decrease {
            0.9
        } else {
            1.0
        }
    }
}

impl StatFormula for Gen3OnwardsStatFormula {
    fn calc(&self, config: &Config, pokemon: &Pokemon) -> Stats {
        let base = pokemon.species.base.get(config.generation);
        let level = pokemon.level;
        let ev = &pokemon.ev;
        let iv = &pokemon.iv;
        let nature = pokemon.nature;

        Stats {
            hp: Self::calc_hp(level, base.hp, ev.hp, iv.hp),
            atk: Self::calc_stat(level, base.atk, ev.atk, iv.atk, Stat::Atk, nature),
            def: Self::calc_stat(level, base.def, ev.def, iv.def, Stat::Def, nature),
            spa: Self::calc_stat(level, base.spa, ev.spa, iv.spa, Stat::SpA, nature),
            spd: Self::calc_stat(level, base.spd, ev.spd, iv.spd, Stat::SpD, nature),
            spe: Self::calc_stat(level, base.spe, ev.spe, iv.spe, Stat::Spe, nature),
        }
    }
}
