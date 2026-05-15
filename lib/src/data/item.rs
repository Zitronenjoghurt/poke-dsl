use crate::data::{Key, Resolvable};
use crate::dsl::effect::Effect;
use crate::dsl::trigger::TriggerEffect;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawItemData {
    id: String,
    held: Vec<TriggerEffect>,
    active: Option<Effect>,
}

pub struct ItemData {
    id: Key<ItemData>,
    held: Vec<TriggerEffect>,
    active: Option<Effect>,
}

impl Resolvable for RawItemData {
    type Output = ItemData;

    fn id(&self) -> &str {
        &self.id
    }

    fn resolve(
        self,
        id: Key<Self::Output>,
        _: &crate::data::Dex,
    ) -> Result<Self::Output, crate::data::ResolveError> {
        Ok(ItemData {
            id,
            held: self.held,
            active: self.active,
        })
    }
}
