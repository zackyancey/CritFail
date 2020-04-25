use std::fmt;

use crate::{ModifiersOutcome, OutcomePart, Score};

#[derive(Clone)]
pub struct DamageOutcome {
    scores: ModifiersOutcome,
}

impl DamageOutcome {
    pub fn new(scores: Vec<OutcomePart>) -> DamageOutcome {
        DamageOutcome {
            scores: scores.into(),
        }
    }

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
