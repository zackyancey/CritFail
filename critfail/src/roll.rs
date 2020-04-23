use crate::RollExpression;
use crate::{Attack, Check, Damage};

pub use rolloutcome::*;

mod rolloutcome;
mod rollparse;

#[derive(PartialEq, Debug, Clone)]
pub enum Roll {
    Check(Check),
    Damage(Damage),
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
