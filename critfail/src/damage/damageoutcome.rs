use std::fmt;

use crate::{Score, Sides};

use crate::util;
use DamageOutcomePart::*;

#[derive(Clone)]
pub struct DamageOutcome {
    sum: Option<Score>,
    scores: Vec<DamageOutcomePart>,
}

#[derive(Clone)]
pub enum DamageOutcomePart {
    Dice(Sides, Vec<Score>),
    Modifier(Score),
}

impl DamageOutcome {
    pub fn new(scores: Vec<DamageOutcomePart>) -> DamageOutcome {
        DamageOutcome { sum: None, scores }
    }

    pub fn score(&self) -> Score {
        self.sum
            .unwrap_or_else(|| self.scores.iter().map(|s| s.score()).sum())
    }
}

impl DamageOutcomePart {
    pub fn score(&self) -> Score {
        match self {
            Dice(sides, d) => {
                let sum: Score = d.iter().sum();
                if *sides < 0 {
                    -sum
                } else {
                    sum
                }
            }
            Modifier(m) => *m,
        }
    }
}

impl fmt::Display for DamageOutcomePart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.score())
    }
}

impl fmt::Debug for DamageOutcomePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DamageOutcomePart::Dice(sides, scores) => {
                if *sides < 0 {
                    write!(f, "-")?;
                }
                let scores = scores.iter().map(|i| format!("{}", i));

                write!(f, "[")?;
                util::write_string_sum(f, scores)?;
                write!(f, "]")
            }

            DamageOutcomePart::Modifier(m) => write!(f, "{}", m),
        }
    }
}

impl fmt::Display for DamageOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.score())
    }
}

impl fmt::Debug for DamageOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scores = self.scores.iter().map(|s| format!("{:?}", s));
        util::write_string_sum(f, scores)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DamageOutcomePart::Dice as Dr;
    use DamageOutcomePart::Modifier as Mr;

    #[test]
    fn empty() {
        let r = DamageOutcome {
            sum: None,
            scores: vec![],
        };
        assert_eq!(r.score(), 0);
        assert_eq!(r.score(), 0); // Check twice to make sure nothing is weird with lazy evaluation
        assert_eq!(format!("{}", r), "0");
        assert_eq!(format!("{:?}", r), "");
    }

    #[test]
    fn just_modifier() {
        let r = DamageOutcome {
            sum: None,
            scores: vec![Mr(2)],
        };
        assert_eq!(r.score(), 2);
        assert_eq!(r.score(), 2); // Check twice to make sure nothing is weird with lazy evaluation
        assert_eq!(format!("{}", r), "2");
        assert_eq!(format!("{:?}", r), "2");
    }

    #[test]
    fn dice_modifier() {
        let r = DamageOutcome {
            sum: None,
            scores: vec![Dr(4, vec![1, 2, 3]), Mr(-2)],
        };

        assert_eq!(r.score(), 4);
        assert_eq!(format!("{}", r), "4");
        assert_eq!(format!("{:?}", r), "[1+2+3]-2");
    }

    #[test]
    fn negative_dice() {
        let r = DamageOutcome {
            sum: None,
            scores: vec![Dr(6, vec![4, 1, 6]), Mr(4), Dr(-4, vec![3, 1])],
        };
        assert_eq!(r.score(), 11);
        assert_eq!(format!("{}", r), "11");
        assert_eq!(format!("{:?}", r), "[4+1+6]+4-[3+1]");
    }
}
