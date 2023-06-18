use clap::{
    error::{Error, ErrorKind}, 
    ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand
};

#[derive(Parser, Debug)]
pub(crate) struct InitArgs {
    /// Your project name
    #[arg(short, long)]
    name: String,
}
#[derive(Parser, Debug)]
pub(crate) struct RemoveArgs {
    #[arg(short, long)]
    name: Vec<String>,
}

#[derive(Debug)]
pub(crate) enum ArgSub {
    /// Initiate Your project name
    Init(InitArgs),
    Remove(RemoveArgs),
}

impl FromArgMatches for ArgSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("init", args)) => Ok(Self::Init(InitArgs::from_arg_matches(args)?)),
            Some(("remove", args)) => Ok(Self::Remove(RemoveArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::InvalidSubcommand,
                "Valid subcommands are `init` and `remove`",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `init` and `remove`",
            )),
        }
    }
    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("init", args)) => *self = Self::Init(InitArgs::from_arg_matches(args)?),
            Some(("remove", args)) => *self = Self::Remove(RemoveArgs::from_arg_matches(args)?),
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "Valid subcommands are `init` and `remove`",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl Subcommand for ArgSub {
    fn augment_subcommands(cmd: Command) -> Command {
        cmd.subcommand(InitArgs::augment_args(Command::new("init")))
            .subcommand(RemoveArgs::augment_args(Command::new("remove")))
            .subcommand_required(true)
    }
    fn augment_subcommands_for_update(cmd: Command) -> Command {
        cmd.subcommand(InitArgs::augment_args(Command::new("init")))
            .subcommand(RemoveArgs::augment_args(Command::new("remove")))
            .subcommand_required(true)
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(name, "init" | "remove")
    }
}

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[command(subcommand)]
    subcommand: ArgSub,
}
