use crate::dsl::condition::BattleCondition;
use crate::engine::battle::Battle;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Effect {
    None,
    Condition {
        cond: BattleCondition,
        success: Box<Effect>,
        failure: Box<Effect>,
    },
    Sequence {
        effects: Vec<Effect>,
    },
}

impl Effect {
    pub fn apply(&self, battle: &mut Battle) {}
}
