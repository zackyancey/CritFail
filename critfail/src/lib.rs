//! Evaluation and rolling of D&D 5e check and damage rolls.
//!
//! This crate provides methods for parsing and rolling dice expressions
//! and for handling the results.
#[macro_use]
extern crate lazy_static;

use std::fmt;

mod attack;
mod check;
mod damage;
mod error;
mod roll;
mod util;

pub use roll::{Roll, RollOutcome};
pub use check::{Check, CheckOutcome, AdvState, CritScore};
pub use damage::{Damage, DamageOutcome};
pub use attack::{Attack, AttackOutcome};
pub use error::ParseError;

// TODO: Replace DamageOutcomePart with a publically-exposable ModifierPart type
use damage::DamageOutcomePart;
pub type OutcomePart = DamageOutcomePart;

pub type Score = i32;
pub type Sides = i32;

/// Used for structs defining a set of dice that can be rolled.
pub trait RollExpression: Sized {
    /// The roll result type should implement both Display and Debug.
    /// Display should print out a consise result for the roll, and
    /// Debug should print out the details (eg the value for each rolled
    /// die).
    type Outcome: fmt::Display + fmt::Debug;

    /// Parse a string defining this roll expression.
    fn new(expression: &str) -> Result<Self, ()>;

    /// Roll the dice and return an outcome.
    fn roll(&self) -> Self::Outcome;

    // TODO: Add an error type for parsing rollexps
    // TODO: Add with_options() and builder() methods.
}
