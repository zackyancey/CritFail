use crate::Attack;
use crate::ParseError;
use regex::Regex;

use std::str::FromStr;

lazy_static! {
    static ref ATTACK_RE: Regex = Regex::new(r"^([^?]+) *\? *([^?]+)$").unwrap();
}

impl FromStr for Attack {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cap) = ATTACK_RE.captures(s) {
            let check = cap[1].parse()?;
            let damage = cap[2].parse()?;

            Ok(Attack { check, damage })
        } else {
            Err(ParseError::new(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Check, Damage, RollExpression};

    #[test]
    fn inferred_r() {
        assert_eq!(
            "+3?2d8-1".parse::<Attack>().unwrap(),
            Attack {
                check: Check::new("r+3").unwrap(),
                damage: Damage::new("2d8-1").unwrap()
            }
        )
    }

    #[test]
    fn advantage() {
        assert_eq!(
            "a-1?2d8+1".parse::<Attack>().unwrap(),
            Attack {
                check: Check::new("a-1").unwrap(),
                damage: Damage::new("2d8+1").unwrap()
            }
        )
    }

    #[test]
    fn neutral() {
        assert_eq!(
            "r+8?3d10+2".parse::<Attack>().unwrap(),
            Attack {
                check: Check::new("r+8").unwrap(),
                damage: Damage::new("3d10+2").unwrap()
            }
        )
    }

    #[test]
    fn complex() {
        assert_eq!(
            "d+1d4+3-1?1d4+4d6+2-1d4".parse::<Attack>().unwrap(),
            Attack {
                check: Check::new("d+1d4+3-1").unwrap(),
                damage: Damage::new("1d4+4d6+2-1d4").unwrap()
            }
        )
    }

    #[test]
    fn invalid() {
        assert!("r+3".parse::<Attack>().is_err());
        assert!("2d8".parse::<Attack>().is_err());
        assert!("".parse::<Attack>().is_err());
        assert!("2d8?3d6".parse::<Attack>().is_err());
        assert!("r+3?r-2".parse::<Attack>().is_err());
        assert!("?".parse::<Attack>().is_err());
        assert!("not?2d10".parse::<Attack>().is_err());
        assert!("d+3?3d6+4?3d8".parse::<Attack>().is_err());
        assert!("d+3??3d8".parse::<Attack>().is_err());
        assert!("d+3??majordamage".parse::<Attack>().is_err());
    }
}
