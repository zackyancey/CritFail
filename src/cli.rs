use critfail::{RollExp, Rollable};
use std::error::Error;

pub fn run_args(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.iter().any(|s| s == "-h" || s == "--help") {
        print_version();
        print_usage();
    } else if args.iter().any(|s| s == "-V" || s == "--version") {
        print_version();
    } else {
        make_roll(&args[1])?
    }

    Ok(())
}

fn make_roll(roll: &str) -> Result<(), Box<dyn Error>> {
    let d = roll.parse::<RollExp>()?;
    let result = d.roll();
    println!("{:?}\n{}", result, result);

    Ok(())
}

fn print_usage() {
    println!(
        "Usage:
    critfail [roll expression]
        roll the expression and show the result
Options:
    -v, --version  Show version info
    -h, --help     Show this help message

Roll Expressions:
    There are 3 kinds of roll expressions: Checks, damage, and attacks.

    r+6 : A check. Roll a d20 and add 6.
    2d6+4 : Damage roll. Rolls 2d8 and adds 4.
    r+3?1d12+3: An attack. Rolls to hit and for damage. Damage is
                automatically double-rolled for a critical hit.

    Checks:
    Roll a d20 with a modifier.
    `r` : Roll a d20.
    `r+5` : Roll a d20 and add 5 to the value.

    Checks can have advantage/disadvantage, and any number of constant
    or dice modifiers can be added.
    `a+5` : roll a d20 with advantage then add 5.
    `d+4` : roll a d20 with disadvantage then add 4.
    `d+4+1d4` : roll a d20 with disadvantage, then add 4.+1d4

    Damage:
    Roll multiple dice to determine damage.
    `2d8+5`
    `2d8-1d4+7-2`

    Attacks:
    An attack consts of both a check and a damage roll, separated by a `?`.
    `r+4?1d8`
    `a+5?1d4+4+5d6`

    If the check part of an attack rolls a 20, all of the positive dice in
    the damage part of the roll will be rolled twice. (Modifiers will only
    be counted once)."
    )
}

fn print_version() {
    println!("Critfail v{}-{}", crate::VERSION, crate::GIT_VERSION);
}
