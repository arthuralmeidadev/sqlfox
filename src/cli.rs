use clap::{Args, Parser, Subcommand, ValueEnum};

fn parse_csv_vec(s: &str) -> anyhow::Result<Vec<String>> {
    if s.is_empty() {
        return anyhow::Ok(Vec::new());
    }

    anyhow::Ok(s.split(',').map(|s| s.to_string()).collect())
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Dialect {
    Postgres,
}

#[derive(Args, Debug)]
pub struct GenerateCommandArgs {
    #[arg(long, value_enum)]
    pub dialect: Dialect,

    #[arg(long)]
    pub connection: String,

    #[arg(long)]
    pub schema: String,

    #[arg(long, value_parser = parse_csv_vec)]
    pub table: Vec<String>,

    #[arg(long, value_parser = parse_csv_vec)]
    pub format: Vec<String>,
}

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
