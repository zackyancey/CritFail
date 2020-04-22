use crate::{Check, CheckRoll, Damage, DamageRoll};

#[derive(PartialEq, Debug)]
pub struct Attack {
    // TODO: impl Rollable
    pub check: Check,
    pub damage: Damage,
}

pub struct AttackRoll {
    // TODO: impl DisplayRoll
    pub check: CheckRoll,
    pub damage: DamageRoll,
}
