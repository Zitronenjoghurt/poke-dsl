use crate::data::dex::Dex;
use crate::data::dex::Resolvable;
use crate::data::dex::ResolveError;
use crate::data::store::Key;
use crate::dsl::generation::GenSpecific;
use crate::dsl::stats::Stats;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawSpeciesData {
    id: String,
    base: GenSpecific<Stats>,
}

#[derive(Debug)]
pub struct SpeciesData {
    pub id: Key<SpeciesData>,
    pub base: GenSpecific<Stats>,
}

impl Resolvable for RawSpeciesData {
    type Output = SpeciesData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(self, id: Key<Self::Output>, _: &Dex) -> Result<Self::Output, ResolveError> {
        Ok(SpeciesData {
            id,
            base: self.base,
        })
    }
}
