use crate::AdvState;
use crate::AdvState::*;
use crate::ModifiersOutcome;
use crate::OutcomePart;
use crate::Score;

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
    /// use critfail::{AdvState, CheckOutcome, OutcomePart};
    ///
    /// // (20)+4
    /// let critical = CheckOutcome::build(AdvState::Neutral, 20, 1, vec![OutcomePart::Modifier(4)]);
    /// // (16)+4
    /// let normal = CheckOutcome::build(AdvState::Neutral, 16, 1, vec![OutcomePart::Modifier(4)]);
    /// // (1)+4
    /// let fail = CheckOutcome::build(AdvState::Neutral, 1, 1, vec![OutcomePart::Modifier(4)]);
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
    /// use critfail::{AdvState, CheckOutcome, OutcomePart, CritScore};
    ///
    /// // (20)+4
    /// let critical = CheckOutcome::build(AdvState::Neutral, 20, 1, vec![OutcomePart::Modifier(4)]);
    /// // (16)+4
    /// let normal = CheckOutcome::build(AdvState::Neutral, 16, 1, vec![OutcomePart::Modifier(4)]);
    /// // (1)+4
    /// let fail = CheckOutcome::build(AdvState::Neutral, 1, 1, vec![OutcomePart::Modifier(4)]);
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

    /// Create a `CheckOutcome` without rolling an expression.
    ///
    /// *This function is only available if the [build-outcomes](index.html#features) feature is enabled*
    ///
    /// r1 and r2 represent the two values used for a check roll with
    /// advantage or disadvantage. If the roll is made with value of
    /// `AdvState::Neutral` for `adv`, r1 will be used.
    ///
    /// ```
    /// use critfail::{AdvState, CheckOutcome, OutcomePart};
    ///
    /// let outcome = CheckOutcome::build(
    ///     AdvState::Neutral,
    ///     10,
    ///     5,
    ///     vec![OutcomePart::Modifier(4)]
    /// );
    ///
    /// assert_eq!(outcome.score(), 14);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "(10)+4"
    /// );
    /// ```
    #[cfg(any(doc, feature = "build-outcomes"))]
    pub fn build(adv: AdvState, r1: Score, r2: Score, modifiers: Vec<OutcomePart>) -> Self {
        Self::new(adv, r1, r2, modifiers)
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
