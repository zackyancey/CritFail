use std::error::Error;
use std::str::FromStr;

use crate::{ParseError, Roll};

impl FromStr for Roll {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseError::new("Empty String").into())
        } else if s.contains('?') {
            Ok(Roll::Attack(s.parse()?))
        } else if "rad+-".contains(&s[0..1]) {
            Ok(Roll::Check(s.parse()?))
        } else {
            Ok(Roll::Damage(s.parse()?))
        }
    }
}

// TODO: Fix these tests for new format
#[cfg(test)]
mod tests {
    use crate::DamagePart::Dice as D;
    use crate::DamagePart::Modifier as M;
    use crate::*;

    #[test]
    fn damage_basic() {
        assert_eq!(
            "2d10".parse::<Roll>().unwrap(),
            Roll::Damage(Damage(vec![D(2, 10)]))
        );
        assert_eq!(
            "5d6".parse::<Roll>().unwrap(),
            Roll::Damage(Damage(vec![D(5, 6)]))
        );
        assert_eq!(
            "25d4".parse::<Roll>().unwrap(),
            Roll::Damage(Damage(vec![D(25, 4)]))
        );
    }

    #[test]
    fn damage_sum() {
        assert_eq!(
            "3d4+5".parse::<Roll>().unwrap(),
            Roll::Damage(Damage(vec![D(3, 4), M(5)]))
        );
        assert_eq!(
            "2d6+4".parse::<Roll>().unwrap(),
            Roll::Damage(Damage(vec![D(2, 6), M(4)]))
        );
        assert_eq!(
            "3d4-5".parse::<Roll>().unwrap(),
            Roll::Damage(Damage(vec![D(3, 4), M(-5)]))
        );
        assert_eq!(
            "7d6+2d8+9".parse::<Roll>().unwrap(),
            Roll::Damage(Damage(vec![D(7, 6), D(2, 8), M(9)]))
        );
        assert_eq!(
            "2d8-1d4-1+5".parse::<Roll>().unwrap(),
            Roll::Damage(Damage(vec![D(2, 8), D(1, -4), M(-1), M(5)]))
        );
    }

    #[test]
    fn check_basic() {
        assert_eq!(
            "r".parse::<Roll>().unwrap(),
            Roll::Check(Check::new("r").unwrap())
        );
        assert_eq!(
            "a".parse::<Roll>().unwrap(),
            Roll::Check(Check::new("a").unwrap())
        );
        assert_eq!(
            "d".parse::<Roll>().unwrap(),
            Roll::Check(Check::new("d").unwrap())
        );
    }

    #[test]
    fn check_modifiers() {
        assert_eq!(
            "r+3".parse::<Roll>().unwrap(),
            Roll::Check(Check::new("r+3").unwrap())
        );

        assert_eq!(
            "d+5".parse::<Roll>().unwrap(),
            Roll::Check(Check::new("d+5").unwrap())
        );

        assert_eq!(
            "a-2".parse::<Roll>().unwrap(),
            Roll::Check(Check::new("a-2").unwrap())
        );

        assert_eq!(
            "r+1d4+2".parse::<Roll>().unwrap(),
            Roll::Check(Check::new("r+1d4+2").unwrap())
        );
    }

    #[test]
    fn check_inferred() {
        assert_eq!(
            "+3".parse::<Roll>().unwrap(),
            Roll::Check(Check::new("+3").unwrap())
        );
        assert_eq!(
            "-2".parse::<Roll>().unwrap(),
            Roll::Check(Check::new("-2").unwrap())
        );
        assert_eq!(
            "-1d4+2".parse::<Roll>().unwrap(),
            Roll::Check(Check::new("-1d4+2").unwrap())
        );
    }

    #[test]
    fn attacks() {
        assert_eq!(
            "+3?2d8-1".parse::<Roll>().unwrap(),
            Roll::Attack(Attack::new("+3?2d8-1").unwrap())
        );

        assert_eq!(
            "a-1?2d8+1".parse::<Roll>().unwrap(),
            Roll::Attack(Attack::new("a-1?2d8+1").unwrap())
        );

        assert_eq!(
            "r+8?3d10+2".parse::<Roll>().unwrap(),
            Roll::Attack(Attack::new("r+8?3d10+2").unwrap())
        );

        assert_eq!(
            "a+1d4+3-1?1d4+4d6+2-1d4".parse::<Roll>().unwrap(),
            Roll::Attack(Attack::new("a+1d4+3-1?1d4+4d6+2-1d4").unwrap())
        );
    }

    // TODO: Finish tests
    /*
        // Multi roll statements
        "2d8;3d6;4d10"
        "r;+3;+2"
        "+2?2d6;+2?2d6;+5,1d8"
        "3d4;+3;-1?4d6"
    */
}
