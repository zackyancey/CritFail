use std::fmt;

use crate::{AttackOutcome, CheckOutcome, DamageOutcome};

#[derive(Clone)]
pub enum RollOutcome {
    CheckOutcome(CheckOutcome),
    DamageOutcome(DamageOutcome),
    AttackOutcome(AttackOutcome),
}

impl fmt::Display for RollOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollOutcome::CheckOutcome(c) => write!(f, "{}", c),
            RollOutcome::DamageOutcome(d) => write!(f, "{}", d),
            RollOutcome::AttackOutcome(a) => write!(f, "{}", a),
        }
    }
}

impl fmt::Debug for RollOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RollOutcome::CheckOutcome(c) => write!(f, "{:?}", c),
            RollOutcome::DamageOutcome(d) => write!(f, "{:?}", d),
            RollOutcome::AttackOutcome(a) => write!(f, "{:?}", a),
        }
    }
}
