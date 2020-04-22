use crate::Score;
use crate::{Rollable, ScoreRoll, DisplayRoll};

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
    Modifier(Score)
}

pub struct DamageRoll {
    scores: DamageResultPart
}

impl ScoreRoll for DamageRoll {
    fn score(&self) -> Score {
        unimplemented!()
    }
}

impl DisplayRoll for DamageRoll {
    fn display_score(&self) -> String {
        unimplemented!()
    }

    fn display_parts(&self) -> String {
        unimplemented!()
    }
}
