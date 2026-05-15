use crate::data::poke_type::TypeData;
use crate::data::{Dex, Key, Resolvable, ResolveError};
use crate::dsl::attempt::Attempt;
use crate::dsl::condition::BattleCondition;
use std::sync::Arc;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawMoveData {
    id: String,
    type_ids: Vec<String>,
    condition: BattleCondition,
    attempt: Attempt,
}

pub struct MoveData {
    id: Key<MoveData>,
    types: Vec<Arc<TypeData>>,
    condition: BattleCondition,
    attempt: Attempt,
}

impl Resolvable for RawMoveData {
    type Output = MoveData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(self, id: Key<Self::Output>, dex: &Dex) -> Result<Self::Output, ResolveError> {
        Ok(MoveData {
            id,
            types: dex.resolve_refs(&self.type_ids)?,
            condition: self.condition,
            attempt: self.attempt,
        })
    }
}
