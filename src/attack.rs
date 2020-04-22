use std::fmt;

use crate::Rollable;
use crate::{Check, CheckRoll, Damage, DamageRoll};

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

pub struct AttackRoll {
    pub check: CheckRoll,
    pub damage: DamageRoll,
}

impl fmt::Display for AttackRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for AttackRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
