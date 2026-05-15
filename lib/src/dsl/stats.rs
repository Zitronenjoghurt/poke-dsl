use strum::{Display, EnumIter, FromRepr};

pub type EV = Stats;
pub type IV = Stats;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display, FromRepr)]
#[repr(u8)]
pub enum Stat {
    Hp = 1,
    Atk = 2,
    Def = 3,
    SpA = 4,
    SpD = 5,
    Spe = 6,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stats {
    pub hp: u16,
    pub atk: u16,
    pub def: u16,
    /// For Gen-1 games this also represents the SPECIAL stat
    pub spa: u16,
    pub spd: u16,
    pub spe: u16,
}

impl Stats {
    pub fn get(&self, stat: Stat) -> u16 {
        match stat {
            Stat::Hp => self.hp,
            Stat::Atk => self.atk,
            Stat::Def => self.def,
            Stat::SpA => self.spa,
            Stat::SpD => self.spd,
            Stat::Spe => self.spe,
        }
    }

    pub fn set(&mut self, stat: Stat, value: u16) {
        match stat {
            Stat::Hp => self.hp = value,
            Stat::Atk => self.atk = value,
            Stat::Def => self.def = value,
            Stat::SpA => self.spa = value,
            Stat::SpD => self.spd = value,
            Stat::Spe => self.spe = value,
        }
    }
}
