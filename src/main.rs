mod data_generator;
mod tally;
mod voting;

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::{data_generator::generate_input, tally::process_tally};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
    /// Generates random input data for the purpose of tallying
    Generate,

    /// Process tally block voting
    Tally,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Please select a command
    #[arg(value_enum)]
    command: Command,

    /// Input (Generate)
    #[arg(short, long)]
    input: PathBuf,

    /// Output (Tally)
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate => {
            generate_input(cli.input);
        }
        Command::Tally => {
            process_tally(cli.input, cli.output);
            println!("tally!");
        }
    }
}
