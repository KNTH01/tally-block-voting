mod input_generator;
mod tally;
mod voting;

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::{input_generator::generate_input, tally::process_tally};

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

    /// Input (Generate) or Output (Tally) file
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate => {
            generate_input(cli.file);
        }
        Command::Tally => {
            process_tally(cli.file);
            println!("tally!");
        }
    }
}
