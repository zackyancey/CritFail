use std::fmt;

use crate::{AttackOutcome, CheckOutcome, DamageOutcome};

/// The outcome of rolling a check, damage, or attack roll.
///
/// This is normally constructed as the result of calling `roll()` on a
/// `Roll` roll expression.
///
/// A `RollOutcome` can be printed with `Display` and `Debug`, but if
/// you need more information about the result you will need to
/// destructure the enum and handle the different types individually.
///
/// ```
/// use critfail::{RollExpression, Roll, RollOutcome};
///
/// let check: RollOutcome = Roll::new("r+6").unwrap().roll();
/// let damage: RollOutcome = Roll::new("4d4+6").unwrap().roll();
/// let attack: RollOutcome = Roll::new("r+3?2d8+3").unwrap().roll();
///
/// fn print_score(outcome: RollOutcome) {
///     match outcome {
///         RollOutcome::Check(check) => println!("Check score: {}", check.score()),
///         RollOutcome::Damage(damage) => println!("Damage score: {}", damage.score()),
///         RollOutcome::Attack(attack) => {
///             println!("Check score: {}", attack.check().score());
///             println!("Damage score: {}", attack.damage().score())
///         }
///     }
/// }
///
/// print_score(check);
/// print_score(damage);
/// print_score(attack);
/// ```
#[derive(Clone)]
pub enum RollOutcome {
    /// The outcome of a `Roll` that contained a `Check`.
    Check(CheckOutcome),
    /// The outcome of a `Roll` that contained a `Damage`.
    Damage(DamageOutcome),
    /// The outcome of a `Roll` that contained an `Attack`.
    Attack(AttackOutcome),
}

impl RollOutcome {
    /// Return true if this `RollOutcome` is the outcome of a check roll.
    ///
    /// ```
    /// use critfail::{RollExpression, Roll};
    ///
    /// assert_eq!(Roll::new("r+3").unwrap().roll().is_check(), true);
    /// assert_eq!(Roll::new("2d8+5").unwrap().roll().is_check(), false);
    /// assert_eq!(Roll::new("r+3?2d8+5").unwrap().roll().is_check(), false);
    /// ```
    pub fn is_check(&self) -> bool {
        match self {
            Self::Check(_) => true,
            _ => false,
        }
    }

    /// Return true if this `RollOutcome` is the outcome of a damage roll.
    ///
    /// ```
    /// use critfail::{RollExpression, Roll};
    ///
    /// assert_eq!(Roll::new("r+3").unwrap().roll().is_damage(), false);
    /// assert_eq!(Roll::new("2d8+5").unwrap().roll().is_damage(), true);
    /// assert_eq!(Roll::new("r+3?2d8+5").unwrap().roll().is_damage(), false);
    /// ```
    pub fn is_damage(&self) -> bool {
        match self {
            Self::Damage(_) => true,
            _ => false,
        }
    }

    /// Return true if this `RollOutcome` is the outcome of an attack roll.
    ///
    /// ```
    /// use critfail::{RollExpression, Roll};
    ///
    /// assert_eq!(Roll::new("r+3").unwrap().roll().is_attack(), false);
    /// assert_eq!(Roll::new("2d8+5").unwrap().roll().is_attack(), false);
    /// assert_eq!(Roll::new("r+3?2d8+5").unwrap().roll().is_attack(), true);
    /// ```
    pub fn is_attack(&self) -> bool {
        match self {
            Self::Attack(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for RollOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollOutcome::Check(c) => write!(f, "{}", c),
            RollOutcome::Damage(d) => write!(f, "{}", d),
            RollOutcome::Attack(a) => write!(f, "{}", a),
        }
    }
}

impl fmt::Debug for RollOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RollOutcome::Check(c) => write!(f, "{:?}", c),
            RollOutcome::Damage(d) => write!(f, "{:?}", d),
            RollOutcome::Attack(a) => write!(f, "{:?}", a),
        }
    }
}

impl From<CheckOutcome> for RollOutcome {
    fn from(o: CheckOutcome) -> Self {
        Self::Check(o)
    }
}

impl From<DamageOutcome> for RollOutcome {
    fn from(o: DamageOutcome) -> Self {
        Self::Damage(o)
    }
}

impl From<AttackOutcome> for RollOutcome {
    fn from(o: AttackOutcome) -> Self {
        Self::Attack(o)
    }
}
