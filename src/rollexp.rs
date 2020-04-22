use crate::Rollable;
use crate::{Attack, Check, Damage};

pub use roll::*;

mod roll;
mod rollexpparse;

#[derive(PartialEq, Debug)]
pub enum RollExp {
    Check(Check),
    Damage(Damage),
    Attack(Attack),
}

impl Rollable for RollExp {
    type Roll = Roll;

    fn roll(&self) -> Self::Roll {
        match self {
            RollExp::Check(c) => Roll::CheckRoll(c.roll()),
            RollExp::Damage(d) => Roll::DamageRoll(d.roll()),
            RollExp::Attack(a) => Roll::AttackRoll(a.roll()),
        }
    }
}
