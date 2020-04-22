use crate::{Check, CheckRoll, Damage, DamageRoll};
use crate::{Rollable, DisplayRoll};

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


impl DisplayRoll for AttackRoll {
    fn display_score(&self) -> String {
        unimplemented!()
    }

    fn display_parts(&self) -> String {
        unimplemented!()
    }
}
