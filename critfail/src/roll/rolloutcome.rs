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
/// use critfail::{RollExpression, Roll, RollOutcome, Score};
///
/// let check: RollOutcome = Roll::new("r+6").unwrap().roll();
/// let damage: RollOutcome = Roll::new("4d4+6").unwrap().roll();
/// let attack: RollOutcome = Roll::new("r+3?2d8+3").unwrap().roll();
/// ```
#[derive(Clone)]
pub enum RollOutcome {
    /// The outcome of a `Roll` that contained a `Check`.
    CheckOutcome(CheckOutcome),
    /// The outcome of a `Roll` that contained a `Damage`.
    DamageOutcome(DamageOutcome),
    /// The outcome of a `Roll` that contained an `Attack`.
    AttackOutcome(AttackOutcome),
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
            Self::CheckOutcome(_) => true,
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
            Self::DamageOutcome(_) => true,
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
            Self::AttackOutcome(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for RollOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollOutcome::CheckOutcome(c) => write!(f, "{}", c),
            RollOutcome::DamageOutcome(d) => write!(f, "{}", d),
            RollOutcome::AttackOutcome(a) => write!(f, "{}", a),
        }
    }
}

impl fmt::Debug for RollOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RollOutcome::CheckOutcome(c) => write!(f, "{:?}", c),
            RollOutcome::DamageOutcome(d) => write!(f, "{:?}", d),
            RollOutcome::AttackOutcome(a) => write!(f, "{:?}", a),
        }
    }
}
