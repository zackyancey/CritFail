#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "gui")]
mod gui;

use git_version::git_version;
use std::{env, error::Error};

const GIT_VERSION: &str = git_version!();
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "cli")]
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            cli::run_args(args)?;
            return Ok(());
        }
    }

    #[cfg(feature = "gui")]
    gui::run();

    Ok(())
}
