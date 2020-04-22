use std::fmt;

use crate::{Damage, Score};
use crate::{Rollable, ScoreRoll};

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
    other: Option<Score>,
}

impl ScoreRoll for CheckRoll {
    fn score(&self) -> Score {
        unimplemented!()
    }
}

impl fmt::Display for CheckRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for CheckRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
