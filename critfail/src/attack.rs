use crate::CritScore;
use crate::Rollable;
use crate::{Check, Damage};

pub use attackroll::*;

mod attackparse;
mod attackroll;

#[derive(PartialEq, Debug, Clone)]
pub struct Attack {
    pub check: Check,
    pub damage: Damage,
}

impl Rollable for Attack {
    type Roll = AttackRoll;

    fn roll(&self) -> Self::Roll {
        let check = self.check.roll();

        let damage = if let CritScore::Critical = check.crit_score() {
            self.damage.crit_roll()
        } else {
            self.damage.roll()
        };

        AttackRoll::new(check, damage)
    }
}
