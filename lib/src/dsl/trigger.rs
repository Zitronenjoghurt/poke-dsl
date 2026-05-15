use crate::dsl::effect::Effect;
use crate::dsl::target::Target;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Trigger {
    TurnStart,
    TurnEnd,
    DamageDealt(Target),
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TriggerEffect {
    trigger: Trigger,
    effect: Effect,
}
