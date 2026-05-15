use crate::data::{Key, Resolvable};
use crate::dsl::trigger::TriggerEffect;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawAbilityData {
    id: String,
    triggers: Vec<TriggerEffect>,
}

pub struct AbilityData {
    id: Key<AbilityData>,
    triggers: Vec<TriggerEffect>,
}

impl Resolvable for RawAbilityData {
    type Output = AbilityData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(
        self,
        id: Key<Self::Output>,
        _: &crate::data::Dex,
    ) -> Result<Self::Output, crate::data::ResolveError> {
        Ok(AbilityData {
            id,
            triggers: self.triggers,
        })
    }
}
