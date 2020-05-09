use crate::AdvState;
use crate::AdvState::*;
use crate::ModifiersOutcome;
use crate::OutcomePart;
use crate::{Score, Sides};

use std::cmp::{max, min};
use std::fmt;

/// The score of a roll that could be a critical hit/failure
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CritScore {
    /// Critical success (rolled20 without modifiers).
    Critical,
    /// A normal roll.
    Normal(Score),
    /// Critical failure (rolled a 1 without modifiers).
    Fail,
}

/// The outcome of a check roll.
///
/// This struct is normally constructed as the result of calling
/// `roll()` on a `Check` roll expression.
///
/// ```
/// use critfail::{RollExpression, Check, CheckOutcome};
///
/// let outcome: CheckOutcome = Check::new("r+1").unwrap().roll();
/// ```
#[derive(Clone, PartialEq)]
pub struct CheckOutcome {
    main: Score,
    other: Option<Score>,
    modifiers: ModifiersOutcome,
}

impl CheckOutcome {
    pub(crate) fn new(adv: AdvState, r1: Score, r2: Score, modifiers: Vec<OutcomePart>) -> Self {
        let (main, other) = match adv {
            Advantage => (max(r1, r2), Some(min(r1, r2))),
            Disadvantage => (min(r1, r2), Some(max(r1, r2))),
            Neutral => (r1, None),
        };
        let modifiers = modifiers.into();

        CheckOutcome {
            main,
            other,
            modifiers,
        }
    }

    /// Get the score of a `CheckOutcome`.
    ///
    /// This is the plain numerical score of a roll, without accounting
    /// for critial success/failure
    ///
    /// ```
    /// use critfail::CheckOutcomeBuilder;
    ///
    /// // (20)+4
    /// let critical = CheckOutcomeBuilder::new()
    ///     .check(20)
    ///     .modifier(4)
    ///     .build();
    ///
    /// // (16)+4
    /// let normal = CheckOutcomeBuilder::new()
    ///     .check(16)
    ///     .modifier(4)
    ///     .build();
    /// // (1)+4
    /// let fail = CheckOutcomeBuilder::new()
    ///     .check(1)
    ///     .modifier(4)
    ///     .build();
    ///
    /// assert_eq!(critical.score(), 24);
    /// assert_eq!(normal.score(), 20);
    /// assert_eq!(fail.score(), 5);
    /// ```
    pub fn score(&self) -> Score {
        self.main + self.modifiers.score()
    }

    /// Get the score of a `CheckOutcome` that could be a critical success/failure.
    ///
    /// ```
    /// use critfail::{CheckOutcomeBuilder, CritScore};
    ///
    /// // (20)+4
    /// let critical = CheckOutcomeBuilder::new()
    ///     .check(20)
    ///     .modifier(4)
    ///     .build();
    ///
    /// // (16)+4
    /// let normal = CheckOutcomeBuilder::new()
    ///     .check(16)
    ///     .modifier(4)
    ///     .build();
    /// // (1)+4
    /// let fail = CheckOutcomeBuilder::new()
    ///     .check(1)
    ///     .modifier(4)
    ///     .build();
    ///
    /// assert_eq!(critical.crit_score(), CritScore::Critical);
    /// assert_eq!(normal.crit_score(), CritScore::Normal(20));
    /// assert_eq!(fail.crit_score(), CritScore::Fail);
    /// ```
    pub fn crit_score(&self) -> CritScore {
        match self.main {
            1 => CritScore::Fail,
            20 => CritScore::Critical,
            _ => CritScore::Normal(self.score()),
        }
    }
}

impl fmt::Display for CheckOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.crit_score() {
            CritScore::Critical => write!(f, "Critical"),
            CritScore::Normal(score) => write!(f, "{}", score),
            CritScore::Fail => write!(f, "Fail"),
        }
    }
}

impl fmt::Debug for CheckOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(other) = self.other {
            write!(f, "({}/{})", self.main, other)?
        } else {
            write!(f, "({})", self.main)?
        }

        let mods = format!("{:?}", self.modifiers);

        if mods != "" {
            if !(mods.starts_with('+') || mods.starts_with('-')) {
                write!(f, "+")?
            }
            write!(f, "{}", mods)?
        }

        Ok(())
    }
}

/// This is used to create a 'fudged' `CheckOutcome` without actually
/// randomly generating anything.
///
/// ```
/// use critfail::CheckOutcomeBuilder;
/// // To create a result that could come from rolling 'r+4-1d4'
/// let outcome = CheckOutcomeBuilder::new()
///     .check(10)
///     .modifier(4)
///     .dice(-4, vec![2])
///     .build();
///
/// assert_eq!(outcome.score(), 12);
/// assert_eq!(
///     format!("{:?}", outcome),
///     "(10)+4-[2]"
/// );
/// ```
#[derive(Default)]
pub struct CheckOutcomeBuilder {
    adv: AdvState,
    r1: Score,
    r2: Score,
    modifiers: Vec<OutcomePart>,
}

impl CheckOutcomeBuilder {
    /// Create a new CheckOutcomeBuilder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the check roll with advantage.
    ///
    /// ```
    /// use critfail::CheckOutcomeBuilder;
    ///
    /// let outcome = CheckOutcomeBuilder::new()
    ///     .check_adv(10, 12)
    ///     .build();
    ///
    /// assert_eq!(outcome.score(), 12);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(12/10)"
    /// );
    /// ```
    pub fn check_adv(self, r1: Score, r2: Score) -> Self {
        Self {
            adv: AdvState::Advantage,
            r1,
            r2,
            ..self
        }
    }

