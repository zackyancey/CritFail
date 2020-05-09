use std::fmt;

use crate::{CheckOutcome, DamageOutcome};

/// The outcome of an attack roll.
///
/// This struct is normally constructed as the result of calling
/// `roll()` on an `Attack` roll expression.
///
/// ```
/// use critfail::{RollExpression, Attack, AttackOutcome};
///
/// let outcome: AttackOutcome = Attack::new("r+1?1d12+1").unwrap().roll();
/// ```
#[derive(Clone, PartialEq)]
pub struct AttackOutcome {
    check: CheckOutcome,
    damage: DamageOutcome,
}

impl AttackOutcome {
    pub(crate) fn new(check: CheckOutcome, damage: DamageOutcome) -> Self {
        AttackOutcome { check, damage }
    }

    /// Get the check portion of this `AttackOutcome`.
    pub fn check(&self) -> &CheckOutcome {
        &self.check
    }

    /// Get the damage portion of this `AttackOutcome`.
    pub fn damage(&self) -> &DamageOutcome {
        &self.damage
    }

    ///  Create an attack outcome without rolling a check.
    ///
    /// *This function is only available if the [build-outcomes](index.html#features) feature is enabled*
    ///
    /// ```
    /// use critfail::{DamageOutcomeBuilder, CheckOutcomeBuilder, AttackOutcome};
    ///
    /// let damage = DamageOutcomeBuilder::new()
    ///     .dice(6, vec![4,6,1])
    ///     .modifier(4)
    ///     .build();
    /// let check = CheckOutcomeBuilder::new()
    ///     .check(10)
    ///     .modifier(4)
    ///     .build();
    ///
    /// let outcome = AttackOutcome::build(check, damage);
    /// ```
    #[cfg(any(doc, feature = "build-outcomes"))]
    pub fn build(check: CheckOutcome, damage: DamageOutcome) -> Self {
        Self::new(check, damage)
    }
}

impl fmt::Display for AttackOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ? {}", self.check, self.damage)
    }
}

impl fmt::Debug for AttackOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ? {:?}", self.check, self.damage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AdvState::*;
    use crate::OutcomePart::Dice as D;
    use crate::OutcomePart::Modifier as M;

    #[test]
    fn no_modifier() {
        let r = AttackOutcome::new(
            CheckOutcome::new(Neutral, 10, 16, vec![]),
            DamageOutcome::new(vec![D(6, vec![5, 4]), M(4)]),
        );

        assert_eq!(format!("{}", r), "10 ? 13");
        assert_eq!(format!("{:?}", r), "(10) ? [5+4]+4");
    }

    #[test]
    fn with_modifier() {
        let r = AttackOutcome::new(
            CheckOutcome::new(Disadvantage, 5, 12, vec![M(3)]),
            DamageOutcome::new(vec![D(8, vec![2, 6, 8]), M(-2)]),
        );

        assert_eq!(format!("{}", r), "8 ? 14");
        assert_eq!(format!("{:?}", r), "(5/12)+3 ? [2+6+8]-2");
    }

    #[test]
    fn critical() {
        let r = AttackOutcome::new(
            CheckOutcome::new(Advantage, 20, 4, vec![M(3)]),
            DamageOutcome::new(vec![D(8, vec![2, 6, 8]), D(8, vec![1, 5, 2]), M(-2)]),
        );

        assert_eq!(format!("{}", r), "Critical ? 22");
        assert_eq!(format!("{:?}", r), "(20/4)+3 ? [2+6+8]+[1+5+2]-2");
    }

    #[test]
    fn critfail() {
        let r = AttackOutcome::new(
            CheckOutcome::new(Disadvantage, 15, 1, vec![M(3)]),
            DamageOutcome::new(vec![D(8, vec![3, 1]), M(-2)]),
        );

        assert_eq!(format!("{}", r), "Fail ? 2");
        assert_eq!(format!("{:?}", r), "(1/15)+3 ? [3+1]-2");
    }
}
