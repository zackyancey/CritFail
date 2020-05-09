use std::fmt;

use crate::{ModifiersOutcome, OutcomePart, Score, Sides};

/// The outcome of a check roll.
///
/// This is normally constructed as the result of calling `roll()` on a
/// `Damage` roll expression.
///
/// ```
/// use critfail::{RollExpression, Damage, DamageOutcome};
///
/// let outcome: DamageOutcome = Damage::new("2d6+6").unwrap().roll();
/// ```
#[derive(Clone, PartialEq)]
pub struct DamageOutcome {
    scores: ModifiersOutcome,
}

impl DamageOutcome {
    pub(crate) fn new(scores: Vec<OutcomePart>) -> Self {
        Self {
            scores: scores.into(),
        }
    }

    /// Get the score of a `DamageOutcome`.
    pub fn score(&self) -> Score {
        self.scores.score()
    }

    // TODO: Get rid of this function once Check isn't using Damage for its rolls anymore.
    pub(crate) fn into_modifiers(self) -> ModifiersOutcome {
        self.scores
    }

}

impl fmt::Display for DamageOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.score())
    }
}

impl fmt::Debug for DamageOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.scores)
    }
}

/// This is used to create a 'fudged' `DamageOutcome` without actually
/// randomly generating anything.
///
/// ```
/// use critfail::DamageOutcomeBuilder;
/// // To create a result that could come from rolling '2d8+4-1'
/// let outcome = DamageOutcomeBuilder::new()
///     .dice(8, vec![2,6])
///     .modifier(4)
///     .modifier(-1)
///     .build();
///
/// assert_eq!(outcome.score(), 11);
/// assert_eq!(
///     format!("{:?}", outcome),
///     "[2+6]+4-1"
/// );
/// ```
#[derive(Default)]
pub struct DamageOutcomeBuilder {
    scores: Vec<OutcomePart>,
}
])
impl DamageOutcomeBuilder {
    /// Create a new DamageOutcomeBuilder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a constant modifier to the roll. This method can be chained
    /// multiple times for multiple modifiers.
    ///
    /// ```
    /// use critfail::DamageOutcomeBuilder;
    ///
    ///
    /// // To create a result that could come from rolling '2d8+6-4'
    /// let outcome = DamageOutcomeBuilder::new()
    ///     .dice(8, vec![1,6])])
    ///     .modifier(6)
    ///     .modifier(-4)
    ///     .build();
    ///
    /// assert_eq!(outcome.score(), 9);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "[1+6]+6-4"
    /// );
    /// ```
    pub fn modifier(self, modifier: Score) -> Self {
        let mut scores = self.scores;
        scores.push(OutcomePart::Modifier(modifier));
        Self { scores }
    }

    /// Add a dice modifier to the roll. This method can be chained
    /// multiple times for multiple modifiers. `sides` specifies the die
    /// that was rolled.
    ///
    /// ```
    /// use critfail::DamageOutcomeBuilder;
    ///
    ///
    /// // To create a result that could come from rolling '2d6+3d10-1d4'
    /// let outcome = DamageOutcomeBuilder::new()
    ///     .dice(6, vec![5, 1])
    ///     .dice(10, vec![4, 9, 2])
    ///     .dice(-4, vec![3])
    ///     .build();
    ///
    /// assert_eq!(outcome.score(), 18);
    /// assert_eq!(
    ///     format!("{:?}", outcome),
    ///     "[5+1]+[4+9+2]-[3]"
    /// );
    /// ```
    pub fn dice(self, sides: Sides, scores: Vec<Score>) -> Self {
        let mut _scores = self.scores;
        _scores.push(OutcomePart::Dice(sides, scores));
        Self { scores: _scores }
    }

    /// Create a DamageOutcome from this builder.
    pub fn build(self) -> DamageOutcome {
        DamageOutcome::new(self.scores)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use OutcomePart::Dice as D;
    use OutcomePart::Modifier as M;

    #[test]
    fn empty() {
        let r = DamageOutcome::new(vec![]);
        assert_eq!(r.score(), 0);
        assert_eq!(r.score(), 0); // Check twice to make sure nothing is weird with lazy evaluation
        assert_eq!(format!("{}", r), "0");
        assert_eq!(format!("{:?}", r), "");
    }

    #[test]
    fn just_modifier() {
        let r = DamageOutcome::new(vec![M(2)]);
        assert_eq!(r.score(), 2);
        assert_eq!(r.score(), 2); // Check twice to make sure nothing is weird with lazy evaluation
        assert_eq!(format!("{}", r), "2");
        assert_eq!(format!("{:?}", r), "2");
    }

    #[test]
    fn dice_modifier() {
        let r = DamageOutcome::new(vec![D(4, vec![1, 2, 3]), M(-2)]);

        assert_eq!(r.score(), 4);
        assert_eq!(format!("{}", r), "4");
        assert_eq!(format!("{:?}", r), "[1+2+3]-2");
    }

    #[test]
    fn negative_dice() {
        let r = DamageOutcome::new(vec![D(6, vec![4, 1, 6]), M(4), D(-4, vec![3, 1])]);
        assert_eq!(r.score(), 11);
        assert_eq!(format!("{}", r), "11");
        assert_eq!(format!("{:?}", r), "[4+1+6]+4-[3+1]");
    }
}
