use crate::dsl::condition::BattleCondition;
use crate::dsl::effect::Effect;
use crate::dsl::number::Number;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attempt {
    Attempt {
        condition: BattleCondition,
        success: Effect,
        failure: Effect,
        after: Effect,
    },
    Cascade {
        attempts: Vec<Attempt>,
    },
    Combo {
        condition: BattleCondition,
        hits: Number,
        effect: Effect,
    },
}
