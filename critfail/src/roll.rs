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

    fn roll(&self) -> Self::Outcome {
        match self {
            Roll::Check(c) => c.roll().into(),
            Roll::Damage(d) => d.roll().into(),
            Roll::Attack(a) => a.roll().into(),
        }
    }
}

impl Roll {
    /// Return true if this `Roll` is a check roll.
    ///
    /// ```
    /// use critfail::{RollExpression, Roll};
    ///
    /// assert_eq!(Roll::new("r+3").unwrap().is_check(), true);
    /// assert_eq!(Roll::new("2d8+5").unwrap().is_check(), false);
    /// assert_eq!(Roll::new("r+3?2d8+5").unwrap().is_check(), false);
    /// ```
    pub fn is_check(&self) -> bool {
        match self {
            Self::Check(_) => true,
            _ => false,
        }
    }

    /// Return true if this `Roll` is a damage roll.
    ///
    /// ```
    /// use critfail::{RollExpression, Roll};
    ///
    /// assert_eq!(Roll::new("r+3").unwrap().is_damage(), false);
    /// assert_eq!(Roll::new("2d8+5").unwrap().is_damage(), true);
    /// assert_eq!(Roll::new("r+3?2d8+5").unwrap().is_damage(), false);
    /// ```
    pub fn is_damage(&self) -> bool {
        match self {
            Self::Damage(_) => true,
            _ => false,
        }
    }

    /// Return true if this `Roll` is an attack roll.
    ///
    /// ```
    /// use critfail::{RollExpression, Roll};
    ///
    /// assert_eq!(Roll::new("r+3").unwrap().is_attack(), false);
    /// assert_eq!(Roll::new("2d8+5").unwrap().is_attack(), false);
    /// assert_eq!(Roll::new("r+3?2d8+5").unwrap().is_attack(), true);
    /// ```
    pub fn is_attack(&self) -> bool {
        match self {
            Self::Attack(_) => true,
            _ => false,
        }
    }
}

impl From<Check> for Roll {
    fn from(check: Check) -> Self {
        Self::Check(check)
    }
}

impl From<Damage> for Roll {
    fn from(damage: Damage) -> Self {
        Self::Damage(damage)
    }
}

impl From<Attack> for Roll {
    fn from(attack: Attack) -> Self {
        Self::Attack(attack)
    }
}
