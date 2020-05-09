use std::fmt;

use crate::{CheckOutcome, CheckOutcomeBuilder, DamageOutcome, DamageOutcomeBuilder, Score, Sides};

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

/// This is used to create a 'fudged' `AttackOutcome` without actually
/// randomly generating anything.
///
/// ```
/// use critfail::AttackOutcomeBuilder;
/// // To create a result that could come from rolling 'r+4?3d10+1'
/// let outcome = AttackOutcomeBuilder::new()
///     .check(5)
///     .check_modifier(4)
///     .damage_dice(3, vec![5, 8, 10])
///     .damage_modifier(1)
///     .build();
///
/// assert_eq!(outcome.check().score(), 9);
/// assert_eq!(outcome.damage().score(), 24);
/// assert_eq!(
///     format!("{:?}", outcome),
///     "(5)+4 ? [5+8+10]+1"
/// );
/// ```
#[derive(Default)]
pub struct AttackOutcomeBuilder {
    check: CheckOutcomeBuilder,
    damage: DamageOutcomeBuilder,
}

impl AttackOutcomeBuilder {
    /// Create a new `AttackOutcomeBuilder`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the check roll with advantage.
    ///
    /// ```
    /// use critfail::AttackOutcomeBuilder;
    ///
    /// // To create a result that could come from rolling 'a?2d8'
    /// let outcome = AttackOutcomeBuilder::new()
    ///     .check_adv(10, 12)
    ///     .damage_dice(8, vec![4, 5])
    ///     .build();
    ///
    /// assert_eq!(outcome.check().score(), 12);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(12/10) ? [4+5]"
    /// );
    /// ```
    pub fn check_adv(self, r1: Score, r2: Score) -> Self {
        Self {
            check: self.check.check_adv(r1, r2),
            ..self
        }
    }

    /// Set the check roll with disadvantage.
    ///
    /// ```
    /// use critfail::AttackOutcomeBuilder;
    ///
    ///
    /// // To create a result that could come from rolling 'd?2d8'
    /// let outcome = AttackOutcomeBuilder::new()
    ///     .check_dis(10, 12)
    ///     .damage_dice(8, vec![4, 5])
    ///     .build();
    ///
    /// assert_eq!(outcome.check().score(), 10);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(10/12) ? [4+5]"
    /// );
    /// ```
    pub fn check_dis(self, r1: Score, r2: Score) -> Self {
        Self {
            check: self.check.check_dis(r1, r2),
            ..self
        }
    }

    /// Set the check roll without advantage or disadvantage.
    ///
    /// ```
    /// use critfail::AttackOutcomeBuilder;
    ///
    /// // To create a result that could come from rolling 'r?2d8'
    /// let outcome = AttackOutcomeBuilder::new()
    ///     .check(8)
    ///     .damage_dice(8, vec![4, 5])
    ///     .build();
    ///
    /// assert_eq!(outcome.check().score(), 8);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(8) ? [4+5]"
    /// );
    /// ```
    pub fn check(self, r: Score) -> Self {
        Self {
            check: self.check.check(r),
            ..self
        }
    }

    /// Add a constant modifier to the check part of this attack roll.
    /// This method can be chained multiple times for multiple
    /// modifiers.
    ///
    /// ```
    /// use critfail::AttackOutcomeBuilder;
    ///
    /// // To create a result that could come from rolling 'r-2+6?2d8'
    /// let outcome = AttackOutcomeBuilder::new()
    ///     .check(10)
    ///     .check_modifier(-2)
    ///     .check_modifier(6)
    ///     .damage_dice(8, vec![4, 5])
    ///     .build();
    ///
    /// assert_eq!(outcome.check().score(), 14);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(10)-2+6 ? [4+5]"
    /// );
    /// ```
    pub fn check_modifier(self, modifier: Score) -> Self {
        Self {
            check: self.check.modifier(modifier),
            ..self
        }
    }

    /// Add a dice modifier to the check part of this attack roll. This
    /// method can be chained multiple times for multiple modifiers.
    /// `sides` specifies the die that was rolled.
    ///
    /// ```
    /// use critfail::AttackOutcomeBuilder;
    ///
    /// // To create a result that could come from rolling 'r-2d4+1d6?2d8'
    /// let outcome = AttackOutcomeBuilder::new()
    ///     .check(10)
    ///     .check_dice(-4, vec![1, 2])
    ///     .check_dice(6, vec![5])
    ///     .damage_dice(8, vec![4, 5])
    ///     .build();
    ///
    /// assert_eq!(outcome.check().score(), 12);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(10)-[1+2]+[5] ? [4+5]"
    /// );
    /// ```
    pub fn check_dice(self, sides: Sides, scores: Vec<Score>) -> Self {
        Self {
            check: self.check.dice(sides, scores),
            ..self
        }
    }

    /// Add a constant modifier to the damage part of this attack roll.
    /// This method can be chained multiple times for multiple
    /// modifiers.
    ///
    /// ```
    /// use critfail::AttackOutcomeBuilder;
    ///
    /// // To create a result that could come from rolling 'r?2d8+6-4'
    /// let outcome = AttackOutcomeBuilder::new()
    ///     .check(10)
    ///     .damage_dice(8, vec![1,6])
    ///     .damage_modifier(6)
    ///     .damage_modifier(-4)
    ///     .build();
    ///
    /// assert_eq!(outcome.damage().score(), 9);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(10) ? [1+6]+6-4"
    /// );
    /// ```
    pub fn damage_modifier(self, modifier: Score) -> Self {
        Self {
            damage: self.damage.modifier(modifier),
            ..self
        }
    }

    /// Add a dice modifier to the damage part of this attack roll. This
    /// method can be chained multiple times for multiple modifiers.
    /// `sides` specifies the die that was rolled.
    ///
    /// ```
    /// use critfail::AttackOutcomeBuilder;
    ///
    /// // To create a result that could come from rolling 'r?2d6+3d10-1d4'
    /// let outcome = AttackOutcomeBuilder::new()
    ///     .check(10)
    ///     .damage_dice(6, vec![5, 1])
    ///     .damage_dice(10, vec![4, 9, 2])
    ///     .damage_dice(-4, vec![3])
    ///     .build();
    ///
    /// assert_eq!(outcome.damage().score(), 18);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(10) ? [5+1]+[4+9+2]-[3]"
    /// );
    /// ```
    pub fn damage_dice(self, sides: Sides, scores: Vec<Score>) -> Self {
        Self {
            damage: self.damage.dice(sides, scores),
            ..self
        }
    }

    /// Create an `AttackOutcome` from this builder.
    pub fn build(self) -> AttackOutcome {
        let check = self.check.build();
        let damage = self.damage.build();
        AttackOutcome::new(check, damage)
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
