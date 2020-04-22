use std::error::Error;
use std::str::FromStr;

use regex::Regex;

use crate::ParseError;
use crate::{Damage, DamagePart};
use crate::{Score, Sides};

lazy_static! {
    static ref DICE_RE: Regex = Regex::new("^(-?)([0-9]+)d([0-9]+)$").unwrap();
}
lazy_static! {
    static ref MODIFIER_RE: Regex = Regex::new("^(-?)([0-9]+)$").unwrap();
}

impl FromStr for DamagePart {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cap) = DICE_RE.captures(s) {
            let sign = if &cap[1] == "-" { -1 } else { 1 };
            Ok(DamagePart::Dice(
                cap[2].parse()?,
                (cap[3].parse::<Sides>()?) * sign,
            ))
        } else if let Some(cap) = MODIFIER_RE.captures(s) {
            let sign = if &cap[1] == "-" { -1 } else { 1 };
            Ok(DamagePart::Modifier(cap[2].parse::<Score>()? * sign))
        } else {
            Err(Box::new(ParseError::new(s)))
        }
    }
}

impl FromStr for Damage {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = 0;
        let mut result = vec![];

        while i < s.len() {
            // Grab everything up to the next +/-, and see if it's a DamagePart
            let end = s[i + 1..]
                .find(|c: char| c == '+' || c == '-')
                .map(|n| (i + 1) + n)
                .unwrap_or_else(|| s.len());

            let part: DamagePart = s[i..end].parse()?;
            result.push(part);

            // Update the index
            i = end;

            // If we stopped on a +, skip it
            if s[i..].starts_with('+') {
                i += 1;
            }
        }

        Ok(Damage(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_damagepart {
        ($string:expr, $damage:expr) => {
            let d = $string.parse::<DamagePart>().unwrap();
            assert_eq!(d, $damage);
        }
    }

    macro_rules! test_damage {
        ($string:expr, $damage:expr) => {
            let d = $string.parse::<Damage>().unwrap();
            assert_eq!(d, $damage);
        }
    }

    mod parse_damagepart {
        use super::*;
        #[test]
        fn simple_damage() {
            test_damagepart!("2d8", DamagePart::Dice(2, 8));
            test_damagepart!("1d12", DamagePart::Dice(1, 12));
            test_damagepart!("3d6", DamagePart::Dice(3, 6));
            test_damagepart!("421d314159", DamagePart::Dice(421, 314159));
        }

        #[test]
        fn negative_damage() {
            test_damagepart!("-2d8", DamagePart::Dice(2, -8));
            test_damagepart!("-1d12", DamagePart::Dice(1, -12));
            test_damagepart!("-3d6", DamagePart::Dice(3, -6));
            test_damagepart!("-421d314159", DamagePart::Dice(421, -314159));
        }

        #[test]
        fn modifier() {
            test_damagepart!("4", DamagePart::Modifier(4));
            test_damagepart!("-3", DamagePart::Modifier(-3));
            test_damagepart!("26", DamagePart::Modifier(26));
            test_damagepart!("-129", DamagePart::Modifier(-129));
        }

        #[test]
        fn invalid() {
            assert!("2q4".parse::<DamagePart>().is_err());
            assert!("d20".parse::<DamagePart>().is_err());
            assert!("r+3".parse::<DamagePart>().is_err());
            assert!("d-3".parse::<DamagePart>().is_err());
            assert!("d".parse::<DamagePart>().is_err());
            assert!("".parse::<DamagePart>().is_err());
            assert!("2d6+3".parse::<DamagePart>().is_err());
        }

    }

    mod parse_damage {
        use super::*;
        use DamagePart::Dice as D;
        use DamagePart::Modifier as M;

        #[test]
        fn normal_damage() {
           test_damage!("2d6+3", Damage(vec![D(2, 6), M(3)]));
           test_damage!("8d4-4", Damage(vec![D(8, 4), M(-4)]));
           test_damage!("-2d8+3", Damage(vec![D(2, -8), M(3)]));
           test_damage!("3d12+3d6-1d4+2-3", Damage(vec![D(3, 12), D(3, 6), D(1, -4), M(2), M(-3)]));
        }

        #[test]
        fn invalid() {
            assert!("+3d6".parse::<Damage>().is_err());
            assert!("3d6+2q".parse::<Damage>().is_err());
            assert!("3d6-2q".parse::<Damage>().is_err());
            assert!("3d6++4".parse::<Damage>().is_err());
        }
    }
}
