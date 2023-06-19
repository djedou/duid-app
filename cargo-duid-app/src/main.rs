mod generate_template;
mod args;

use clap::Parser;
//use std::process::Command;
//use clap::{arg, builder::PossibleValue, command, Command, value_parser, ValueEnum};


pub(crate) use self::args::*;
pub(crate) use self::generate_template::*;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.subcommand {
        ArgSub::Init(InitArgs{name}) => {
            println!("init: {:#?}", name);
            generate_template(&name);
        },
        _ => {
            println!("not yet implemented!");
        }
    };

    Ok(())
}
