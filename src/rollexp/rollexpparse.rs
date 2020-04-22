use std::error::Error;
use std::str::FromStr;

use crate::RollExp;
use crate::{Damage, Check, Attack};

impl FromStr for RollExp {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('?') {
            Ok(RollExp::Attack(s.parse()?))
        } else if "rad+-".contains(&s[0..1]) {
            Ok(RollExp::Check(s.parse()?))
        } else {
            Ok(RollExp::Damage(s.parse()?))
        }
    }
}

// TODO: Fix these tests for new format
#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::AdvState::*;
    use crate::DamagePart::Dice as D;
    use crate::DamagePart::Modifier as M;

    #[test]
    fn damage_basic() {
        assert_eq!(
            "2d10".parse::<RollExp>().unwrap(),
            RollExp::Damage(Damage(vec![D(2, 10)]))
        );
        assert_eq!(
            "5d6".parse::<RollExp>().unwrap(),
            RollExp::Damage(Damage(vec![D(5, 6)]))
        );
        assert_eq!(
            "25d4".parse::<RollExp>().unwrap(),
            RollExp::Damage(Damage(vec![D(25, 4)]))
        );
    }

    #[test]
    fn damage_sum() {
        assert_eq!(
            "3d4+5".parse::<RollExp>().unwrap(),
            RollExp::Damage(Damage(vec![D(3, 4), M(5)]))
        );
        assert_eq!(
            "2d6+4".parse::<RollExp>().unwrap(),
            RollExp::Damage(Damage(vec![D(2, 6), M(4)]))
        );
        assert_eq!(
            "3d4-5".parse::<RollExp>().unwrap(),
            RollExp::Damage(Damage(vec![D(3, 4), M(-5)]))
        );
        assert_eq!(
            "7d6+2d8+9".parse::<RollExp>().unwrap(),
            RollExp::Damage(Damage(vec![D(7, 6), D(2, 8), M(9)]))
        );
        assert_eq!(
            "2d8-1d4-1+5".parse::<RollExp>().unwrap(),
            RollExp::Damage(Damage(vec![D(2, 8), D(1, -4), M(-1), M(5)]))
        );
    }

    #[test]
    fn check_basic() {
        assert_eq!(
            "r".parse::<RollExp>().unwrap(),
            RollExp::Check(Check {
                adv: Neutral,
                modifier: Damage(vec![])
            })
        );
        assert_eq!(
            "a".parse::<RollExp>().unwrap(),
            RollExp::Check(Check {
                adv: Advantage,
                modifier: Damage(vec![])
            })
        );
        assert_eq!(
            "d".parse::<RollExp>().unwrap(),
            RollExp::Check(Check {
                adv: Disadvantage,
                modifier: Damage(vec![])
            })
        );
    }

    #[test]
    fn check_modifiers() {
        assert_eq!(
            "r+3".parse::<RollExp>().unwrap(),
            RollExp::Check(Check {
                adv: Neutral,
                modifier: Damage(vec![M(3)])
            })
        );

        assert_eq!(
            "d+5".parse::<RollExp>().unwrap(),
            RollExp::Check(Check {
                adv: Disadvantage,
                modifier: Damage(vec![M(5)])
            })
        );

        assert_eq!(
            "a-2".parse::<RollExp>().unwrap(),
            RollExp::Check(Check {
                adv: Advantage,
                modifier: Damage(vec![M(-2)])
            })
        );

        assert_eq!(
            "r+1d4+2".parse::<RollExp>().unwrap(),
            RollExp::Check(Check {
                adv: Neutral,
                modifier: Damage(vec![D(1, 4), M(2)])
            })
        );
    }

    #[test]
    fn check_inferred() {
        assert_eq!(
            "+3".parse::<RollExp>().unwrap(),
            RollExp::Check(Check {
                adv: Neutral,
                modifier: Damage(vec![M(3)])
            })
        );
        assert_eq!(
            "-2".parse::<RollExp>().unwrap(),
            RollExp::Check(Check {
                adv: Neutral,
                modifier: Damage(vec![M(-2)])
            })
        );
        assert_eq!(
            "-1d4+2".parse::<RollExp>().unwrap(),
            RollExp::Check(Check {
                adv: Neutral,
                modifier: Damage(vec![D(1, -4), M(2)])
            })
        );
    }

    #[test]
    fn attacks() {
        assert_eq!(
            "+3?2d8-1".parse::<RollExp>().unwrap(),
            RollExp::Attack(Attack {
                check: Check {
                    adv: Neutral,
                    modifier: Damage(vec![M(3)])
                },
                damage: Damage(vec![D(2,8), M(-1)])
            })
        );

        assert_eq!(
            "a-1?2d8+1".parse::<RollExp>().unwrap(),
            RollExp::Attack(Attack {
                check: Check {
                    adv: Advantage,
                    modifier: Damage(vec![M(-1)])
                },
                damage: Damage(vec![D(2,8), M(1)])
            })
        );

        assert_eq!(
            "r+8?3d10+2".parse::<RollExp>().unwrap(),
            RollExp::Attack(Attack {
                check: Check {
                    adv: Neutral,
                    modifier: Damage(vec![M(8)])
                },
                damage: Damage(vec![D(3,10), M(2)])
            })
        );

        assert_eq!(
            "a+1d4+3-1?1d4+4d6+2-1d4".parse::<RollExp>().unwrap(),
            RollExp::Attack(Attack {
                check: Check {
                    adv: Advantage,
                    modifier: Damage(vec![D(1,4), M(3), M(-1)])
                },
                damage: Damage(vec![D(1,4), D(4,6), M(2), D(1, -4)])
            })
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
