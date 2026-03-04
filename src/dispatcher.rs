use crate::cli::{Cli as Args, Command};

pub fn handle_command(args: Args) {
    match args.command {
        Command::Generate(args) => {}
    }
}
