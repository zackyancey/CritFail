use crate::{Damage, Score};
use crate::{Rollable, ScoreRoll, DisplayRoll};


/// The advantage state of an ability check
#[derive(PartialEq, Debug)]
pub enum AdvState {
    Advantage,
    Neutral,
    Disadvantage,
}

#[derive(PartialEq, Debug)]
pub struct Check {
    pub adv: AdvState,
    pub modifier: Damage,
}

impl Rollable for Check {
    type Roll = CheckRoll;

    fn roll(&self) -> Self::Roll {
        unimplemented!()
    }
}

pub struct CheckRoll {
    score: Score,
    other: Option<Score>
}

impl ScoreRoll for CheckRoll {
    fn score(&self) -> Score {
        unimplemented!()
    }
}

impl DisplayRoll for CheckRoll {
    fn display_score(&self) -> String {
        unimplemented!()
    }

    fn display_parts(&self) -> String {
        unimplemented!()
    }
}
