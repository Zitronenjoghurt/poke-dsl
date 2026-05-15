use crate::dsl::probability::Probability;
use crate::dsl::target::Target;
use crate::engine::battle::Battle;
use crate::engine::fighter::Fighter;

pub type BattleCondition = Condition<BattlePredicate>;
pub type FighterCondition = Condition<FighterPredicate>;

pub trait Checkable {
    type Context;
    fn check(&self, ctx: &mut Self::Context) -> bool;
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Condition<L> {
    Always,
    And(Box<Condition<L>>, Box<Condition<L>>),
    Or(Box<Condition<L>>, Box<Condition<L>>),
    Not(Box<Condition<L>>),
    Predicate(L),
}

impl<L: Checkable> Condition<L> {
    pub fn check(&self, ctx: &mut L::Context) -> bool {
        match self {
            Condition::Always => true,
            Condition::And(a, b) => a.check(ctx) && b.check(ctx),
            Condition::Or(a, b) => a.check(ctx) || b.check(ctx),
            Condition::Not(a) => !a.check(ctx),
            Condition::Predicate(p) => p.check(ctx),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BattlePredicate {
    Prob(Probability),
    Target {
        target: Target,
        cond: FighterCondition,
    },
}

impl Checkable for BattlePredicate {
    type Context = Battle;
    fn check(&self, battle: &mut Battle) -> bool {
        match self {
            BattlePredicate::Prob(p) => p.roll(battle.rng()),
            BattlePredicate::Target { target, cond } => true,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FighterPredicate {}

impl Checkable for FighterPredicate {
    type Context = Fighter;

    fn check(&self, ctx: &mut Self::Context) -> bool {
        true
    }
}
