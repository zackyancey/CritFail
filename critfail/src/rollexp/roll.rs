use std::fmt;

use crate::{AttackRoll, CheckRoll, DamageRoll};

#[derive(Clone)]
pub enum Roll {
    CheckRoll(CheckRoll),
    DamageRoll(DamageRoll),
    AttackRoll(AttackRoll),
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Roll::CheckRoll(c) => write!(f, "{}", c),
            Roll::DamageRoll(d) => write!(f, "{}", d),
            Roll::AttackRoll(a) => write!(f, "{}", a),
        }
    }
}

impl fmt::Debug for Roll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Roll::CheckRoll(c) => write!(f, "{:?}", c),
            Roll::DamageRoll(d) => write!(f, "{:?}", d),
            Roll::AttackRoll(a) => write!(f, "{:?}", a),
        }
    }
}
