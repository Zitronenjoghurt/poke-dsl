#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Effectiveness {
    #[default]
    Normal,
    NoEffect,
    NotVeryEffective,
    SuperEffective,
}
