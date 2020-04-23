use rand::Rng;

use crate::Damage;
use crate::RollExpression;

pub use checkroll::*;

mod checkparse;
mod checkroll;

/// The advantage state of an ability check
#[derive(PartialEq, Debug, Clone)]
pub enum AdvState {
    Advantage,
    Neutral,
    Disadvantage,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Check {
    adv: AdvState,
    modifier: Damage,
}

impl RollExpression for Check {
    type Outcome = CheckRoll;

    fn new(expression: &str) -> Result<Self, ()> {
        expression.parse().map_err(|_| ())
    }

    fn roll(&self) -> Self::Outcome {
        let r1 = rand::thread_rng().gen_range(1, 21);
        let r2 = rand::thread_rng().gen_range(1, 21);
        let mods = self.modifier.roll();
        CheckRoll::new(&self.adv, r1, r2, mods)
    }
}
