use crate::CritScore;
use crate::RollExpression;
use crate::{Check, Damage};

pub use attackoutcome::AttackOutcome;

mod attackoutcome;
mod attackparse;

/// An attack roll consisting of a check and a damage roll.
///
/// ```
/// use critfail::{RollExpression, Attack};
///
/// let atk = Attack::new("r+3?2d8+4").unwrap();
///
/// let outcome = atk.roll();
///
/// print!("{}", outcome);   // Prints something like "10 ? 13"
/// print!("{:?}", outcome); // Prints something like "(7)+3 ? [4+5]+4"
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct Attack {
    check: Check,
    damage: Damage,
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
