use std::fmt;

use crate::Score;
use crate::ScoreRoll;

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