    /// Set the check roll with disadvantage.
    ///
    /// ```
    /// use critfail::CheckOutcomeBuilder;
    ///
    ///
    /// let outcome = CheckOutcomeBuilder::new()
    ///     .check_dis(10, 12)
    ///     .build();
    ///
    /// assert_eq!(outcome.score(), 10);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(10/12)"
    /// );
    /// ```
    pub fn check_dis(self, r1: Score, r2: Score) -> Self {
        Self {
            adv: AdvState::Disadvantage,
            r1,
            r2,
            ..self
        }
    }

    /// Set the check roll without advantage or disadvantage.
    ///
    /// ```
    /// use critfail::CheckOutcomeBuilder;
    ///
    ///
    /// let outcome = CheckOutcomeBuilder::new()
    ///     .check(8)
    ///     .build();
    ///
    /// assert_eq!(outcome.score(), 8);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(8)"
    /// );
    /// ```
    pub fn check(self, r: Score) -> Self {
        Self {
            adv: AdvState::Neutral,
            r1: r,
            ..self
        }
    }

    /// Add a constant modifier to the roll. This method can be chained
    /// multiple times for multiple modifiers.
    ///
    /// ```
    /// use critfail::CheckOutcomeBuilder;
    ///
    ///
    /// // To create a result that could come from rolling 'r-2+6'
    /// let outcome = CheckOutcomeBuilder::new()
    ///     .check(10)
    ///     .modifier(-2)
    ///     .modifier(6)
    ///     .build();
    ///
    /// assert_eq!(outcome.score(), 14);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(10)-2+6"
    /// );
    /// ```
    pub fn modifier(self, modifier: Score) -> Self {
        let mut modifiers = self.modifiers;
        modifiers.push(OutcomePart::Modifier(modifier));
        Self { modifiers, ..self }
    }

    /// Add a dice modifier to the roll. This method can be chained
    /// multiple times for multiple modifiers. `sides` specifies the die
    /// that was rolled.
    ///
    /// ```
    /// use critfail::CheckOutcomeBuilder;
    ///
    ///
    /// // To create a result that could come from rolling 'r-2d4+1d6'
    /// let outcome = CheckOutcomeBuilder::new()
    ///     .check(10)
    ///     .dice(-4, vec![1, 2])
    ///     .dice(6, vec![5])
    ///     .build();
    ///
    /// assert_eq!(outcome.score(), 12);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(10)-[1+2]+[5]"
    /// );
    /// ```
    pub fn dice(self, sides: Sides, scores: Vec<Score>) -> Self {
        let mut modifiers = self.modifiers;
        modifiers.push(OutcomePart::Dice(sides, scores));
        Self { modifiers, ..self }
    }

    /// Create a CheckOutcome from this builder.
    pub fn build(self) -> CheckOutcome {
        CheckOutcome::new(self.adv, self.r1, self.r2, self.modifiers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OutcomePart::Dice as D;
    use crate::OutcomePart::Modifier as M;

    #[test]
    fn neutral() {
        let r = CheckOutcome::new(Neutral, 10, 16, vec![]);
        assert_eq!(r.score(), 10);
        assert_eq!(format!("{}", r), "10");
        assert_eq!(format!("{:?}", r), "(10)");
    }

    #[test]
    fn advantage() {
        let r = CheckOutcome::new(Advantage, 8, 15, vec![]);
        assert_eq!(r.score(), 15);
        assert_eq!(format!("{}", r), "15");
        assert_eq!(format!("{:?}", r), "(15/8)");
    }

    #[test]
    fn disadvantage() {
        let r = CheckOutcome::new(Disadvantage, 12, 7, vec![]);
        assert_eq!(r.score(), 7);
        assert_eq!(format!("{}", r), "7");
        assert_eq!(format!("{:?}", r), "(7/12)");
    }

    #[test]
    fn die_modifier() {
        let r = CheckOutcome::new(Neutral, 6, 15, vec![D(4, vec![1])]);
        assert_eq!(r.score(), 7);
        assert_eq!(format!("{}", r), "7");
        assert_eq!(format!("{:?}", r), "(6)+[1]");
    }

    #[test]
    fn mixed_modifiers() {
        let r = CheckOutcome::new(Advantage, 12, 4, vec![D(-4, vec![2, 3]), M(3)]);
        assert_eq!(r.score(), 10);
        assert_eq!(format!("{}", r), "10");
        assert_eq!(format!("{:?}", r), "(12/4)-[2+3]+3");
    }

    #[test]
    fn critical() {
        let r = CheckOutcome::new(Advantage, 20, 4, vec![D(-4, vec![2, 3]), M(3)]);
        assert_eq!(r.score(), 18);
        assert_eq!(format!("{}", r), "Critical");
        assert_eq!(format!("{:?}", r), "(20/4)-[2+3]+3");
    }

    #[test]
    fn fail() {
        let r = CheckOutcome::new(Disadvantage, 1, 4, vec![D(-4, vec![2, 3]), M(3)]);
        assert_eq!(r.score(), -1);
        assert_eq!(format!("{}", r), "Fail");
        assert_eq!(format!("{:?}", r), "(1/4)-[2+3]+3");
    }
}
