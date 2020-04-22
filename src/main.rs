mod cli;
mod gui;

use git_version::git_version;
use std::{env, error::Error};

const GIT_VERSION: &str = git_version!();
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        cli::run_args(args)?;
    } else {
        gui::run();
    }

    Ok(())
}
