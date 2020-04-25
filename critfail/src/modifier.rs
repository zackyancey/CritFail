use crate::{util, Score, Sides};
use std::fmt;

/// Internally used wrapper struct for a collection of OutcomeParts.
#[derive(Clone, PartialEq)]
pub(crate) struct ModifiersOutcome {
    // OPTIMIZE: Cache sum in this struct
    scores: Vec<OutcomePart>,
}

impl ModifiersOutcome {
    pub fn score(&self) -> Score {
        self.scores.iter().map(|s| s.score()).sum()
    }

    pub(crate) fn into_inner(self) -> Vec<OutcomePart> {
        self.scores
    }
}

impl IntoIterator for ModifiersOutcome {
    type Item = OutcomePart;
    type IntoIter = std::vec::IntoIter<OutcomePart>;

    fn into_iter(self) -> std::vec::IntoIter<OutcomePart> {
        self.scores.into_iter()
    }
}

impl From<Vec<OutcomePart>> for ModifiersOutcome {
    fn from(scores: Vec<OutcomePart>) -> Self {
        Self { scores }
    }
}

/// Enum representing the different kinds of values that can be returned
/// for damage or the modifier on a check.
#[derive(Clone, PartialEq)]
pub enum OutcomePart {
    /// The result of rolling a set of dice.
    ///
    /// For example, If I rolled 3d6 and got a 4, a 2, and a 1, that
    /// would be `Dice(6, vec![4,2,1])`.
    Dice(Sides, Vec<Score>),
    /// A constant modifier value.
    Modifier(Score),
}

impl OutcomePart {
    /// Get the score of this `OutcomePart`.
    ///
    /// Either the sum of the die rolls, or the value of the modifier.
    pub fn score(&self) -> Score {
        match self {
            Self::Dice(sides, d) => {
                let sum: Score = d.iter().sum();
                if *sides < 0 {
                    -sum
                } else {
                    sum
                }
            }
            Self::Modifier(m) => *m,
        }
    }
}

impl fmt::Display for ModifiersOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.score())
    }
}

impl fmt::Debug for ModifiersOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scores = self.scores.iter().map(|s| format!("{:?}", s));
        util::write_string_sum(f, scores)
    }
}

impl fmt::Display for OutcomePart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.score())
    }
}

impl fmt::Debug for OutcomePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutcomePart::Dice(sides, scores) => {
                if *sides < 0 {
                    write!(f, "-")?;
                }
                let scores = scores.iter().map(|i| format!("{}", i));

                write!(f, "[")?;
                util::write_string_sum(f, scores)?;
                write!(f, "]")
            }

            OutcomePart::Modifier(m) => write!(f, "{}", m),
        }
    }
}
