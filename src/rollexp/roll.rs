use std::fmt;

use crate::{AttackRoll, CheckRoll, DamageRoll};

pub enum Roll {
    CheckRoll(CheckRoll),
    DamageRoll(DamageRoll),
    AttackRoll(AttackRoll),
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for Roll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
