use crate::{Damage, Check, Attack};
use crate::DamagePart;

#[derive(PartialEq, Debug)]
pub enum RollExp {
    Check(Check),
    Damage(Damage),
    Attack(Attack),
}

// TODO: Make this RollExp::from_str() or ::from::<&str>
pub fn parse(e: &str) -> Result<RollExp, ()> {
    Ok(RollExp::Damage(vec![DamagePart::Dice(2, 10)]))
}

#[cfg(test)]
mod tests {
    use crate::check::AdvState::*;
    use crate::DamagePart::Dice as D;
    use crate::DamagePart::Modifier as M;
    use super::*;

    #[test]
    fn damage_basic() {
        assert_eq!(parse("2d10"), Ok(RollExp::Damage(vec![D(2, 10)])));
        assert_eq!(parse("5d6"), Ok(RollExp::Damage(vec![D(5, 6)])));
        assert_eq!(parse("25d4"), Ok(RollExp::Damage(vec![D(25, 4)])));
    }

    #[test]
    fn damage_sum() {
        assert_eq!(parse("3d4+5"), Ok(RollExp::Damage(vec![D(3, 4), M(5)])));
        assert_eq!(parse("2d6+4"), Ok(RollExp::Damage(vec![D(2, 6), M(4)])));
        assert_eq!(parse("3d4-5"), Ok(RollExp::Damage(vec![D(3, 4), M(-5)])));
        assert_eq!(
            parse("7d6+2d8+9"),
            Ok(RollExp::Damage(vec![D(7, 6), D(2, 8), M(9)]))
        );
        assert_eq!(
            parse("2d8-1d4-1+5"),
            Ok(RollExp::Damage(vec![D(2, 6), M(4)]))
        );
    }

    #[test]
    fn check_basic() {
        assert_eq!(
            parse("r"),
            Ok(RollExp::Check(Check {
                adv: Neutral,
                modifier: vec![]
            }))
        );
        assert_eq!(
            parse("a"),
            Ok(RollExp::Check(Check {
                adv: Advantage,
                modifier: vec![]
            }))
        );
        assert_eq!(
            parse("d"),
            Ok(RollExp::Check(Check {
                adv: Disadvantage,
                modifier: vec![]
            }))
        );
    }

    #[test]
    fn check_modifiers() {
        assert_eq!(
            parse("r+3"),
            Ok(RollExp::Check(Check {
                adv: Neutral,
                modifier: vec![M(3)]
            }))
        );

        assert_eq!(
            parse("d+5"),
            Ok(RollExp::Check(Check {
                adv: Disadvantage,
                modifier: vec![M(5)]
            }))
        );

        assert_eq!(
            parse("a-2"),
            Ok(RollExp::Check(Check {
                adv: Advantage,
                modifier: vec![M(-2)]
            }))
        );

        assert_eq!(
            parse("r+1d4+2"),
            Ok(RollExp::Check(Check {
                adv: Disadvantage,
                modifier: vec![D(-1, 4), M(2)]
            }))
        );
    }

    #[test]
    fn check_inferred() {
        assert_eq!(
            parse("+3"),
            Ok(RollExp::Check(Check {
                adv: Neutral,
                modifier: vec![M(3)]
            }))
        );
        assert_eq!(
            parse("-2"),
            Ok(RollExp::Check(Check {
                adv: Neutral,
                modifier: vec![M(-2)]
            }))
        );
        assert_eq!(
            parse("-1d4+2"),
            Ok(RollExp::Check(Check {
                adv: Neutral,
                modifier: vec![D(-1, 4), M(2)]
            }))
        );
    }

    // TODO: Finish tests
    /*
        // Attacks
        "+3?2d8-1"
        "a-1?2d8+1"
        "r+8?3d10+2"
        "a+1d4+3-1?1d4+4d6+2-1d4"

        // Multi roll statements
        "2d8;3d6;4d10"
        "r;+3;+2"
        "+2?2d6;+2?2d6;+5,1d8"
        "3d4;+3;-1?4d6"
    */
}
