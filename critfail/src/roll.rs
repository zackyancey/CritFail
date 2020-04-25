use crate::RollExpression;
use crate::{Attack, Check, Damage};

pub use rolloutcome::RollOutcome;

mod rolloutcome;
mod rollparse;

/// Any kind of roll—either a check, damage, or attack roll.
///
/// This struct is useful when parsing a roll expression if you don't
/// know what type of roll expression it will be.
///
/// ```
/// use critfail::{RollExpression, Roll};
///
/// let check = Roll::new("r-3").unwrap();
/// let check_outcome = check.roll();
/// print!("{}", check_outcome); // eg. "11"
/// print!("{:?}", check_outcome); // eg. "(14)-3"
///
/// let damage = Roll::new("2d8+6").unwrap();
/// let damage_outcome = damage.roll();
/// print!("{}", damage_outcome); // eg. "13"
/// print!("{:?}", damage_outcome); // eg. "[2+5]+6"
///
/// let attack = Roll::new("r+1?2d6+4").unwrap();
/// let attack_outcome = attack.roll();
/// print!("{}", attack_outcome); // eg. "10 ? 16"
/// print!("{:?}", attack_outcome); // eg. "(9)+1 ? [6+6]+4"
/// ```
#[derive(PartialEq, Debug, Clone)]
pub enum Roll {
    /// A `Roll` containing a `Check` roll.
    Check(Check),
    /// A `Roll` containing a `Damage` roll.
    Damage(Damage),
    /// A `Roll` containing an `Attack` roll.
    Attack(Attack),
}

impl RollExpression for Roll {
    type Outcome = RollOutcome;

    fn new(expression: &str) -> Result<Self, ()> {
        expression.parse().map_err(|_| ())
    }

    fn roll(&self) -> Self::Outcome {
        match self {
            Roll::Check(c) => RollOutcome::CheckOutcome(c.roll()),
            Roll::Damage(d) => RollOutcome::DamageOutcome(d.roll()),
            Roll::Attack(a) => RollOutcome::AttackOutcome(a.roll()),
        }
    }
}
