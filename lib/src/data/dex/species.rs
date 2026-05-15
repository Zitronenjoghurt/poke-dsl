use crate::data::dex::poke_type::TypeData;
use crate::data::dex::Dex;
use crate::data::dex::Resolvable;
use crate::data::dex::ResolveError;
use crate::data::store::Key;
use crate::dsl::generation::GenSpecific;
use crate::dsl::stats::Stats;
use std::sync::Arc;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawSpeciesData {
    id: String,
    base: RawFormData,
    forms: Vec<RawFormData>,
}

#[derive(Debug)]
pub struct SpeciesData {
    pub id: Key<SpeciesData>,
    pub base: FormData,
    pub forms: Vec<FormData>,
}

impl SpeciesData {
    pub fn get_form(&self, index: usize) -> &FormData {
        if index == 0 {
            return &self.base;
        }
        self.forms.get(index - 1).unwrap_or(&self.base)
    }
}

impl Resolvable for RawSpeciesData {
    type Output = SpeciesData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(self, id: Key<Self::Output>, dex: &Dex) -> Result<Self::Output, ResolveError> {
        Ok(SpeciesData {
            id,
            base: self.base.resolve(dex)?,
            forms: self
                .forms
                .into_iter()
                .map(|form| form.resolve(dex))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawFormData {
    name: String,
    type_ids: Vec<String>,
    stats: GenSpecific<Stats>,
}

impl RawFormData {
    fn resolve(self, dex: &Dex) -> Result<FormData, ResolveError> {
        Ok(FormData {
            name: self.name,
            types: dex.resolve_refs(&self.type_ids)?,
            stats: self.stats,
        })
    }
}

#[derive(Debug)]
pub struct FormData {
    pub name: String,
    pub types: Vec<Arc<TypeData>>,
    pub stats: GenSpecific<Stats>,
}
