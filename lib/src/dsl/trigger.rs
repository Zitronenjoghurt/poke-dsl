use crate::dsl::effect::Effect;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Trigger {}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TriggerEffect {
    trigger: Trigger,
    effect: Effect,
}
