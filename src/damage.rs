use crate::Score;
use crate::Rollable;

#[derive(PartialEq, Debug)]
pub enum DamagePart {
    Dice(i32, u32),
    Modifier(Score),
}

// TODO: Impl Rollable
pub type Damage = Vec<DamagePart>;

enum DamageResultPart {
    Dice(Vec<Score>),
    Modifier(Score)
}

pub struct DamageRoll {
    // TODO: impl ScoreRoll
    // TODO: impl DisplayRoll
    scores: DamageResultPart
}
