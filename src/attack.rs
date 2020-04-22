use std::fmt;

use crate::Rollable;
use crate::{Check, CheckRoll, Damage, DamageRoll};

pub use attackroll::*;

mod attackparse;
mod attackroll;

#[derive(PartialEq, Debug)]
pub struct Attack {
    pub check: Check,
    pub damage: Damage,
}

impl Rollable for Attack {
    type Roll = AttackRoll;

    fn roll(&self) -> Self::Roll {
        unimplemented!()
    }
}
