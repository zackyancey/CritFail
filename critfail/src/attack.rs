use crate::ParseError;
use crate::RollExpression;
use crate::{AdvState, Check, Damage};

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

impl Attack {
    /// Roll this check using `adv` to override the advantage state.
    ///
    /// ```
    /// use critfail::{RollExpression, Attack, AdvState};
    /// let attack = Attack::new("r+3?3d6+4").unwrap();
    ///
    /// attack.roll(); // Roll without advantage
    /// attack.roll_with_advantage(AdvState::Advantage); // Roll with advantage
    /// attack.roll_with_advantage(AdvState::Neutral); // Roll without advantage
    /// attack.roll_with_advantage(AdvState::Disadvantage); // Roll with disadvantage
    /// ```
    pub fn roll_with_advantage(&self, adv: AdvState) -> AttackOutcome {
        let check = self.check.roll_with_advantage(adv);
        let damage = self.damage.roll_with_check(&check);

        AttackOutcome::new(check, damage)
    }
}

impl RollExpression for Attack {
    type Outcome = AttackOutcome;

    fn new(expression: &str) -> Result<Self, ParseError> {
        expression.parse()
    }

    fn roll(&self) -> Self::Outcome {
        let check = self.check.roll();
        let damage = self.damage.roll_with_check(&check);

        AttackOutcome::new(check, damage)
    }
}
