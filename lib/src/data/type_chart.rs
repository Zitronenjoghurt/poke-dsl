use crate::data::poke_type::TypeData;
use crate::data::{Dex, Key, Resolvable, ResolveError};
use crate::dsl::effectiveness::Effectiveness;
use std::collections::HashMap;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawTypeChartData {
    id: String,
    effectiveness: HashMap<(String, String), Effectiveness>,
}

pub struct TypeChartData {
    id: Key<TypeChartData>,
    effectiveness: HashMap<(Key<TypeData>, Key<TypeData>), Effectiveness>,
}

impl Resolvable for RawTypeChartData {
    type Output = TypeChartData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(self, id: Key<Self::Output>, dex: &Dex) -> Result<Self::Output, ResolveError> {
        let effectiveness = self
            .effectiveness
            .into_iter()
            .map(|((atk, def), eff)| {
                let atk_key = dex.resolve_id::<TypeData>(&atk)?;
                let def_key = dex.resolve_id::<TypeData>(&def)?;
                Ok(((atk_key, def_key), eff))
            })
            .collect::<Result<HashMap<_, _>, ResolveError>>()?;

        Ok(TypeChartData { id, effectiveness })
    }
}
