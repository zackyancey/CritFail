use crate::AdvState;
use crate::AdvState::*;
use crate::ModifiersOutcome;
use crate::OutcomePart;
use crate::Score;

use std::cmp::{max, min};
use std::fmt;

pub enum CritScore {
    Critical,
    Normal(Score),
    Fail,
}

#[derive(Clone)]
pub struct CheckOutcome {
    main: Score,
    other: Option<Score>,
    modifiers: ModifiersOutcome,
}

impl CheckOutcome {
    pub fn new(adv: &AdvState, r1: Score, r2: Score, modifiers: Vec<OutcomePart>) -> CheckOutcome {
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

    pub fn score(&self) -> Score {
        self.main + self.modifiers.score()
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OutcomePart::Dice as D;
    use crate::OutcomePart::Modifier as M;

    #[test]
    fn neutral() {
        let r = CheckOutcome::new(&Neutral, 10, 16, vec![]);
        assert_eq!(r.score(), 10);
        assert_eq!(format!("{}", r), "10");
        assert_eq!(format!("{:?}", r), "(10)");
    }

    #[test]
    fn advantage() {
        let r = CheckOutcome::new(&Advantage, 8, 15, vec![]);
        assert_eq!(r.score(), 15);
        assert_eq!(format!("{}", r), "15");
        assert_eq!(format!("{:?}", r), "(15/8)");
    }

    #[test]
    fn disadvantage() {
        let r = CheckOutcome::new(&Disadvantage, 12, 7, vec![]);
        assert_eq!(r.score(), 7);
        assert_eq!(format!("{}", r), "7");
        assert_eq!(format!("{:?}", r), "(7/12)");
    }

    #[test]
    fn die_modifier() {
        let r = CheckOutcome::new(&Neutral, 6, 15, vec![D(4, vec![1])]);
        assert_eq!(r.score(), 7);
        assert_eq!(format!("{}", r), "7");
        assert_eq!(format!("{:?}", r), "(6)+[1]");
    }

    #[test]
    fn mixed_modifiers() {
        let r = CheckOutcome::new(&Advantage, 12, 4, vec![D(-4, vec![2, 3]), M(3)]);
        assert_eq!(r.score(), 10);
        assert_eq!(format!("{}", r), "10");
        assert_eq!(format!("{:?}", r), "(12/4)-[2+3]+3");
    }

    #[test]
    fn critical() {
        let r = CheckOutcome::new(&Advantage, 20, 4, vec![D(-4, vec![2, 3]), M(3)]);
        assert_eq!(r.score(), 18);
        assert_eq!(format!("{}", r), "Critical");
        assert_eq!(format!("{:?}", r), "(20/4)-[2+3]+3");
    }

    #[test]
    fn fail() {
        let r = CheckOutcome::new(&Disadvantage, 1, 4, vec![D(-4, vec![2, 3]), M(3)]);
        assert_eq!(r.score(), -1);
        assert_eq!(format!("{}", r), "Fail");
        assert_eq!(format!("{:?}", r), "(1/4)-[2+3]+3");
    }
}
