use std::fmt;

use crate::Damage;
use crate::DamageRoll;
use crate::Score;
use crate::ScoreRoll;

pub struct CheckRoll {
    main: Score,
    other: Option<Score>,
    modifiers: DamageRoll,
}

impl ScoreRoll for CheckRoll {
    fn score(&self) -> Score {
        unimplemented!()
    }
}

impl fmt::Display for CheckRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for CheckRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DamageRollPart::Dice as Dr;
    use crate::DamageRollPart::Modifier as Mr;

    #[test]
    fn neutral() {
        let r = CheckRoll {
            main: 10,
            other: None,
            modifiers: DamageRoll::new(vec![]),
        };
        assert_eq!(r.score(), 10);;
        assert_eq!(format!("{}", r), "10");
        assert_eq!(format!("{:?}", r), "(10)");
    }

    #[test]
    fn advantage() {
        let r = CheckRoll {
            main: 15,
            other: Some(8),
            modifiers: DamageRoll::new(vec![]),
        };
        assert_eq!(r.score(), 15);
        assert_eq!(format!("{}", r), "15");
        assert_eq!(format!("{:?}", r), "(15/8)");
    }

    #[test]
    fn disadvantage() {
        let r = CheckRoll {
            main: 7,
            other: Some(12),
            modifiers: DamageRoll::new(vec![]),
        };
        assert_eq!(r.score(), 7);
        assert_eq!(format!("{}", r), "7");
        assert_eq!(format!("{:?}", r), "(7/12)");
    }

    #[test]
    fn die_modifier() {
        let r = CheckRoll {
            main: 6,
            other: None,
            modifiers: DamageRoll::new(vec![Dr(4, vec![1])]),
        };
        assert_eq!(r.score(), 7);;
        assert_eq!(format!("{}", r), "7");
        assert_eq!(format!("{:?}", r), "(6)+[1]");
    }

    #[test]
    fn mixed_modifiers() {
        let r = CheckRoll {
            main: 12,
            other: Some(4),
            modifiers: DamageRoll::new(vec![Dr(-4, vec![2, 3]), Mr(3)]),
        };
        assert_eq!(r.score(), 10);;
        assert_eq!(format!("{}", r), "10");
        assert_eq!(format!("{:?}", r), "(12/4)-[2+3]+3");
    }
}
