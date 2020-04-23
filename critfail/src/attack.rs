use crate::CritScore;
use crate::RollExpression;
use crate::{Check, Damage};

pub use attackoutcome::*;

mod attackoutcome;
mod attackparse;

#[derive(PartialEq, Debug, Clone)]
pub struct Attack {
    pub check: Check,
    pub damage: Damage,
}

impl RollExpression for Attack {
    type Outcome = AttackOutcome;

    fn new(expression: &str) -> Result<Self, ()> {
        expression.parse().map_err(|_| ())
    }

    fn roll(&self) -> Self::Outcome {
        let check = self.check.roll();

        let damage = if let CritScore::Critical = check.crit_score() {
            self.damage.crit_roll()
        } else {
            self.damage.roll()
        };

        AttackOutcome::new(check, damage)
    }
}
