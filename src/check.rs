use std::fmt;

use crate::{Damage, Score};
use crate::{Rollable, ScoreRoll};

pub use checkroll::*;

mod checkparse;
mod checkroll;

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
