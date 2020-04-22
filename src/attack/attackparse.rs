use std::error::Error;
use std::str::FromStr;

use crate::Attack;

impl FromStr for Attack {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}
