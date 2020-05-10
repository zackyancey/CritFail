# Critfail

CritFail is a dice simulator designed for use with D&D (specifically
5e). It can handle ability checks, damage rolls, and
advantage/disadvantage. Attacks (check and damage) can be rolled in one
expression, and critical hit damage is calculated accordingly.

Critfail is available as a desktop application, a cli program, or a web
app. The web app can be accessed
[here](http://apps.zackyancey.com/critfail).

The business logic for Critfail is also available as a rust library on
crates.io. See the [api docs](http://docs.rs/critfail) for more info on
using the library.

## Installation

The desktop version of critfail is a single executable that can be
downloaded from the
[releases](https://github.com/zackyancey/CritFail/releases) page and run
directly. The same executable is used for the cli interface—just put it
somewhere on your path.

## Usage

Whatever interface you are using, rolls are made using roll expressions.
There are 3 kinds of roll expressions: Checks, damage, and attacks.

* `r+6` : A check. Roll a d20 and add 6.
* `2d6+4` : Damage roll. Rolls 2d8 and adds 4.
* `r+3?1d12+3`: An attack. Rolls to hit and for damage. Damage is automatically double-rolled for a critical hit.

### Checks
Roll a d20 with a modifier.  Checks can have advantage/disadvantage, and
any number of constant or dice modifiers can be added.
* `r` : Roll a d20.
* `r+5` : Roll a d20 and add 5 to the value.
* `a+5` : roll a d20 with advantage then add 5.
* `d+4` : roll a d20 with disadvantage then add 4.
* `d+4+1d4` : roll a d20 with disadvantage, then add 4.+1d4

### Damage
Roll multiple dice to determine damage. Damage rolls use the usual dice notation.
* `2d8+5`
* `2d8-1d4+7-2`

### Attacks
An attack consts of both a check and a damage roll, separated by a `?`.
If the check part of an attack rolls a 20, all of the positive dice in
the damage part of the roll will be rolled twice. (Modifiers will only
be counted once)."
* `r+4?1d8`
* `a+5?1d4+4+5d6`

## Building

### Desktop

To build for desktop, run `cargo build --release`

### Web

Building for web is handled using the
[Just](https://github.com/casey/just) command runner tool. To build the
web application, run `just build` from the project root. The `web`
folder will contain the build output. Run `web-serve` to start a server
running the web app.

Building for web requires the following dependencies:
* [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) available on the path
* [binaryen](https://github.com/WebAssembly/binaryen) installed and available on path (specifically the `wasm-opt` binary)
* To run the local server: python 3 installed
