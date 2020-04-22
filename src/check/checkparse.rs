use std::error::Error;
use std::str::FromStr;

use crate::Check;

impl FromStr for Check {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}
