use std::fmt;

use crate::ScoreRoll;
use crate::{Score, Sides};

use crate::util;
use DamageRollPart::*;

pub struct DamageRoll {
    sum: Option<Score>,
    scores: Vec<DamageRollPart>,
}

pub enum DamageRollPart {
    Dice(Sides, Vec<Score>),
    Modifier(Score),
}

impl DamageRoll {
    pub fn new(scores: Vec<DamageRollPart>) -> DamageRoll {
        DamageRoll { sum: None, scores }
    }
}

impl ScoreRoll for DamageRollPart {
    fn score(&self) -> Score {
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

impl fmt::Display for DamageRollPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.score())
    }
}

impl fmt::Debug for DamageRollPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DamageRollPart::Dice(sides, scores) => {
                if *sides < 0 {
                    write!(f, "-")?;
                }
                let scores = scores.iter().map(|i| format!("{}", i));

                write!(f, "[")?;
                util::write_string_sum(f, scores)?;
                write!(f, "]")
            }

            DamageRollPart::Modifier(m) => write!(f, "{}", m),
        }
    }
}

impl ScoreRoll for DamageRoll {
    fn score(&self) -> Score {
        self.sum
            .unwrap_or_else(|| self.scores.iter().map(|s| s.score()).sum())
    }
}

impl fmt::Display for DamageRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.score())
    }
}

impl fmt::Debug for DamageRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scores = self.scores.iter().map(|s| format!("{:?}", s));
        util::write_string_sum(f, scores)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DamageRollPart::Dice as Dr;
    use DamageRollPart::Modifier as Mr;

    #[test]
    fn empty() {
        let r = DamageRoll {
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
        let r = DamageRoll {
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
        let r = DamageRoll {
            sum: None,
            scores: vec![Dr(4, vec![1, 2, 3]), Mr(-2)],
        };

        assert_eq!(r.score(), 4);
        assert_eq!(format!("{}", r), "4");
        assert_eq!(format!("{:?}", r), "[1+2+3]-2");
    }

    #[test]
    fn negative_dice() {
        let r = DamageRoll {
            sum: None,
            scores: vec![Dr(6, vec![4, 1, 6]), Mr(4), Dr(-4, vec![3, 1])],
        };
        assert_eq!(r.score(), 11);
        assert_eq!(format!("{}", r), "11");
        assert_eq!(format!("{:?}", r), "[4+1+6]+4-[3+1]");
    }
}
