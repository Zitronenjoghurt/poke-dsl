use crate::config::Config;
use crate::dsl::generation::Generation;
use crate::dsl::trigger::Trigger;
use crate::engine::fighter::Fighter;
use rand::Rng;

pub struct Battle {
    config: Config,
    fighters: Vec<Fighter>,
    rng: Box<dyn Rng>,
}

impl Battle {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            fighters: Vec::new(),
            rng: Box::new(rand::rng()),
        }
    }

    pub fn from_generation(generation: Generation) -> Self {
        Self::new(Config::from_generation(generation))
    }

    pub fn add_fighter(&mut self, fighter: Fighter) {
        self.fighters.push(fighter);
    }

    pub fn rng(&mut self) -> &mut dyn Rng {
        &mut self.rng
    }

    pub fn fire(&mut self, trigger: Trigger) {}
}
