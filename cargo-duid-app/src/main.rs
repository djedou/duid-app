mod generate_template;
mod args;

use clap::Parser;
//use std::process::Command;
//use clap::{arg, builder::PossibleValue, command, Command, value_parser, ValueEnum};


pub(crate) use self::args::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("args: {:#?}", args);
    Ok(())
}
