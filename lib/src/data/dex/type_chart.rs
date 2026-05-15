use crate::config::Config;
use crate::data::dex::poke_type::TypeData;
use crate::data::dex::Dex;
use crate::data::dex::Resolvable;
use crate::data::dex::ResolveError;
use crate::data::store::Key;
use crate::dsl::effectiveness::Effectiveness;
use crate::dsl::generation::GenSpecific;
use std::collections::HashMap;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawTypeChartData {
    id: String,
    /// If a type combination is not listed, this default value is used
    default: Option<Effectiveness>,
    effectiveness: HashMap<(String, String), GenSpecific<Effectiveness>>,
}

#[derive(Debug)]
pub struct TypeChartData {
    id: Key<TypeChartData>,
    default: Option<Effectiveness>,
    effectiveness: HashMap<(Key<TypeData>, Key<TypeData>), GenSpecific<Effectiveness>>,
}

impl TypeChartData {
    pub fn effectiveness(
        &self,
        config: &Config,
        atk: Key<TypeData>,
        def: Key<TypeData>,
    ) -> Effectiveness {
        match self.effectiveness.get(&(atk, def)) {
            Some(eff) => *eff.get(config.generation),
            None => self.default.unwrap_or_default(),
        }
    }
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

        Ok(TypeChartData {
            id,
            default: self.default,
            effectiveness,
        })
    }
}
