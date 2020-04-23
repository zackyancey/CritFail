use crate::RollExpression;
use crate::{Attack, Check, Damage};

pub use roll::*;

mod roll;
mod rollexpparse;

#[derive(PartialEq, Debug, Clone)]
pub enum RollExp {
    Check(Check),
    Damage(Damage),
    Attack(Attack),
}

impl RollExpression for RollExp {
    type Outcome = Roll;

    fn new(expression: &str) -> Result<Self, ()> {
        expression.parse().map_err(|_| ())
    }

    fn roll(&self) -> Self::Outcome {
        match self {
            RollExp::Check(c) => Roll::CheckRoll(c.roll()),
            RollExp::Damage(d) => Roll::DamageRoll(d.roll()),
            RollExp::Attack(a) => Roll::AttackRoll(a.roll()),
        }
    }
}
