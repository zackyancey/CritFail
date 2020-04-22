use rand::Rng;

use crate::Damage;
use crate::Rollable;

pub use checkroll::*;

mod checkparse;
mod checkroll;

/// The advantage state of an ability check
#[derive(PartialEq, Debug, Clone)]
pub enum AdvState {
    Advantage,
    Neutral,
    Disadvantage,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Check {
    pub adv: AdvState,
    pub modifier: Damage,
}

impl Rollable for Check {
    type Roll = CheckRoll;

    fn roll(&self) -> Self::Roll {
        let r1 = rand::thread_rng().gen_range(1, 21);
        let r2 = rand::thread_rng().gen_range(1, 21);
        let mods = self.modifier.roll();
        CheckRoll::new(&self.adv, r1, r2, mods)
    }
}
