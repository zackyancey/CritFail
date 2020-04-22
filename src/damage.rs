use std::error::Error;
use std::str::FromStr;

use rand::Rng;
use regex::Regex;

use crate::ParseError;
use crate::Rollable;
use crate::{Score, Sides};

mod damageroll;
pub use damageroll::*;

lazy_static! {
    static ref DICE_RE: Regex = Regex::new("(-?)([0-9]+)d([0-9]+)").unwrap();
}
lazy_static! {
    static ref MODIFIER_RE: Regex = Regex::new("(-?)([0-9]+)").unwrap();
}
lazy_static! {
    static ref DAMAGEPART_RE: Regex = Regex::new("(-?[0-9]+|-?[0-9]+d[0-9]+)+?").unwrap();
}

#[derive(PartialEq, Debug)]
pub enum DamagePart {
    Dice(u32, Sides),
    Modifier(Score),
}

// TODO: Make Damage a struct so I can impl FromStr for it
pub type Damage = Vec<DamagePart>;

impl Rollable for Damage {
    type Roll = DamageRoll;

    fn roll(&self) -> Self::Roll {
        DamageRoll::new(self.iter().map(|part| part.roll()).collect())
    }
}

impl Rollable for DamagePart {
    type Roll = damageroll::DamageRollPart;

    fn roll(&self) -> Self::Roll {
        match self {
            DamagePart::Dice(num, sides) => {
                let rolls: Vec<Score> = (0..*num)
                    .map(|_| rand::thread_rng().gen_range(1, sides.abs() + 1))
                    .collect();

                DamageRollPart::Dice(*sides, rolls)
            }

            DamagePart::Modifier(value) => DamageRollPart::Modifier(*value),
        }
    }
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
            Err(Box::new(ParseError::new("Invalid Expression")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_DamagePart {
        use super::*;
        #[test]
        fn simple_damage() {
            let d: DamagePart = "2d8".parse().unwrap();
            assert_eq!(d, DamagePart::Dice(2, 8));

            let d: DamagePart = "1d12".parse().unwrap();
            assert_eq!(d, DamagePart::Dice(1, 12));

            let d: DamagePart = "3d6".parse().unwrap();
            assert_eq!(d, DamagePart::Dice(3, 6));

            let d: DamagePart = "421d314159".parse().unwrap();
            assert_eq!(d, DamagePart::Dice(421, 314159));
        }

        #[test]
        fn negative_damage() {
            let d: DamagePart = "-2d8".parse().unwrap();
            assert_eq!(d, DamagePart::Dice(2, -8));

            let d: DamagePart = "-1d12".parse().unwrap();
            assert_eq!(d, DamagePart::Dice(1, -12));

            let d: DamagePart = "-3d6".parse().unwrap();
            assert_eq!(d, DamagePart::Dice(3, -6));

            let d: DamagePart = "-421d314159".parse().unwrap();
            assert_eq!(d, DamagePart::Dice(421, -314159));
        }

        #[test]
        fn modifier() {
            let d: DamagePart = "4".parse().unwrap();
            assert_eq!(d, DamagePart::Modifier(4));

            let d: DamagePart = "-3".parse().unwrap();
            assert_eq!(d, DamagePart::Modifier(-3));

            let d: DamagePart = "26".parse().unwrap();
            assert_eq!(d, DamagePart::Modifier(26));

            let d: DamagePart = "-129".parse().unwrap();
            assert_eq!(d, DamagePart::Modifier(-129));
        }

    }

    mod parse_Damage {}
}
