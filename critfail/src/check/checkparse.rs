use std::error::Error;
use std::str::FromStr;

use crate::AdvState::*;
use crate::Check;
use crate::ParseError;

impl FromStr for Check {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Figure out the advantage from the first character
        let (adv, i) = if s.starts_with('r') {
            (Neutral, 1)
        } else if s.starts_with('a') {
            (Advantage, 1)
        } else if s.starts_with('d') {
            (Disadvantage, 1)
        } else if s.starts_with('+') || s.starts_with('-') {
            (Neutral, 0)
        } else {
            return Err(Box::new(ParseError::new(s)));
        };

        // If the character after the advantage character is a +, skip it
        let i = if s[i..].starts_with('+') { i + 1 } else { i };

        // The rest is the modifier
        let modifier = s[i..].parse()?;

        Ok(Check { adv, modifier })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::damage::DamagePart::Dice as D;
    use crate::damage::DamagePart::Modifier as M;
    use crate::*;

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
                adv: Advantage,
                modifier: Damage(vec![])
            }
        );

        assert_eq!(
            "d".parse::<Check>().unwrap(),
            Check {
                adv: Disadvantage,
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
                adv: Neutral,
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
                modifier: Damage(vec![D(1, -4), M(2)])
            }
        );

        assert_eq!(
            "+1d4".parse::<Check>().unwrap(),
            Check {
                adv: Neutral,
                modifier: Damage(vec![D(1, 4)])
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
