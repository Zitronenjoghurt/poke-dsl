use crate::data::dex::Dex;
use crate::data::dex::Resolvable;
use crate::data::dex::ResolveError;
use crate::data::store::Key;
use crate::dsl::effect::Effect;
use crate::dsl::trigger::TriggerEffect;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawItemData {
    id: String,
    held: Vec<TriggerEffect>,
    active: Effect,
}

#[derive(Debug)]
pub struct ItemData {
    id: Key<ItemData>,
    held: Vec<TriggerEffect>,
    active: Effect,
}

impl Resolvable for RawItemData {
    type Output = ItemData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(self, id: Key<Self::Output>, _: &Dex) -> Result<Self::Output, ResolveError> {
        Ok(ItemData {
            id,
            held: self.held,
            active: self.active,
        })
    }
}
