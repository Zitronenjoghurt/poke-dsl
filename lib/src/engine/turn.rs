use crate::engine::battle::Battle;

pub struct Turn<'a> {
    battle: &'a mut Battle,
}
