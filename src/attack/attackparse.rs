use std::error::Error;
use std::str::FromStr;

use crate::Attack;

impl FromStr for Attack {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::AdvState::*;
    use crate::DamagePart::Dice as D;
    use crate::DamagePart::Modifier as M;
    use crate::{Check, Damage};

    #[test]
    fn inferred_r() {
        assert_eq!(
            "+3?2d8-1".parse::<Attack>().unwrap(),
            Attack {
                check: Check {
                    adv: Neutral,
                    modifier: Damage(vec![M(3)])
                },
                damage: Damage(vec![D(2, 8), M(-1)])
            }
        )
    }

    #[test]
    fn advantage() {
        assert_eq!(
            "a-1?2d8+1".parse::<Attack>().unwrap(),
            Attack {
                check: Check {
                    adv: Advantage,
                    modifier: Damage(vec![M(-1)])
                },
                damage: Damage(vec![D(2, 8), M(1)])
            }
        )
    }

    #[test]
    fn neutral() {
        assert_eq!(
            "r+8?3d10+2".parse::<Attack>().unwrap(),
            Attack {
                check: Check {
                    adv: Neutral,
                    modifier: Damage(vec![M(8)])
                },
                damage: Damage(vec![D(3, 10), M(2)])
            }
        )
    }

    #[test]
    fn complex() {
        assert_eq!(
            "d+1d4+3-1?1d4+4d6+2-1d4".parse::<Attack>().unwrap(),
            Attack {
                check: Check {
                    adv: Disadvantage,
                    modifier: Damage(vec![D(1, 4), M(3), M(-1)])
                },
                damage: Damage(vec![D(1, 4), D(4, 6), M(2), D(1, -4)])
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
