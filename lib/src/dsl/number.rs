use crate::engine::battle::Battle;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Number {
    Exact(usize),
}

impl Number {
    pub fn evaluate(&self, battle: &Battle) -> usize {
        match self {
            Number::Exact(n) => *n,
        }
    }
}
