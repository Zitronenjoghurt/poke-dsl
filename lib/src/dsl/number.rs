use crate::engine::battle::Battle;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Number {
    Exact(usize),
}

impl Number {
    pub fn evaluate(&self, battle: &Battle) -> usize {
        0
    }
}
