#[macro_use]
extern crate nom;

extern crate clap;

use clap::{App, Arg, SubCommand};

pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
