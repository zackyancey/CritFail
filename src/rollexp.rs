use crate::Rollable;
use crate::ScoreRoll;
use crate::{Attack, Check, Damage};
use crate::{AttackRoll, CheckRoll, DamageRoll};

use rollexpparse::*;

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
        unimplemented!()
    }
}
