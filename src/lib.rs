use std::fmt;

pub use attack::*;
pub use check::*;
pub use damage::*;
pub use rollexp::*;

mod attack;
mod check;
mod damage;
mod rollexp;
mod util;

pub type Score = i32;
pub type Sides = i32;

/// Represents an object that defines a set of dice that can be rolled
/// to get a result.
pub trait Rollable {
    /// The roll result type should implement both Display and Debug.
    /// Display should print out a consise result for the roll, and
    /// Debug should print out the details (eg the value for each rolled
    /// die)
    type Roll: fmt::Display + fmt::Debug;

    fn roll(&self) -> Self::Roll;
}

/// Trait for a roll that can be boiled down to a single numerical
/// score.
pub trait ScoreRoll {
    fn score(&self) -> Score;
}
