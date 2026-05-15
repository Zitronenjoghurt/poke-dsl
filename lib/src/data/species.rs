use crate::data::{Key, Resolvable};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawSpeciesData {
    id: String,
}

pub struct SpeciesData {
    id: Key<SpeciesData>,
}

impl Resolvable for RawSpeciesData {
    type Output = SpeciesData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(
        self,
        id: Key<Self::Output>,
        _: &crate::data::Dex,
    ) -> Result<Self::Output, crate::data::ResolveError> {
        Ok(SpeciesData { id })
    }
}
