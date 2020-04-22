use crate::AdvState;
use crate::AdvState::*;
use crate::Damage;
use crate::DamageRoll;
use crate::Score;
use crate::ScoreRoll;
use std::cmp::{max, min};
use std::fmt;

pub struct CheckRoll {
    main: Score,
    other: Option<Score>,
    modifiers: DamageRoll,
}

impl CheckRoll {
    pub fn new(adv: AdvState, r1: Score, r2: Score, modifiers: DamageRoll) -> CheckRoll {
        let (main, other) = match adv {
            Advantage => (max(r1, r2), Some(min(r1, r2))),
            Disadvantage => (min(r1, r2), Some(max(r1, r2))),
            Neutral => (r1, None),
        };

        CheckRoll {
            main,
            other,
            modifiers,
        }
    }
}

impl ScoreRoll for CheckRoll {
    fn score(&self) -> Score {
        self.main + self.modifiers.score()
    }
}

impl fmt::Display for CheckRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.score())
    }
}

impl fmt::Debug for CheckRoll {
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
    use crate::DamageRollPart::Dice as Dr;
    use crate::DamageRollPart::Modifier as Mr;

    #[test]
    fn neutral() {
        let r = CheckRoll::new(Neutral, 10, 16, DamageRoll::new(vec![]));
        assert_eq!(r.score(), 10);;
        assert_eq!(format!("{}", r), "10");
        assert_eq!(format!("{:?}", r), "(10)");
    }

    #[test]
    fn advantage() {
        let r = CheckRoll::new(Advantage, 8, 15, DamageRoll::new(vec![]));
        assert_eq!(r.score(), 15);
        assert_eq!(format!("{}", r), "15");
        assert_eq!(format!("{:?}", r), "(15/8)");
    }

    #[test]
    fn disadvantage() {
        let r = CheckRoll::new(Disadvantage, 12, 7, DamageRoll::new(vec![]));
        assert_eq!(r.score(), 7);
        assert_eq!(format!("{}", r), "7");
        assert_eq!(format!("{:?}", r), "(7/12)");
    }

    #[test]
    fn die_modifier() {
        let r = CheckRoll::new(Neutral, 6, 15, DamageRoll::new(vec![Dr(4, vec![1])]));
        assert_eq!(r.score(), 7);;
        assert_eq!(format!("{}", r), "7");
        assert_eq!(format!("{:?}", r), "(6)+[1]");
    }

    #[test]
    fn mixed_modifiers() {
        let r = CheckRoll::new(Advantage, 12, 4,DamageRoll::new(vec![Dr(-4, vec![2, 3]), Mr(3)]));
        assert_eq!(r.score(), 10);;
        assert_eq!(format!("{}", r), "10");
        assert_eq!(format!("{:?}", r), "(12/4)-[2+3]+3");
    }
}
