use rand::Rng;

mod damage;
pub use damage::*;
mod check;
pub use check::*;
mod attack;
pub use attack::*;
mod rollexp;
pub use rollexp::*;

pub type Score = i32;

/// Represents an object that defines a set of dice that can be rolled
/// to get a result.
pub trait Rollable {
    type Roll: DisplayRoll;

    fn roll(&self) -> Self::Roll;
}

/// Trait for a roll that can be boiled down to a single numerical
/// score.
pub trait ScoreRoll {
    fn score(&self) -> Score;
}

/// All rolls should implement this trait, which is used to display the
/// result
pub trait DisplayRoll {
    // TODO: Document these functions
    // TODO: Maybe just replace this with Display/Debug? Or auto-implement Display/Debug
    fn display_score(&self) -> String;
    fn display_parts(&self) -> String;
}
