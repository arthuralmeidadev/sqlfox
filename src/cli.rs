use clap::{Args, Parser, Subcommand};

#[derive(Args, Debug)]
pub struct GenerateCommandArgs {}

#[derive(Subcommand, Debug)]
pub enum Command {
    Generate(GenerateCommandArgs),
}

#[derive(Parser, Debug)]
#[command(author="rthi_dev - Arthur Almeida", name = "Sqlfox", version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
