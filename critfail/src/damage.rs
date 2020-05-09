use rand::Rng;

use crate::OutcomePart;
use crate::RollExpression;
use crate::{CheckOutcome, CritScore};
use crate::{Score, Sides};

mod damageoutcome;
mod damageparse;

pub use damageoutcome::{DamageOutcome, DamageOutcomeBuilder};

// TODO: DamagePart should not be `pub` (once Check stops using Damage)
#[derive(PartialEq, Debug, Clone)]
pub enum DamagePart {
    Dice(u32, Sides),
    Modifier(Score),
}

/// A list of dice to roll and modifiers to add, usually used for
/// damage.
///
/// ```
/// use critfail::{RollExpression, Damage};
///
/// let damage = Damage::new("2d8+1").unwrap();
///
/// let outcome = damage.roll();
///
/// print!("{}", outcome);   // Prints something like "11"
/// print!("{:?}", outcome); // Prints something like "[2+5]+4"
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Damage(pub Vec<DamagePart>);

impl Damage {
    /// Roll this `Damage` as though it were a critical hit.
    ///
    /// This rolls all the positive dice in the `Damage` twice.
    /// Modifiers and dice with negative values are only counted once.
    ///
    /// ```
    /// use critfail::{RollExpression, Damage};
    ///
    /// let damage = Damage::new("2d8+3-1d4").unwrap();
    /// let outcome = damage.crit_roll();
    /// // The d8s are rolled twice, the +3 is counted once, and the d4 is rolled once
    /// print!("{:?}", damage)  // Prints something like "[5+6+4+7]+3-[2]"
    /// ```
    pub fn crit_roll(&self) -> DamageOutcome {
        let mut result = Vec::new();

        for part in &self.0 {
            match part {
                DamagePart::Dice(_, sides) if *sides > 0 => {
                    result.push(part.roll());
                    result.push(part.roll());
                }
                _ => {
                    result.push(part.roll());
                }
            }
        }

        DamageOutcome::new(result)
    }

    /// Roll for damage, doubling if the check was a critical success.
    pub fn roll_with_check(&self, check: &CheckOutcome) -> DamageOutcome {
        self.roll_with_score(check.crit_score())
    }

    /// Roll for damage, doubling if the check was a critical success.
    pub fn roll_with_score(&self, score: CritScore) -> DamageOutcome {
        match score {
            CritScore::Critical => self.crit_roll(),
            _ => self.roll(),
        }
    }
}

impl RollExpression for Damage {
    type Outcome = DamageOutcome;

    fn roll(&self) -> Self::Outcome {
        DamageOutcome::new(self.0.iter().map(|part| part.roll()).collect())
    }
}

impl RollExpression for DamagePart {
    type Outcome = OutcomePart;

    fn roll(&self) -> Self::Outcome {
        match self {
            DamagePart::Dice(num, sides) => {
                let rolls: Vec<Score> = (0..*num)
                    .map(|_| rand::thread_rng().gen_range(1, sides.abs() + 1))
                    .collect();

                OutcomePart::Dice(*sides, rolls)
            }

            DamagePart::Modifier(value) => OutcomePart::Modifier(*value),
        }
    }
}
