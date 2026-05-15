use crate::dsl::trigger::Trigger;
use crate::engine::fighter::Fighter;
use rand::Rng;

pub struct Battle {
    fighters: Vec<Fighter>,
    rng: Box<dyn Rng>,
}

impl Battle {
    pub fn rng(&mut self) -> &mut dyn Rng {
        &mut self.rng
    }

    pub fn fire(&mut self, trigger: Trigger) {}
}
