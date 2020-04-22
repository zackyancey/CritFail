use crate::{Damage, Score};

/// The advantage state of an ability check
#[derive(PartialEq, Debug)]
pub enum AdvState {
    Advantage,
    Neutral,
    Disadvantage,
}

#[derive(PartialEq, Debug)]
pub struct Check {
    // TODO: impl Rollable
    pub adv: AdvState,
    pub modifier: Damage,
}

pub struct CheckRoll {
    // TODO: impl ScoreRoll
    // TODO: impl DisplayRoll
    score: Score,
    other: Option<Score>
}
