use std::error::Error;
use std::str::FromStr;

use crate::AdvState::*;
use crate::Check;
use crate::Damage;

impl FromStr for Check {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DamagePart::Dice as D;
    use crate::DamagePart::Modifier as M;

    #[test]
    fn basic() {
        assert_eq!(
            "r".parse::<Check>().unwrap(),
            Check {
                adv: Neutral,
                modifier: Damage(vec![])
            }
        );

        assert_eq!(
            "a".parse::<Check>().unwrap(),
            Check {
                adv: Neutral,
                modifier: Damage(vec![])
            }
        );

        assert_eq!(
            "d".parse::<Check>().unwrap(),
            Check {
                adv: Neutral,
                modifier: Damage(vec![])
            }
        );
    }

    #[test]
    fn with_modifiers() {
        assert_eq!(
            "r+3".parse::<Check>().unwrap(),
            Check {
                adv: Neutral,
                modifier: Damage(vec![M(3)])
            }
        );

        assert_eq!(
            "d+5".parse::<Check>().unwrap(),
            Check {
                adv: Disadvantage,
                modifier: Damage(vec![M(5)])
            }
        );

        assert_eq!(
            "a-2".parse::<Check>().unwrap(),
            Check {
                adv: Advantage,
                modifier: Damage(vec![M(-2)])
            }
        );

        assert_eq!(
            "r+1d4+2".parse::<Check>().unwrap(),
            Check {
                adv: Disadvantage,
                modifier: Damage(vec![D(1, 4), M(2)])
            }
        );
    }

    #[test]
    fn inferred_r() {
        assert_eq!(
            "+3".parse::<Check>().unwrap(),
            Check {
                adv: Neutral,
                modifier: Damage(vec![M(3)])
            }
        );

        assert_eq!(
            "-2".parse::<Check>().unwrap(),
            Check {
                adv: Neutral,
                modifier: Damage(vec![M(-2)])
            }
        );

        assert_eq!(
            "-1d4+2".parse::<Check>().unwrap(),
            Check {
                adv: Neutral,
                modifier: Damage(vec![D(1, 4), M(2)])
            }
        );

        assert_eq!(
            "+1d4".parse::<Check>().unwrap(),
            Check {
                adv: Neutral,
                modifier: Damage(vec![D(1, 4), M(2)])
            }
        );
    }

    #[test]
    fn invalid() {
        assert!("r+r+3".parse::<Check>().is_err());
        assert!("1+r+3".parse::<Check>().is_err());
        assert!("2d8".parse::<Check>().is_err());
        assert!("5".parse::<Check>().is_err());
        assert!("+r+2d8".parse::<Check>().is_err());
        assert!("r+3+bad".parse::<Check>().is_err());
        assert!("r+3+1d4/2".parse::<Check>().is_err());
    }

}
