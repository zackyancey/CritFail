use std::fmt;

use crate::Score;
use crate::{Rollable, ScoreRoll};

#[derive(PartialEq, Debug)]
pub enum DamagePart {
    Dice(i32, u32),
    Modifier(Score),
}

pub type Damage = Vec<DamagePart>;

impl Rollable for Damage {
    type Roll = DamageRoll;

    fn roll(&self) -> Self::Roll {
        unimplemented!()
    }
}

enum DamageResultPart {
    Dice(Vec<Score>),
    Modifier(Score),
}

pub struct DamageRoll {
    scores: DamageResultPart,
}

impl ScoreRoll for DamageRoll {
    fn score(&self) -> Score {
        unimplemented!()
    }
}

impl fmt::Display for DamageRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for DamageRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
