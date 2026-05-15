use rand::Rng;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Probability {}

impl Probability {
    pub fn roll(&self, rng: &mut dyn Rng) -> bool {
        true
    }
}
