use crate::data::dex::Dex;
use crate::data::dex::Resolvable;
use crate::data::dex::ResolveError;
use crate::data::store::Key;
use crate::dsl::trigger::TriggerEffect;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct RawAbilityData {
    id: String,
    triggers: Vec<TriggerEffect>,
}

#[derive(Debug)]
pub struct AbilityData {
    id: Key<AbilityData>,
    triggers: Vec<TriggerEffect>,
}

impl Resolvable for RawAbilityData {
    type Output = AbilityData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(self, id: Key<Self::Output>, _: &Dex) -> Result<Self::Output, ResolveError> {
        Ok(AbilityData {
            id,
            triggers: self.triggers,
        })
    }
}
