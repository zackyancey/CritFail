use rand::Rng;

use crate::Rollable;
use crate::{Score, Sides};

mod damageparse;
mod damageroll;

pub use damageroll::*;

#[derive(PartialEq, Debug)]
pub enum DamagePart {
    Dice(u32, Sides),
    Modifier(Score),
}

#[derive(Debug, PartialEq)]
pub struct Damage(pub Vec<DamagePart>);

impl Damage {
    pub fn crit_roll(&self)->DamageRoll {
        let mut result = Vec::new();

        for part in &self.0 {
            match part {
                DamagePart::Dice(_, sides) if *sides > 0 => {
                    result.push(part.roll());
                    result.push(part.roll());
                },
                _ => {result.push(part.roll());}
            }
        }

        DamageRoll::new(result)
    }
}

impl Rollable for Damage {
    type Roll = DamageRoll;

    fn roll(&self) -> Self::Roll {
        DamageRoll::new(self.0.iter().map(|part| part.roll()).collect())
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
