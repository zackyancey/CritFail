use std::fmt;

use crate::{CheckRoll, DamageRoll};

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
