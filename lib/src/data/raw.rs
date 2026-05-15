use crate::data::dex;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum RawData {
    Ability(dex::ability::RawAbilityData),
    Item(dex::item::RawItemData),
    Move(dex::poke_move::RawMoveData),
    Type(dex::poke_type::RawTypeData),
    Species(dex::species::RawSpeciesData),
    TypeChart(dex::type_chart::RawTypeChartData),
}

impl RawData {
    pub fn phase(&self) -> u8 {
        match self {
            RawData::Type(_) => 0,
            RawData::Ability(_) => 0,
            RawData::Item(_) => 0,
            RawData::Move(_) => 1,
            RawData::TypeChart(_) => 1,
            RawData::Species(_) => 2,
        }
    }
}
