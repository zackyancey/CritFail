use rand::Rng;

use crate::Damage;
use crate::RollExpression;

pub use checkoutcome::{CheckOutcome, CritScore};

mod checkoutcome;
mod checkparse;

/// The advantage state of an ability check.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AdvState {
    /// Check rolled with advantage (roll twice, take the higher value).
    Advantage,
    /// Check rolled with no advantage (only roll once).
    Neutral,
    /// Check rolled with disadvantage (roll twice, take the lower value).
    Disadvantage,
}

/// An ability check - roll a d20, potentially with modifiers or
/// advantage.
///
/// ```
/// use critfail::{RollExpression, Check};
///
/// let check = Check::new("r+4").unwrap();
///
/// let outcome = check.roll();
///
/// print!("{}", outcome);   // Prints something like "16"
/// print!("{:?}", outcome); // Prints something like "(12)+4"
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct Check {
    adv: AdvState,
    // TODO: the modifier should be a Vec<Modifier> instead of a Damage
    modifier: Damage,
}

impl Check {
    /// Roll this check using `adv` to override the advantage state.
    ///
    /// ```
    /// use critfail::{RollExpression, Check, AdvState};
    /// let check = Check::new("r+3").unwrap();
    ///
    /// check.roll(); // Roll without advantage
    /// check.roll_with_advantage(AdvState::Advantage); // Roll with advantage
    /// check.roll_with_advantage(AdvState::Neutral); // Roll without advantage
    /// check.roll_with_advantage(AdvState::Disadvantage); // Roll with disadvantage
    /// ```
    pub fn roll_with_advantage(&self, adv: AdvState) -> CheckOutcome {
        let r1 = rand::thread_rng().gen_range(1, 21);
        let r2 = rand::thread_rng().gen_range(1, 21);
        let mods = self.modifier.roll();
        CheckOutcome::new(adv, r1, r2, mods.into_modifiers().into_inner())
    }
}

impl RollExpression for Check {
    type Outcome = CheckOutcome;

    fn new(expression: &str) -> Result<Self, ()> {
        expression.parse().map_err(|_| ())
    }

    fn roll(&self) -> Self::Outcome {
        self.roll_with_advantage(self.adv)
    }
}
