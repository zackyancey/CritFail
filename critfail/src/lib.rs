//! Evaluation and rolling of D&D 5e die roll expressions.
//!
//! The `RollExpression` trait provides methods for dealing with the
//! various kinds of rolls. `Roll` provides the simplest text-in,
//! text-out interface for rolling expressions an printing the result
//! regardless of the type of roll.
//!
//! ```
//! use critfail::{RollExpression, Roll};
//!
//! let check = Roll::new("r-3").unwrap();
//! let check_outcome = check.roll();
//! print!("{}", check_outcome); // eg. "11"
//! print!("{:?}", check_outcome); // eg. "(14)-3"
//!
//! let damage = Roll::new("2d8+6").unwrap();
//! let damage_outcome = damage.roll();
//! print!("{}", damage_outcome); // eg. "13"
//! print!("{:?}", damage_outcome); // eg. "[2+5]+6"
//!
//! let attack = Roll::new("r+1?2d6+4").unwrap();
//! let attack_outcome = attack.roll();
//! print!("{}", attack_outcome); // eg. "10 ? 16"
//! print!("{:?}", attack_outcome); // eg. "(9)+1 ? [6+6]+4"
//! ```
//!
//! In order to handle the outcome of a `Roll` programatically, roll
//! expressions are split into `Check` rolls, `Damage` rolls, and
//! `Attack` rolls, each with their own outcome type which provides
//! methods for determining the score and makeup of the results for
//! each.
//!
//! # Features
//! * `wasm-bindgen`: Enable this when compiling for wasm32 targets, or
//!   random number generation won't work.
//! * `build-outcomes`: Adds the `build` method to outcome types, which
//!   lets you create an outcome with chosen results. This is used in
//!   the examples and tests, but the interface may change in the future
//!   and currently allows you to create impossible results (rolling a
//!   21 on a d20, etc.) that could result in unpredictable behavior. Be
//!   aware of this before enabling this feature.
#![warn(missing_docs)]
#[macro_use]
extern crate lazy_static;

use std::fmt;

mod attack;
mod check;
mod damage;
mod error;
mod modifier;
mod roll;
mod util;

pub use attack::{Attack, AttackOutcome};
pub use check::{AdvState, Check, CheckOutcome, CritScore};
pub use damage::{Damage, DamageOutcome};
pub use error::ParseError;
pub(crate) use modifier::ModifiersOutcome;
pub use modifier::OutcomePart;

pub use roll::{Roll, RollOutcome};

/// The number type that is used when reporting the score of a roll
pub type Score = i32;
/// The number type that is used for specifying the number of sides on a
/// die
pub type Sides = i32;

/// Used for structs defining a set of dice that can be rolled.
pub trait RollExpression: Sized {
    /// The roll result type should implement both Display and Debug.
    /// Display should print out a consise result for the roll, and
    /// Debug should print out the details (eg the value for each rolled
    /// die).
    type Outcome: fmt::Display + fmt::Debug;

    /// Create a new roll expression by parsing the given string.
    fn new(expression: &str) -> Result<Self, ()>;

    /// Roll the dice and return an outcome.
    fn roll(&self) -> Self::Outcome;

    // TODO: Add an error type for parsing rollexps
    // TODO: Add with_options() and builder() methods.
}
